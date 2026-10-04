#!/usr/bin/env python3
"""🐍️ Runs every stdio Python oracle's `patch-snapshot` rows in the oracle role, the way the Python test host does, without
the runner (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): each case whose feature lists a `patch-snapshot` Examples
row and whose adapter is `🐍️.py` has that row expanded into its Scenario Outline (tags, steps, doc string, data tables),
its `shared://` fixtures resolved under the subset's (else the artifact's) `🧫️fixtures/` and its `asset://` ones under
the subset's `🖼️assets/`, and the adapter's registered
oracle handler called with a host `Context`. A handler that raises fails the row. Writes nothing outside `--out`; exits 1
on any failure.

usage: python3 🧪️s4-stdio-python-oracle-arms-run.py [--out <dir>]
"""
from __future__ import annotations

import importlib.util
import json
import pathlib
import re
import subprocess
import sys
import tempfile
import traceback

ROOT = pathlib.Path(__file__).resolve().parents[7]
HOST = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
STEP = re.compile(r"^\s*(Given|When|Then|And|But)\s+(.*)$")


def host():
    spec = importlib.util.spec_from_file_location("semio_repo_test", HOST)
    loaded = importlib.util.module_from_spec(spec)
    sys.modules["semio_repo_test"] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def cells(line: str) -> list[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def outlines(feature: str) -> list[dict]:
    """📖️ Every Scenario Outline: its `@id-` base, its steps (text, doc string, data table) and its Examples rows."""
    found, tags, current, mode = [], [], None, None
    lines = feature.split("\n")
    index = 0
    while index < len(lines):
        line = lines[index]
        stripped = line.strip()
        if stripped.startswith("@"):
            if current is not None and mode == "examples":
                current = None
            tags.append(stripped)
        elif stripped.startswith("Scenario Outline:"):
            base = next((tag[4:] for tag in tags if tag.startswith("@id-")), None)
            level = next((tag[7:] for tag in tags if tag.startswith("@level-")), "exhaustive")
            current = {"base": base, "level": level, "steps": [], "header": None, "rows": []}
            found.append(current)
            tags, mode = [], "steps"
        elif stripped.startswith("Scenario:") or stripped.startswith("Feature:"):
            current, tags, mode = None, [], None
        elif current is not None and stripped.startswith("Examples:"):
            mode = "examples"
        elif current is not None and mode == "steps" and STEP.match(line):
            current["steps"].append({"keyword": STEP.match(line).group(1), "text": STEP.match(line).group(2)})
        elif current is not None and mode == "steps" and stripped == '"""':
            body = []
            index += 1
            while lines[index].strip() != '"""':
                body.append(lines[index].strip())
                index += 1
            current["steps"][-1]["docString"] = "\n".join(body)
        elif current is not None and mode == "steps" and stripped.startswith("|"):
            current["steps"][-1].setdefault("dataTable", []).append(cells(stripped))
        elif current is not None and mode == "examples" and stripped.startswith("|"):
            if current["header"] is None:
                current["header"] = cells(stripped)
            else:
                current["rows"].append(cells(stripped))
        index += 1
    return found


def expanded(text: str, values: dict[str, str]) -> str:
    for key, value in values.items():
        text = text.replace(f"<{key}>", value)
    return text


def resolve(subset: pathlib.Path, uri: str) -> str | None:
    scheme, rest = uri.split("://", 1)
    bases = (subset / "🖼️assets",) if scheme == "asset" else (subset / "🧫️fixtures", subset.parents[2] / "🧫️fixtures")
    for base in bases:
        if (base / rest).exists():
            return str((base / rest).relative_to(ROOT))
    return None


def main() -> int:
    library = host()
    out = pathlib.Path(sys.argv[sys.argv.index("--out") + 1]) if "--out" in sys.argv else pathlib.Path(tempfile.mkdtemp())
    listed = subprocess.run(["git", "ls-files", "-z", "--", ARTIFACTS], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    features = [name for name in listed if name.endswith("/🥒️.feature") and (ROOT / name).exists() and (ROOT / name).with_name("🐍️.py").exists()]
    passed, failed = 0, []
    for name in sorted(features):
        feature_path = ROOT / name
        text = feature_path.read_text(encoding="utf-8")
        if "| patch-snapshot" not in text:
            continue
        subset = feature_path.parents[2]
        adapter = library._load_adapter(str(feature_path.with_name("🐍️.py")))
        for outline in outlines(text):
            for row in outline["rows"]:
                values = dict(zip(outline["header"], row))
                if values.get("id") != "patch-snapshot":
                    continue
                steps = [{key: (expanded(value, values) if isinstance(value, str) else [[expanded(cell, values) for cell in line] for line in value]) for key, value in step.items()} for step in outline["steps"]]
                scenario = {"id": f"{outline['base']}-patch-snapshot", "outlineOf": outline["base"], "steps": steps, "level": outline["level"]}
                probe = library.Context({"workDir": str(out), "fixtures": [], "case": feature_path.parent.name}, scenario, "oracle", str(ROOT))
                uris = probe.step_fixture_uris()
                fixtures = [{"uri": uri, "path": resolve(subset, uri)} for uri in uris]
                missing = [entry["uri"] for entry in fixtures if entry["path"] is None]
                label = f"{feature_path.parent.name} {scenario['id']}"
                handler = adapter.handler(scenario, "oracle")
                if missing or handler is None:
                    failed.append(f"{label}: {'unresolved ' + ', '.join(missing) if missing else 'no oracle handler'}")
                    continue
                context = library.Context({"workDir": str(out / feature_path.parent.name), "fixtures": [entry for entry in fixtures], "case": feature_path.parent.name}, scenario, "oracle", str(ROOT))
                try:
                    handler(context)
                    passed += 1
                    print(f"passed {label}")
                except Exception:
                    failed.append(f"{label}: {traceback.format_exc().strip().splitlines()[-1]}")
    for failure in failed:
        print(f"FAILED {failure}")
    print(f"python oracle patch-snapshot rows: {passed} passed / {len(failed)} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
