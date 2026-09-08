#!/usr/bin/env python3
"""🔎️ Groups every catalogued export that its scope's module root does not declare, by partition owner.

Answers cross-partition request #43: for each `schema-export-unknown` the harness reports, say whether the
export is declared in a taxonomy facet child (so the regenerated catalog's `exports.<Id>.file` fixes it) or
declared nowhere at all (so the partition owner must declare it or drop the reference).

Reads only. Writes `🗑️generated/wp1b-export-unknown-by-partition.json`.
"""
import json
import os
import sys
from collections import defaultdict

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
LIB = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library")
TAXONOMY = os.path.join(LIB, "🔣️taxonomy.json")
CATALOG = os.path.join(LIB, "🔣️schema-catalog.json")
OUT = os.path.join(os.path.dirname(__file__), "🗑️generated", "wp1b-export-unknown-by-partition.json")

PARTITIONS = [
    ("mutations", lambda p: "/🧬️mutations/" in p),
    ("hub", lambda p: p.startswith("🌎️hub/")),
    ("os", lambda p: p.startswith("🧰️framework/🛍️products/💻️os/")),
    ("repo", lambda p: p.startswith("🧰️framework/🛍️products/🦑️repo/")),
    ("plugins", lambda p: p.startswith("✏️s/🔌️plugins/")),
    ("framework", lambda p: p.startswith("🧰️framework/")),
]


def partition_of(path):
    for name, test in PARTITIONS:
        if test(path):
            return name
    return "other"


def declares(document, export_id, root_keyword):
    if not isinstance(document, dict):
        return False
    defs = document.get("$defs")
    if isinstance(defs, dict) and export_id in defs:
        return True
    return document.get(root_keyword) == export_id


def read_json(abs_path):
    try:
        with open(abs_path, encoding="utf-8") as handle:
            return json.load(handle)
    except Exception:
        return None


_MODULE_CACHE = {}


def module_documents(module_abs):
    """📚️ Every JSON document inside one 🧬️schema module, as (module-relative dir, parsed) pairs."""
    if module_abs in _MODULE_CACHE:
        return _MODULE_CACHE[module_abs]
    found = []
    for current, directories, filenames in os.walk(module_abs):
        directories[:] = [name for name in directories if not name.startswith(".")]
        for filename in filenames:
            if not filename.endswith(".json"):
                continue
            relative = os.path.relpath(os.path.join(current, filename), module_abs)
            found.append((os.path.dirname(relative), read_json(os.path.join(current, filename)), relative))
    found.sort(key=lambda row: (row[0].count(os.sep), row[0]))
    _MODULE_CACHE[module_abs] = found
    return found


def main():
    taxonomy = read_json(TAXONOMY)
    catalog = read_json(CATALOG)
    if taxonomy is None or catalog is None:
        print("[wp1b] taxonomy or catalog unreadable", file=sys.stderr)
        return 1
    resolution = taxonomy["schemaExportResolution"]
    root_keyword = resolution["rootExportKeyword"]
    normative_kind = taxonomy["schemaFormats"][taxonomy["schemaFacetKinds"][taxonomy["schemaDefaultFacetKind"]]["normativeFormat"]]
    del normative_kind
    rows = defaultdict(list)
    totals = defaultdict(lambda: {"facet-declared": 0, "undeclared": 0, "module-unreadable": 0})
    scopes = catalog["scopes"]
    for scope_id, scope in scopes.items():
        path = scope["path"]
        exports = scope["exports"]
        names = list(exports.keys()) if isinstance(exports, dict) else list(exports)
        json_file = scope.get("formats", {}).get("🔣️jsonschema")
        module = read_json(os.path.join(ROOT, path, json_file)) if json_file else None
        owner = partition_of(path)
        for export_id in names:
            if module is not None and declares(module, export_id, root_keyword):
                continue
            carrier = None
            carrier_file = None
            for candidate in module_documents(os.path.join(ROOT, path)):
                if candidate[1] is not None and declares(candidate[1], export_id, root_keyword):
                    carrier = candidate[0]
                    carrier_file = candidate[2].replace(os.sep, "/")
                    break
            state = "module-unreadable" if module is None and carrier is None else ("facet-declared" if carrier else "undeclared")
            totals[owner][state] += 1
            rows[owner].append({
                "scope": scope_id,
                "export": export_id,
                "modulePath": path,
                "state": state,
                "carrierFacetDir": carrier or "",
                "catalogExportsShape": "object" if isinstance(exports, dict) else "array",
                "proposedCatalogFile": carrier_file,
            })
    sub = defaultdict(lambda: defaultdict(lambda: {"facet-declared": 0, "undeclared": 0, "module-unreadable": 0}))
    for owner, entries in rows.items():
        for entry in entries:
            segments = entry["modulePath"].split("/")
            key = "/".join(segments[:3]) if entry["modulePath"].startswith("✏️s/🔌️plugins/") else "/".join(segments[:2])
            sub[owner][key][entry["state"]] += 1
    report = {
        "request": "cross-partition-requests.md row 43",
        "totalsByOwnerRoot": {owner: {key: dict(counts) for key, counts in sorted(inner.items())} for owner, inner in sorted(sub.items())},
        "catalog": os.path.relpath(CATALOG, ROOT),
        "catalogExportsShape": "object" if scopes and isinstance(next(iter(scopes.values()))["exports"], dict) else "array",
        "scopesInCatalog": len(scopes),
        "totalsByPartition": {owner: dict(counts) for owner, counts in sorted(totals.items())},
        "grandTotal": sum(sum(counts.values()) for counts in totals.values()),
        "byPartition": {owner: sorted(entries, key=lambda row: (row["scope"], row["export"])) for owner, entries in sorted(rows.items())},
    }
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as handle:
        json.dump(report, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print(json.dumps({"grandTotal": report["grandTotal"], "totalsByPartition": report["totalsByPartition"], "catalogExportsShape": report["catalogExportsShape"]}, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
