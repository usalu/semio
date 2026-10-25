#!/usr/bin/env python3
"""Independent oracle of the codebase module: real `git check-ignore` decides the gitignore verdicts, plain Python
models decide the structural exclusion, the generated-folder test and the lexical walk of the synthetic repository.

Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the codebase module.
"""
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = FIXTURES / "🔬️unit-vectors.json"
SOURCE_EXTENSIONS = {"ts", "tsx", "py", "cs", "go", "rs"}

ASSIST = [
    "plain/📁️x.ts", "./plain/📁️x.ts", "a/b/c.rs", "a/node_modules/x.ts", "node_modules", "node_modules/x", ".git", ".git/config", "x/.git/hooks", ".🧬semio", ".🧬semio/a", ".🧬semio-extra/a",
    "assets/repo", "assets/repo/x.png", "assets/repository/x.png", "p/asset/repo/x.png", "a/dist/x.js", "dist/x.js", "a/build/x.js", "a/target/x.rs", "a/__pycache__/x.pyc", "a/.next/x.js",
    "a/coverage/x", "Form.Designer.cs", "a/Form.Designer.cs", "a/Form.designer.cs", "a/codegen/x.ts", "codegen/x.ts", "a/codegenerator/x.ts", "", ".", "LICENSE.md", "a/LICENSE.md", "a/license.md",
    "generated/x.ts", "a/generated", "a/Generated/x.ts", "js/vscode/generated", "js/vscode/generated/x", "js/compose/generated/y", "js/vscode/generatedx", "a/b/dist", "x/build/y/z", "coverage", "a/.venv/x.py",
]


def structural(path):
    path = path.replace("\\", "/")
    while path.startswith("./"):
        path = path[2:]
    if path == "":
        return False
    for root in (".🧬semio", "node_modules", ".git"):
        if path == root or path.startswith(root + "/"):
            return True
    if "/node_modules/" in path or "/.git/" in path:
        return True
    if path == "assets/repo" or path.startswith("assets/repo/") or "/asset/repo/" in path:
        return True
    if any(segment in path for segment in ("/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/")):
        return True
    if path.rsplit("/", 1)[-1].endswith(".Designer.cs"):
        return True
    return "/codegen/" in path


def generated(path):
    path = path.replace("\\", "/")
    while path.startswith("./"):
        path = path[2:]
    if any(part in ("generated", "dist", "build", "node_modules", "__pycache__", ".next", "coverage") for part in path.split("/")):
        return True
    return any(path == g or path.startswith(g + "/") for g in ("js/vscode/generated", "js/compose/generated"))


def materialise(tree, directory):
    for entry in tree["entries"]:
        target = directory / entry["path"]
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(entry["content"], encoding="utf8")
    subprocess.run(["git", "init", "-q"], cwd=directory, check=True)


def normalise(path):
    path = path.replace("\\", "/")
    while path.startswith("./"):
        path = path[2:]
    return path


def git_ignored(directory, path):
    """Real git's verdict (`git check-ignore --no-index`) for a path against the materialised .gitignore."""
    path = normalise(path)
    if path == "":
        return False
    result = subprocess.run(["git", "check-ignore", "-q", "--no-index", "--", path], cwd=directory)
    return result.returncode == 0


def owned_ignored(directory, path):
    """The owned matcher of 🏠️workspace: the last matching rule decides, `dir/` expands to `dir/**` and a rule that then
    contains a separator is anchored at the root (the recorded divergence from git), a separator-free rule matches at any depth,
    and `LICENSE.md` is always ignored."""
    path = normalise(path)
    if path == "":
        return False
    if path.rsplit("/", 1)[-1] == "LICENSE.md":
        return True
    ignored = False
    for line in (directory / ".gitignore").read_text(encoding="utf8").split("\n"):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        negated = line.startswith("!")
        line = line[1:] if negated else line
        if line.endswith("/"):
            hit = path.startswith(line)
        elif "/" not in line:
            hit = re.fullmatch(re.escape(line).replace("\\*", "[^/]*"), path.rsplit("/", 1)[-1]) is not None
        else:
            raise ValueError("unmodelled rule " + line)
        if hit:
            ignored = not negated
    return ignored


