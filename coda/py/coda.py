# region Header

# [💻coda/py/coda.py](semiorepo://file/coda/py/coda.py)

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

# [🔖coda/py/coda.py#Imports](semiorepo://section/coda/py/coda.py/IMPORTS)
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

# [🔖coda/py/coda.py#Helpers](semiorepo://section/coda/py/coda.py/HELPERS)
# Helpers MUST provide private functions for config loading and project root resolution.

def _load_json_with_comments(path: Path) -> dict:
    """Load JSON file, stripping // line comments."""
    text = path.read_text(encoding="utf-8")
    text = re.sub(r"\s*//.*$", "", text, flags=re.MULTILINE)
    return json.loads(text)

def _get_project_root() -> Path | None:
    """Resolve project root from CODA_PROJECT or cwd."""
    if val := os.environ.get(_PROJECT_ENV):
        p = Path(val).resolve()
        return p if (p / ".coda" / "project.json").exists() else None
    cwd = Path.cwd()
    for d in [cwd, *cwd.parents]:
        if (d / ".coda" / "project.json").exists():
            return d
    return None

def _get_coda_config() -> dict:
    config_path = Path(os.environ.get("CODA_CONFIG", _CODA_JSON_PATH))
    if not config_path.is_absolute():
        config_path = _CODA_ROOT / config_path
    return _load_json_with_comments(config_path)

def _get_project_config() -> dict | None:
    root = _get_project_root()
    if not root:
        return None
    path = root / ".coda" / "project.json"
    return json.loads(path.read_text(encoding="utf-8")) if path.exists() else None

def _get_latest_run(root: Path) -> Path | None:
    runs = root / ".coda" / "runs"
    if not runs.exists():
        return None
    dirs = sorted(d for d in runs.iterdir() if d.is_dir())
    return dirs[-1] if dirs else None

def _get_latest_iteration(run_dir: Path) -> Path | None:
    iters = run_dir / "iterations"
    if not iters.exists():
        return None
    dirs = sorted(int(d.name) for d in iters.iterdir() if d.is_dir() and d.name.isdigit())
    return iters / str(dirs[-1]) if dirs else None

# endregion Helpers

# region Resources

# [🔖coda/py/coda.py#Resources](semiorepo://section/coda/py/coda.py/RESOURCES)
# Resources MUST expose MCP resource handlers for measures, targets, properties, rules, and project data.

@mcp.resource("coda://measures")
# Lists all available measures as JSON from the coda configuration.
# Implementations MUST load the coda config and return the measures array.
# [🛠️coda/py/coda.py#Resources§get_measures](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-MEASURES)
def get_measures() -> str:
    """List all measures that are available."""
    config = _get_coda_config()
    return json.dumps(config.get("measures", []), indent=2)

@mcp.resource("coda://measure/{id}")
# Retrieves a single measure by its id.
# Implementations MUST return an error JSON object when the measure is not found.
# [🛠️coda/py/coda.py#Resources§get_measure](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-MEASURE)
def get_measure(id: str) -> str:
    """Get a measure by id."""
    config = _get_coda_config()
    for m in config.get("measures", []):
        if m.get("id") == id:
            return json.dumps(m, indent=2)
    return json.dumps({"error": f"measure not found: {id}"})

@mcp.resource("coda://targets")
# Lists all available targets as JSON.
# Implementations MUST load the coda config and return the targets array.
# [🛠️coda/py/coda.py#Resources§get_targets](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGETS)
def get_targets() -> str:
    """List all targets."""
    config = _get_coda_config()
    return json.dumps(config.get("targets", []), indent=2)

@mcp.resource("coda://target/{id}")
# Retrieves a single target by its id.
# Implementations MUST return an error JSON object when the target is not found.
# [🛠️coda/py/coda.py#Resources§get_target](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGET)
def get_target(id: str) -> str:
    """Get a target by id."""
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == id:
            return json.dumps(t, indent=2)
    return json.dumps({"error": f"target not found: {id}"})

