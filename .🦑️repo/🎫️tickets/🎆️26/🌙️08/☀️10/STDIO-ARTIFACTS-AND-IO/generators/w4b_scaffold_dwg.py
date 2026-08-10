#!/usr/bin/env python3
"""Scaffold stdio dwg facet tree from bmp (binary-shaped)."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
SRC = PLUGIN / "🗿️artifacts" / "🖼️bmp"
DST = PLUGIN / "🗿️artifacts" / "🖊️dwg"

PAIRS = [
    ("🖼️bmp", "🖊️dwg"),
    ("stdio.bmp", "stdio.dwg"),
    ("s.stdio.bmp", "s.stdio.dwg"),
    ("Bmp", "Dwg"),
    ("bmp", "dwg"),
    ("BMP", "DWG"),
    ("example.bmp", "example.dwg"),
]


def replace_all(text: str) -> str:
    for old, new in PAIRS:
        text = text.replace(old, new)
    return text


def main() -> None:
    if DST.exists():
        shutil.rmtree(DST)
    shutil.copytree(SRC, DST)
    for path in DST.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix in (
            ".rs",
            ".ts",
            ".json",
            ".semio",
            ".proto",
            ".graphql",
            ".ebnf",
            ".g4",
            ".abnf",
            ".ksy",
            ".spicy",
        ):
            path.write_text(replace_all(path.read_text(encoding="utf-8")), encoding="utf-8")
    assets = DST / "📚️examples/🎬️demo/🖼️assets"
    for p in assets.iterdir():
        if p.is_file() and p.name.endswith(".bmp"):
            p.unlink()
    (assets / "example.dwg").write_bytes(b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    print(f"scaffolded {DST}")


if __name__ == "__main__":
    main()
