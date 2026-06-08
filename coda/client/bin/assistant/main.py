# #region 📊Header

# 2026 Ueli Saluz <ueli@semio-tech.de>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# #endregion 📊Header

# #region 🔌Adapters
# Third-party imports MUST stay in this region (FastMCP, rdflib, owlready2, starlette, uvicorn).

"""coda - ACC design assistant. Runs as MCP server or Electron sidecar binary."""

from __future__ import annotations


import argparse
import collections
import contextlib
import json
import logging
import os
import re
import subprocess
import sys
import time
import typing
from typing import Tuple, List, Dict, Any
from dataclasses import dataclass, field
import uuid
import weakref
from pathlib import Path

import rdflib
import uvicorn
from mcp.server.fastmcp import Context, FastMCP
from mcp.types import CallToolResult, EmbeddedResource, TextContent, TextResourceContents
from pydantic import BaseModel, Field
from owlready2 import get_ontology, sync_reasoner_pellet
from starlette.applications import Starlette
from starlette.responses import HTMLResponse, JSONResponse
from starlette.routing import Route

mcp = FastMCP("coda", json_response=True)

_CODA_ROOT = Path(__file__).resolve().parent
_CODA_JSON_PATH = _CODA_ROOT / "coda.json"
_PROJECT_ENV = "CODA_PROJECT"
_PROPERTY_KIND_MEASURE_KINDS = {
    "number": ["increase", "decrease"],
    "object": ["add", "remove"],
    "array": ["append", "remove_item"],
    "level": ["raise", "lower"],
    "category": ["include", "exclude"],
}

# #endregion 🔌Adapters

# #region CodaMcpAppRuntime
# MCP App HTTP helpers share the streamable-http port; stdio mode omits fetchUrl on tool payloads.

CODA_HTTP_PORT = int(os.environ.get("CODA_HTTP_PORT", "8080"))
_HTTP_PAYLOADS_ENABLED = False
_SIDECAR_MODE = False

_mcp_app_payloads: collections.OrderedDict[str, dict[str, typing.Any]] = collections.OrderedDict()
_MCP_APP_PAYLOADS_MAX_SIZE = 100

_WORKSPACE_APP_URI = "ui://coda/workspace"
_WORKSPACE_APP_META: dict[str, typing.Any] = {
    "ui": {"resourceUri": _WORKSPACE_APP_URI},
    "ui/resourceUri": _WORKSPACE_APP_URI,
}


def _emit_event(event: str, data: dict[str, typing.Any]) -> None:
    """Emit a sidecar event line when running under Electron stdio (ignored in MCP-only processes)."""
    if not _SIDECAR_MODE:
        return
    _write_stdout(
        {"id": None, "event": event, "data": data, "timestamp": time.time()}
    )


def _mcp_app_html_resource_meta() -> dict[str, typing.Any]:
    """Resource _meta for MCP App HTML: hosts apply _meta.ui.csp to the sandbox."""
    origins = [
        f"http://127.0.0.1:{CODA_HTTP_PORT}",
        f"http://localhost:{CODA_HTTP_PORT}",
        f"http://[::1]:{CODA_HTTP_PORT}",
        f"ws://127.0.0.1:{CODA_HTTP_PORT}",
        f"ws://localhost:{CODA_HTTP_PORT}",
    ]
    csp = {"connectDomains": origins, "resourceDomains": origins}
    return {"ui": {"csp": csp}, "ui/csp": csp}


def _build_mcp_app_html(*, panel: str = "dashboard") -> str:
    """Load the single-file MCP App HTML bundle; swap initial panel for the workspace shell."""
    app_html_path = Path(__file__).resolve().parent / "dist" / "mcp-app.html"
    if app_html_path.is_file():
        html = app_html_path.read_text(encoding="utf-8")
    else:
        return """<!doctype html><html><body><p>MCP App not built. Run: npm run build:mcp-app in coda/assistant</p></body></html>"""
    if "data-coda-panel=" in html:
        html = re.sub(
            r'data-coda-panel="[^"]*"',
            f'data-coda-panel="{panel}"',
            html,
            count=1,
        )
    return html


def _mcp_app_csp_value() -> str:
    """CSP header matching semio engine MCP apps (iframe-friendly, allows wasm for elements/ui)."""
    return (
        "default-src 'self' 'unsafe-inline'; "
        "script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; "
        "frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;"
    )


def _as_mcp_app_tool_result(
    payload: dict[str, typing.Any], *, is_error: bool = False
) -> CallToolResult:
    """Return tools/call payload with optional fetchUrl when HTTP server is up."""
    token = uuid.uuid4().hex
    _mcp_app_payloads[token] = payload
    while len(_mcp_app_payloads) > _MCP_APP_PAYLOADS_MAX_SIZE:
        _mcp_app_payloads.popitem(last=False)
    fetch_url: str | None = None
    if _HTTP_PAYLOADS_ENABLED:
        fetch_url = f"http://127.0.0.1:{CODA_HTTP_PORT}/app/payload/{token}"
        payload = {**payload, "fetchUrl": fetch_url}
    text = json.dumps(payload)
    hint: dict[str, typing.Any] = {"panel": payload.get("panel"), "kind": payload.get("kind")}
    if fetch_url:
        hint["fetchUrl"] = fetch_url
    return CallToolResult(
        content=[
            TextContent(type="text", text=json.dumps(hint)),
            TextContent(type="text", text=text),
            EmbeddedResource(
                type="resource",
                resource=TextResourceContents(
                    uri="coda://mcp-app/tool-payload",
                    mimeType="application/json",
                    text=text,
                ),
            ),
        ],
        structuredContent=payload,
        isError=is_error,
    )


# #endregion CodaMcpAppRuntime

# #region 🎢Helpers
# Helpers MUST provide private functions for config loading and project root resolution.


def _load_json_with_comments(path: Path) -> dict:
    """💬Load JSON file, stripping // line comments.
    _load_json_with_comments MUST strip full-line // comments without corrupting URLs inside strings.
    """
    text = path.read_text(encoding="utf-8")
    text = re.sub(r"^\s*//.*$", "", text, flags=re.MULTILINE)
    return json.loads(text)


def _get_project_root() -> Path | None:
    """🌱Resolve project root from CODA_PROJECT or cwd.
    _get_project_root MUST perform the _get_project_root operation.
    """
    if val := os.environ.get(_PROJECT_ENV):
        p = Path(val).resolve()
        return p if (p / ".coda" / "project.json").exists() else None
    cwd = Path.cwd()
    for d in [cwd, *cwd.parents]:
        if (d / ".coda" / "project.json").exists():
            return d
    return None


def _get_coda_config() -> dict:
    """⚙️_get_coda_config performs the _get_coda_config operation.
    _get_coda_config MUST perform the _get_coda_config operation.
    """
    config_path = Path(os.environ.get("CODA_CONFIG", _CODA_JSON_PATH))
    if not config_path.is_absolute():
        config_path = _CODA_ROOT / config_path
    return _load_json_with_comments(config_path)


def _canonicalize_property_kind(raw_kind: str | None) -> str:
    """🏷️Map legacy property kind values to canonical coda kinds.
    _canonicalize_property_kind MUST map to one of: number, object, array, level, category.
    """
    if not raw_kind:
        return "object"
    kind = str(raw_kind).strip().lower()
    _ALIASES = {"collection": "array"}
    kind = _ALIASES.get(kind, kind)
    if kind in _PROPERTY_KIND_MEASURE_KINDS:
        return kind
    return "object"


def _normalize_property_definition(property_definition: dict) -> dict:
    """📖Normalize a property definition to the canonical coda property kind system.
    _normalize_property_definition MUST expose canonical kind and kind-specific measure_kinds.
    """
    normalized = dict(property_definition)
    kind = _canonicalize_property_kind(normalized.get("kind") or normalized.get("type"))
    normalized["kind"] = kind
    normalized["measure_kinds"] = normalized.get("measure_kinds") or list(
        _PROPERTY_KIND_MEASURE_KINDS[kind]
    )

    # Property schema now uses kind as the canonical key.
    normalized.pop("type", None)

    if kind == "object":
        properties = normalized.get("properties", [])
        if isinstance(properties, list):
            normalized["properties"] = [
                _normalize_property_definition(p) if isinstance(p, dict) else p
                for p in properties
            ]

    if kind == "array":
        items = normalized.get("items")
        if isinstance(items, dict):
            normalized["items"] = _normalize_property_definition(items)

    if kind == "level":
        levels = normalized.get("levels", [])
        if isinstance(levels, list):
            normalized_levels = []
            for level in levels:
                if not isinstance(level, dict):
                    normalized_levels.append(level)
                    continue
                normalized_level = dict(level)
                measures = normalized_level.get("measures")
                if isinstance(measures, dict):
                    if "higher" in measures and "raise" not in measures:
                        measures["raise"] = measures.pop("higher")
                instructions = normalized_level.get("instructions")
                if isinstance(instructions, dict):
                    if "higher" in instructions and "raise" not in instructions:
                        instructions["raise"] = instructions.pop("higher")
                normalized_level["measure_kinds"] = ["raise", "lower"]
                normalized_levels.append(normalized_level)
            normalized["levels"] = normalized_levels

    return normalized


def _normalize_target_definition(target_definition: dict) -> dict:
    """🎛️Normalize target property definitions to canonical coda property kinds.
    _normalize_target_definition MUST normalize all target properties recursively.
    """
    normalized = dict(target_definition)
    properties = normalized.get("properties", [])
    if isinstance(properties, list):
        normalized["properties"] = [
            _normalize_property_definition(p) if isinstance(p, dict) else p
            for p in properties
        ]
    return normalized


def _get_project_config() -> dict | None:
    """📋_get_project_config performs the _get_project_config operation.
    _get_project_config MUST perform the _get_project_config operation.
    """
    root = _get_project_root()
    if not root:
        return None
    path = root / ".coda" / "project.json"
    return json.loads(path.read_text(encoding="utf-8")) if path.exists() else None


def _ensure_validation_envelope(raw: str) -> dict:
    """🔷Normalize arbitrary validator output into the canonical validation envelope."""
    text = raw if isinstance(raw, str) else str(raw)
    try:
        obj = json.loads(text)
    except Exception:
        truth = "unknown"
        return {
            "valid": None,
            "validations": [
                {
                    "instance": "validation",
                    "expression": "",
                    "truth": truth,
                    "tree": {
                        "id": "root",
                        "kind": "DataValue",
                        "label": "validation output",
                        "fragment": None,
                        "truth": truth,
                        "summary": "Validator returned plain text output.",
                        "value": text.strip(),
                        "datatype": "text/plain",
                        "children": [],
                    },
                }
            ],
        }

    if (
        isinstance(obj, dict)
        and "validations" in obj
        and isinstance(obj["validations"], list)
    ):
        return obj

    breachs = []
    if isinstance(obj, dict):
        breachs = obj.get("breachs") or obj.get("breaches") or []
    has_breaches = isinstance(breachs, list) and len(breachs) > 0

    truth = "false" if has_breaches else "unknown"
    valid_overall = not has_breaches

    return {
        "valid": valid_overall,
        "validations": [
            {
                "instance": "validation",
                "expression": "",
                "truth": truth,
                "tree": {
                    "id": "root",
                    "kind": "DataValue",
                    "label": "legacy report",
                    "fragment": None,
                    "truth": truth,
                    "summary": "Legacy validator report wrapped as a DataValue node.",
                    "value": json.dumps(obj, indent=2),
                    "datatype": "application/json",
                    "children": [],
                },
            }
        ],
    }


def _run_ontology_validator(
    target_id: str, translation_path: Path, validator_cfg: dict
) -> dict:
    """Run the default ontology-based validator for a target translation.
    _run_ontology_validator MUST return an envelope with valid and validations fields.
    """
    root = _get_project_root()
    if not root:
        raise RuntimeError("No project root for ontology validator")

    # Resolve ontology path: validator-specific override or default ontology in engine folder.
    ontology_path_str = validator_cfg.get("ontology_path")
    if ontology_path_str:
        ontology_path = Path(ontology_path_str)
        if not ontology_path.is_absolute():
            ontology_path = (
                Path(__file__).resolve().parent / ontology_path_str
            ).resolve()
    else:
        ontology_path = Path(__file__).resolve().parent / "ontology.owl"

    if not ontology_path.exists():
        raise FileNotFoundError(f"Ontology file not found: {ontology_path}")

    # Merge data (translation) and ontology into a temporary ontology file.
    engine_root = Path(__file__).resolve().parent
    temp_dir = engine_root / "temp"
    temp_dir.mkdir(parents=True, exist_ok=True)
    temp_merged = temp_dir / f"{target_id}-merged.owl"

    data_graph = rdflib.Graph().parse(str(translation_path))
    rule_graph = rdflib.Graph().parse(str(ontology_path))
    merged_graph = data_graph + rule_graph
    merged_graph.serialize(str(temp_merged), format="xml")

    onto = get_ontology(temp_merged.as_uri()).load()

    try:
        with onto:
            sync_reasoner_pellet(
                infer_property_values=True, infer_data_property_values=True, debug=0
            )

        # Find NotCompliant class and its instances as failing validations.
        not_compliant_cls = None
        for cl in onto.classes():
            if cl.name == "NotCompliant":
                not_compliant_cls = cl
                break

        validations: list[dict] = []
        if not_compliant_cls is not None:
            for ind in not_compliant_cls.instances():
                instance_name = ind.name
                validations.append(
                    {
                        "instance": instance_name,
                        "expression": validator_cfg.get("expression") or "NotCompliant",
                        "truth": "false",
                        "tree": {
                            "id": f"{instance_name}-root",
                            "kind": "ClassAssertion",
                            "label": "NotCompliant",
                            "fragment": None,
                            "truth": "false",
                            "summary": "Instance is classified as NotCompliant by the ontology.",
                            "className": "NotCompliant",
                            "subject": instance_name,
                            "children": [],
                        },
                    }
                )

        valid_overall = len(validations) == 0
        return {"valid": valid_overall, "validations": validations}
    finally:
        try:
            onto.destroy(update_relation=True, update_is_a=True)
        except Exception:
            pass


