#!/usr/bin/env python3
"""📝️ The `📝️text` / `💾️binary` facet documents that state `{"type": "object"}` and nothing else.

254 representation facets declare an export (`ProgramDiffText`, `Block2dSnapshotText`, …) whose body
admits every object and describes nothing, while the sibling `🦀️.rs` carries the real grammar and a
`parse(&str)` / `print(…) -> String` pair. The export is therefore missing from every format the
module provides — not because the format is incomplete, but because the JSON says the wrong thing:
a text representation is a **string**, and a binary representation is a byte payload.

This script states the truth in both places at once:

- the JSON document becomes `{"type": "string"}` (text) or a base64-encoded string (binary), and
- the sibling `🦀️.rs` gains `pub type <Export> = String;` / `= Vec<u8>;`, the carrier its own
  `parse`/`print` already speak, so the export exists in Rust under its own name.

Documents that already describe a real shape (`properties`, `$defs`, `required`) are left alone:
those are contracts someone wrote, not placeholders.

Usage: python3 wp4c-representation-facets.py report|apply [--under ✏️s/🔌️plugins/🏛️architect]
"""
from __future__ import annotations

import json
import os
import re
import sys
import collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
JSON_LEAF, RUST_LEAF = "🔣️.json", "🦀️.rs"
TEXT_DIR, BINARY_DIR = "📝️text", "💾️binary"
EXPORT_RE = re.compile(r"^[A-Z][A-Za-z0-9]*$")
REGION = "//#region 🚚️Carrier"


def placeholder(document: dict) -> bool:
    keys = {key for key in document if key not in ("$schema", "$id", "title", "description")}
    return keys == {"type"} and document.get("type") == "object" and EXPORT_RE.match(str(document.get("title") or ""))


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    counts = collections.Counter()
    touched: set[str] = set()
    for base, dirs, files in os.walk(os.path.join(REPO, under)):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv", "target", "dist")]
        kind = os.path.basename(base)
        if kind not in (TEXT_DIR, BINARY_DIR) or JSON_LEAF not in files or "/🧬️schema/" not in f"{base}/":
            continue
        path = os.path.join(base, JSON_LEAF)
        with open(path, encoding="utf-8") as handle:
            document = json.load(handle)
        if not placeholder(document):
            counts["kept-real-shape"] += 1
            continue
        export = document["title"]
        binary = kind == BINARY_DIR
        document["type"] = "string"
        if binary:
            document["contentEncoding"] = "base64"
        document.setdefault("description", f"🚚️ The {'binary' if binary else 'text'} representation of {export[:-4] if export.endswith('Text') else export}: "
                                           f"the instance is the {'base64-encoded byte payload' if binary else 'serialized document'} this facet's grammar and codec speak.")
        counts["representation-restated"] += 1
        rust = os.path.join(base, RUST_LEAF)
        alias = None
        if os.path.exists(rust):
            with open(rust, encoding="utf-8") as handle:
                source = handle.read()
            if re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{re.escape(export)}\b", source, re.M) is None:
                alias = f"pub type {export} = {'Vec<u8>' if binary else 'String'};"
                counts["rust-carrier"] += 1
        if mode != "apply":
            continue
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        touched.add(os.path.relpath(path, REPO))
        if alias:
            block = "\n".join([REGION, f"/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.", alias, "//#endregion 🚚️Carrier"])
            with open(rust, "w", encoding="utf-8") as handle:
                handle.write(f"{source.rstrip()}\n\n{block}\n")
            touched.add(os.path.relpath(rust, REPO))
    print(json.dumps({"mode": mode, "under": under, **counts, "filesTouched": len(touched)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
