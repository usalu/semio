#!/usr/bin/env python3
"""🧬️ Proves `🎭️actor`'s committed TypeScript mirror is a verbatim projection of `🎭️actor/🦀️.rs`.

`SchemaMetadata::render_typescript` (`🦀️.rs`) concatenates the `@generated` header with each
`TYPES[i].typescript` literal verbatim. This script checks that every such literal in the Rust
source occurs byte-for-byte in `🤖️generated/🎭️actor/🟦️.ts`, which is what makes an identical
textual edit on both sides equivalent to re-running
`bun nx run @semio-tech/framework-actor-rs:typegen` (a cargo build this slice cannot run).
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
RUST = ROOT / "🧰️framework/🔨️modules/🎭️actor/🦀️.rs"
MIRROR = ROOT / "🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts"
LITERAL = re.compile(r'^\s*typescript: "((?:[^"\\]|\\.)*)",\s*$')


def unescape(raw: str) -> str:
    return raw.replace('\\"', '"').replace("\\n", "\n").replace("\\\\", "\\")


def main() -> int:
    mirror = MIRROR.read_text(encoding="utf8")
    literals = [unescape(match.group(1)) for line in RUST.read_text(encoding="utf8").split("\n") if (match := LITERAL.match(line))]
    missing = [literal for literal in literals if literal + "\n\n" not in mirror]
    print(f"{len(literals)} `typescript:` literals in 🦀️.rs, {len(literals) - len(missing)} present verbatim in the mirror")
    for literal in missing:
        print(f"  NOT IN MIRROR: {literal[:160]}", file=sys.stderr)
    return 1 if missing else 0


if __name__ == "__main__":
    raise SystemExit(main())