def _get_latest_run(root: Path) -> Path | None:
    """🧪_get_latest_run performs the _get_latest_run operation.
    _get_latest_run MUST perform the _get_latest_run operation.
    """
    runs = root / ".coda" / "runs"
    if not runs.exists():
        return None
    dirs = sorted(d for d in runs.iterdir() if d.is_dir())
    return dirs[-1] if dirs else None


def _get_latest_iteration(run_dir: Path) -> Path | None:
    """🔶_get_latest_iteration performs the _get_latest_iteration operation.
    _get_latest_iteration MUST perform the _get_latest_iteration operation.
    """
    iters = run_dir / "iterations"
    if not iters.exists():
        return None
    dirs = sorted(
        int(d.name) for d in iters.iterdir() if d.is_dir() and d.name.isdigit()
    )
    return iters / str(dirs[-1]) if dirs else None


# #endregion 🎢Helpers

# #region 🔗Session
# Session MUST hold mutable state for the current project, run, iteration, and target.


class Session:
    """🪪Stateful session tracking the current project, run, iteration, and target.
    Shared by both MCP and sidecar modes.
    """

    def __init__(self) -> None:
        self.project_root: Path | None = None
        self.run_dir: Path | None = None
        self.iteration_dir: Path | None = None
        self.target_id: str | None = None

    def start_working_on_project(self, path: str) -> dict:
        """▶️Set the active project root. Resets run/iteration/target.
        """
        p = Path(path).resolve()
        if not (p / ".coda" / "project.json").exists():
            return {"error": f"No coda project at {p}"}
        self.project_root = p
        self.run_dir = _get_latest_run(p)
        self.iteration_dir = (
            _get_latest_iteration(self.run_dir) if self.run_dir else None
        )
        self.target_id = None
        os.environ[_PROJECT_ENV] = str(p)
        proj = json.loads((p / ".coda" / "project.json").read_text(encoding="utf-8"))
        return {
            "ok": True,
            "project_root": str(p),
            "project": proj,
            "has_run": self.run_dir is not None,
            "has_iteration": self.iteration_dir is not None,
        }

    def start_run(self) -> dict:
        """🆕Create a new run in the current project. Sets it as active run.
        """
        if not self.project_root:
            return {"error": "No project. Call start_working_on_project first."}
        from datetime import datetime, timezone

        run_id = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"
        run_dir = self.project_root / ".coda" / "runs" / run_id
        run_dir.mkdir(parents=True)
        (run_dir / "run.json").write_text(
            json.dumps({"id": run_id, "started": run_id}, indent=2), encoding="utf-8"
        )
        (run_dir / "iterations").mkdir()
        self.run_dir = run_dir
        self.iteration_dir = None
        self.target_id = None
        return {"run_id": run_id, "path": str(run_dir)}

    def start_iteration(self, run_id: str | None = None) -> dict:
        """🗃️Create a new iteration in the active or specified run. Sets it as active iteration.
        """
        if not self.project_root:
            return {"error": "No project. Call start_working_on_project first."}
        if run_id:
            rd = self.project_root / ".coda" / "runs" / run_id
            if not rd.exists():
                return {"error": f"Run not found: {run_id}"}
        else:
            rd = self.run_dir
            if not rd:
                return {"error": "No run. Call start_run first."}
        iters_dir = rd / "iterations"
        iters_dir.mkdir(parents=True, exist_ok=True)
        existing = [
            int(d.name) for d in iters_dir.iterdir() if d.is_dir() and d.name.isdigit()
        ]
        idx = max(existing, default=-1) + 1
        iter_dir = iters_dir / str(idx)
        iter_dir.mkdir()
        (iter_dir / "targets").mkdir()
        proj = _get_project_config()
        target_ids = [t.get("id") for t in (proj or {}).get("targets", [])]
        for tid in target_ids:
            (iter_dir / "targets" / tid).mkdir(parents=True, exist_ok=True)
        (iter_dir / "iteration.json").write_text(
            json.dumps({"index": idx, "targets": target_ids}, indent=2),
            encoding="utf-8",
        )
        self.run_dir = rd
        self.iteration_dir = iter_dir
        self.target_id = None
        return {
            "run_id": rd.name,
            "iteration_index": idx,
            "path": str(iter_dir),
            "targets": target_ids,
        }

    def start_translation(self, target_id: str) -> dict:
        """🔹Set the active target and prepare for translation.
        """
        if not self.project_root:
            return {"error": "No project. Call start_working_on_project first."}
        if not self.iteration_dir:
            return {"error": "No iteration. Call start_iteration first."}
        proj = _get_project_config()
        if not proj:
            return {"error": "No project config"}
        targets = proj.get("targets", [])
        if not any(t.get("id") == target_id for t in targets):
            return {
                "error": f"Target not in project: {target_id}",
                "targets": [t.get("id") for t in targets],
            }
        target_dir = self.iteration_dir / "targets" / target_id
        target_dir.mkdir(parents=True, exist_ok=True)
        self.target_id = target_id
        design_id = proj.get("design", {}).get("id", "unknown")
        design_slug = design_id.split(".")[-1] if "." in design_id else design_id
        target_slug = target_id.split(".")[-1] if "." in target_id else target_id
        agent_name = f"{design_slug}-to-{target_slug}-translation-subagent"
        return {
            "action": "invoke_subagent",
            "agent_name": agent_name,
            "target_id": target_id,
            "design_id": design_id,
            "output_path": str(target_dir / "translation.json"),
            "instruction": (
                f"Invoke the @{agent_name} subagent to translate the {design_id} "
                f"design into the {target_id} target format. "
                f"The subagent MUST write its output to {target_dir / 'translation.json'}."
            ),
        }

    def get_status(self) -> dict:
        """🔸Return current session state.
        """
        return {
            "project_root": str(self.project_root) if self.project_root else None,
            "run_dir": str(self.run_dir) if self.run_dir else None,
            "iteration_dir": str(self.iteration_dir) if self.iteration_dir else None,
            "target_id": self.target_id,
        }


# Singleton session for sidecar mode; MCP mode uses per-MCP-session dicts.
_sidecar_session = Session()

# MCP session-scoped state (keyed by session id).
_mcp_sessions: weakref.WeakKeyDictionary[typing.Any, Session] = weakref.WeakKeyDictionary()


def _mcp_session_id(ctx) -> typing.Any | None:
    """📝Get MCP session object from context."""
    return ctx.session if ctx and hasattr(ctx, "session") else None


def _get_mcp_session(ctx) -> Session:
    """🔺Get or create Session for an MCP session."""
    sid = _mcp_session_id(ctx)
    if sid is None:
        return _sidecar_session
    if sid not in _mcp_sessions:
        _mcp_sessions[sid] = Session()
    return _mcp_sessions[sid]




def _gather_workspace_payload(sess: Session, panel: str) -> dict[str, typing.Any]:
    """Shallow snapshot of coda workspace for MCP Apps (mirrors desktop views, not full on-disk dumps)."""
    config = _get_coda_config()
    project = _get_project_config()
    status = sess.get_status()
    root = sess.project_root or _get_project_root()
    measures = config.get("measures", [])
    property_kinds = config.get("property_kinds", {})
    correlation = config.get("correlation", {})
    properties = [
        _normalize_property_definition(p) if isinstance(p, dict) else p
        for p in config.get("properties", [])
    ]
    frameworks = [_normalize_target_definition(t) for t in config.get("targets", [])]
    platforms: list[typing.Any] = []
    if root:
        platforms_dir = root / ".coda" / "platforms"
        if platforms_dir.is_dir():
            for pf in platforms_dir.iterdir():
                if pf.is_file() and pf.suffix == ".json":
                    try:
                        platforms.append(json.loads(pf.read_text(encoding="utf-8")))
                    except Exception:
                        continue
    if not platforms:
        platforms = list(config.get("platforms", []) or [])

    run_summary: dict[str, typing.Any] | None = None
    iteration_summary: dict[str, typing.Any] | None = None
    report_summary: dict[str, typing.Any] | None = None
    breachs_shallow: list[typing.Any] = []
    translations: dict[str, typing.Any] = {}

    if root:
        run_dir = sess.run_dir or _get_latest_run(root)
        if run_dir and run_dir.is_dir():
            run_json = run_dir / "run.json"
            run_summary = (
                json.loads(run_json.read_text(encoding="utf-8"))
                if run_json.is_file()
                else {"id": run_dir.name}
            )
        iter_dir = sess.iteration_dir or (
            _get_latest_iteration(run_dir) if run_dir else None
        )
        if iter_dir and iter_dir.is_dir():
            iter_json = iter_dir / "iteration.json"
            iteration_summary = (
                json.loads(iter_json.read_text(encoding="utf-8"))
                if iter_json.is_file()
                else {"index": iter_dir.name}
            )
            agg = iter_dir / "targets" / "report.json"
            if agg.is_file():
                report_full = json.loads(agg.read_text(encoding="utf-8"))
                breachs = report_full.get("breachs") or []
                validations = report_full.get("validations")
                report_summary = {
                    "summary_keys": list(report_full.keys())[:24],
                    "breachs_count": len(breachs) if isinstance(breachs, list) else 0,
                    "validations_count": len(validations)
                    if isinstance(validations, list)
                    else 0,
                }
                if isinstance(breachs, list):
                    breachs_shallow = breachs[:80]
            targets_dir = iter_dir / "targets"
            if targets_dir.is_dir():
                for tdir in targets_dir.iterdir():
                    if not tdir.is_dir():
                        continue
                    tr = tdir / "translation.json"
                    translations[tdir.name] = {
                        "has_translation": tr.is_file(),
                        "path": str(tr) if tr.is_file() else None,
                    }

    return {
        "kind": "coda-workspace",
        "panel": panel,
        "session": status,
        "project": project,
        "measures": measures,
        "property_kinds": property_kinds,
        "correlation": correlation,
        "properties": properties,
        "frameworks": frameworks,
        "platforms": platforms,
        "run": run_summary,
        "iteration": iteration_summary,
        "report": report_summary,
        "breachs_shallow": breachs_shallow,
        "translations": translations,
    }


# #endregion 🔗Session

# #region 🕸️Resources
# Resources MUST expose MCP resource handlers for measures, targets, properties, rules, and project data.

_BUILDING_PLANNER_URI = "ui://building-planner/canvas"
_BUILDING_PLANNER_META: dict[str, typing.Any] = {
    "ui": {
        "resourceUri": _BUILDING_PLANNER_URI,
        "csp": {
            "connectDomains": ["https://unpkg.com", "https://cdn.jsdelivr.net"],
            "resourceDomains": ["https://unpkg.com", "https://cdn.jsdelivr.net", "https://fonts.googleapis.com", "https://fonts.gstatic.com"],
        }
    },
    "ui/resourceUri": _BUILDING_PLANNER_URI,
    "ui/csp": {
        "connectDomains": ["https://unpkg.com", "https://cdn.jsdelivr.net"],
        "resourceDomains": ["https://unpkg.com", "https://cdn.jsdelivr.net", "https://fonts.googleapis.com", "https://fonts.gstatic.com"],
    }
}

