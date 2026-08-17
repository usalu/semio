#!/usr/bin/env python3
"""Fix mutation/diff/io import paths after schema absorb."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SPECS = [
    ("✒️writer", "writer", "Writer"),
    ("➗️mathematical", "mathematical", "Mathematical"),
    ("🌊️flow", "flow", "Flow"),
    ("🌿️vcs", "vcs", "Vcs"),
    ("🕸️dag", "dag", "Dag"),
]


def rewrite(t: str, mod: str, prefix: str) -> str:
    t = t.replace(f"crate::artifacts::{mod}::diff::", f"crate::artifacts::{mod}::schema::diff::text::")
    t = t.replace(f"crate::artifacts::{mod}::mutations::", f"crate::artifacts::{mod}::schema::mutations::")
    t = re.sub(
        rf"use super::\{{({prefix}Snapshot[^}}]*)\}};",
        rf"use crate::artifacts::{mod}::{{\1}};",
        t,
    )
    t = re.sub(
        rf"use super::({prefix}Snapshot);",
        rf"use crate::artifacts::{mod}::\1;",
        t,
    )
    if "document:" in t and f"into_{prefix.lower()}_diff" in t:
        t = t.replace(
            """WriterDiff {
            text: None,
            document: self.mutation.and_then(|m| match m { WriterMutation::SetSnapshot { snapshot } => Some(document), _ => None }),
        }""",
            """WriterDiff {
            text: None,
            artifact: self.mutation.and_then(|m| match m {
                WriterMutation::SetSnapshot { snapshot } => Some(Box::new(crate::artifacts::writer::schema::WriterArtifact::from_snapshot(snapshot))),
                _ => None,
            }),
            ..Default::default()
        }""",
        )
    return t


def main() -> None:
    for plug, mod, prefix in SPECS:
        art = ROOT / "✏️s/🔌️plugins" / plug / "🗿️artifacts" / plug
        for rs in art.rglob("🦀️component.rs"):
            rel = str(rs.relative_to(art))
            if "📚️examples" in rel:
                continue
            raw = rs.read_text(encoding="utf-8")
            new = rewrite(raw, mod, prefix)
            if new != raw:
                rs.write_text(new, encoding="utf-8")
                print(plug, rel)


if __name__ == "__main__":
    main()
