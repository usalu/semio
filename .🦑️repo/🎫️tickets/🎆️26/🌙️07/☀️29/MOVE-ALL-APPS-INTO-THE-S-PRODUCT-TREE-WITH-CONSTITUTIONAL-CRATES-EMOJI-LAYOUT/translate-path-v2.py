#!/usr/bin/env python3
"""Path translator for the emoji+name rename pass. Given any repo-root-relative
path (old, pre-rename form), returns its new form by walking segment-by-segment
and remapping every prefix that appears in rename-map-v2.json. Segments outside
the renamed scope (excluded content, filenames, compose/, ..) pass through
unchanged. This is prefix-based (not a blind global segment substitution), so
dual-status basenames (e.g. "projekt" existing both inside an excluded ♻️/recherche
subtree and inside a renamed ♻️/bericht subtree) resolve correctly per their
actual location."""
import json
import os

_HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(_HERE, "rename-map-v2.json"), encoding="utf-8") as f:
    _RENAMES = json.load(f)

# old full repo-root-relative path -> new basename (just the renamed final segment)
_PREFIX_TO_NEW_BASENAME = {r["old"]: os.path.basename(r["new"]) for r in _RENAMES}

# Entry-file basename renames, applied at the very end regardless of directory renames.
# Includes both the first-time word-based names AND the already-bare-📦️ names left by the
# PREVIOUS (emoji-only) migration round, since this script also runs as a second pass over
# already-renamed content.
_ENTRY_RENAMES = {
    "lib.rs": "📦️lib.rs",
    "main.rs": "📦️main.rs",
    "bin.rs": "📦️bin.rs",
    "index.ts": "📦️index.ts",
    "index.tsx": "📦️index.tsx",
    "📦️.rs": "📦️lib.rs",
    "📦️.ts": "📦️index.ts",
    "📦️.tsx": "📦️index.tsx",
}


def translate(old_relpath: str) -> str:
    """old_relpath: a repo-root-relative POSIX path (no leading ./), pre-rename form.
    Returns the equivalent post-rename repo-root-relative path."""
    parts = old_relpath.split("/")
    new_parts = []
    old_prefix_parts = []
    for i, part in enumerate(parts):
        is_last = i == len(parts) - 1
        old_prefix_parts.append(part)
        old_prefix = "/".join(old_prefix_parts)
        if old_prefix in _PREFIX_TO_NEW_BASENAME:
            new_parts.append(_PREFIX_TO_NEW_BASENAME[old_prefix])
        elif is_last and part in _ENTRY_RENAMES:
            new_parts.append(_ENTRY_RENAMES[part])
        else:
            new_parts.append(part)
    return "/".join(new_parts)


if __name__ == "__main__":
    import sys

    for line in sys.stdin:
        line = line.rstrip("\n")
        if not line:
            continue
        print(translate(line))