@mcp.resource(
    _BUILDING_PLANNER_URI,
    name="building planner",
    description="Interactive 3D Building Sketchpad UI for drawing building geometry.",
    mime_type="text/html;profile=mcp-app",
    meta=_BUILDING_PLANNER_META,
)
def get_building_ui() -> str:
    """🏗️Returns the Building Planner Sketchpad as a self-contained MCP App."""
    dir_path = os.path.dirname(__file__)
    
    html_path = os.path.join(dir_path, "sketchpad", "index_mcp.html")
    js_path = os.path.join(dir_path, "sketchpad", "main_mcp.js")
    rs_js_path = os.path.join(dir_path, "sketchpad-rs", "pkg", "sketchpad_rs.js")
    rs_wasm_path = os.path.join(dir_path, "sketchpad-rs", "pkg", "sketchpad_rs_bg.wasm")

    if os.path.exists(html_path) and os.path.exists(js_path) and os.path.exists(rs_js_path):
        import base64
        with open(html_path, "r", encoding="utf-8") as f:
            html = f.read()
        with open(js_path, "r", encoding="utf-8") as f:
            js = f.read()
        with open(rs_js_path, "r", encoding="utf-8") as f:
            rs_js = f.read()
            rs_js = rs_js.replace(
                "script_src = new URL(document.currentScript.src, location.href).toString();",
                "try { script_src = new URL(document.currentScript.src, location.href).toString(); } catch(e) { script_src = ''; }"
            )
        with open(rs_wasm_path, "rb") as f:
            rs_wasm_b64 = base64.b64encode(f.read()).decode("utf-8")

        inlined_rs = f'''<script>
window.__wasmLogs = [];
function initLog(msg, isErr=false) {{
    window.__wasmLogs.push({{msg: msg, err: isErr}});
    if (window.logToConsole) window.logToConsole(msg);
}}
{rs_js}
try {{
    initLog("[WASM-Init] Decoding wasmBase64...");
    const wasmBase64 = "{rs_wasm_b64}";
    const wasmBinary = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    initLog("[WASM-Init] Decoded size: " + wasmBinary.length);
    window.wasmInitPromise = wasm_bindgen(wasmBinary.buffer).then(() => {{
        wasm_bindgen.init_engine();
        window.sketchpadRs = wasm_bindgen;
        initLog("[WASM-Init] WASM Engine initialized successfully!");
    }}).catch(e => {{
        initLog("[WASM-Init] Error in wasm_bindgen: " + e.message, true);
    }});
}} catch (err) {{
    initLog("[WASM-Init] Error decoding or initializing WASM: " + err.message, true);
}}
</script>'''

        inlined_script = f'<script type="module">\n{js}\n</script>'
        html = html.replace(
            '<script type="module" src="./main_mcp.js"></script>',
            inlined_rs + '\n' + inlined_script,
        )
        return html

    bundled_path = os.path.join(dir_path, "ui", "sketchpad", "index_mcp.html")
    if not os.path.exists(bundled_path):
        bundled_path = os.path.join(dir_path, "ui", "index_mcp.html")
        
    if os.path.exists(bundled_path):
        with open(bundled_path, "r", encoding="utf-8") as f:
            return f.read()

    raise FileNotFoundError("Could not find Building Planner UI assets (neither unbundled sketchpad/ nor bundled ui/sketchpad/).")





@mcp.resource(
    _WORKSPACE_APP_URI,
    name="coda workspace",
    description="Interactive coda ACC workspace using elements/ui primitives (dashboard, config, runs, report).",
    mime_type="text/html;profile=mcp-app",
    meta=_mcp_app_html_resource_meta(),
)
def coda_workspace_viewer_resource() -> str:
    """Serve the MCP App HTML shell built from coda/assistant mcp-app (elements/ui)."""
    return _build_mcp_app_html(panel="dashboard")


@mcp.resource("coda://measures")
def get_measures() -> str:
    """🔻List all measures that are available.
    Implementations MUST load the coda config and return the measures array.
    """
    config = _get_coda_config()
    return json.dumps(config.get("measures", []), indent=2)


@mcp.resource("coda://measure/{id}")
def get_measure(id: str) -> str:
    """⬛Get a measure by id.
    Implementations MUST return an error JSON object when the measure is not found.
    """
    config = _get_coda_config()
    for m in config.get("measures", []):
        if m.get("id") == id:
            return json.dumps(m, indent=2)
    return json.dumps({"error": f"measure not found: {id}"})


@mcp.resource("coda://property-kinds")
def get_property_kinds() -> str:
    """⬜List all property kinds with their measures.
    """
    config = _get_coda_config()
    return json.dumps(config.get("property_kinds", {}), indent=2)


@mcp.resource("coda://correlation")
def get_correlation() -> str:
    """🟥Get the property correlation matrix.
    """
    config = _get_coda_config()
    return json.dumps(config.get("correlation", {}), indent=2)


@mcp.resource("coda://properties")
def get_properties() -> str:
    """🟧List all root-level property definitions with normalized kinds and measure_kinds.
    Implementations MUST load the coda config and return the normalized properties array.
    """
    config = _get_coda_config()
    properties = [
        _normalize_property_definition(p) if isinstance(p, dict) else p
        for p in config.get("properties", [])
    ]
    return json.dumps(properties, indent=2)


@mcp.resource("coda://property/{id}")
def get_property(id: str) -> str:
    """🟨Get a root-level property by id with normalized kind and measure_kinds.
    Implementations MUST return an error JSON object when the property is not found.
    """
    config = _get_coda_config()
    for p in config.get("properties", []):
        if isinstance(p, dict) and p.get("id") == id:
            return json.dumps(_normalize_property_definition(p), indent=2)
    return json.dumps({"error": f"property not found: {id}"})


@mcp.resource("coda://targets")
def get_targets() -> str:
    """🟩List all targets.
    Implementations MUST load the coda config and return the targets array.
    """
    config = _get_coda_config()
    targets = [_normalize_target_definition(t) for t in config.get("targets", [])]
    return json.dumps(targets, indent=2)


@mcp.resource("coda://frameworks")
def get_frameworks() -> str:
    """🔭List all frameworks. Frameworks are the same as targets in coda.json but general (not project-scoped).
    Implementations MUST load the coda config and return the targets array.
    """
    config = _get_coda_config()
    return json.dumps(config.get("targets", []), indent=2)


@mcp.resource("coda://framework/{id}")
def get_framework(id: str) -> str:
    """💼Get a framework by id. Frameworks are the same as targets in coda.json but general (not project-scoped).
    Implementations MUST return an error JSON object when the framework is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == id:
            return json.dumps(t, indent=2)
    return json.dumps({"error": f"framework not found: {id}"})


@mcp.resource("coda://target/{id}")
def get_target(id: str) -> str:
    """🟦Get a target by id.
    Implementations MUST return an error JSON object when the target is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == id:
            return json.dumps(_normalize_target_definition(t), indent=2)
    return json.dumps({"error": f"target not found: {id}"})


@mcp.resource("coda://{target_id}/properties")
def get_target_properties(target_id: str) -> str:
    """🟪Get properties for a target.
    Implementations MUST return an error JSON object when the target is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            target = _normalize_target_definition(t)
            return json.dumps(target.get("properties", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/property/{id}")
def get_target_property(target_id: str, id: str) -> str:
    """🟫Get a property by id for a target.
    Implementations MUST return an error JSON object when the target or property is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            target = _normalize_target_definition(t)
            for p in target.get("properties", []):
                if p.get("id") == id:
                    return json.dumps(p, indent=2)
            return json.dumps({"error": f"property not found: {id}"})
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/rules")
def get_target_rules(target_id: str) -> str:
    """💠Get rules for a target.
    Implementations MUST return an error JSON object when the target is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            return json.dumps(t.get("rules", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/rule/{id}")
def get_target_rule(target_id: str, id: str) -> str:
    """🔳Get a rule by id for a target.
    Implementations MUST return an error JSON object when the target or rule is not found.
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            for r in t.get("rules", []):
                if r.get("id") == id:
                    return json.dumps(r, indent=2)
            return json.dumps({"error": f"rule not found: {id}"})
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://project")
def get_project() -> str:
    """🔲Get the current project configuration.
    Implementations MUST return an error JSON object when no project root is found.
    """
    proj = _get_project_config()
    if proj is None:
        return json.dumps(
            {
                "error": "No coda project found. Set CODA_PROJECT or run from project root."
            }
        )
    return json.dumps(proj, indent=2)


@mcp.resource("coda://current-run")
def get_current_run() -> str:
    """▪️Get the current run metadata.
    Implementations MUST return an error JSON object when no project or run exists.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    run_json = run_dir / "run.json"
    data = (
        json.loads(run_json.read_text(encoding="utf-8"))
        if run_json.exists()
        else {"id": run_dir.name}
    )
    return json.dumps(data, indent=2)


@mcp.resource("coda://current-iteration")
def get_current_iteration() -> str:
    """▫️Get the current iteration metadata.
    Implementations MUST return an error JSON object when no project, run, or iteration exists.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return json.dumps({"error": "No iterations found"})
    iter_json = iter_dir / "iteration.json"
    data = (
        json.loads(iter_json.read_text(encoding="utf-8"))
        if iter_json.exists()
        else {"index": iter_dir.name}
    )
    return json.dumps(data, indent=2)


@mcp.resource("coda://iterations")
def get_iterations() -> str:
    """◾List iterations in the current run.
    Implementations MUST return an empty array when no runs or iterations exist.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps([])
    iters = run_dir / "iterations"
    if not iters.exists():
        return json.dumps([])
    dirs = [d for d in iters.iterdir() if d.is_dir() and d.name.isdigit()]
    entries = [{"index": d.name} for d in sorted(dirs, key=lambda x: int(x.name))]
    return json.dumps(entries, indent=2)


@mcp.resource("coda://report")
def get_report() -> str:
    """◽Get the current report from the latest iteration.
    get_report MUST perform the get_report operation.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return json.dumps({"error": "No iterations found"})
    report_json = iter_dir / "targets" / "report.json"
    if not report_json.exists():
        return json.dumps({"error": "No report found"})
    return report_json.read_text(encoding="utf-8")


@mcp.resource("coda://platforms")
def get_platforms() -> str:
    """🧱List all platforms with their measure instructions.
    Implementations MUST load the coda config and return the platforms array.
    """
    config = _get_coda_config()
    return json.dumps(config.get("platforms", []), indent=2)


@mcp.resource("coda://platform/{id}")
def get_platform(id: str) -> str:
    """◻️Get a platform by id with its measure instructions.
    Implementations MUST return an error JSON object when the platform is not found.
    """
    config = _get_coda_config()
    for p in config.get("platforms", []):
        if p.get("id") == id:
            return json.dumps(p, indent=2)
    return json.dumps({"error": f"platform not found: {id}"})


@mcp.resource("coda://breachs")
def get_breachs() -> str:
    """◼️Get breachs from the current report of the latest iteration.
    Implementations MUST return an empty array when no breachs exist.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return json.dumps({"error": "No iterations found"})
    report_json = iter_dir / "targets" / "report.json"
    if not report_json.exists():
        return json.dumps([])
    report = json.loads(report_json.read_text(encoding="utf-8"))
    breachs = report.get("breachs", [])
    return json.dumps(breachs, indent=2)


@mcp.resource("coda://translation/{target_id}")
def get_translation(target_id: str) -> str:
    """🔵Get the translation output for a target in the current iteration.
    Implementations MUST return an error JSON object when no translation exists.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return json.dumps({"error": "No iterations found"})
    translation_json = iter_dir / "targets" / target_id / "translation.json"
    if not translation_json.exists():
        return json.dumps({"error": f"No translation found for target: {target_id}"})
    return translation_json.read_text(encoding="utf-8")


@mcp.resource("coda://validation/{target_id}")
def get_validation(target_id: str) -> str:
    """📜Get the validation report for a target in the current iteration (ontology or binary).
    get_validation MUST return the per-target validation envelope written by validate/save_validation.
    """
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return json.dumps({"error": "No iterations found"})
    report_json = iter_dir / "targets" / target_id / "report.json"
    if not report_json.exists():
        return json.dumps({"error": f"No validation found for target: {target_id}"})
    return report_json.read_text(encoding="utf-8")


# #endregion 🕸️Resources
# #region 🥁Tools
# Tools MUST expose stateful MCP tool handlers following the engine pattern.
# Call start_working_on_project(path) first; then start_run, start_iteration, start_translation.

# --- Merged from building_energy.py ---
# =============================================================================
# Building Model — Simplified Flat Geometry (v2)
# =============================================================================
#
# ARCHITECTURAL PIVOT (v2):
# ────────────────────────
# Previously, users had to manually specify every Wall, Window, and Door
# in a deep nested ontology (Building → Story → Space → Wall → Window).
# This caused LLM hallucinations and was a terrible user experience.
#
# NEW APPROACH:
# The user draws 2D zones in a web UI. The geometry_engine.py computes
# the exterior envelope automatically via Shapely polygon union.
# This model simply holds the FLAT results + TABULA lookup parameters.
#
# Hierarchy:  GeometryPayload → BuildingGeometry (computed) → Energy Results
# =============================================================================

class ZoneGeometry(BaseModel):
    """Position and dimensions of a single zone rectangle."""
    x: float = Field(..., description="X coordinate of the zone origin [m]")
    y: float = Field(..., description="Y coordinate of the zone origin [m]")
    width: float = Field(..., gt=0, description="Zone width along X axis [m]")
    length: float = Field(..., gt=0, description="Zone length along Y axis [m]")


class BuildingZone(BaseModel):
    """A single room/zone as drawn by the user in the 2D UI."""
    id: str = Field(..., description="Unique zone identifier, e.g. 'zone_1'")
    type: str = Field(default="Room", description="Room type, e.g. 'Living Room', 'Kitchen'")
    geometry: ZoneGeometry


class GeometryPayload(BaseModel):
    """
    The JSON payload generated by the 2D zone-drawing UI.
    
    Example:
    {
      "version": "1.0",
      "units": "meters",
      "building_zones": [
        {"id": "zone_1", "type": "Living Room", "geometry": {"x": 0, "y": 0, "width": 5, "length": 4}},
        {"id": "zone_2", "type": "Kitchen",     "geometry": {"x": 5, "y": 0, "width": 4, "length": 4}}
      ]
    }
    """
    version: str = Field(default="1.0")
    units: str = Field(default="meters")
    building_zones: List[BuildingZone] = Field(..., min_length=1)


