#!/usr/bin/env python3
"""Rewrite path and basename references after file emoji renames."""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

MAP_PATH = os.path.join(HERE, "file-rename-map.json")
SKIP_DIRS = {"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg"}
TEXT_EXT = {
    ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".json", ".toml", ".md", ".mdx",
    ".rs", ".go", ".py", ".css", ".html", ".graphql", ".sql", ".yaml", ".yml",
    ".cypher", ".wit", ".tex", ".sty", ".cls", ".cjs", ".mjs", ".ps1", ".sh",
    ".dockerfile", ".graphql", ".bib", ".csv", ".txt", ".log", ".dsl", ".flow",
    ".ops", ".s", ".puzzle2d", ".puzzle3d", ".puzzle5d", ".block2d", ".block3d",
    ".block5d", ".fem2d", ".fem3d", ".procedural2d", ".procedural3d", ".gismap",
    ".gisterrain", ".lowpoly", ".wires", ".shooting", ".process3d", ".dag",
    ".curate", ".forms", ".writer", ".trinity", ".sequence", ".draw", ".note",
    ".layout", ".raster", ".imperative", ".manifest", ".cjs",
}
ROOT_FILES = [
    "Cargo.toml", "package.json", "nx.json", "go.work", "go.mod", "script.ts",
    "vitest.config.ts", "tsconfig.json", ".gitignore", ".dependency-cruiser.cjs",
    "eslint.config.mjs", "project.json", "bunfig.toml",
]


def load_map():
    with open(MAP_PATH, encoding="utf-8") as f:
        entries = json.load(f)
    entries.sort(key=lambda e: -len(e["old"]))
    basename_map = {}
    for e in entries:
        ob = os.path.basename(e["old"])
        nb = os.path.basename(e["new"])
        if ob in basename_map and basename_map[ob] != nb:
            raise SystemExit(f"basename conflict: {ob} -> {basename_map[ob]} vs {nb}")
        basename_map[ob] = nb
    return entries, basename_map


def is_text_file(path: str) -> bool:
    if path in ROOT_FILES or os.path.basename(path) in ROOT_FILES:
        return True
    ext = os.path.splitext(path)[1].lower()
    if ext in TEXT_EXT:
        return True
    for suffix in (".manifest.json", ".config.ts", ".config.mjs"):
        if path.endswith(suffix):
            return True
    if path.endswith("Dockerfile") or path.endswith("Caddyfile"):
        return True
    return False


def iter_files():
    for rf in ROOT_FILES:
        if os.path.isfile(rf):
            yield rf
    for root in ("🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "compose", ".vscode", ".github", ".storybook"):
        if not os.path.isdir(root):
            continue
        for dirpath, dirs, files in os.walk(root):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
            if any(s in dirpath.split(os.sep) for s in SKIP_DIRS):
                continue
            for f in files:
                rel = os.path.join(dirpath, f).replace(os.sep, "/")
                if is_text_file(rel):
                    yield rel


def rewrite_content(content: str, entries, basename_map) -> str:
    for e in entries:
        content = content.replace(e["old"], e["new"])
    for ob, nb in sorted(basename_map.items(), key=lambda x: -len(x[0])):
        if ob == nb:
            continue
        content = re.sub(
            r"(?<=[/\"'`(])" + re.escape(ob) + r"(?=[\"'`?#]|$)",
            nb,
            content,
        )
    return content


def main():
    entries, basename_map = load_map()
    changed = 0
    for path in iter_files():
        try:
            raw = open(path, encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue
        new = rewrite_content(raw, entries, basename_map)
        if new != raw:
            open(path, "w", encoding="utf-8").write(new)
            changed += 1
    print(f"updated {changed} files")


if __name__ == "__main__":
    main()
