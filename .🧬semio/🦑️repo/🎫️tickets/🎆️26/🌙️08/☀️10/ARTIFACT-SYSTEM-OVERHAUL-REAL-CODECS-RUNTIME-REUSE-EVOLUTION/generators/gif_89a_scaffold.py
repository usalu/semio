#!/usr/bin/env python3
"""Scaffold `standards/🔖️89a` as a sibling of the existing `standards/🔖️87a` tree for the gif
artifact, mirroring its directory shape (⚙️engine/🧬️schema/🪆️subsets/✳️any/{🏗️builder,🧐️analyzer,🚪️io,...}).
Copies the 87a tree verbatim then does mechanical token substitution (87a->89a, v87a->v89a,
GIF87a->GIF89a) for the *boilerplate* wiring files. The substantive logic files (engine, schema
snapshot/diff/mutations, analyzer, builder) are then hand-rewritten separately with real 89a
content (frames/GCE/loop) -- this script only saves the directory-shape + sidecar-file typing.
"""
from __future__ import annotations

import shutil
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
GIF = ROOT / "✏️s" / "🔌️plugins" / "🗄️stdio" / "🗿️artifacts" / "🎞️gif"
SRC = GIF / "🏅️standards" / "🔖️87a"
DST = GIF / "🏅️standards" / "🔖️89a"

TEXT_SUFFIXES = {
    ".rs", ".ts", ".json", ".semio", ".proto", ".graphql", ".ebnf", ".g4", ".abnf", ".ksy", ".spicy",
}

REPLACEMENTS = [
    ("🔖️87a", "🔖️89a"),
    ("v87a", "v89a"),
    ("GIF87a", "GIF89a"),
    ("87a", "89a"),
]


def main() -> None:
    if DST.exists():
        raise SystemExit(f"refusing to overwrite existing {DST}")
    shutil.copytree(SRC, DST)
    for path in DST.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix not in TEXT_SUFFIXES:
            continue
        raw = path.read_text(encoding="utf-8")
        for old, new in REPLACEMENTS:
            raw = raw.replace(old, new)
        path.write_text(raw, encoding="utf-8")
    print("scaffolded", DST)


if __name__ == "__main__":
    main()