class BuildingGeometry(BaseModel):
    """
    Flat, computed building geometry — derived from the 2D zone layout.
    
    All areas are automatically calculated:
    - Gross Exterior Wall Area = exterior_perimeter × story_height × num_stories
    - Window Area = gross_wall_area × window_to_wall_ratio
    - Net Wall Area = gross_wall_area - window_area
    - Roof Area ≈ footprint_area (flat roof assumption)
    - Floor Area = footprint_area × num_stories
    """
    footprint_area_m2: float = Field(..., gt=0, description="Merged footprint area [m²]")
    exterior_perimeter_m: float = Field(..., gt=0, description="True exterior perimeter [m]")
    story_height_m: float = Field(default=2.8, gt=0, description="Floor-to-ceiling height [m]")
    number_of_stories: int = Field(default=1, ge=1, description="Number of heated stories")
    window_to_wall_ratio: float = Field(
        default=0.15, ge=0.0, le=1.0,
        description="Fraction of exterior wall that is glazed (0.0–1.0). Default 0.15 = 15%"
    )
    
    @property
    def total_heated_floor_area_m2(self) -> float:
        return round(self.footprint_area_m2 * self.number_of_stories, 2)
    
    @property
    def gross_exterior_wall_area_m2(self) -> float:
        return round(self.exterior_perimeter_m * self.story_height_m * self.number_of_stories, 2)
    
    @property
    def window_area_m2(self) -> float:
        return round(self.gross_exterior_wall_area_m2 * self.window_to_wall_ratio, 2)
    
    @property
    def net_wall_area_m2(self) -> float:
        return round(self.gross_exterior_wall_area_m2 - self.window_area_m2, 2)
    
    @property
    def roof_area_m2(self) -> float:
        """Roof area ≈ footprint (flat roof assumption)."""
        return round(self.footprint_area_m2, 2)
    
    @property
    def ground_floor_area_m2(self) -> float:
        return round(self.footprint_area_m2, 2)
    
    @property
    def total_envelope_area_m2(self) -> float:
        return round(
            self.net_wall_area_m2 + self.window_area_m2 +
            self.roof_area_m2 + self.ground_floor_area_m2, 2
        )
    
    @property
    def air_volume_m3(self) -> float:
        return round(self.total_heated_floor_area_m2 * self.story_height_m, 2)


class TABULAParams(BaseModel):
    """Parameters needed to look up U-values from the TABULA database."""
    building_type: Literal["SFH", "TH", "MFH", "AB"] = Field(
        ...,
        description="SFH=Single-Family, TH=Terraced, MFH=Multi-Family, AB=Apartment Block"
    )
    year_class: str = Field(
        ...,
        description="TABULA year class, e.g. '1969-1978', '...1859', '2016+'"
    )
    scenario: Literal["Existing State", "Usual Refurbishment", "Advanced Refurbishment"] = Field(
        default="Existing State"
    )
    country: Literal["DE"] = Field(default="DE")


class EnergyCalculationRequest(BaseModel):
    """
    Complete request for energy calculation.
    Combines the UI geometry payload with TABULA parameters and building specs.
    """
    geometry_payload: GeometryPayload
    building_type: Literal["SFH", "TH", "MFH", "AB"] = Field(
        default="SFH",
        description="SFH=Single-Family, TH=Terraced, MFH=Multi-Family, AB=Apartment Block"
    )
    year_class: str = Field(
        default="2016+",
        description="TABULA year class, e.g. '1969-1978', '2016+'"
    )
    scenario: Literal["Existing State", "Usual Refurbishment", "Advanced Refurbishment"] = Field(
        default="Existing State"
    )
    story_height_m: float = Field(default=2.8, gt=0, description="Floor-to-ceiling height [m]")
    number_of_stories: int = Field(default=1, ge=1, description="Number of heated stories")
    window_to_wall_ratio: float = Field(
        default=0.15, ge=0.0, le=1.0,
        description="Window-to-wall ratio (0.0–1.0). 0.15 = 15%"
    )
    climate_factor_kkh: float = Field(default=66.0, gt=0, description="Climate factor [kKh]")
    air_exchange_rate_1_h: float = Field(default=0.5, gt=0, description="Air changes per hour [1/h]")
    name: str = Field(default="My Building", description="Building name/identifier")


# =============================================================================
# Geometry Engine — 2D Zone → Exterior Envelope Calculator
# =============================================================================
#
# Takes a list of rectangular building zones (from the UI JSON payload)
# and uses Shapely to compute the TRUE exterior envelope by merging all
# overlapping/touching rectangles via unary_union.
#
# Key Concept:
#   Interior walls (shared between adjacent zones) are automatically
#   excluded from the thermal envelope because unary_union dissolves
#   shared edges into the interior of the merged polygon.
#
# v3 UPGRADE: Orientation-Aware Wall Segments
#   After computing the exterior polygon, each edge is classified by its
#   outward-facing compass direction (N, E, S, W) using the edge's normal
#   vector. This enables per-direction solar gain calculations.
#
# Input:  List of zone dicts with {id, type, geometry: {x, y, width, length}}
# Output: GeometryResult with floor_area, exterior_perimeter, zone_details,
#         wall_segments by orientation, and merged polygon coordinates.
# =============================================================================

@dataclass
class WallSegment:
    """A single exterior wall segment with its compass orientation."""
    start: Tuple[float, float]
    end: Tuple[float, float]
    length_m: float
    orientation: str  # "N", "E", "S", "W"
    normal_angle_deg: float  # The exact outward-facing angle (0°=N, 90°=E, etc.)


@dataclass
class GeometryResult:
    """Results from the 2D geometry analysis."""
    gross_floor_area_m2: float          # Total footprint area (merged)
    exterior_perimeter_m: float         # True exterior perimeter length
    zone_details: List[Dict[str, Any]] = field(default_factory=list)
    exterior_coords: List[tuple] = field(default_factory=list)
    raw_zone_area_m2: float = 0.0
    zone_count: int = 0
    interior_wall_length_m: float = 0.0
    wall_segments: List[WallSegment] = field(default_factory=list)
    walls_by_orientation: Dict[str, float] = field(default_factory=dict)


def _classify_orientation(dx: float, dy: float, is_ccw: bool) -> Tuple[str, float]:
    """
    Classify an exterior wall edge by its outward-facing compass direction.
    
    The outward normal depends on the polygon's winding order:
      - CCW exterior: outward normal of edge (dx, dy) is (dy, -dx)
      - CW exterior:  outward normal of edge (dx, dy) is (-dy, dx)
    
    In our coordinate system:
      - +Y = North (up on the sketchpad)
      - +X = East (right on the sketchpad)
    """
    if is_ccw:
        normal_x = dy
        normal_y = -dx
    else:
        normal_x = -dy
        normal_y = dx
    
    angle_rad = math.atan2(normal_x, normal_y)
    angle_deg = math.degrees(angle_rad)
    
    if angle_deg < 0:
        angle_deg += 360.0
    
    if angle_deg >= 315 or angle_deg < 45:
        return "N", angle_deg
    elif 45 <= angle_deg < 135:
        return "E", angle_deg
    elif 135 <= angle_deg < 225:
        return "S", angle_deg
    else:
        return "W", angle_deg


def _extract_wall_segments(polygon: Polygon, building_rotation_deg: float = 0.0) -> Tuple[List[WallSegment], Dict[str, float]]:
    """Walk the exterior ring of a polygon and classify each edge by compass direction."""
    coords = list(polygon.exterior.coords)
    is_ccw = polygon.exterior.is_ccw
    segments = []
    orientation_totals = {"N": 0.0, "E": 0.0, "S": 0.0, "W": 0.0}
    
    for i in range(len(coords) - 1):
        x1, y1 = coords[i]
        x2, y2 = coords[i + 1]
        
        dx = x2 - x1
        dy = y2 - y1
        length = math.sqrt(dx * dx + dy * dy)
        
        if length < 0.001:
            continue
        
        if building_rotation_deg != 0.0:
            rot_rad = math.radians(-building_rotation_deg)
            cos_r = math.cos(rot_rad)
            sin_r = math.sin(rot_rad)
            dx_rot = dx * cos_r - dy * sin_r
            dy_rot = dx * sin_r + dy * cos_r
        else:
            dx_rot = dx
            dy_rot = dy
        
        direction, angle = _classify_orientation(dx_rot, dy_rot, is_ccw)
        
        seg = WallSegment(
            start=(round(x1, 3), round(y1, 3)),
            end=(round(x2, 3), round(y2, 3)),
            length_m=round(length, 3),
            orientation=direction,
            normal_angle_deg=round(angle, 1),
        )
        segments.append(seg)
        orientation_totals[direction] += length
    
    orientation_totals = {k: round(v, 2) for k, v in orientation_totals.items()}
    return segments, orientation_totals


def _zone_to_polygon(zone: Dict[str, Any], buffer_tolerance: float = 0.01) -> Polygon:
    """Convert a single zone dict into a Shapely Polygon."""
    geom = zone["geometry"]
    x = float(geom["x"])
    y = float(geom["y"])
    w = float(geom["width"])
    l = float(geom["length"])
    return box(x, y, x + w, y + l)


def compute_exterior_envelope(
    building_zones: List[Dict[str, Any]],
    buffer_tolerance: float = 0.01,
    building_rotation_deg: float = 0.0,
) -> GeometryResult:
    """
    Main entry point: takes the building_zones list from the UI JSON
    and computes the merged exterior envelope with orientation-aware wall segments.
    """
    if not building_zones:
        return GeometryResult(
            gross_floor_area_m2=0.0,
            exterior_perimeter_m=0.0,
        )
    
    polygons = []
    zone_details = []
    raw_total_area = 0.0
    raw_total_perimeter = 0.0
    
    for zone in building_zones:
        poly = _zone_to_polygon(zone, buffer_tolerance)
        polygons.append(poly)
        
        geom = zone["geometry"]
        zone_area = float(geom["width"]) * float(geom["length"])
        zone_perimeter = 2 * (float(geom["width"]) + float(geom["length"]))
        raw_total_area += zone_area
        raw_total_perimeter += zone_perimeter
        
        zone_details.append({
            "id": zone.get("id", f"zone_{len(zone_details) + 1}"),
            "type": zone.get("type", "Unknown"),
            "individual_area_m2": round(zone_area, 2),
            "individual_perimeter_m": round(zone_perimeter, 2),
            "bounds": {
                "x": float(geom["x"]),
                "y": float(geom["y"]),
                "width": float(geom["width"]),
                "length": float(geom["length"]),
            },
        })
    
    merged = unary_union(polygons)
    
    if buffer_tolerance > 0:
        snap_tol = max(buffer_tolerance, 0.1)
        merged = merged.buffer(snap_tol, join_style=2).buffer(-snap_tol, join_style=2)
    
    wall_segments = []
    walls_by_orientation = {"N": 0.0, "E": 0.0, "S": 0.0, "W": 0.0}
    
    if isinstance(merged, MultiPolygon):
        floor_area = round(merged.area, 2)
        perimeter = round(sum(geom.exterior.length for geom in merged.geoms), 2)
        largest = max(merged.geoms, key=lambda p: p.area)
        exterior_coords = list(largest.exterior.coords)
        
        for poly in merged.geoms:
            segs, orient = _extract_wall_segments(poly, building_rotation_deg)
            wall_segments.extend(segs)
            for direction, length in orient.items():
                walls_by_orientation[direction] += length
                
    elif isinstance(merged, Polygon):
        floor_area = round(merged.area, 2)
        perimeter = round(merged.exterior.length, 2)
        exterior_coords = list(merged.exterior.coords)
        
        wall_segments, walls_by_orientation = _extract_wall_segments(
            merged, building_rotation_deg
        )
    else:
        floor_area = 0.0
        perimeter = 0.0
        exterior_coords = []
    
    walls_by_orientation = {k: round(v, 2) for k, v in walls_by_orientation.items()}
    interior_wall_length = max(0.0, (raw_total_perimeter - perimeter) / 2)
    
    return GeometryResult(
        gross_floor_area_m2=floor_area,
        exterior_perimeter_m=perimeter,
        zone_details=zone_details,
        exterior_coords=exterior_coords,
        raw_zone_area_m2=round(raw_total_area, 2),
        zone_count=len(building_zones),
        interior_wall_length_m=round(interior_wall_length, 2),
        wall_segments=wall_segments,
        walls_by_orientation=walls_by_orientation,
    )


