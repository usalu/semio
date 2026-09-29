#!/usr/bin/env python3
"""🪚️ L1 T2 fix (ST2's family split made it necessary): the `🧿️semio` stdio family (38 editors) crosses the 256 MiB raw-component
bound in wasm-dev exactly like `🗄️stdio` did (describe 2026-09-29 04:20: 330 MiB core) → the same `strip = "symbols"` override.
usage: l1-stdio-semio-strip.py [--write]   (default dry run; idempotent)"""
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/Cargo.toml")
OLD = """# its wasm-release component ships 53 681 326 B without that section. Same trade, stdio-only.
[profile.wasm-dev.package.semio-s-plugin-norm]
strip = "symbols"

[profile.wasm-dev.package.semio-s-plugin-stdio]
strip = "symbols"
"""
NEW = """# its wasm-release component ships 53 681 326 B without that section. Same trade, stdio-only.
# Its `🧿️semio` family package (38 editors, 2026-09-29) crossed it again: 330 MiB core wasm at describe. Same trade.
[profile.wasm-dev.package.semio-s-plugin-norm]
strip = "symbols"

[profile.wasm-dev.package.semio-s-plugin-stdio]
strip = "symbols"

[profile.wasm-dev.package.semio-s-plugin-stdio-semio]
strip = "symbols"
"""
text = PATH.read_text(encoding="utf-8")
if NEW in text:
    print("applied")
    sys.exit(0)
if text.count(OLD) != 1:
    print(f"PROBLEM anchor found {text.count(OLD)}x")
    sys.exit(1)
if "--write" in sys.argv:
    PATH.write_text(text.replace(OLD, NEW), encoding="utf-8")
    print("written")
else:
    print("dry-run: 1 hunk, 0 problems")
