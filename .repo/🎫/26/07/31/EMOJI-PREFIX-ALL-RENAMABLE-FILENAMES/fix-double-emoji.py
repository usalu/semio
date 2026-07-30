#!/usr/bin/env python3
"""Fix double emoji prefixes introduced by basename rewrite overlapping prior emoji."""
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)
SKIP = frozenset({"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg", ".repo"})

FIXES = [
    ("🧪🧪vitest.config.ts", "🧪vitest.config.ts"),
    ("📦📦build.rs", "📦build.rs"),
    ("🦀🦀hosts.rs", "🦀hosts.rs"),
    ("🦀🦀icon_name.rs", "🦀icon_name.rs"),
    ("🦀🦀metabolism_🦀icon_name.rs", "🦀metabolism_icon_name.rs"),
    ("📦📦client-", "📦client-"),
    ("📦📦server-", "📦server-"),
    ("🧪🧪index.test.ts", "🧪index.test.ts"),
    ("🟨🟨nx-plugin.mjs", "🟨nx-plugin.mjs"),
    ("🟨🟨boot.js", "🟨boot.js"),
    ("🟨🟨host-shim.js", "🟨host-shim.js"),
    ("🟨🟨plugin-worker.js", "🟨plugin-worker.js"),
    ("🟨🟨puzzle_plugin.js", "🟨puzzle_plugin.js"),
    ("🟨🟨puzzle_plugin_component.js", "🟨puzzle_plugin_component.js"),
]

changed = 0
for dirpath, dirs, files in os.walk(".", topdown=True):
    dirs[:] = [d for d in dirs if d not in SKIP and not d.startswith(".")]
    if any(p in SKIP for p in dirpath.split(os.sep)):
        continue
    for f in files:
        path = os.path.join(dirpath, f).replace(os.sep, "/").lstrip("./")
        try:
            raw = open(path, encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue
        new = raw
        for a, b in FIXES:
            new = new.replace(a, b)
        if new != raw:
            open(path, "w", encoding="utf-8").write(new)
            changed += 1
print(f"fixed {changed} files")