def compute_building_areas(
    geo: GeometryResult,
    story_height_m: float = 2.8,
    number_of_stories: int = 1,
    window_to_wall_ratio: float = 0.15,
    windows_by_orientation: Dict[str, float] = None,
) -> Dict[str, Any]:
    """Derive all thermal envelope areas from the geometry result."""
    total_floor_area = geo.gross_floor_area_m2 * number_of_stories
    gross_wall_area = geo.exterior_perimeter_m * story_height_m * number_of_stories
    
    gross_walls_by_dir = {}
    net_walls_by_dir = {}
    windows_by_dir = {}
    
    for direction in ["N", "E", "S", "W"]:
        wall_length = geo.walls_by_orientation.get(direction, 0.0)
        gross_area = wall_length * story_height_m * number_of_stories
        gross_walls_by_dir[direction] = round(gross_area, 2)
        
        if windows_by_orientation and direction in windows_by_orientation:
            win_area = windows_by_orientation[direction]
        else:
            win_area = gross_area * window_to_wall_ratio
        
        windows_by_dir[direction] = round(win_area, 2)
        net_walls_by_dir[direction] = round(max(0.0, gross_area - win_area), 2)
    
    window_area = sum(windows_by_dir.values())
    net_wall_area = sum(net_walls_by_dir.values())
    roof_area = geo.gross_floor_area_m2
    ground_floor_area = geo.gross_floor_area_m2
    total_envelope = net_wall_area + window_area + roof_area + ground_floor_area
    air_volume = total_floor_area * story_height_m
    
    return {
        "total_heated_floor_area_m2": round(total_floor_area, 2),
        "gross_exterior_wall_area_m2": round(gross_wall_area, 2),
        "window_area_m2": round(window_area, 2),
        "net_wall_area_m2": round(net_wall_area, 2),
        "roof_area_m2": round(roof_area, 2),
        "ground_floor_area_m2": round(ground_floor_area, 2),
        "total_envelope_area_m2": round(total_envelope, 2),
        "air_volume_m3": round(air_volume, 2),
        "exterior_perimeter_m": round(geo.exterior_perimeter_m, 2),
        "interior_wall_excluded_m": round(geo.interior_wall_length_m, 2),
        "gross_walls_by_orientation_m2": gross_walls_by_dir,
        "windows_by_orientation_m2": windows_by_dir,
        "net_walls_by_orientation_m2": net_walls_by_dir,
    }


# =============================================================================
# TABULA Germany Database — Official EPISCOPE/IWU Building Typology
# =============================================================================

YEAR_CLASSES = ["...1859", "1860-1918", "1919-1948", "1949-1957", "1958-1968", 
                "1969-1978", "1979-1983", "1984-1994", "1995-2001", "2002-2009", 
                "2010-2015", "2016-..."]

BUILDING_TYPES = ["SFH", "TH", "MFH", "AB"]
SCENARIOS = ["Existing State", "Usual Refurbishment", "Advanced Refurbishment"]

FX_FACTORS = {
    "Exterior": 1.0,
    "Ground": 0.6,
    "Unheated": 0.5
}

SOLAR_IRRADIATION_KWH_M2A = {
    "N": 350, "E": 550, "S": 900, "W": 550, "Horizontal": 1000
}

VENTILATION_SYSTEMS = {
    "Natural": {
        "air_exchange_rate": 0.5,
        "heat_recovery_efficiency": 0.0,
        "description": "Natural ventilation (window airing)"
    },
    "Exhaust Only": {
        "air_exchange_rate": 0.5,
        "heat_recovery_efficiency": 0.0,
        "description": "Mechanical exhaust fan, no heat recovery"
    },
    "Balanced with HR": {
        "air_exchange_rate": 0.4,
        "heat_recovery_efficiency": 0.75,
        "description": "Balanced mechanical ventilation with 75% heat recovery"
    },
    "Balanced with HR+": {
        "air_exchange_rate": 0.4,
        "heat_recovery_efficiency": 0.90,
        "description": "High-efficiency balanced ventilation with 90% heat recovery"
    }
}

HEATING_SYSTEMS = {
    "Gas Boiler (old)": {
        "generation_efficiency": 0.85,
        "primary_energy_factor": 1.1,
        "energy_carrier": "Natural Gas",
        "co2_factor_kg_kwh": 0.201
    },
    "Gas Condensing Boiler": {
        "generation_efficiency": 0.95,
        "primary_energy_factor": 1.1,
        "energy_carrier": "Natural Gas",
        "co2_factor_kg_kwh": 0.201
    },
    "Oil Boiler (old)": {
        "generation_efficiency": 0.80,
        "primary_energy_factor": 1.1,
        "energy_carrier": "Heating Oil",
        "co2_factor_kg_kwh": 0.266
    },
    "Oil Condensing Boiler": {
        "generation_efficiency": 0.92,
        "primary_energy_factor": 1.1,
        "energy_carrier": "Heating Oil",
        "co2_factor_kg_kwh": 0.266
    },
    "Heat Pump (Air-Source)": {
        "generation_efficiency": 3.0,  # COP
        "primary_energy_factor": 1.8,
        "energy_carrier": "Electricity",
        "co2_factor_kg_kwh": 0.421
    },
    "Heat Pump (Ground-Source)": {
        "generation_efficiency": 4.0,  # COP
        "primary_energy_factor": 1.8,
        "energy_carrier": "Electricity",
        "co2_factor_kg_kwh": 0.421
    },
    "District Heating": {
        "generation_efficiency": 0.98,
        "primary_energy_factor": 0.7,
        "energy_carrier": "District Heat",
        "co2_factor_kg_kwh": 0.167
    },
    "Pellet Boiler": {
        "generation_efficiency": 0.90,
        "primary_energy_factor": 0.2,
        "energy_carrier": "Wood Pellets",
        "co2_factor_kg_kwh": 0.023
    }
}

YEAR_TO_CODE = {
    "...1859": "01", "1860-1918": "02", "1919-1948": "03", "1949-1957": "04",
    "1958-1968": "05", "1969-1978": "06", "1979-1983": "07", "1984-1994": "08",
    "1995-2001": "09", "2002-2009": "10", "2010-2015": "11", "2016-...": "12"
}

SCENARIO_TO_CODE = {
    "Existing State": "1", "Usual Refurbishment": "2", "Advanced Refurbishment": "3"
}

def _load_data():
    path = os.path.join(os.path.dirname(__file__), "tabula_data/extracted_data.json")
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}

TABULA_DB = _load_data()

def lookup_archetype(building_type: str, year_class: str, scenario: str = "Existing State", country: str = "DE"):
    """
    Look up 100% accurate TABULA data for a German building.
    Handles long scientific keys like 'DE_TH_05_DE.N.TH.05.Gen.ReEx.001.001'
    """
    if year_class == "2016+":
        year_class = "2016-..."
    age_code = YEAR_TO_CODE.get(year_class, "01")
    
    suffix_map = {
        "Existing State": ".001",
        "Usual Refurbishment": ".002",
        "Advanced Refurbishment": ".003"
    }
    suffix = suffix_map.get(scenario, ".001")
    prefix = f"{country}_{building_type}_{age_code}"
    
    for k, v in TABULA_DB.items():
        if k.startswith(prefix) and k.endswith(suffix):
            return {
                "component_u_values": {
                    "wall_u_value_w_m2k": v["u_wall"],
                    "roof_u_value_w_m2k": v["u_roof"],
                    "window_u_value_w_m2k": v["u_window"],
                    "floor_u_value_w_m2k": v["u_floor"],
                    "door_u_value_w_m2k": 1.8,
                    "window_g_value": v.get("g_value") or 0.7
                },
                "outputs": {
                    "q_h_nd_kWh_m2a": v["q_h_nd"]
                }
            }
            
    variant_code = SCENARIO_TO_CODE.get(scenario, "1")
    short_key = f"{prefix}_{variant_code}"
    for k, v in TABULA_DB.items():
        if k.startswith(short_key):
            return {
                "component_u_values": {
                    "wall_u_value_w_m2k": v["u_wall"],
                    "roof_u_value_w_m2k": v["u_roof"],
                    "window_u_value_w_m2k": v["u_window"],
                    "floor_u_value_w_m2k": v["u_floor"],
                    "door_u_value_w_m2k": 1.8,
                    "window_g_value": v.get("g_value") or 0.7
                },
                "outputs": {
                    "q_h_nd_kWh_m2a": v["q_h_nd"]
                }
            }
            
    return None

def list_archetype_keys(): return list(TABULA_DB.keys())
def list_year_classes(): return YEAR_CLASSES
def list_building_types(): return BUILDING_TYPES


# =============================================================================
# ISO 13790 Energy Calculation Engine — TABULA Edition
# =============================================================================

def _calc_H_tr(surfaces: List[Dict[str, float]], delta_U_tbr: float) -> float:
    """Transmission Heat Transfer Coefficient [W/K]"""
    sum_bAU = sum(s["b_tr"] * s["A_env"] * s["U_eff"] for s in surfaces)
    sum_A   = sum(s["A_env"] for s in surfaces)
    return sum_bAU + sum_A * delta_U_tbr


def _calc_H_ve(c_p_air: float, n_use: float, n_infiltr: float,
               A_ref: float, h_ref: float) -> float:
    """Ventilation Heat Transfer Coefficient [W/K]"""
    return c_p_air * (n_use + n_infiltr) * A_ref * h_ref


def _calc_Q_sol(F_sh: float, F_F: float, F_W: float, g_gl_n: float,
                windows: List[Dict[str, float]]) -> float:
    """Solar Heat Gains [kWh/a]"""
    sum_AI = sum(w["A_window"] * w["I_sol"] for w in windows)
    return F_sh * (1.0 - F_F) * F_W * g_gl_n * sum_AI


def _calc_Q_ht(H_tr: float, H_ve: float, F_nu: float,
               theta_int: float, theta_e: float, d_hs: float) -> float:
    """Total Heat Transfer (Losses) [kWh/a]"""
    return 0.024 * (H_tr + H_ve) * F_nu * (theta_int - theta_e) * d_hs


def _calc_Q_H_nd(Q_ht: float, eta: float, Q_gn: float) -> float:
    """Annual Heating Demand [kWh/a]"""
    return max(0.0, Q_ht - eta * Q_gn)


# Climatic Data for DE.N (Standard Deutschland)
CLIMATE_DE_N = {
    "theta_e_b":  12,
    "d_hs":       222,
    "theta_e":    4.4,
    "I_sol": {
        "S": 392.0,
        "E": 271.0,
        "W": 271.0,
        "N": 160.0,
    },
}

# Thermal Bridge Classification
THERMAL_BRIDGE_MAP = {
    "Advanced Refurbishment": 0.0,
    "Usual Refurbishment":    0.05,
    "Existing State":         0.10,
}

OLDER_YEAR_CLASSES = frozenset({
    "...1859", "1860-1918", "1919-1948", "1949-1957", "1958-1968", "1969-1978",
})

def _get_infiltration_rate(scenario: str, year_class: str) -> float:
    """Derive n_air,infiltr from refurbishment status and building age."""
    if scenario == "Advanced Refurbishment":
        return 0.05
    elif scenario == "Usual Refurbishment":
        return 0.1
    else:
        if year_class in OLDER_YEAR_CLASSES:
            return 0.4
        else:
            return 0.2

SOLAR_CONSTANTS = {
    "F_sh": 0.6,
    "F_F":  0.3,
    "F_W":  0.9,
}

H_ROOM_VE_REF = 2.5
C_P_AIR   = 0.34
N_AIR_USE = 0.4
THETA_INT = 20.0
F_NU      = 1.0
ETA_H_GN  = 0.95

B_TR = {
    "wall":   1.0,
    "roof":   1.0,
    "window": 1.0,
    "floor":  0.5,
}

Q_INT_DENSITY = 0.0528

