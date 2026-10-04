#!/usr/bin/env python3
"""🇪️🇺️ S3-NORM: EN 1997 `annex` on the wire is the shared `AnnexChoice` (`En` | `De`, `⚖️compliance/🦀️.rs`), schema-first.

The snapshot schema declared `annex` inline as `["de", "en"]`, contradicting its own `$defs/AnnexChoice`, the Rust enum, the
sqlite companion and every committed vector (`De`/`En`); the norm wire-twin witness exposed it once it demanded that every
non-negative wire be admitted. The snapshot property now `$ref`s the one def (the EN 1992 pattern); the artifact schema, which
carries no `$defs`, states the same enum inline.

Usage: python3 🧪️s3-norm-en1997-annex.py [--check]
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"


def snapshot(document: dict) -> None:
    assert document["$defs"]["AnnexChoice"]["enum"] == ["En", "De"]
    document["properties"]["annex"] = {"x-semio-state": "artifact", "$ref": "#/$defs/AnnexChoice"}


def artifact(document: dict) -> None:
    document["properties"]["annex"] = {"type": "string", "enum": ["En", "De"], "x-semio-state": "artifact"}


def main() -> int:
    check = "--check" in sys.argv
    pending = 0
    for path, edit in ((SCHEMA / "📸️snapshot/🔣️.json", snapshot), (SCHEMA / "🔣️.json", artifact)):
        text = path.read_text()
        document = json.loads(text)
        edit(document)
        out = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
        if out != text:
            pending += 1
            if not check:
                path.write_text(out)
    print(f"{'pending' if check else 'rewritten'}={pending}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
