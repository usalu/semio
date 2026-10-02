"""🔎️ S2-NORM WP-6 census: the norm slice of `test schema`'s `schema-export-parser-missing` / `schema-export-incomplete` rule,
replicated over the committed central catalog (`📚️library/🔣️schema-catalog.json`) so a twin edit can be re-measured in a
second instead of a full `test schema` run. Same predicates as `🧪️test/🟦️.ts` `schemaExportCompletenessDiagnostics`
(TypeScript half: `export interface|type|const|class <Export>` and `export function|const|let parse<Export>`).

  python3 🧪️s2-norm-ts-twin-census.py [--list]
"""

import collections
import json
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
CATALOG = f"{ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
NORM = "✏️s/🔌️plugins/📕️norm/"
TS, JSON_FORMAT = "🟦️typescript", "🔣️jsonschema"


def export_file(scope, row, format_id):
    if format_id == JSON_FORMAT:
        return f"{scope['path']}/{row['file']}"
    relative = scope["formats"][format_id]
    facet = row["file"].rsplit("/", 1)[0] if "/" in row["file"] else ""
    name = relative.rsplit("/", 1)[-1]
    return f"{scope['path']}/{facet}/{name}" if facet else f"{scope['path']}/{name}"


def definition(document, exported):
    defs = document.get("$defs") or {}
    if exported in defs:
        return defs[exported]
    return document if document.get("title") == exported else None


def census():
    scopes = json.load(open(CATALOG, encoding="utf-8"))["scopes"]
    found = []
    for scope_id, scope in scopes.items():
        if not scope["path"].startswith(NORM) or TS not in scope["formats"]:
            continue
        for exported, row in scope["exports"].items():
            if not re.fullmatch(r"[A-Z][A-Za-z0-9]*", exported):
                continue
            json_path = f"{ROOT}/{export_file(scope, row, JSON_FORMAT)}"
            if not os.path.exists(json_path):
                found.append(("schema-file-missing", scope_id, exported, json_path))
                continue
            node = definition(json.load(open(json_path, encoding="utf-8")), exported)
            formats = (node or {}).get("x-semio-formats")
            if formats is not None and TS not in formats:
                continue
            ts_path = export_file(scope, row, TS)
            if not os.path.exists(f"{ROOT}/{ts_path}"):
                found.append(("schema-file-missing", scope_id, exported, ts_path))
                continue
            source = open(f"{ROOT}/{ts_path}", encoding="utf-8").read()
            name = re.escape(exported)
            if not re.search(rf"^\s*export\s+(?:interface|type|const|class)\s+{name}\b", source, re.M):
                found.append(("schema-export-incomplete", scope_id, exported, ts_path))
            elif not re.search(rf"^\s*export\s+(?:(?:async\s+)?function|const|let|declare\s+function)\s+parse{name}\b", source, re.M):
                found.append(("schema-export-parser-missing", scope_id, exported, ts_path))
    return found


if __name__ == "__main__":
    rows = census()
    print(collections.Counter(code for code, *_ in rows))
    by_file = collections.Counter((code, re.sub(r".*✳️any/", "", path)) for code, _, _, path in rows)
    shape = collections.Counter((code, re.sub(r"🧬️mutations/[^/]+/", "🧬️mutations/<leaf>/", re.sub(r"^.*?🗿️artifacts/[^/]+/", "", path) if "🗿️artifacts" in path else path)) for code, _, _, path in rows)
    for (code, path), count in sorted(shape.items(), key=lambda item: -item[1]):
        print(f"{count:4} {code} {path}")
    if "--list" in sys.argv:
        for row in rows:
            print("\t".join(row))