def calculate_energy(
    building_type: str,
    year_class: str,
    scenario: str,
    sketchpad_areas: Dict[str, Any],
    windows_by_orientation: Dict[str, float],
    heating_system: str = "Gas Condensing Boiler",
) -> Dict[str, Any]:
    """Complete ISO 13790 seasonal monthly calculation."""
    print("CALCULATE ENERGY: start")

    archetype = lookup_archetype(building_type, year_class, scenario, "DE")
    if archetype is None:
        return {"status": "error",
                "message": f"No TABULA archetype for DE_{building_type}_{year_class}_{scenario}"}

    uv = archetype["component_u_values"]
    u_wall   = uv["wall_u_value_w_m2k"]
    u_roof   = uv["roof_u_value_w_m2k"]
    u_floor  = uv["floor_u_value_w_m2k"]
    u_window = uv["window_u_value_w_m2k"]
    g_value  = uv["window_g_value"]

    a_wall   = sketchpad_areas.get("net_wall_area_m2", 0.0)
    a_roof   = sketchpad_areas.get("roof_area_m2", 0.0)
    a_floor  = sketchpad_areas.get("ground_floor_area_m2", 0.0)
    a_window = sketchpad_areas.get("window_area_m2", 0.0)
    a_floor_total = sketchpad_areas.get("total_heated_floor_area_m2", 0.0)

    if a_floor_total <= 0:
        return {"status": "error", "message": "Total floor area is zero. Draw zones first."}

    surfaces = [
        {"name": "Wall",   "b_tr": B_TR["wall"],   "A_env": a_wall,   "U_eff": u_wall},
        {"name": "Roof",   "b_tr": B_TR["roof"],   "A_env": a_roof,   "U_eff": u_roof},
        {"name": "Floor",  "b_tr": B_TR["floor"],  "A_env": a_floor,  "U_eff": u_floor},
        {"name": "Window", "b_tr": B_TR["window"], "A_env": a_window, "U_eff": u_window},
    ]
    delta_U_tbr = THERMAL_BRIDGE_MAP.get(scenario, 0.10)
    H_tr = _calc_H_tr(surfaces, delta_U_tbr)

    H_tr_by_dir = {}
    net_walls_by_dir = sketchpad_areas.get("net_walls_by_orientation_m2", {})
    for d in ("N", "E", "S", "W"):
        wall_a = net_walls_by_dir.get(d, 0.0)
        win_a  = windows_by_orientation.get(d, 0.0)
        H_tr_by_dir[d] = {
            "H_tr_wall_W_K":   round(wall_a * u_wall * B_TR["wall"], 2),
            "H_tr_window_W_K": round(win_a * u_window * B_TR["window"], 2),
        }

    n_infiltr = _get_infiltration_rate(scenario, year_class)
    H_ve = _calc_H_ve(C_P_AIR, N_AIR_USE, n_infiltr, a_floor_total, H_ROOM_VE_REF)

    climate = CLIMATE_DE_N
    Q_ht = _calc_Q_ht(H_tr, H_ve, F_NU, THETA_INT, climate["theta_e"], climate["d_hs"])

    windows_data = []
    Q_sol_by_dir = {}
    for d in ("N", "E", "S", "W"):
        win_a = windows_by_orientation.get(d, 0.0)
        i_sol = climate["I_sol"].get(d, 0.0)
        if win_a > 0:
            windows_data.append({"A_window": win_a, "I_sol": i_sol})
        q_sol_d = SOLAR_CONSTANTS["F_sh"] * (1 - SOLAR_CONSTANTS["F_F"]) * \
                  SOLAR_CONSTANTS["F_W"] * g_value * win_a * i_sol
        Q_sol_by_dir[d] = round(q_sol_d, 1)

    Q_sol = _calc_Q_sol(
        SOLAR_CONSTANTS["F_sh"], SOLAR_CONSTANTS["F_F"],
        SOLAR_CONSTANTS["F_W"], g_value, windows_data)

    Q_int = Q_INT_DENSITY * climate["d_hs"] * a_floor_total
    Q_H_gn = Q_sol + Q_int
    Q_H_nd = _calc_Q_H_nd(Q_ht, ETA_H_GN, Q_H_gn)
    specific_Q_H_nd = Q_H_nd / a_floor_total

    hs = HEATING_SYSTEMS.get(heating_system, HEATING_SYSTEMS["Gas Condensing Boiler"])
    Q_final   = Q_H_nd / hs["generation_efficiency"]
    specific_Q_final   = Q_final / a_floor_total

    return {
        "status": "success",
        "tabula_archetype": f"DE_{building_type}_{year_class}_{scenario}",
        "tabula_u_values": {
            "wall_W_m2K":     u_wall,
            "roof_W_m2K":     u_roof,
            "floor_W_m2K":    u_floor,
            "window_W_m2K":   u_window,
            "window_g_value": g_value,
        },
        "envelope_areas_m2": {
            "net_wall":   round(a_wall, 2),
            "roof":       round(a_roof, 2),
            "floor":      round(a_floor, 2),
            "window":     round(a_window, 2),
            "total_floor": round(a_floor_total, 2),
        },
        "transmission": {
            "H_tr_total_W_K":   round(H_tr, 2),
            "delta_U_tbr":      delta_U_tbr,
            "b_tr_factors":     B_TR,
            "by_orientation":   H_tr_by_dir,
            "surfaces":         [{k: round(v, 3) if isinstance(v, float) else v
                                  for k, v in s.items()} for s in surfaces],
        },
        "ventilation": {
            "H_ve_W_K":          round(H_ve, 2),
            "c_p_air":           C_P_AIR,
            "n_air_use":         N_AIR_USE,
            "n_air_infiltr":     n_infiltr,
            "h_room_ve_ref_m":   H_ROOM_VE_REF,
            "A_C_ref_m2":        round(a_floor_total, 2),
        },
        "heat_losses": {
            "Q_ht_kWh_a":     round(Q_ht, 1),
            "theta_int_C":    THETA_INT,
            "theta_e_C":      climate["theta_e"],
            "d_hs_days":      climate["d_hs"],
        },
        "solar_gains": {
            "Q_sol_kWh_a":          round(Q_sol, 1),
            "Q_sol_by_dir_kWh_a":   Q_sol_by_dir,
            "F_sh": SOLAR_CONSTANTS["F_sh"],
            "F_F":  SOLAR_CONSTANTS["F_F"],
            "F_W":  SOLAR_CONSTANTS["F_W"],
            "g_gl_n": g_value,
            "I_sol_kWh_m2a": climate["I_sol"],
        },
        "internal_gains": {
            "Q_int_kWh_a":      round(Q_int, 1),
            "density_kWh_m2d":  Q_INT_DENSITY,
        },
        "heating_demand": {
            "Q_H_nd_kWh_a":             round(Q_H_nd, 1),
            "specific_Q_H_nd_kWh_m2a":  round(specific_Q_H_nd, 1),
            "eta_H_gn":                 ETA_H_GN,
        },
        "final_energy": {
            "Q_final_kWh_a":             round(Q_final, 1),
            "specific_Q_final_kWh_m2a":  round(specific_Q_final, 1),
            "heating_system":            heating_system,
            "generation_efficiency":     hs["generation_efficiency"],
        },
        "tabula_reference": archetype.get("outputs", {}),
    }




PROMPT_TEXT = '''
You are a certified building energy engineer. You assist users in assessing the thermal performance of their buildings according to ISO 13790, using the German TABULA/EPISCOPE typology database.

Your communication style is concise, technically precise, and professional — as a practicing engineer would speak with a client. Avoid unnecessary formatting, long bullet lists, or emojis in chat. Use plain, direct language.


=== WORKFLOW ===

When a user expresses interest in an energy analysis, proceed as follows:

STEP 1 — INTAKE ASSESSMENT (3 SEQUENTIAL QUESTIONS)
Ask one question at a time. Do not present all questions at once.

  a) Ask for the construction year of the building in plain language:
     "What year was the building constructed?"
     — Internally map the answer to the correct TABULA year class without showing the user the classification list.
       Mapping reference (internal, do not recite to user):
       before 1860 → "...1859" | 1860–1918 → "1860-1918" | 1919–1948 → "1919-1948"
       1949–1957 → "1949-1957" | 1958–1968 → "1958-1968" | 1969–1978 → "1969-1978"
       1979–1983 → "1979-1983" | 1984–1994 → "1984-1994" | 1995–2001 → "1995-2001"
       2002–2009 → "2002-2009" | 2010–2015 → "2010-2015" | 2016 or later → "2016-..."

  b) Ask for the building typology:
     "How would you classify the building — is it a detached single-family house, a terraced house, a multi-family building, or an apartment block?"
     — Map internally: single-family → "SFH", terraced → "TH", multi-family → "MFH", apartment block → "AB".

  c) Ask for the thermal refurbishment status:
     "Has the building undergone thermal refurbishment? If so, would you describe it as standard (e.g. partial insulation upgrades) or advanced (near-passive standard)? Or is it in its original state?"
     — Map internally: none → "Existing State", standard → "Usual Refurbishment", advanced → "Advanced Refurbishment".

After collecting all three answers, briefly confirm them in a single sentence before proceeding.


STEP 2 — GEOMETRY COLLECTION
Ask the user:
  "Shall I open the drawing tool so you can sketch the floor plan?"
Wait for explicit confirmation before calling `collect_building_geometry`.
Do NOT open the tool automatically.

When calling `collect_building_geometry`, pass the resolved `building_type`, `year_class`, and `scenario`.
These values will be locked in the drawing interface — the user only needs to draw the zones.


STEP 3 — SIMULATION
Once the user submits geometry from the drawing tool, the backend automatically:
  - Computes the exterior building envelope using geometric union (Shapely)
  - Classifies wall segments by compass orientation (N/E/S/W)
  - Retrieves construction U-values and window g-values from the TABULA archetype
  - Runs the ISO 13790 steady-state monthly energy balance

If the user later requests a parameter change (e.g. different heating system), call
`calculate_building_energy` directly with the current geometry payload.


STEP 4 — RESULTS REPORT
Present ONLY the values returned by the tool. Do not add interpretations, recommendations,
energy labels, comparisons, or any text not directly found in the tool result.

Structure the report exactly as follows — use the actual numbers from the tool JSON:

  TABULA Archetype: [tabula_archetype]

  Geometry
    Heated floor area:   [envelope_areas_m2.total_floor] m²
    Net wall area:       [envelope_areas_m2.net_wall] m²
    Roof area:           [envelope_areas_m2.roof] m²
    Floor area:          [envelope_areas_m2.floor] m²
    Window area:         [envelope_areas_m2.window] m²

  Construction U-values  (from TABULA database)
    Wall:    [tabula_u_values.wall_W_m2K] W/m²K
    Roof:    [tabula_u_values.roof_W_m2K] W/m²K
    Floor:   [tabula_u_values.floor_W_m2K] W/m²K
    Window:  [tabula_u_values.window_W_m2K] W/m²K   (g = [tabula_u_values.window_g_value])

  Heat Losses
    Transmission H_tr:   [transmission.H_tr_total_W_K] W/K
      Thermal bridge ΔU: [transmission.delta_U_tbr] W/m²K
    Ventilation H_ve:    [ventilation.H_ve_W_K] W/K
      Air change (use):  [ventilation.n_air_use] 1/h
      Air change (infil):[ventilation.n_air_infiltr] 1/h
    Total heat loss Q_ht:[heat_losses.Q_ht_kWh_a] kWh/a
      Climate θ_e:       [heat_losses.theta_e_C] °C   over [heat_losses.d_hs_days] days

  Heat Gains
    Solar Q_sol:         [solar_gains.Q_sol_kWh_a] kWh/a
      by orientation:    N=[solar_gains.Q_sol_by_dir_kWh_a.N]  E=[...E]  S=[...S]  W=[...W] kWh/a
    Internal Q_int:      [internal_gains.Q_int_kWh_a] kWh/a

  Heating Demand  (ISO 13790)
    Q_H,nd:              [heating_demand.Q_H_nd_kWh_a] kWh/a
    Specific demand:     [heating_demand.specific_Q_H_nd_kWh_m2a] kWh/m²a
    Gain utilisation η:  [heating_demand.eta_H_gn]

  Final Energy  (after heating system)
    Q_final:             [final_energy.Q_final_kWh_a] kWh/a
    Specific final:      [final_energy.specific_Q_final_kWh_m2a] kWh/m²a
    System:              [final_energy.heating_system]   η = [final_energy.generation_efficiency]

  TABULA reference value for this archetype:
    q_h,nd (TABULA):     [tabula_reference.q_h_nd_kWh_m2a] kWh/m²a


=== ENGINEERING CONSTRAINTS ===
- Report ONLY values present in the tool result JSON. Do not add anything that is not in the data.
- Do not write recommendations, retrofitting advice, energy labels, or qualitative judgements.
- Do not round or reformat numbers beyond what the tool already provides.
- Do not explain the formulas unless the user explicitly asks.
- Always state the TABULA archetype key so the user can verify the source data.
- If a field is missing or null in the tool result, write "not available" — do not substitute or guess.
'''

# --- End Merged from building_energy.py ---
@mcp.tool(meta=_BUILDING_PLANNER_META)
def collect_building_geometry(
    building_type: str = "SFH",
    year_class: str = "2016-...",
    scenario: str = "Existing State",
) -> str:
    """
    Opens the interactive 3D Building Sketchpad for the user to draw their building geometry.
    The building type, construction year class, and refurbishment scenario are pre-set from the
    user's chat answers and are locked in the UI — the user can only draw zones.

    TOOL INVOCATION RULES:
    - Do NOT call this tool until ALL three parameters are resolved from the conversation.
    - Do NOT ask the user to select from a dropdown list of year classes — interpret their
      plain-language answer (e.g. "built in 1974") and silently map it to the correct
      TABULA year class (e.g. "1969-1978").
    - Do NOT call this tool automatically. First confirm: "Shall I open the drawing tool so
      you can sketch the floor plan?" and wait for the user's explicit approval.
    - Pass the resolved `building_type`, `year_class`, and `scenario` as parameters.
    """
    return json.dumps({
        "status": "sketchpad_opened",
        "message": (
            "The Building Energy Sketchpad has been opened. "
            "Please draw your building zones using the controls on the left panel, "
            "then click '⚡ Calculate Energy' to run the analysis."
        ),
        "defaults": {
            "building_type": building_type,
            "year_class": year_class,
            "scenario": scenario,
        },
    })

#@mcp.tool()
def lookup_tabula_archetype(
    building_type: str,
    year_class: str,
    scenario: str = "Existing State",
    country: str = "DE",
) -> str:
    """
    PRIMARY LOOKUP TOOL — Use this FIRST for any building energy query.
    """
    result = lookup_archetype(building_type, year_class, scenario, country)
    if result is None:
        return json.dumps({
            "status": "error",
            "message": f"No archetype found for {country}_{building_type}_{year_class}_{scenario}",
            "available_building_types": BUILDING_TYPES,
            "available_year_classes": YEAR_CLASSES,
            "available_scenarios": SCENARIOS,
        }, indent=2)
    return json.dumps({"status": "success", **result}, indent=2)

@mcp.tool()
def list_tabula_archetypes(
    building_type: typing.Optional[str] = None,
) -> str:
    """
    Lists all available TABULA archetype keys.
    Optionally filter by building_type ("SFH", "TH", "MFH", "AB").
    Use this to discover what archetypes are available before looking one up.
    """
    keys = list_archetype_keys()
    if building_type:
        keys = [k for k in keys if f"_{building_type}_" in k]
    return json.dumps({
        "status": "success",
        "count": len(keys),
        "archetypes": keys,
        "building_types": BUILDING_TYPES,
        "year_classes": YEAR_CLASSES,
        "scenarios": SCENARIOS,
    }, indent=2)

