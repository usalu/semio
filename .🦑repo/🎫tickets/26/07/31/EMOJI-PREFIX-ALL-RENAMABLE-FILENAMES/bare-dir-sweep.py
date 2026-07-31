#!/usr/bin/env python3
"""Sweep bare-emoji directory names under product roots (emoji-only segment names)."""
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

BARE_WORDS = {
    "⚡": "⚡cache",
    "🔀": "🔀diff",
    "🤖": "🤖generated",
}

SKIP = frozenset({"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg", ".🦑repo"})
ROOTS = ["✏️s", "🧰framework", "🌎hub", "♻️mit-bestand"]

def is_bare(name):
    if not name or ord(name[0]) < 128:
        return False
    for word in BARE_WORDS:
        if name == word:
            return True
    return False

renamed = 0
for root in ROOTS:
    if not os.path.isdir(root):
        continue
    for dirpath, dirs, _ in os.walk(root, topdown=False):
        if any(p in SKIP for p in dirpath.split(os.sep)):
            continue
        for d in dirs:
            if not is_bare(d):
                continue
            old = os.path.join(dirpath, d)
            new = os.path.join(dirpath, BARE_WORDS[d])
            if not os.path.exists(new):
                os.rename(old, new)
                renamed += 1
                print("dir", old, "->", new)
print(f"renamed {renamed} bare dirs")
