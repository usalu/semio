#!/usr/bin/env python3
"""Rename script.ts → 📜️script.ts and project.json → 📋️project.json; path-aware text rewrites."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

SKIP_DIRS = frozenset({"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg"})

FILE_RENAMES = [
    ("script.ts", "📜️script.ts"),
    ("project.json", "📋️project.json"),
]

def should_skip_dir(path: str) -> bool:
    parts = path.split(os.sep)
    return any(p in SKIP_DIRS for p in parts)


def rename_files():
    pairs = []
    for dirpath, dirs, files in os.walk(".", topdown=True):
        if should_skip_dir(dirpath):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
        for old, new in FILE_RENAMES:
            if old in files:
                oldp = os.path.join(dirpath, old).replace(os.sep, "/").lstrip("./")
                newp = os.path.join(dirpath, new).replace(os.sep, "/").lstrip("./")
                pairs.append((oldp, newp))
    pairs.sort(key=lambda p: -p[0].count("/"))
    for oldp, newp in pairs:
        if not os.path.isfile(oldp):
            continue
        if os.path.exists(newp):
            print("collision", newp)
            sys.exit(1)
        os.rename(oldp, newp)
    print(f"renamed {len(pairs)} files")
    return pairs


def rewrite_text():
    # project.json replacement must not break package.json - replace specific patterns only
    safe = [
        ("bun ./📜️script.ts", "bun ./📜️script.ts"),
        ("./📜️script.ts", "./📜️script.ts"),
        ('"script.ts"', '"📜️script.ts"'),
        ("**/📜️script.ts", "**/📜️script.ts"),
        ("base !== \"script.ts\"", "base !== \"📜️script.ts\""),
        ("expected script.ts, got", "expected 📜️script.ts, got"),
        ("ent.name === \"script.ts\"", "ent.name === \"📜️script.ts\""),
        ("endsWith(\"/script.ts\")", "endsWith(\"/📜️script.ts\")"),
        ("endsWith('/📜️script.ts')", "endsWith('/📜️script.ts')"),
        ("=== \"script.ts\"", "=== \"📜️script.ts\""),
        ("=== 'script.ts'", "=== '📜️script.ts'"),
        ("name !== \"script.ts\"", "name !== \"📜️script.ts\""),
        ("includes(\"script.ts\")", "includes(\"📜️script.ts\")"),
        ("entry.name !== \"project.json\"", "entry.name !== \"📋️project.json\""),
        ("/project.json", "/📋️project.json"),
        ("{projectRoot}/project.json", "{projectRoot}/📋️project.json"),
        ("glob '**/project.json'", "glob '**/📋️project.json'"),
        ("glob \"**/project.json\"", "glob \"**/📋️project.json\""),
    ]
    changed = 0
    for dirpath, dirs, files in os.walk(".", topdown=True):
        if should_skip_dir(dirpath):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            if f == "AGENTS.md":
                continue
            path = os.path.join(dirpath, f).replace(os.sep, "/").lstrip("./")
            try:
                raw = open(path, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError):
                continue
            new = raw
            for a, b in safe:
                new = new.replace(a, b)
            if new != raw:
                open(path, "w", encoding="utf-8").write(new)
                changed += 1
    print(f"text files updated {changed}")


def main():
    rename_files()
    rewrite_text()


if __name__ == "__main__":
    main()