@mcp.tool()
def calculate_building_energy(
    geometry_payload: dict,
    building_type: str = "SFH",
    year_class: str = "2016-...",
    scenario: str = "Existing State",
    story_height_m: float = 2.8,
    number_of_stories: int = 1,
    window_to_wall_ratio: float = 0.15,
    climate_factor_kkh: float = 66.0,
    air_exchange_rate_1_h: float = 0.5,
    name: str = "My Building",
    ventilation_system: str = "Natural",
    heating_system: str = "Gas Condensing Boiler",
    building_rotation_deg: float = 0.0,
    windows_by_orientation: dict = None,
) -> str:
    """
    ORIENTATION-AWARE BUILDING ENERGY CALCULATION (v3).
    Takes a 2D zone layout and TABULA parameters, then automatically calculates everything.
    """
    try:
        zones = geometry_payload.get("building_zones", [])
        if not zones:
            return json.dumps({
                "status": "error",
                "message": "No building_zones found in geometry_payload."
            }, indent=2)

        geo_result = compute_exterior_envelope(
            zones,
            building_rotation_deg=building_rotation_deg,
        )
        
        if geo_result.gross_floor_area_m2 == 0:
            return json.dumps({"status": "error", "message": "Zero floor area."}, indent=2)

        areas = compute_building_areas(
            geo_result,
            story_height_m=story_height_m,
            number_of_stories=number_of_stories,
            window_to_wall_ratio=window_to_wall_ratio,
            windows_by_orientation=windows_by_orientation,
        )

        windows_by_dir = areas["windows_by_orientation_m2"]

        engine_result = calculate_energy(
            building_type=building_type,
            year_class=year_class,
            scenario=scenario,
            sketchpad_areas=areas,
            windows_by_orientation=windows_by_dir,
            heating_system=heating_system
        )

        if engine_result["status"] == "error":
            return json.dumps(engine_result, indent=2)

        final_response = {
            "status": "success",
            "building_name": name,
            "geometry_analysis": {
                "zones_count": geo_result.zone_count,
                "merged_footprint_area_m2": geo_result.gross_floor_area_m2,
                "exterior_perimeter_m": geo_result.exterior_perimeter_m,
            }
        }
        final_response.update(engine_result)
        return json.dumps(final_response, indent=2)

    except Exception as e:
        return json.dumps({"status": "error", "message": str(e)}, indent=2)


@mcp.tool()
def start_working_on_project(path: str, ctx: Context) -> dict:
    """🔴Set the active project for this MCP session. MUST be called first.
    Path: absolute path to a folder containing .coda/project.json.
    """
    return _get_mcp_session(ctx).start_working_on_project(path)


@mcp.tool()
def start_run(ctx: Context) -> dict:
    """📂Start a new run in the active project. Creates run directory under .coda/runs.
    MUST call start_working_on_project first.
    """
    return _get_mcp_session(ctx).start_run()


@mcp.tool()
def start_iteration(ctx: Context, run_id: str | None = None) -> dict:
    """🟠Start a new iteration in the active or specified run.
    MUST call start_run first (or specify run_id).
    """
    return _get_mcp_session(ctx).start_iteration(run_id)


@mcp.tool()
def start_translation(target_id: str, ctx: Context) -> dict:
    """🟡Set the active target and prepare for translation.
    MUST call start_iteration first.
    """
    return _get_mcp_session(ctx).start_translation(target_id)


@mcp.tool()
def get_status(ctx: Context) -> dict:
    """🟢Return the current session state (project, run, iteration, target).
    """
    return _get_mcp_session(ctx).get_status()


@mcp.tool()
def translate(target_id: str, ctx: Context) -> dict:
    """🟣Translate design to target format by invoking the translator subagent.
    Uses the session's active iteration or falls back to latest.
    """
    sess = _get_mcp_session(ctx)
    if sess.project_root and sess.iteration_dir:
        return sess.start_translation(target_id)
    # Fallback: use helper-based resolution for backward compatibility
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    if not any(t.get("id") == target_id for t in targets):
        return {
            "error": f"Target not in project: {target_id}",
            "targets": [t.get("id") for t in targets],
        }
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found. Call start_run first."}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found. Call start_iteration first."}
    target_dir = iter_dir / "targets" / target_id
    target_dir.mkdir(parents=True, exist_ok=True)
    design_id = proj.get("design", {}).get("id", "unknown")
    design_slug = design_id.split(".")[-1] if "." in design_id else design_id
    target_slug = target_id.split(".")[-1] if "." in target_id else target_id
    agent_name = f"{design_slug}-to-{target_slug}-translation-subagent"
    return {
        "action": "invoke_subagent",
        "agent_name": agent_name,
        "target_id": target_id,
        "design_id": design_id,
        "output_path": str(target_dir / "translation.json"),
        "instruction": (
            f"Invoke the @{agent_name} subagent to translate the {design_id} "
            f"design into the {target_id} target format. "
            f"The subagent MUST write its output to {target_dir / 'translation.json'}."
        ),
    }


