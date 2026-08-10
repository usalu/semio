#!/usr/bin/env python3
"""Restore diff/text apply codecs from git HEAD (clobbered by leaf scaffold)."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TEXT = "📝️text"
RS = "🦀️component.rs"

ARTIFACTS = [
    ("✒️writer", "✒️writer", "writer"),
    ("➗️mathematical", "➗️mathematical", "mathematical"),
    ("🌊️flow", "🌊️flow", "flow"),
    ("🌿️vcs", "🌿️vcs", "vcs"),
    ("🕸️dag", "🕸️dag", "dag"),
]


def git_show(rel: str) -> str | None:
    r = subprocess.run(
        ["git", "show", f"HEAD:{rel}"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return r.stdout if r.returncode == 0 else None


def restore(plug: str, art: str, mod: str) -> None:
    old = f"✏️s/🔌️plugins/{plug}/🗿️artifacts/{art}/🔺️diff/{RS}"
    body = git_show(old)
    if not body:
        print("skip", plug, "no old diff")
        return
    dst = ROOT / "✏️s/🔌️plugins" / plug / "🗿️artifacts" / art / "🧬️schema/🔺️diff" / TEXT / RS
    body = body.replace(f"crate::artifacts::{mod}::diff::schema::", f"crate::artifacts::{mod}::schema::diff::")
    body = body.replace(f"crate::artifacts::{mod}::diff::", f"crate::artifacts::{mod}::schema::diff::")
    body = body.replace("use super::schema::*;", "use crate::artifacts::%s::schema::diff::*;" % mod)
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(body, encoding="utf-8")
    print("restored", dst)


def main() -> None:
    for plug, art, mod in ARTIFACTS:
        restore(plug, art, mod)


if __name__ == "__main__":
    main()
