# region Header
# [🔬coda📚py💻coda](semiorepo://p/r/coda/b/l/py/f/coda.py)

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

# endregion Header

# region Imports
# [🔬coda📚py💻coda🔖imports](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Imports)
# Imports MUST include standard library, third-party FastMCP, and module-level configuration.

"""coda MCP server - ACC design assistant with resources, tools, and prompts."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
from pathlib import Path

from mcp.server.fastmcp import FastMCP

mcp = FastMCP("coda", json_response=True)

_CODA_ROOT = Path(__file__).resolve().parent
_CODA_JSON_PATH = _CODA_ROOT / "coda.json"
_PROJECT_ENV = "CODA_PROJECT"

# endregion Imports

# region Helpers
# [🔬coda📚py💻coda🔖helpers](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers)
# Helpers MUST provide private functions for config loading and project root resolution.


def _load_json_with_comments(path: Path) -> dict:
    """Load JSON file, stripping // line comments.
    _load_json_with_comments MUST perform the _load_json_with_comments operation.
    [🔬coda📚py💻coda🔖helpers🛠️loadjsonwithcomments](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_load_json_with_comments)
    """
    text = path.read_text(encoding="utf-8")
    text = re.sub(r"\s*//.*$", "", text, flags=re.MULTILINE)
    return json.loads(text)


def _get_project_root() -> Path | None:
    """Resolve project root from CODA_PROJECT or cwd.
    _get_project_root MUST perform the _get_project_root operation.
    [🔬coda📚py💻coda🔖helpers🛠️getprojectroot](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_get_project_root)
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
    """_get_coda_config performs the _get_coda_config operation.
    [🔬coda📚py💻coda🔖helpers🛠️getcodaconfig](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_get_coda_config)
    _get_coda_config MUST perform the _get_coda_config operation.
    """
    config_path = Path(os.environ.get("CODA_CONFIG", _CODA_JSON_PATH))
    if not config_path.is_absolute():
        config_path = _CODA_ROOT / config_path
    return _load_json_with_comments(config_path)


def _get_project_config() -> dict | None:
    """_get_project_config performs the _get_project_config operation.
    [🔬coda📚py💻coda🔖helpers🛠️getprojectconfig](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_get_project_config)
    _get_project_config MUST perform the _get_project_config operation.
    """
    root = _get_project_root()
    if not root:
        return None
    path = root / ".coda" / "project.json"
    return json.loads(path.read_text(encoding="utf-8")) if path.exists() else None


def _get_latest_run(root: Path) -> Path | None:
    """_get_latest_run performs the _get_latest_run operation.
    [🔬coda📚py💻coda🔖helpers🛠️getlatestrun](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_get_latest_run)
    _get_latest_run MUST perform the _get_latest_run operation.
    """
    runs = root / ".coda" / "runs"
    if not runs.exists():
        return None
    dirs = sorted(d for d in runs.iterdir() if d.is_dir())
    return dirs[-1] if dirs else None


def _get_latest_iteration(run_dir: Path) -> Path | None:
    """_get_latest_iteration performs the _get_latest_iteration operation.
    [🔬coda📚py💻coda🔖helpers🛠️getlatestiteration](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Helpers/d/i/_get_latest_iteration)
    _get_latest_iteration MUST perform the _get_latest_iteration operation.
    """
    iters = run_dir / "iterations"
    if not iters.exists():
        return None
    dirs = sorted(
        int(d.name) for d in iters.iterdir() if d.is_dir() and d.name.isdigit()
    )
    return iters / str(dirs[-1]) if dirs else None


# endregion Helpers

# region Resources
# [🔬coda📚py💻coda🔖resources](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources)
# Resources MUST expose MCP resource handlers for measures, targets, properties, rules, and project data.


@mcp.resource("coda://measures")
def get_measures() -> str:
    """List all measures that are available.
    Implementations MUST load the coda config and return the measures array.
    [🔬coda📚py💻coda🔖resources🛠️getmeasures](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_measures)
    """
    config = _get_coda_config()
    return json.dumps(config.get("measures", []), indent=2)


@mcp.resource("coda://measure/{id}")
def get_measure(id: str) -> str:
    """Get a measure by id.
    Implementations MUST return an error JSON object when the measure is not found.
    [🔬coda📚py💻coda🔖resources🛠️getmeasure](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_measure)
    """
    config = _get_coda_config()
    for m in config.get("measures", []):
        if m.get("id") == id:
            return json.dumps(m, indent=2)
    return json.dumps({"error": f"measure not found: {id}"})


@mcp.resource("coda://targets")
def get_targets() -> str:
    """List all targets.
    Implementations MUST load the coda config and return the targets array.
    [🔬coda📚py💻coda🔖resources🛠️gettargets](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_targets)
    """
    config = _get_coda_config()
    return json.dumps(config.get("targets", []), indent=2)