def walk(directory, base, patterns_ok):
    found = []
    for name in sorted(os.listdir(base), key=lambda n: n.encode("utf8")):
        full = base / name
        if full.is_dir():
            if name.startswith("."):
                continue
            found += walk(directory, full, patterns_ok)
            continue
        relative = str(full.relative_to(directory)).replace(os.sep, "/")
        if "node_modules" in relative.split("/") or ".venv" in relative.split("/"):
            continue
        if name.rsplit(".", 1)[-1].lower() in SOURCE_EXTENSIONS and "." in name:
            found.append(relative)
    return found


def considered(directory, files):
    return [f for f in files if not structural(f) and not owned_ignored(directory, f)]


def bundles(directory):
    result = []
    for name in sorted(os.listdir(directory), key=lambda n: n.encode("utf8")):
        readme = directory / name / "README.md"
        if not (directory / name).is_dir() or name.startswith(".") or name in ("node_modules", "target", "temp") or not readme.exists() or not readme.read_text(encoding="utf8").startswith("---"):
            continue
        match = re.search(r"^name:\s*(\S+)", readme.read_text(encoding="utf8"), re.M)
        technology = match.group(1) if match else name
        for sub in sorted(os.listdir(directory / name), key=lambda n: n.encode("utf8")):
            if not (directory / name / sub).is_dir() or sub.startswith(".") or sub == "node_modules":
                continue
            if sub == "sites":
                for site in sorted(os.listdir(directory / name / sub), key=lambda n: n.encode("utf8")):
                    if (directory / name / sub / site).is_dir() and not site.startswith(".") and site != "node_modules":
                        result.append((f"{technology}/{site}", f"{name}/{sub}/{site}"))
                continue
            result.append((f"{technology}/{sub}", f"{name}/{sub}"))
    return result


def build():
    tree = json.loads((FIXTURES / "📡️repo-tree.json").read_text(encoding="utf8"))
    vectors = json.loads((FIXTURES / "📡️ignore-vectors.json").read_text(encoding="utf8"))
    scratch = Path(tempfile.mkdtemp(prefix="semio-codebase-oracle-"))
    try:
        materialise(tree, scratch)
        paths = [v["path"] for v in vectors["vectors"]] + ASSIST
        ignore = [{"path": p, "structural": structural(p), "gitignored": owned_ignored(scratch, p), "git": git_ignored(scratch, p) or normalise(p).rsplit("/", 1)[-1] == "LICENSE.md", "generated": generated(p)} for p in paths]
        everything = walk(scratch, scratch, None)
        scopes = {"repo": considered(scratch, everything), "technologies": [], "folders": [], "files": []}
        for name, root in bundles(scratch):
            files = considered(scratch, walk(scratch, scratch / root, None))
            scopes["technologies"].append({"name": name, "root": root, "files": files})
        for folder in ("compose/", "compose/🖥️app/", "compose/📚️core/", "docs/", "ignored/", "compose/🖥️app/ignored/", "missing/"):
            base = scratch / folder.rstrip("/")
            files = considered(scratch, walk(scratch, base, None)) if base.is_dir() else []
            scopes["folders"].append({"path": folder, "files": files})
        for path in ("compose/🖥️app/🧩️widget.ts", "compose/🖥️app/ignored/💀️dead.ts", "LICENSE.md", "node_modules/vendored/index.ts", "README.md", "missing.ts"):
            scopes["files"].append({"path": path, "files": [] if structural(path) or owned_ignored(scratch, path) else [path]})
        gitignore = [line.strip() for line in (scratch / ".gitignore").read_text(encoding="utf8").split("\n") if line.strip() and not line.strip().startswith("#")]
        return {
            "schema": "semio.repo.codebase.unit-vectors/1",
            "$comment": "🔬️ Expected answers for the cargo unit tests over the repository materialised from 📡️repo-tree.json. `gitignored` is the verdict of the owned matcher modelled in Python (separator rules anchored at the root: the recorded divergence from git), `git` the verdict of real `git check-ignore` for comparison, `structural`, `generated` and the lexical walk by plain Python models. Regenerate with the oracle; never edit by hand.",
            "oracle": {"script": ORACLE_SCRIPT, "tool": "git check-ignore + python3", "libraries": ["git", "os", "re"], "model": "structural exclusion, generated folders, lexical walk, scope resolution"},
            "gitignorePatterns": gitignore,
            "ignore": ignore,
            "scopes": scopes,
        }
    finally:
        shutil.rmtree(scratch, ignore_errors=True)


def main():
    text = json.dumps(build(), ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET)


main()