@mcp.resource("coda://{target_id}/properties")
# Lists all properties for a given target.
# Implementations MUST return an error JSON object when the target is not found.
# [🛠️coda/py/coda.py#Resources§get_target_properties](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGET-PROPERTIES)
def get_target_properties(target_id: str) -> str:
    """Get properties for a target."""
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            return json.dumps(t.get("properties", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})

@mcp.resource("coda://{target_id}/property/{id}")
# Retrieves a single property by id for a given target.
# Implementations MUST return an error JSON object when the target or property is not found.
# [🛠️coda/py/coda.py#Resources§get_target_property](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGET-PROPERTY)
def get_target_property(target_id: str, id: str) -> str:
    """Get a property by id for a target."""
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            for p in t.get("properties", []):
                if p.get("id") == id:
                    return json.dumps(p, indent=2)
            return json.dumps({"error": f"property not found: {id}"})
    return json.dumps({"error": f"target not found: {target_id}"})

@mcp.resource("coda://{target_id}/rules")
# Lists all rules for a given target.
# Implementations MUST return an error JSON object when the target is not found.
# [🛠️coda/py/coda.py#Resources§get_target_rules](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGET-RULES)
def get_target_rules(target_id: str) -> str:
    """Get rules for a target."""
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            return json.dumps(t.get("rules", []), indent=2)
    return json.dumps({"error": f"target not found: {target_id}"})

@mcp.resource("coda://{target_id}/rule/{id}")
# Retrieves a single rule by id for a given target.
# Implementations MUST return an error JSON object when the target or rule is not found.
# [🛠️coda/py/coda.py#Resources§get_target_rule](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-TARGET-RULE)
def get_target_rule(target_id: str, id: str) -> str:
    """Get a rule by id for a target."""
    config = _get_coda_config()
    for t in config.get("targets", []):
        if t.get("id") == target_id:
            for r in t.get("rules", []):
                if r.get("id") == id:
                    return json.dumps(r, indent=2)
            return json.dumps({"error": f"rule not found: {id}"})
    return json.dumps({"error": f"target not found: {target_id}"})

@mcp.resource("coda://project")
# Returns the current project configuration.
# Implementations MUST return an error JSON object when no project root is found.
# [🛠️coda/py/coda.py#Resources§get_project](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-PROJECT)
def get_project() -> str:
    """Get the current project configuration."""
    proj = _get_project_config()
    if proj is None:
        return json.dumps({"error": "No coda project found. Set CODA_PROJECT or run from project root."})
    return json.dumps(proj, indent=2)

@mcp.resource("coda://current-run")
# Returns the metadata of the current run.
# Implementations MUST return an error JSON object when no project or run exists.
# [🛠️coda/py/coda.py#Resources§get_current_run](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-CURRENT-RUN)
def get_current_run() -> str:
    """Get the current run metadata."""
    root = _get_project_root()
    if not root:
        return json.dumps({"error": "No project root"})
    run_dir = _get_latest_run(root)
    if not run_dir:
        return json.dumps({"error": "No runs found"})
    run_json = run_dir / "run.json"
    data = json.loads(run_json.read_text(encoding="utf-8")) if run_json.exists() else {"id": run_dir.name}
    return json.dumps(data, indent=2)

@mcp.resource("coda://current-iteration")
# Returns the metadata of the current iteration.
# Implementations MUST return an error JSON object when no project, run, or iteration exists.
# [🛠️coda/py/coda.py#Resources§get_current_iteration](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-CURRENT-ITERATION)
def get_current_iteration() -> str:
    """Get the current iteration metadata."""
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
    data = json.loads(iter_json.read_text(encoding="utf-8")) if iter_json.exists() else {"index": iter_dir.name}
    return json.dumps(data, indent=2)

@mcp.resource("coda://iterations")
# Lists all iterations in the current run.
# Implementations MUST return an empty array when no runs or iterations exist.
# [🛠️coda/py/coda.py#Resources§get_iterations](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-ITERATIONS)
def get_iterations() -> str:
    """List iterations in the current run."""
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
# Returns the report from the latest iteration.
# Implementations MUST return an error JSON object when no project, run, iteration, or report exists.
# [🛠️coda/py/coda.py#Resources§get_report](semiorepo://definition/coda/py/coda.py/RESOURCES/GET-REPORT)
def get_report() -> str:
    """Get the current report from the latest iteration."""
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

