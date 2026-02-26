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

# endregion Resources
# region Tools
# [🔬coda📚py💻coda🔖tools](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools)
# Tools MUST expose MCP tool handlers for run management, translation, and design fixes.

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
    (run_dir / "run.json").write_text("{}", encoding="utf-8")
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
            return {"error": "No runs found"}
    iters_dir = run_dir / "iterations"
    iters_dir.mkdir(parents=True, exist_ok=True)
    existing = [
        int(d.name) for d in iters_dir.iterdir() if d.is_dir() and d.name.isdigit()
    ]
    idx = max(existing, default=-1) + 1
    iter_dir = iters_dir / str(idx)
    iter_dir.mkdir()
    (iter_dir / "targets").mkdir()
    (iter_dir / "iteration.json").write_text("{}", encoding="utf-8")
    return {"run_id": run_dir.name, "iteration_index": idx, "path": str(iter_dir)}

@mcp.tool()
def translate(target_id: str) -> dict:
    """Translate design to target format. Invokes the translator agent for the given target.
    Implementations MUST verify the target exists in the project before invoking translation.
    [🔬coda📚py💻coda🔖tools🛠️translate](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/translate)
    """
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    if not any(t.get("id") == target_id for t in targets):
        return {"error": f"Target not in project: {target_id}", "targets": targets}
    return {
        "message": "Translate tool: invoke DESIGN-to-TARGET translator agent",
        "target_id": target_id}

@mcp.tool()
def fix(prompt: str) -> dict:
    """Fix design to address breachs. Invokes the fixer agent with the given prompt.
    Implementations MUST verify project existence before invoking the fixer.
    [🔬coda📚py💻coda🔖tools🛠️fix](semiorepo://p/r/coda/b/l/py/f/coda.py/s/Tools/d/i/fix)
    """
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    return {"message": "Fix tool: invoke fixer agent via design MCP", "prompt": prompt}

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
