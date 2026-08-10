#!/usr/bin/env python3
"""The 89a scaffold (gif_89a_scaffold.py) copied 87a's files verbatim, so every internal
cross-reference of the shape `crate::artifacts::gif::Xxx` still resolves through the crate-root
shim module, which points at v87a (`pub mod schema { pub use super::standards::v87a...::schema::*; }`
in glue.rs). That's correct for 87a's own files (self-referential) but wrong once the SAME text
lives under 🔖️89a: it would silently bind 89a's mutation/diff/analyzer/etc. leaves to v87a's
RasterImage-shaped types instead of the real 89a frame/GCE/loop model. This repoints every such
bare crate::artifacts::gif::{schema,engine,builder,analyzer,composer,Gif*} reference found inside
the 89a subtree to the fully-qualified `crate::artifacts::gif::standards::v89a::...` path.
`STDIO_GIF_DOCUMENT_SCHEMA` is intentionally left alone -- it's one shared, standard-agnostic
constant defined once at the crate-root gif module, not part of the 87a-vs-89a split.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DST = ROOT / "✏️s" / "🔌️plugins" / "🗄️stdio" / "🗿️artifacts" / "🎞️gif" / "🏅️standards" / "🔖️89a"

QUALIFIED = "crate::artifacts::gif::standards::v89a::subsets::any::"
QUALIFIED_ENGINE = "crate::artifacts::gif::standards::v89a::"

# Order matters: longest/most-specific prefixes first.
REPLACEMENTS = [
    ("crate::artifacts::gif::schema::", QUALIFIED + "schema::"),
    ("crate::artifacts::gif::engine::", QUALIFIED_ENGINE + "engine::"),
    ("crate::artifacts::gif::builder::", QUALIFIED + "builder::"),
    ("crate::artifacts::gif::analyzer::", QUALIFIED + "analyzer::"),
    ("crate::artifacts::gif::composer::", QUALIFIED + "composer::"),
    ("crate::artifacts::gif::GifSnapshot", QUALIFIED + "schema::snapshot::GifSnapshot"),
    ("crate::artifacts::gif::GifDiff", QUALIFIED + "schema::diff::GifDiff"),
    ("crate::artifacts::gif::GifMutation", QUALIFIED + "schema::mutations::GifMutation"),
    ("crate::artifacts::gif::GifArtifact", QUALIFIED + "schema::GifArtifact"),
]


def main() -> None:
    changed = 0
    for path in DST.rglob("*.rs"):
        raw = path.read_text(encoding="utf-8")
        new = raw
        for old, repl in REPLACEMENTS:
            new = new.replace(old, repl)
        if new != raw:
            path.write_text(new, encoding="utf-8")
            changed += 1
            print("repointed", path.relative_to(DST))
    print(f"done, {changed} files changed")


if __name__ == "__main__":
    main()
