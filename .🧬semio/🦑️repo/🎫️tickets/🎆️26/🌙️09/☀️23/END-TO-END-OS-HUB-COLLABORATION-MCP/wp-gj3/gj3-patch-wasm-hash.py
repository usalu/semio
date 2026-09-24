#!/usr/bin/env python3
"""Patch wfc owner descriptor hashes.wasmSha256 to match the staged wasm-dev artifact."""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
DESC = (HERE / "links" / "wfc-descriptor.json").resolve()
WASM = (HERE / "links" / "wfc-staged.wasm").resolve()


def main() -> int:
    if not WASM.is_file():
        print(f"missing staged wasm: {WASM}", file=sys.stderr)
        return 2
    if not DESC.is_file():
        print(f"missing descriptor: {DESC}", file=sys.stderr)
        return 2
    digest = hashlib.sha256(WASM.read_bytes()).hexdigest()
    data = json.loads(DESC.read_text(encoding="utf-8"))
    hashes = data.setdefault("hashes", {})
    old = hashes.get("wasmSha256")
    hashes["wasmSha256"] = digest
    # descriptorSha256 is the hash of the descriptor WITHOUT the hashes block being circular —
    # describe recomputes it; for MCP preflight only wasmSha256 is checked against bytes.
    DESC.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"patched wasmSha256 {old} -> {digest}")
    print(f"wasm bytes={WASM.stat().st_size} mtime={WASM.stat().st_mtime}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
