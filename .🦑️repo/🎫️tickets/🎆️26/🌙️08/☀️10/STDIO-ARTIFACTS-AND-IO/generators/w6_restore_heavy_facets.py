#!/usr/bin/env python3
"""Restore pre-migration facet impls from git HEAD into stdio text/binary leaves."""
from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parents[1]
BATCH = json.loads((TICKET / "generators" / "w6-heavy.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
TEXT, BINARY = TOK["text"], TOK["binary"]


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))


def git_show(rel: str) -> str | None:
    r = subprocess.run(
        ["git", "show", f"HEAD:{rel}"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return r.stdout if r.returncode == 0 and r.stdout.strip() else None


def rewrite_diff(content: str, rust_mod: str) -> str:
    m = rust_mod
    content = content.replace(
        f"crate::artifacts::{m}::diff::schema::",
        f"crate::artifacts::{m}::schema::diff::",
    )
    content = content.replace("pub use super::schema::*;\n", "")
    return content


def rewrite_dsl(content: str, rust_mod: str) -> str:
    m = rust_mod
    content = content.replace(
        f"crate::artifacts::{m}::snapshot::schema::",
        f"crate::artifacts::{m}::schema::snapshot::",
    )
    return content


def rewrite_spr(content: str, rust_mod: str) -> str:
    m = rust_mod
    content = content.replace(
        f"crate::artifacts::{m}::mutations::",
        f"crate::artifacts::{m}::schema::mutations::",
    )
    content = content.replace(
        f"crate::artifacts::{m}::op::",
        f"crate::artifacts::{m}::schema::mutations::text::",
    )
    return content


def rewrite(content: str, rust_mod: str, kind: str) -> str:
    if kind == "pack":
        return content
    if kind == "diff":
        return rewrite_diff(content, rust_mod)
    if kind == "dsl":
        return rewrite_dsl(content, rust_mod)
    if kind == "spr":
        return rewrite_spr(content, rust_mod)
    return content.replace(
        f"crate::artifacts::{rust_mod}::mutations::",
        f"crate::artifacts::{rust_mod}::schema::mutations::",
    )


def restore_one(entry: dict) -> None:
    row = owner_row(entry["plugin"], entry["artifact"])
    art = ROOT / row["path"]
    rust_mod = entry["rust_mod"]
    rel_base = row["path"]
    pairs = [
        ("🔺️diff/🦀️component.rs", f"🧬️schema/🔺️diff/{TEXT}/🦀️component.rs", "diff"),
        ("🗣️dsl/🦀️component.rs", f"🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs", "dsl"),
        ("📡️spr/🦀️component.rs", f"🧬️schema/🧬️mutations/{BINARY}/🦀️component.rs", "spr"),
        ("🔧️op/🦀️component.rs", f"🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs", "op"),
        ("📸️snapshot/🎒️pack/🦀️component.rs", f"🧬️schema/📸️snapshot/{BINARY}/🦀️component.rs", "pack"),
    ]
    for old, new, kind in pairs:
        src = git_show(f"{rel_base}/{old}")
        if not src:
            continue
        body = rewrite(src, rust_mod, kind)
        out = art / new
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(body, encoding="utf-8")
        print("restored", entry["artifact"], new)


def pascal(slug: str) -> str:
    parts = re.split(r"[^a-zA-Z0-9]+", slug)
    return "".join(p[:1].upper() + p[1:] for p in parts if p)


def patch_root_exports(entry: dict) -> None:
    row = owner_row(entry["plugin"], entry["artifact"])
    art = ROOT / row["path"]
    p = art / "🦀️component.rs"
    if not p.exists():
        return
    m = entry["rust_mod"]
    snap = pascal(m) + "Snapshot"
    mut = pascal(m) + "Mutation"
    diff = pascal(m) + "Diff"
    t = p.read_text(encoding="utf-8")
    t = re.sub(
        rf"pub use crate::artifacts::{m}::snapshot::schema::{snap};",
        f"pub use crate::artifacts::{m}::schema::snapshot::{snap};",
        t,
    )
    for line in (
        f"pub use crate::artifacts::{m}::schema::mutations::{mut};",
        f"pub use crate::artifacts::{m}::schema::diff::{diff};",
    ):
        if line not in t:
            t = t.replace(
                f"pub const ",
                f"{line}\n\npub const ",
                1,
            )
    p.write_text(t, encoding="utf-8")


if __name__ == "__main__":
    for e in BATCH:
        restore_one(e)
        patch_root_exports(e)
