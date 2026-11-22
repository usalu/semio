# 🔍️ Read-only structural check of process plugin and composition paths (#[path], include_*, Cargo path=, project.json cwd)
import json
import os
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SCOPES = [
    ROOT / "✏️s/🔌️plugins/🏭️process",
    ROOT / "🌎️hub/🧩️compositions/🏭️process",
]
SKIP_DIRS = {"node_modules", "target", ".git", "dist"}

RE_PATH_ATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
RE_INCLUDE = re.compile(r'include_(?:str|bytes)!\s*\(\s*"([^"]+)"\s*\)')
RE_CARGO_PATH = re.compile(r'\bpath\s*=\s*"([^"]+)"')

problems = []
checked = {"path_attr": 0, "include": 0, "cargo_path": 0, "project_cwd": 0}


def walk(scope):
    for dirpath, dirnames, filenames in os.walk(scope):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            yield Path(dirpath) / name


def rel(p):
    return os.path.relpath(p, ROOT)


def check_rs(f):
    text = f.read_text(encoding="utf-8", errors="replace")
    for lineno, line in enumerate(text.splitlines(), 1):
        for m in RE_PATH_ATTR.finditer(line):
            checked["path_attr"] += 1
            target = (f.parent / m.group(1)).resolve()
            if not target.exists():
                problems.append(f"{rel(f)}:{lineno}: #[path] target missing: {m.group(1)}")
        for m in RE_INCLUDE.finditer(line):
            checked["include"] += 1
            target = (f.parent / m.group(1)).resolve()
            if not target.exists():
                problems.append(f"{rel(f)}:{lineno}: include target missing: {m.group(1)}")


def check_cargo(f):
    text = f.read_text(encoding="utf-8", errors="replace")
    for lineno, line in enumerate(text.splitlines(), 1):
        if line.lstrip().startswith("#"):
            continue
        for m in RE_CARGO_PATH.finditer(line):
            checked["cargo_path"] += 1
            target = (f.parent / m.group(1)).resolve()
            if not target.exists():
                problems.append(f"{rel(f)}:{lineno}: Cargo path missing: {m.group(1)}")


def find_cwds(node, out):
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "cwd" and isinstance(v, str):
                out.append(v)
            else:
                find_cwds(v, out)
    elif isinstance(node, list):
        for v in node:
            find_cwds(v, out)


def check_project(f):
    try:
        data = json.loads(f.read_text(encoding="utf-8"))
    except Exception as e:
        problems.append(f"{rel(f)}:1: invalid JSON: {e}")
        return
    cwds = []
    find_cwds(data, cwds)
    for cwd in cwds:
        checked["project_cwd"] += 1
        from_project = (f.parent / cwd).resolve()
        from_root = (ROOT / cwd).resolve()
        if not from_project.exists() and not from_root.exists():
            problems.append(f"{rel(f)}: cwd missing (relative to project and workspace root): {cwd}")
        elif not from_root.exists():
            problems.append(f"{rel(f)}: cwd only resolves relative to project dir, not workspace root: {cwd}")


def main():
    for scope in SCOPES:
        if not scope.exists():
            problems.append(f"scope missing: {rel(scope)}")
            continue
        for f in walk(scope):
            if f.suffix == ".rs":
                check_rs(f)
            elif f.name == "Cargo.toml":
                check_cargo(f)
            elif f.name.endswith("project.json"):
                check_project(f)
    print("checked:", checked)
    print(f"problems: {len(problems)}")
    for p in problems:
        print(p)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
