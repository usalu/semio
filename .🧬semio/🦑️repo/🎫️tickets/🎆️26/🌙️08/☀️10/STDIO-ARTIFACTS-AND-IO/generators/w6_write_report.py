#!/usr/bin/env python3
"""Verify W6 batch1a migration on disk."""
from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
BUILDER, DECOMPOSER = TOK["builder"], TOK["decomposer"]
TEXT, BINARY = TOK["text"], TOK["binary"]
DESER, SER = TOK["deserializers"], TOK["serializers"]

BATCH = json.loads((TICKET / "generators/w6_batch1a_migrate.json").read_text())
OLD = ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr", "🧬️mutations"]
REQUIRED_REL = [
    BUILDER,
    DECOMPOSER,
    f"🧬️schema/📸️snapshot/{TEXT}",
    f"🧬️schema/📸️snapshot/{BINARY}",
    f"🚪️io/📥️import/{DESER}/🗿️artifacts",
    f"🚪️io/📤️export/{SER}/🗿️artifacts",
]

lines = ["# W6 Batch1a Report", "", f"Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`", ""]

for row in BATCH:
    plug = row["plugin"]
    crate = row["crate"]
    art = Path(row["art"])
    lines.append(f"## {plug}")
    lines.append("")
    r = subprocess.run(["cargo", "check", "-p", crate], cwd=ROOT, capture_output=True, text=True)
    ok = r.returncode == 0
    lines.append(f"- **cargo check `-p {crate}`**: {'✅ green' if ok else '❌ failed'}")
    if ok:
        tail = (r.stdout + r.stderr).strip().splitlines()[-1] if (r.stdout + r.stderr).strip() else ""
        if tail:
            lines.append(f"  - last line: `{tail}`")
    else:
        lines.append("```")
        lines.append((r.stdout + r.stderr)[-2000:])
        lines.append("```")
    lines.append("- **Path.exists verification**:")
    for rel in REQUIRED_REL:
        p = art / rel
        lines.append(f"  - `{rel}`: `{p.exists()}`")
    for old in OLD:
        p = art / old
        if old == "🧬️mutations":
            bad = p.exists() and p.parent == art
        else:
            bad = p.exists()
        lines.append(f"  - old `{old}` gone: `{not bad}` (`{p.exists()}`)")
    lines.append("")

out = TICKET / "🧪w6-batch1a-report.md"
out.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(out)
