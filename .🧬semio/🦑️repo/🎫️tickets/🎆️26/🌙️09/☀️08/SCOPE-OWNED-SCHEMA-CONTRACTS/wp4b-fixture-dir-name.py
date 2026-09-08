#!/usr/bin/env python3
"""🧫 WP4b helper — renames the plugins partition's `🧪️fixtures` directories to the taxonomy name.

Taxonomy `testFixturesDirName` is `🧫️fixtures` and `testsDirName` is `🧪️tests`; `🧪️fixtures` is not a
taxonomy name at all (cross-partition ledger row 27). This renames every such directory under `✏️s`
and rewrites the literal in every file that names one. Files outside `✏️s` are reported, never
written: they belong to another partition (execution contract §E).

Usage: wp4b-fixture-dir-name.py plan|apply
"""
import json
import os
import subprocess
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
ROOT = "✏️s"
OLD = "🧪️fixtures"
NEW = "🧫️fixtures"


def directories():
    found = []
    for directory, names, files in os.walk(os.path.join(REPO, ROOT)):
        names[:] = [name for name in names if name not in ("target", "node_modules", ".venv", "dist")]
        if os.path.basename(directory) == OLD:
            found.append(os.path.relpath(directory, REPO))
    return sorted(found, key=len, reverse=True)


def referencing_files():
    inside, outside = [], []
    outside = [line for line in subprocess.run(
        ["git", "grep", "-l", "✏️s[^\"' ]*" + OLD, "--", "🧰️framework", "🌎️hub", ".vscode", "📜️script.ts"],
        cwd=REPO, capture_output=True, text=True).stdout.splitlines() if line]
    for directory, names, files in os.walk(os.path.join(REPO, ROOT)):
        names[:] = [name for name in names
                    if name not in ("target", "node_modules", ".venv", "dist", ".git") and not name.startswith(".git")]
        relative_dir = os.path.relpath(directory, REPO)
        if relative_dir.split(os.sep)[0] == ".🧬semio":
            continue
        for name in files:
            path = os.path.join(directory, name)
            try:
                with open(path, encoding="utf-8") as handle:
                    text = handle.read()
            except (UnicodeDecodeError, OSError):
                continue
            if OLD not in text:
                continue
            relative = os.path.relpath(path, REPO)
            inside.append(relative)
    return inside, outside


def main():
    action = sys.argv[1] if len(sys.argv) > 1 else "plan"
    dirs = directories()
    inside, outside = referencing_files()
    print(f"directories: {len(dirs)}  files naming {OLD} inside {ROOT}: {len(inside)}  outside: {len(outside)}")
    for path in outside:
        print(f"  CROSS-PARTITION (not written): {path}")
    if action != "apply":
        for path in dirs[:10]:
            print(f"  {path}")
        return
    for path in dirs:
        target = os.path.join(os.path.dirname(path), NEW)
        if os.path.exists(os.path.join(REPO, target)):
            print(f"  SKIP (target exists): {path}")
            continue
        os.rename(os.path.join(REPO, path), os.path.join(REPO, target))
    for relative in inside:
        path = os.path.join(REPO, relative)
        if not os.path.exists(path):
            continue
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text.replace(OLD, NEW))
    print(f"renamed {len(dirs)} directories, rewrote {len(inside)} files under {ROOT}")


if __name__ == "__main__":
    main()
