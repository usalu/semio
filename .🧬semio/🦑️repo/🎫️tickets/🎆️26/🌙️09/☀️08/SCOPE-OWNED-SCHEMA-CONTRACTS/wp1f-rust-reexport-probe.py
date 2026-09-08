#!/usr/bin/env python3
"""🦀️ How many catalogued exports the `pub use` half of the Rust presence rule admits (ledger row 148).

Reads the derived catalog, and for every scope that provides a `🦀️.rs` asks the OLD rule
(`pub struct|enum|type <Export>`) and the NEW one (that, or a per-name `pub use`) whether each export
is present. The difference is the number of `schema-export-incomplete` rows the amendment clears.
"""
import json
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
CATALOG = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json")
PATH = r"[A-Za-z_][A-Za-z0-9_]*(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)*"


def definition(source, name):
    return re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{re.escape(name)}\b", source, re.M) is not None


def reexport(source, name):
    n = re.escape(name)
    return re.search(rf"^\s*pub\s+use\s+(?![^;\n]*[{{*]){PATH}\s*::\s*(?:{n}|[A-Za-z_][A-Za-z0-9_]*\s+as\s+{n})\s*;", source, re.M) is not None


scopes = json.load(open(CATALOG, encoding="utf-8"))["scopes"]
old = new = considered = 0
gained = []
for scope_id, scope in scopes.items():
    rust = scope.get("formats", {}).get("🦀️rust")
    if not rust:
        continue
    try:
        source = open(os.path.join(ROOT, scope["path"], rust), encoding="utf-8").read()
    except OSError:
        continue
    for export in scope.get("exports", {}):
        if not re.fullmatch(r"[A-Z][A-Za-z0-9]*", export):
            continue
        considered += 1
        a, b = definition(source, export), definition(source, export) or reexport(source, export)
        old += a
        new += b
        if b and not a:
            gained.append(f"{scope_id}/{export}")

print(json.dumps({
    "rust_providing_scopes": sum(1 for s in scopes.values() if s.get("formats", {}).get("🦀️rust")),
    "exports_considered": considered,
    "present_under_definition_only": old,
    "present_under_definition_or_re_export": new,
    "cleared_by_row_148": len(gained),
    "sample": sorted(gained)[:12],
}, indent=2, ensure_ascii=False))