# [🔖coda/py/coda.py#Tools](semiorepo://section/coda/py/coda.py/TOOLS)
# Tools MUST expose MCP tool handlers for run management, translation, and design fixes.

@mcp.tool()
# Starts a new run directory under .coda/runs.
# Implementations MUST create the run directory with run.json and iterations subdirectory.
# [🛠️coda/py/coda.py#Tools§start_run](semiorepo://definition/coda/py/coda.py/TOOLS/START-RUN)
def start_run() -> dict:
    """Start a new run. Creates a new run directory under .coda/runs."""
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
# Starts a new iteration in the current or specified run.
# Implementations MUST create the iteration directory with targets subdirectory and iteration.json.
# [🛠️coda/py/coda.py#Tools§start_iteration](semiorepo://definition/coda/py/coda.py/TOOLS/START-ITERATION)
def start_iteration(run_id: str | None = None) -> dict:
    """Start a new iteration in the current or specified run."""
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
    existing = [int(d.name) for d in iters_dir.iterdir() if d.is_dir() and d.name.isdigit()]
    idx = max(existing, default=-1) + 1
    iter_dir = iters_dir / str(idx)
    iter_dir.mkdir()
    (iter_dir / "targets").mkdir()
    (iter_dir / "iteration.json").write_text("{}", encoding="utf-8")
    return {"run_id": run_dir.name, "iteration_index": idx, "path": str(iter_dir)}

@mcp.tool()
# Translates a design to the specified target format.
# Implementations MUST verify the target exists in the project before invoking translation.
# [🛠️coda/py/coda.py#Tools§translate](semiorepo://definition/coda/py/coda.py/TOOLS/TRANSLATE)
def translate(target_id: str) -> dict:
    """Translate design to target format. Invokes the translator agent for the given target."""
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    targets = proj.get("targets", [])
    if not any(t.get("id") == target_id for t in targets):
        return {"error": f"Target not in project: {target_id}"}
    return {"message": "Translate tool: invoke DESIGN-to-TARGET translator agent", "target_id": target_id}

@mcp.tool()
# Fixes design violations using the fixer agent.
# Implementations MUST verify project existence before invoking the fixer.
# [🛠️coda/py/coda.py#Tools§fix](semiorepo://definition/coda/py/coda.py/TOOLS/FIX)
def fix(prompt: str) -> dict:
    """Fix design to address violations. Invokes the fixer agent with the given prompt."""
    proj = _get_project_config()
    if not proj:
        return {"error": "No project"}
    return {"message": "Fix tool: invoke fixer agent via design MCP", "prompt": prompt}

# endregion Tools

# region Prompts

# [🔖coda/py/coda.py#Prompts](semiorepo://section/coda/py/coda.py/PROMPTS)
# Prompts MUST expose MCP prompt handlers for design change instructions.

@mcp.prompt()
# Generates a design change prompt for the fixer agent.
# Implementations MUST return a formatted instruction string from the user prompt.
# [🛠️coda/py/coda.py#Prompts§change](semiorepo://definition/coda/py/coda.py/PROMPTS/CHANGE)
def change(prompt: str) -> str:
    """Change the design according to the given prompt. Use with the fixer agent."""
    return f"Change the design to address the following: {prompt}"

# endregion Prompts

# region Main

# [🔖coda/py/coda.py#Main](semiorepo://section/coda/py/coda.py/MAIN)
# Main MUST provide the CLI entry point for the coda MCP server.

# Parses CLI arguments and starts the MCP server.
# Implementations MUST support both stdio and HTTP transport modes.
# [🛠️coda/py/coda.py#Main§main](semiorepo://definition/coda/py/coda.py/MAIN/MAIN)
def main() -> None:
    parser = argparse.ArgumentParser(description="coda MCP server")
    parser.add_argument("--mcp-stdio", action="store_true", help="Run MCP server over stdio")
    args = parser.parse_args()
    if args.mcp_stdio:
        mcp.run(transport="stdio")
    else:
        mcp.run(transport="streamable-http", host="127.0.0.1", port=8080)

if __name__ == "__main__":
    main()

# endregion Main
