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

# #region ⭐Imports
# Imports MUST include standard library, third-party FastMCP, and module-level configuration.

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
import uuid
import weakref
from pathlib import Path

import rdflib
import uvicorn
from mcp.server.fastmcp import Context, FastMCP
from mcp.types import CallToolResult, EmbeddedResource, TextContent, TextResourceContents
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

# #endregion Imports

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
