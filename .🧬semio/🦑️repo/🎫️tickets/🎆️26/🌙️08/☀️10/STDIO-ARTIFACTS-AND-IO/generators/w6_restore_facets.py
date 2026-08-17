#!/usr/bin/env python3
"""Restore facet apply codecs from git HEAD into new schema paths."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TEXT, BINARY, RS = "📝️text", "💾️binary", "🦀️component.rs"

ARTIFACTS = [
    ("✒️writer", "✒️writer", "writer"),
    ("➗️mathematical", "➗️mathematical", "mathematical"),
    ("🌊️flow", "🌊️flow", "flow"),
    ("🌿️vcs", "🌿️vcs", "vcs"),
    ("🕸️dag", "🕸️dag", "dag"),
]

MOVES = [
    ("🔺️diff/🦀️component.rs", f"🧬️schema/🔺️diff/{TEXT}/{RS}", [
        (".diff::schema::", ".schema::diff::"),
        (".diff::", ".schema::diff::"),
        ("use super::schema::*;", "use crate::artifacts::{mod}::schema::diff::*;"),
    ]),
    ("🗣️dsl/🦀️component.rs", f"🧬️schema/📸️snapshot/{TEXT}/{RS}", [
        ('include_str!("../📚️examples/', 'include_str!("../../../📚️examples/'),
    ]),
    ("📡️spr/🦀️component.rs", f"🧬️schema/🧬️mutations/{BINARY}/{RS}", []),
    ("🔧️op/🦀️component.rs", f"🧬️schema/🧬️mutations/{TEXT}/{RS}", [
        (".mutations::", ".schema::mutations::"),
    ]),
    ("📸️snapshot/🎒️pack/🦀️component.rs", f"🧬️schema/📸️snapshot/{BINARY}/{RS}", []),
]


def git_show(rel: str) -> str | None:
    r = subprocess.run(["git", "show", f"HEAD:{rel}"], cwd=ROOT, capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else None


def restore_one(plug: str, art: str, mod: str) -> None:
    base = f"✏️s/🔌️plugins/{plug}/🗿️artifacts/{art}"
    for src_rel, dst_rel, reps in MOVES:
        body = git_show(f"{base}/{src_rel}")
        if not body:
            continue
        for old, new in reps:
            body = body.replace(old, new.format(mod=mod) if "{mod}" in new else new)
        dst = ROOT / base / dst_rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_text(body, encoding="utf-8")
        print("ok", plug, dst_rel)


def main() -> None:
    for plug, art, mod in ARTIFACTS:
        restore_one(plug, art, mod)


if __name__ == "__main__":
    main()