@mcp.tool()
def save_translation(target_id: str, data: str) -> dict:
    """🟤Save translation output for a target in the current iteration.
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    target_dir = iter_dir / "targets" / target_id
    target_dir.mkdir(parents=True, exist_ok=True)
    translation_path = target_dir / "translation.json"
    translation_path.write_text(data, encoding="utf-8")
    return {"saved": True, "path": str(translation_path)}


@mcp.tool()
def validate(target_id: str) -> dict:
    """⚪Validate a target by running its validator on the translation output.
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    target_cfg = next((t for t in targets if t.get("id") == target_id), None)
    if not target_cfg:
        return {"error": f"Target not in project: {target_id}"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    translation_path = iter_dir / "targets" / target_id / "translation.json"
    if not translation_path.exists():
        return {
            "error": f"No translation found for target: {target_id}. Call translate first."
        }

    validator_cfg = (target_cfg or {}).get("validator", {}) or {}
    validator_kind = str(validator_cfg.get("kind") or "binary").lower()

    if validator_kind == "ontology":
        try:
            report = _run_ontology_validator(target_id, translation_path, validator_cfg)
        except Exception as e:
            return {
                "error": f"Ontology validator failed for target: {target_id}: {e!s}"
            }
        report_path = iter_dir / "targets" / target_id / "report.json"
        report_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
        return {
            "validated": True,
            "target_id": target_id,
            "report_path": str(report_path),
            "report": report,
        }
    validators_dir = root / ".coda" / "validators"
    validator_bin = None
    if validators_dir.exists():
        for candidate in [target_id, target_id.replace(".", "-")]:
            for ext in ["", ".exe"]:
                p = validators_dir / f"{candidate}{ext}"
                if p.exists() and p.is_file():
                    validator_bin = p
                    break
            if validator_bin:
                break
    if validator_bin:
        try:
            result = subprocess.run(
                [str(validator_bin)],
                input=translation_path.read_text(encoding="utf-8"),
                capture_output=True,
                text=True,
                timeout=120,
                cwd=str(root),
            )
            report_raw = result.stdout.strip() if result.stdout else "{}"
            envelope = _ensure_validation_envelope(report_raw)
            report_path = iter_dir / "targets" / target_id / "report.json"
            report_path.write_text(json.dumps(envelope, indent=2), encoding="utf-8")
            return {
                "validated": True,
                "target_id": target_id,
                "report_path": str(report_path),
                "report": envelope,
            }
        except subprocess.TimeoutExpired:
            return {"error": f"Validator timed out for target: {target_id}"}
        except Exception as e:
            return {"error": f"Validator failed for target: {target_id}: {e!s}"}
    return {
        "action": "manual_validation",
        "target_id": target_id,
        "translation_path": str(translation_path),
        "instruction": (
            f"No validator binary found for {target_id}. "
            f"Validate the translation manually or provide a validator in "
            f".coda/validators/{target_id}."
        ),
    }


@mcp.tool()
def save_report(report_data: str) -> dict:
    """⚫Save aggregated report for the current iteration.
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    report_path = iter_dir / "targets" / "report.json"
    report_path.write_text(report_data, encoding="utf-8")
    return {"saved": True, "path": str(report_path)}


@mcp.tool()
def fix(prompt: str) -> dict:
    """🔧Fix design to address breachs by invoking the fixer subagent.
    """
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    design_id = proj.get("design", {}).get("id", "unknown")
    design_slug = design_id.split(".")[-1] if "." in design_id else design_id
    agent_name = f"{design_slug}-fixing-subagent"
    config = _get_coda_config()
    measures = config.get("measures", [])
    run_dir = _get_latest_run(root)
    report = {}
    if run_dir:
        iter_dir = _get_latest_iteration(run_dir)
        if iter_dir:
            report_path = iter_dir / "targets" / "report.json"
            if report_path.exists():
                report = json.loads(report_path.read_text(encoding="utf-8"))
    return {
        "action": "invoke_subagent",
        "agent_name": agent_name,
        "design_id": design_id,
        "prompt": prompt,
        "report": report,
        "available_measures": [m.get("id") for m in measures],
        "instruction": (
            f"Invoke the @{agent_name} subagent to fix the design using the "
            f"{design_id} MCP server. The subagent MUST use the design MCP tools "
            f"to address: {prompt}"
        ),
    }




@mcp.tool(meta=_WORKSPACE_APP_META)
def show_coda_workspace(ctx: Context, panel: str = "dashboard") -> CallToolResult:
    """Show the coda workspace in an MCP App. Panels: dashboard, config, runs, report, translations, actions, events."""
    p = (panel or "dashboard").strip().lower()
    allowed = {
        "dashboard",
        "config",
        "runs",
        "report",
        "translations",
        "actions",
        "events",
    }
    if p not in allowed:
        p = "dashboard"
    try:
        return _as_mcp_app_tool_result(
            _gather_workspace_payload(_get_mcp_session(ctx), p)
        )
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


# #endregion 🥁Tools
# #region 📋Prompts
# Prompts MUST expose MCP prompt handlers for design change instructions.

@mcp.prompt()
def energy_consultant_prompt() -> str:
    """System prompt for building energy consultation — MCP App workflow."""
    return PROMPT_TEXT


@mcp.prompt()
def change(prompt: str) -> str:
    """♻️Change the design according to the given prompt. Use with the fixer agent.
    """
    return f"Change the design to address the following: {prompt}"


# #endregion 📋Prompts

# #region 🧱Sidecar
# Sidecar MUST implement a JSON-over-stdio protocol for Electron integration.
# Protocol: one JSON object per line on stdin/stdout.
# Each request: {"id": "<uuid>", "method": "<name>", "params": {...}}
# Each response: {"id": "<uuid>", "result": {...}} or {"id": "<uuid>", "error": {...}}
# Heartbeat: method "heartbeat" → {"status": "alive", ...}

_SIDECAR_METHODS: dict[str, callable] = {}


def _register_sidecar(name: str):
    """🎯Decorator to register a sidecar method handler."""

    def decorator(fn):
        _SIDECAR_METHODS[name] = fn
        return fn

    return decorator


@_register_sidecar("heartbeat")
def _sidecar_heartbeat(params: dict) -> dict:
    return {"status": "alive", "timestamp": time.time()}


@_register_sidecar("start_working_on_project")
def _sidecar_start_working_on_project(params: dict) -> dict:
    path = params.get("path", "")
    if not path:
        return {"error": "Missing 'path' parameter"}
    r = _sidecar_session.start_working_on_project(path)
    if r.get("ok"):
        _emit_event("project_ready", {"project_root": str(path)})
    return r


@_register_sidecar("start_run")
def _sidecar_start_run(params: dict) -> dict:
    r = _sidecar_session.start_run()
    if "error" not in r:
        _emit_event("run_started", {"run_id": r.get("run_id"), "path": r.get("path")})
    return r


@_register_sidecar("start_iteration")
def _sidecar_start_iteration(params: dict) -> dict:
    r = _sidecar_session.start_iteration(params.get("run_id"))
    if "error" not in r:
        _emit_event(
            "iteration_started",
            {
                "iteration_index": r.get("iteration_index"),
                "run_id": r.get("run_id"),
                "path": r.get("path"),
            },
        )
    return r


@_register_sidecar("start_translation")
def _sidecar_start_translation(params: dict) -> dict:
    target_id = params.get("target_id", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    return _sidecar_session.start_translation(target_id)


@_register_sidecar("get_status")
def _sidecar_get_status(params: dict) -> dict:
    return _sidecar_session.get_status()


@_register_sidecar("get_properties")
def _sidecar_get_properties(params: dict) -> dict:
    config = _get_coda_config()
    return [
        _normalize_property_definition(p) if isinstance(p, dict) else p
        for p in config.get("properties", [])
    ]


@_register_sidecar("get_frameworks")
def _sidecar_get_frameworks(params: dict) -> dict:
    config = _get_coda_config()
    return [_normalize_target_definition(t) for t in config.get("targets", [])]


@_register_sidecar("get_measures")
def _sidecar_get_measures(params: dict) -> dict:
    config = _get_coda_config()
    return config.get("measures", [])


@_register_sidecar("get_targets")
def _sidecar_get_targets(params: dict) -> dict:
    config = _get_coda_config()
    return [_normalize_target_definition(t) for t in config.get("targets", [])]


@_register_sidecar("get_project")
def _sidecar_get_project(params: dict) -> dict:
    proj = _get_project_config()
    if proj is None:
        return {"error": "No coda project found"}
    return proj


@_register_sidecar("get_report")
def _sidecar_get_report(params: dict) -> dict:
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    report_json = iter_dir / "targets" / "report.json"
    if not report_json.exists():
        return {"error": "No report found"}
    return json.loads(report_json.read_text(encoding="utf-8"))


@_register_sidecar("get_breachs")
def _sidecar_get_breachs(params: dict) -> dict:
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    report_json = iter_dir / "targets" / "report.json"
    if not report_json.exists():
        return []
    report = json.loads(report_json.read_text(encoding="utf-8"))
    if "breachs" in report:
        return report.get("breachs", [])
    if "validations" in report and isinstance(report["validations"], list):
        return [v for v in report["validations"] if v.get("truth") == "false"]
    return []


@_register_sidecar("get_iterations")
def _sidecar_get_iterations(params: dict) -> dict:
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return []
    iters = run_dir / "iterations"
    if not iters.exists():
        return []
    dirs = [d for d in iters.iterdir() if d.is_dir() and d.name.isdigit()]
    entries = [{"index": d.name} for d in sorted(dirs, key=lambda x: int(x.name))]
    return entries


@_register_sidecar("get_platforms")
def _sidecar_get_platforms(params: dict) -> dict:
    # Try project-local .coda/platforms/ directory first
    root = _get_project_root()
    if root:
        platforms_dir = root / ".coda" / "platforms"
        if platforms_dir.exists():
            platforms = []
            for p in platforms_dir.iterdir():
                if p.is_file() and p.suffix == ".json":
                    try:
                        platforms.append(json.loads(p.read_text(encoding="utf-8")))
                    except Exception:
                        pass
            if platforms:
                return platforms
    # Fall back to coda.json platforms
    config = _get_coda_config()
    return config.get("platforms", [])


@_register_sidecar("get_current_run")
def _sidecar_get_current_run(params: dict) -> dict:
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    run_json = run_dir / "run.json"
    if run_json.exists():
        return json.loads(run_json.read_text(encoding="utf-8"))
    return {"id": run_dir.name, "run_id": run_dir.name}


@_register_sidecar("get_current_iteration")
def _sidecar_get_current_iteration(params: dict) -> dict:
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    iter_json = iter_dir / "iteration.json"
    if iter_json.exists():
        return json.loads(iter_json.read_text(encoding="utf-8"))
    return {"index": iter_dir.name}


@_register_sidecar("get_translation")
def _sidecar_get_translation(params: dict) -> dict:
    target_id = params.get("target_id", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    translation_json = iter_dir / "targets" / target_id / "translation.json"
    if not translation_json.exists():
        return {"error": f"No translation found for target: {target_id}"}
    return json.loads(translation_json.read_text(encoding="utf-8"))


@_register_sidecar("translate")
def _sidecar_translate(params: dict) -> dict:
    target_id = params.get("target_id", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    _emit_event("translate_requested", {"target_id": target_id})
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    if not any(t.get("id") == target_id for t in targets):
        return {"error": f"Target not in project: {target_id}"}
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found. Call start_run first."}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found. Call start_iteration first."}
    target_dir = iter_dir / "targets" / target_id
    target_dir.mkdir(parents=True, exist_ok=True)
    design_id = proj.get("design", {}).get("id", "unknown")
    design_slug = design_id.split(".")[-1] if "." in design_id else design_id
    target_slug = target_id.split(".")[-1] if "." in target_id else target_id
    agent_name = f"{design_slug}-to-{target_slug}-translation-subagent"
    result = {
        "action": "invoke_subagent",
        "agent_name": agent_name,
        "target_id": target_id,
        "design_id": design_id,
        "output_path": str(target_dir / "translation.json"),
        "instruction": f"Invoke the @{agent_name} subagent to translate the {design_id} design into the {target_id} target format.",
    }
    _emit_event(
        "translate_started",
        {"target_id": target_id, "agent_name": agent_name, "design_id": design_id},
    )
    return result


@_register_sidecar("save_validation")
def _sidecar_save_validation(params: dict) -> dict:
    target_id = params.get("target_id", "")
    data = params.get("data", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    report_path = iter_dir / "targets" / target_id / "report.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        data if isinstance(data, str) else json.dumps(data), encoding="utf-8"
    )
    result = {"saved": True, "path": str(report_path)}
    _emit_event("validation_saved", {"target_id": target_id, "path": str(report_path)})
    return result


@_register_sidecar("save_translation")
def _sidecar_save_translation(params: dict) -> dict:
    target_id = params.get("target_id", "")
    data = params.get("data", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    target_dir = iter_dir / "targets" / target_id
    target_dir.mkdir(parents=True, exist_ok=True)
    translation_path = target_dir / "translation.json"
    translation_path.write_text(data, encoding="utf-8")
    result = {"saved": True, "path": str(translation_path)}
    _emit_event("translation_saved", {"target_id": target_id, "path": result["path"]})
    return result


@_register_sidecar("save_report")
def _sidecar_save_report(params: dict) -> dict:
    data = params.get("report_data", "")
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    report_path = iter_dir / "targets" / "report.json"
    report_path.write_text(data, encoding="utf-8")
    result = {"saved": True, "path": str(report_path)}
    _emit_event("report_saved", {"path": result["path"]})
    return result


@_register_sidecar("validate")
def _sidecar_validate(params: dict) -> dict:
    target_id = params.get("target_id", "")
    if not target_id:
        return {"error": "Missing 'target_id' parameter"}
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    target_cfg = next((t for t in targets if t.get("id") == target_id), None)
    if not target_cfg:
        return {"error": f"Target not in project: {target_id}"}
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    if not run_dir:
        return {"error": "No runs found"}
    iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
    if not iter_dir:
        return {"error": "No iterations found"}
    translation_path = iter_dir / "targets" / target_id / "translation.json"
    if not translation_path.exists():
        return {"error": f"No translation for target: {target_id}"}

    validator_cfg = (target_cfg or {}).get("validator", {}) or {}
    validator_kind = str(validator_cfg.get("kind") or "binary").lower()

    if validator_kind == "ontology":
        try:
            report = _run_ontology_validator(target_id, translation_path, validator_cfg)
        except Exception as e:
            return {
                "error": f"Ontology validator failed for target: {target_id}: {e!s}"
            }
        report_path = iter_dir / "targets" / target_id / "report.json"
        report_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
        out = {
            "validated": True,
            "target_id": target_id,
            "report_path": str(report_path),
            "report": report,
        }
        _emit_event(
            "validation_completed",
            {"target_id": target_id, "path": str(report_path)},
        )
        return out

    validators_dir = root / ".coda" / "validators"
    validator_bin = None
    if validators_dir.exists():
        for candidate in [target_id, target_id.replace(".", "-")]:
            for ext in ["", ".exe"]:
                p = validators_dir / f"{candidate}{ext}"
                if p.exists() and p.is_file():
                    validator_bin = p
                    break
            if validator_bin:
                break
    if validator_bin:
        try:
            result = subprocess.run(
                [str(validator_bin)],
                input=translation_path.read_text(encoding="utf-8"),
                capture_output=True,
                text=True,
                timeout=120,
                cwd=str(root),
            )
            report_data = result.stdout.strip() if result.stdout else "{}"
            report_path = iter_dir / "targets" / target_id / "report.json"
            report_path.write_text(report_data, encoding="utf-8")
            out = {
                "validated": True,
                "target_id": target_id,
                "report_path": str(report_path),
                "report": json.loads(report_data) if report_data else {},
            }
            _emit_event(
                "validation_completed",
                {"target_id": target_id, "path": str(report_path)},
            )
            return out
        except subprocess.TimeoutExpired:
            return {"error": f"Validator timed out for target: {target_id}"}
        except Exception as e:
            return {"error": f"Validator failed: {e!s}"}
    return {
        "action": "manual_validation",
        "target_id": target_id,
        "translation_path": str(translation_path),
    }


@_register_sidecar("fix")
def _sidecar_fix(params: dict) -> dict:
    prompt = params.get("prompt", "")
    if not prompt:
        return {"error": "Missing 'prompt' parameter"}
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    root = _sidecar_session.project_root or _get_project_root()
    if not root:
        return {"error": "No project root"}
    design_id = proj.get("design", {}).get("id", "unknown")
    design_slug = design_id.split(".")[-1] if "." in design_id else design_id
    agent_name = f"{design_slug}-fixing-subagent"
    config = _get_coda_config()
    measures = config.get("measures", [])
    run_dir = _sidecar_session.run_dir or _get_latest_run(root)
    report = {}
    if run_dir:
        iter_dir = _sidecar_session.iteration_dir or _get_latest_iteration(run_dir)
        if iter_dir:
            report_path = iter_dir / "targets" / "report.json"
            if report_path.exists():
                report = json.loads(report_path.read_text(encoding="utf-8"))
    return {
        "action": "invoke_subagent",
        "agent_name": agent_name,
        "design_id": design_id,
        "prompt": prompt,
        "report": report,
        "available_measures": [m.get("id") for m in measures],
    }


def _handle_sidecar_request(request: dict) -> dict:
    """📩Dispatch a sidecar JSON request and return a response dict.
    """
    req_id = request.get("id")
    method = request.get("method", "")
    params = request.get("params", {})

    handler = _SIDECAR_METHODS.get(method)
    if not handler:
        return {
            "id": req_id,
            "error": {"code": -32601, "message": f"Unknown method: {method}"},
        }

    try:
        result = handler(params)
        if "error" in result and not any(k for k in result if k != "error"):
            return {"id": req_id, "error": result}
        return {"id": req_id, "result": result}
    except Exception as e:
        return {"id": req_id, "error": {"code": -32603, "message": str(e)}}


def _run_sidecar() -> None:
    """📡Run the sidecar stdio event loop. Reads JSON lines from stdin, writes responses to stdout.
    """
    global _SIDECAR_MODE
    _SIDECAR_MODE = True
    # Write a ready message so Electron knows we've started
    _write_stdout({"id": None, "result": {"status": "ready", "pid": os.getpid()}})

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            request = json.loads(line)
        except json.JSONDecodeError as e:
            _write_stdout(
                {"id": None, "error": {"code": -32700, "message": f"Parse error: {e}"}}
            )
            continue

        response = _handle_sidecar_request(request)
        _write_stdout(response)


def _write_stdout(obj: dict) -> None:
    """✏️Write a JSON object as a single line to stdout and flush."""
    sys.stdout.write(json.dumps(obj) + "\n")
    sys.stdout.flush()


# #endregion 🧱Sidecar

# #region 🕸️HTTP

async def _coda_http_payload_handler(request) -> JSONResponse:
    """📡Serve large payload bundles to clients via token lookups."""
    token = request.path_params.get("token")
    payload = _mcp_app_payloads.get(token)
    if payload is None:
        return JSONResponse({"error": "Payload not found"}, status_code=404)
    return JSONResponse(payload)


@contextlib.asynccontextmanager
async def _coda_http_app_lifespan(app):
    """🔄Manage the FastMCP session lifecycle during the HTTP server execution."""
    async with mcp.session_manager.run():
        yield


mcp.settings.streamable_http_path = "/"
_coda_streamable_app = mcp.streamable_http_app()

_coda_http_app = Starlette(
    lifespan=_coda_http_app_lifespan,
    routes=[
        Route("/app/payload/{token}", _coda_http_payload_handler, methods=["GET", "OPTIONS"]),
    ],
)

from starlette.middleware.cors import CORSMiddleware
_coda_http_app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

_coda_http_app.mount("/mcp", _coda_streamable_app)

# #endregion 🕸️HTTP

# #region 🐼Main
# Main MUST provide the CLI entry point supporting MCP (stdio/HTTP) and sidecar modes.


def main() -> None:
    """🔬Parses CLI arguments and starts in the selected mode.
    --sidecar: Electron sidecar (JSON-over-stdio).
    --mcp-stdio: MCP server over stdio.
    Default: HTTP server on 127.0.0.1:8080 with MCP at /mcp and MCP App routes under /app/*.
    """
    parser = argparse.ArgumentParser(description="coda - ACC design assistant")
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "--sidecar",
        action="store_true",
        help="Run as Electron sidecar (JSON-over-stdio)",
    )
    group.add_argument(
        "--mcp-stdio",
        action="store_true",
        help="Run MCP server over stdio",
    )
    parser.add_argument(
        "--project",
        type=str,
        default=None,
        help="Initial project path (sidecar mode)",
    )
    args = parser.parse_args()

    if args.sidecar:
        if args.project:
            _sidecar_session.start_working_on_project(args.project)
        _run_sidecar()
    elif args.mcp_stdio:
        mcp.run(transport="stdio")
    else:
        global _HTTP_PAYLOADS_ENABLED
        _HTTP_PAYLOADS_ENABLED = True
        logging.basicConfig(level=logging.INFO)
        uvicorn.run(
            _coda_http_app,
            host="127.0.0.1",
            port=CODA_HTTP_PORT,
            log_level="info",
            access_log=False,
            log_config=None,
        )


if __name__ == "__main__":
    main()

# #endregion 🐼Main
