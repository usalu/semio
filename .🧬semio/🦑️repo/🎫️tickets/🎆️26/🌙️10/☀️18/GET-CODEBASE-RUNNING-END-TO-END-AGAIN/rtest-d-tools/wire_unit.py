#!/usr/bin/env python3
"""Wires the cargo unit test file of one repo-*-rs crate: a `[dev-dependencies]` fixture reader in Cargo.toml and the
`#[cfg(test)] #[path] mod tests;` region at the end of the crate root.  usage: wire_unit.py <module-dir-name>"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
base = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules" / sys.argv[1] / "📦️packages/🦀️rust"
cargo = base / "Cargo.toml"
text = cargo.read_text(encoding="utf8")
dep = 'semio-framework-test-fixtures = { path = "../../../../../../🔨️modules/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust" }'
if dep not in text:
    if "[dev-dependencies]" in text:
        head, tail = text.split("[dev-dependencies]\n", 1)
        text = head + "[dev-dependencies]\n" + dep + "\n" + tail
    else:
        text = text.rstrip("\n") + "\n\n[dev-dependencies]\n" + dep + "\n"
    cargo.write_text(text, encoding="utf8")
root = base / "🦀️.rs"
source = root.read_text(encoding="utf8")
if "../../🧪️tests/🔬️unit/🦀️.rs" not in source:
    source = source.rstrip("\n") + '\n\n//#region 🧪️Tests\n\n#[cfg(test)]\n#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n\n//#endregion 🧪️Tests\n'
    root.write_text(source, encoding="utf8")
print("wired", sys.argv[1])
