#!/usr/bin/env python3
"""Fast basename-only reference rewrite after file emoji renames."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

MAP_PATH = os.path.join(HERE, "file-rename-map.json")
SKIP_DIRS = frozenset({"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg", ".repo"})


def load_basename_map():
    with open(MAP_PATH, encoding="utf-8") as f:
        entries = json.load(f)
    m = {}
    for e in entries:
        ob = os.path.basename(e["old"])
        nb = os.path.basename(e["new"])
        if ob in m and m[ob] != nb:
            raise SystemExit(f"basename conflict: {ob}")
        m[ob] = nb
    return m


def rewrite(content: str, basename_map) -> str:
    for ob, nb in sorted(basename_map.items(), key=lambda x: -len(x[0])):
        if ob != nb:
            content = content.replace(ob, nb)
    return content


def main():
    basename_map = load_basename_map()
    changed = 0
    for dirpath, dirs, files in os.walk(".", topdown=True):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
        if any(p in SKIP_DIRS for p in dirpath.split(os.sep)):
            continue
        for f in files:
            if f.startswith(".") and f not in ("gitignore", "gitattributes", "gitmodules", "nxignore", "prettierignore"):
                continue
            path = os.path.join(dirpath, f).replace(os.sep, "/")
            if path.startswith("./"):
                path = path[2:]
            try:
                raw = open(path, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError):
                continue
            new = rewrite(raw, basename_map)
            if new != raw:
                open(path, "w", encoding="utf-8").write(new)
                changed += 1
    print(f"updated {changed} files")


if __name__ == "__main__":
    main()
