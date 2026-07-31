#!/usr/bin/env python3
"""Remove double-emoji corruption from basenames (files on disk + text refs)."""
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

SKIP_DIRS = frozenset(
    {
        "node_modules",
        "target",
        ".git",
        ".nx",
        "dist",
        "pkg",
        ".repo-cache",
    }
)
SKIP_FILES = frozenset({"AGENTS.md"})

# Longest / most specific first
TEXT_FIXES = [
    ("🟦backbone-worker.ts", "🟦backbone-worker.ts"),
    ("🔣compose_codeicon.svg", "🔣compose_codeicon.svg"),
    ("🔣repo_codeicon.svg", "🔣repo_codeicon.svg"),
    ("🦀metabolism_icon_name.rs", "🦀metabolism_icon_name.rs"),
    ("🧪vitest.config.ts", "🧪vitest.config.ts"),
    ("🧪index.test.ts", "🧪index.test.ts"),
    ("📦build.rs", "📦build.rs"),
    ("📦client-", "📦client-"),
    ("📦server-", "📦server-"),
    ("🟨nx-plugin.mjs", "🟨nx-plugin.mjs"),
    ("🟨boot.js", "🟨boot.js"),
    ("🟨host-shim.js", "🟨host-shim.js"),
    ("🟨plugin-worker.js", "🟨plugin-worker.js"),
    ("🟨puzzle_plugin.js", "🟨puzzle_plugin.js"),
    ("🟨puzzle_plugin_component.js", "🟨puzzle_plugin_component.js"),
    ("🟦extension.test.ts", "🟦extension.test.ts"),
    ("🟦extension.ts", "🟦extension.ts"),
    ("🟦vite.test.config.ts", "🟦vite.test.config.ts"),
    ("🟦eslint.config.ts", "🟦eslint.config.ts"),
    ("🟦next-env.d.ts", "🟦next-env.d.ts"),
    ("🟦worker.ts", "🟦worker.ts"),
    ("🟦boot.ts", "🟦boot.ts"),
    ("🟦session.ts", "🟦session.ts"),
    ("🟦plugins.ts", "🟦plugins.ts"),
    ("🟦playgrounds.ts", "🟦playgrounds.ts"),
    ("🔣plugins.json", "🔣plugins.json"),
    ("🔣playgrounds.json", "🔣playgrounds.json"),
    ("🔣fixture.json", "🔣fixture.json"),
    ("🔣mono.theme.json", "🔣mono.theme.json"),
    ("🔣tokens.json", "🔣tokens.json"),
    ("📜world.wit", "📜world.wit"),
    ("📋activation.log", "📋activation.log"),
    ("🖼️compose.png", "🖼️compose.png"),
    ("🖼️logo.png", "🖼️logo.png"),
    ("🖼️sketch.png", "🖼️sketch.png"),
    ("🐳Dockerfile", "🐳Dockerfile"),
    ("⚙️vite.config.ts", "⚙️vite.config.ts"),
    ("🦀hosts.rs", "🦀hosts.rs"),
    ("🦀icon_name.rs", "🦀icon_name.rs"),
]

FILE_RENAME_FIXES = [
    ("🟦session.ts", "🟦session.ts"),
    ("🧪vitest.config.ts", "🧪vitest.config.ts"),
    ("🧪index.test.ts", "🧪index.test.ts"),
    ("📦build.rs", "📦build.rs"),
    ("🟨nx-plugin.mjs", "🟨nx-plugin.mjs"),
    ("🟨boot.js", "🟨boot.js"),
    ("🟨host-shim.js", "🟨host-shim.js"),
    ("🟨plugin-worker.js", "🟨plugin-worker.js"),
    ("🟨puzzle_plugin.js", "🟨puzzle_plugin.js"),
    ("🟨puzzle_plugin_component.js", "🟨puzzle_plugin_component.js"),
    ("🟦extension.ts", "🟦extension.ts"),
    ("🟦extension.test.ts", "🟦extension.test.ts"),
    ("🟦vite.test.config.ts", "🟦vite.test.config.ts"),
    ("🟦eslint.config.ts", "🟦eslint.config.ts"),
    ("🟦boot.ts", "🟦boot.ts"),
    ("🟦plugins.ts", "🟦plugins.ts"),
    ("🟦playgrounds.ts", "🟦playgrounds.ts"),
    ("🔣plugins.json", "🔣plugins.json"),
    ("🔣playgrounds.json", "🔣playgrounds.json"),
]


def should_skip_dir(dp: str) -> bool:
    parts = dp.split(os.sep)
    if any(p in SKIP_DIRS for p in parts):
        return True
    for p in parts:
        if p.startswith(".") and p not in (".🦑repo"):
            return True
    return False


renamed = 0
for dirpath, dirs, files in os.walk(".", topdown=False):
    if should_skip_dir(dirpath):
        continue
    for f in files:
        new_f = f
        for a, b in FILE_RENAME_FIXES:
            if a in new_f:
                new_f = new_f.replace(a, b)
        if new_f != f:
            old = os.path.join(dirpath, f)
            new = os.path.join(dirpath, new_f)
            if not os.path.exists(new):
                os.rename(old, new)
                renamed += 1
                print("rename", old, "->", new)

text_changed = 0
for dirpath, dirs, files in os.walk(".", topdown=True):
    if should_skip_dir(dirpath):
        dirs[:] = []
        continue
    dirs[:] = [
        d
        for d in dirs
        if d not in SKIP_DIRS and (not d.startswith(".") or d == ".🦑repo")
    ]
    for f in files:
        if f in SKIP_FILES:
            continue
        path = os.path.join(dirpath, f)
        try:
            raw = open(path, encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue
        new = raw
        for a, b in TEXT_FIXES:
            new = new.replace(a, b)
        if new != raw:
            open(path, "w", encoding="utf-8").write(new)
            text_changed += 1

print(f"renamed {renamed} files, fixed {text_changed} text files")