@mcp.resource("coda://target/{id}")
def get_target(id: str) -> str:
    """Get a target by id.
    Implementations MUST return an error JSON object when the target is not found.
    [🔬coda📚py💻coda🔖resources🛠️gettarget](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_target)
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == id:
            return json.dumps(t, indent=2)
    return json.dumps({"error": f"target not found: {id}"})


@mcp.resource("coda://{target_id}/properties")
def get_target_properties(target_id: str) -> str:
    """Get properties for a target.
    Implementations MUST return an error JSON object when the target is not found.
    [🔬coda📚py💻coda🔖resources🛠️gettargetproperties](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_target_properties)
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            return json.dumps(t.get("properties", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/property/{id}")
def get_target_property(target_id: str, id: str) -> str:
    """Get a property by id for a target.
    Implementations MUST return an error JSON object when the target or property is not found.
    [🔬coda📚py💻coda🔖resources🛠️gettargetproperty](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_target_property)
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            for p in t.get("properties", []):
                if p.get("id") == id:
                    return json.dumps(p, indent=2)
            return json.dumps({"error": f"property not found: {id}"})
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/rules")
def get_target_rules(target_id: str) -> str:
    """Get rules for a target.
    Implementations MUST return an error JSON object when the target is not found.
    [🔬coda📚py💻coda🔖resources🛠️gettargetrules](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_target_rules)
    """
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            return json.dumps(t.get("rules", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})


@mcp.resource("coda://{target_id}/rule/{id}")
def get_target_rule(target_id: str, id: str) -> str:
    """Get a rule by id for a target.
    Implementations MUST return an error JSON object when the target or rule is not found.
    [🔬coda📚py💻coda🔖resources🛠️gettargetrule](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_target_rule)
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
    """Get the current project configuration.
    Implementations MUST return an error JSON object when no project root is found.
    [🔬coda📚py💻coda🔖resources🛠️getproject](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_project)
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
    """Get the current run metadata.
    Implementations MUST return an error JSON object when no project or run exists.
    [🔬coda📚py💻coda🔖resources🛠️getcurrentrun](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_current_run)
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
    """Get the current iteration metadata.
    Implementations MUST return an error JSON object when no project, run, or iteration exists.
    [🔬coda📚py💻coda🔖resources🛠️getcurrentiteration](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_current_iteration)
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
    """List iterations in the current run.
    Implementations MUST return an empty array when no runs or iterations exist.
    [🔬coda📚py💻coda🔖resources🛠️getiterations](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_iterations)
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
    """Get the current report from the latest iteration.
    [🔬coda📚py💻coda🔖resources🛠️getreport](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_report)
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
    """List all platforms with their measure instructions.
    Implementations MUST load the coda config and return the platforms array.
    [🔬coda📚py💻coda🔖resources🛠️getplatforms](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_platforms)
    """
    config = _get_coda_config()
    return json.dumps(config.get("platforms", []), indent=2)


@mcp.resource("coda://platform/{id}")
def get_platform(id: str) -> str:
    """Get a platform by id with its measure instructions.
    Implementations MUST return an error JSON object when the platform is not found.
    [🔬coda📚py💻coda🔖resources🛠️getplatform](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_platform)
    """
    config = _get_coda_config()
    for p in config.get("platforms", []):
        if p.get("id") == id:
            return json.dumps(p, indent=2)
    return json.dumps({"error": f"platform not found: {id}"})


@mcp.resource("coda://breachs")
def get_breachs() -> str:
    """Get breachs from the current report of the latest iteration.
    Implementations MUST return an empty array when no breachs exist.
    [🔬coda📚py💻coda🔖resources🛠️getbreachs](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_breachs)
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
    """Get the translation output for a target in the current iteration.
    Implementations MUST return an error JSON object when no translation exists.
    [🔬coda📚py💻coda🔖resources🛠️gettranslation](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Resources/d/i/get_translation)
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


# endregion Resources
# region Tools
# [🔬coda📚py💻coda🔖tools](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools)
# Tools MUST expose MCP tool handlers for run management, translation, validation, and design fixes.


@mcp.tool()
def start_run() -> dict:
    """Start a new run. Creates a new run directory under .coda/runs.
    Implementations MUST create the run directory with run.json and iterations subdirectory.
    [🔬coda📚py💻coda🔖tools🛠️startrun](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/start_run)
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    from datetime import datetime, timezone

    run_id = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"
    run_dir = root / ".coda" / "runs" / run_id
    run_dir.mkdir(parents=True)
    (run_dir / "run.json").write_text(
        json.dumps({"id": run_id, "started": run_id}, indent=2), encoding="utf-8"
    )
    (run_dir / "iterations").mkdir()
    return {"run_id": run_id, "path": str(run_dir)}


@mcp.tool()
def start_iteration(run_id: str | None = None) -> dict:
    """Start a new iteration in the current or specified run.
    Implementations MUST create the iteration directory with targets subdirectory and iteration.json.
    [🔬coda📚py💻coda🔖tools🛠️startiteration](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/start_iteration)
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    runs_dir = root / ".coda" / "runs"
    if run_id:
        run_dir = runs_dir / run_id
        if not run_dir.exists():
            return {"error": f"Run not found: {run_id}"}
    else:
        run_dir = _get_latest_run(root)
        if not run_dir:
            return {"error": "No runs found. Call start_run first."}
    iters_dir = run_dir / "iterations"
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
        json.dumps({"index": idx, "targets": target_ids}, indent=2), encoding="utf-8"
    )
    return {
        "run_id": run_dir.name,
        "iteration_index": idx,
        "path": str(iter_dir),
        "targets": target_ids,
    }


@mcp.tool()
def translate(target_id: str) -> dict:
    """Translate design to target format by invoking the translator subagent.
    The translator subagent is identified by the design and target ids in the project config.
    The coda main agent MUST invoke the appropriate translator subagent after calling this tool.
    Implementations MUST verify the target exists in the project before proceeding.
    [🔬coda📚py💻coda🔖tools🛠️translate](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/translate)
    """
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
        "instruction": f"Invoke the @{agent_name} subagent to translate the {design_id} design into the {target_id} target format. The subagent MUST write its output to {target_dir / 'translation.json'}.",
    }


@mcp.tool()
def save_translation(target_id: str, data: str) -> dict:
    """Save translation output for a target in the current iteration.
    Implementations MUST write the data to the target translation.json file.
    [🔬coda📚py💻coda🔖tools🛠️savetranslation](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/save_translation)
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
    """Validate a target by running its validator on the translation output.
    Implementations MUST locate the validator binary in .coda/validators/ and run it.
    If no validator binary exists, returns an instruction to validate manually.
    [🔬coda📚py💻coda🔖tools🛠️validate](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/validate)
    """
    root = _get_project_root()
    if not root:
        return {"error": "No project root"}
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    if not any(t.get("id") == target_id for t in targets):
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
            return {
                "validated": True,
                "target_id": target_id,
                "report_path": str(report_path),
                "report": json.loads(report_data) if report_data else {},
            }
        except subprocess.TimeoutExpired:
            return {"error": f"Validator timed out for target: {target_id}"}
        except Exception as e:
            return {"error": f"Validator failed for target: {target_id}: {e!s}"}
    return {
        "action": "manual_validation",
        "target_id": target_id,
        "translation_path": str(translation_path),
        "instruction": f"No validator binary found for {target_id}. Validate the translation manually or provide a validator in .coda/validators/{target_id}.",
    }


@mcp.tool()
def save_report(report_data: str) -> dict:
    """Save aggregated report for the current iteration.
    Implementations MUST write the report data to the iteration targets/report.json file.
    [🔬coda📚py💻coda🔖tools🛠️savereport](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/save_report)
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
    """Fix design to address breachs by invoking the fixer subagent.
    The fixer subagent uses the design MCP server to modify the design.
    The coda main agent MUST invoke the fixer subagent after calling this tool.
    Implementations MUST verify project existence before proceeding.
    [🔬coda📚py💻coda🔖tools🛠️fix](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/fix)
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
        "instruction": f"Invoke the @{agent_name} subagent to fix the design using the {design_id} MCP server. The subagent MUST use the design MCP tools to address: {prompt}",
    }


# endregion Tools
# region Prompts
# [🔬coda📚py💻coda🔖prompts](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Prompts)
# Prompts MUST expose MCP prompt handlers for design change instructions.


@mcp.prompt()
def change(prompt: str) -> str:
    """Change the design according to the given prompt. Use with the fixer agent.
    Implementations MUST return a formatted instruction string from the user prompt.
    [🔬coda📚py💻coda🔖prompts🛠️change](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Prompts/d/i/change)
    """
    return f"Change the design to address the following: {prompt}"


# endregion Prompts
# region Main
# [🔬coda📚py💻coda🔖main](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Main)
# Main MUST provide the CLI entry point for the coda MCP server.


def main() -> None:
    """Parses CLI arguments and starts the MCP server.
    Implementations MUST support both stdio and HTTP transport modes.
    [🔬coda📚py💻coda🔖main🛠️main](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Main/d/i/main)
    """
    parser = argparse.ArgumentParser(description="coda MCP server")
    parser.add_argument(
        "--mcp-stdio", action="store_true", help="Run MCP server over stdio"
    )
    args = parser.parse_args()
    if args.mcp_stdio:
        mcp.run(transport="stdio")
    else:
        mcp.run(transport="streamable-http", host="127.0.0.1", port=8080)


if __name__ == "__main__":
    main()

# endregion Main
