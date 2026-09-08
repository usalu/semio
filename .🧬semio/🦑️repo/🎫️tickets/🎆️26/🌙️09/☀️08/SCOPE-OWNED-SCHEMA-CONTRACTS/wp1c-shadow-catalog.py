#!/usr/bin/env python3
"""📚️ Projects the live schema catalog into the row-43 shape and mounts it over a shadow repository root.

The tooling worker regenerates `🔣️schema-catalog.json` with `exports: {ExportId: {file, facet}}`; until it
lands, the on-disk catalog is still the old `exports: [ExportId, …]` array and the harness — which
implements the new shape only, no compatibility layer — refuses every row. This script builds the
regenerated shape from the live modules (locating the document that actually declares each export,
nested facet directories included) and writes it into a SHADOW root whose every other entry is a symlink
to the real tree, so the harness can be measured against real modules with a new-shape catalog.

Reads the repository; writes only the shadow root and the ticket's `🗑️generated/` projection.

Usage: wp1c-shadow-catalog.py <shadow root>
"""
import json
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
LIB_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CATALOG_REL = f"{LIB_REL}/🔣️schema-catalog.json"
TAXONOMY = os.path.join(ROOT, LIB_REL, "🔣️taxonomy.json")
OUT = os.path.join(os.path.dirname(__file__), "🗑️generated", "wp1c-shadow-catalog.json")


def read_json(path):
    try:
        with open(path, encoding="utf-8") as handle:
            return json.load(handle)
    except Exception:
        return None


def declares(document, export_id, root_keyword):
    if not isinstance(document, dict):
        return False
    defs = document.get("$defs")
    if isinstance(defs, dict) and export_id in defs:
        return True
    return document.get(root_keyword) == export_id


_MODULE_CACHE = {}


def module_documents(module_abs):
    if module_abs in _MODULE_CACHE:
        return _MODULE_CACHE[module_abs]
    found = []
    for current, directories, filenames in os.walk(module_abs):
        directories[:] = [name for name in directories if not name.startswith(".")]
        for filename in filenames:
            if not filename.endswith(".json"):
                continue
            relative = os.path.relpath(os.path.join(current, filename), module_abs).replace(os.sep, "/")
            found.append((relative, read_json(os.path.join(current, filename))))
    found.sort(key=lambda row: (row[0].count("/"), row[0]))
    _MODULE_CACHE[module_abs] = found
    return found


def facet_of(relative, default_facet):
    """🏷️ The facet LABEL of a carrier: the directory chain it sits in, or the module's own facet."""
    directory = os.path.dirname(relative)
    return default_facet if directory == "" else directory


def project(taxonomy, catalog):
    resolution = taxonomy["schemaExportResolution"]
    root_keyword = resolution["rootExportKeyword"]
    normative = taxonomy["schemaFacetKinds"][taxonomy["schemaDefaultFacetKind"]]["normativeFormat"]
    scopes = {}
    stats = {"scopes": 0, "exports": 0, "root": 0, "facet": 0, "unlocated": 0}
    for scope_id, scope in catalog["scopes"].items():
        exports = scope["exports"]
        names = list(exports.keys()) if isinstance(exports, dict) else list(exports)
        module_rel = scope["path"]
        normative_file = scope.get("formats", {}).get(normative)
        module = read_json(os.path.join(ROOT, module_rel, normative_file)) if normative_file else None
        rows = {}
        for export_id in names:
            stats["exports"] += 1
            if module is not None and declares(module, export_id, root_keyword):
                rows[export_id] = {"file": normative_file, "facet": "schema"}
                stats["root"] += 1
                continue
            carrier = None
            for relative, document in module_documents(os.path.join(ROOT, module_rel)):
                if declares(document, export_id, root_keyword):
                    carrier = relative
                    break
            if carrier is None:
                # 📄️An export nothing declares still gets a row naming where it WOULD live, so the
                # harness reports `schema-export-unknown` for it instead of dropping the whole scope.
                rows[export_id] = {"file": normative_file or "🔣️.json", "facet": "schema"}
                stats["unlocated"] += 1
                continue
            rows[export_id] = {"file": carrier, "facet": facet_of(carrier, "schema")}
            stats["facet"] += 1
        scopes[scope_id] = {
            "path": module_rel,
            "level": scope.get("level"),
            "facetKind": scope.get("facetKind"),
            "formats": scope.get("formats", {}),
            "exports": rows,
            "dependsOn": scope.get("dependsOn", []),
            "hashes": scope.get("hashes", {}),
        }
        stats["scopes"] += 1
    return {"provenance": "projected from the live catalog by wp1c-shadow-catalog.py; not an authority", "scopes": scopes}, stats


def mount(shadow, catalog_document):
    """🔗️ A repository root that IS the live tree, except for the one file this projection replaces."""
    chain = CATALOG_REL.split("/")
    os.makedirs(shadow, exist_ok=True)
    current_rel = ""
    for depth in range(len(chain)):
        current_abs = os.path.join(shadow, *chain[:depth]) if depth else shadow
        source_abs = os.path.join(ROOT, current_rel) if current_rel else ROOT
        os.makedirs(current_abs, exist_ok=True)
        for entry in os.listdir(source_abs):
            if entry == chain[depth]:
                continue
            link = os.path.join(current_abs, entry)
            if not os.path.lexists(link):
                os.symlink(os.path.join(source_abs, entry), link)
        current_rel = "/".join(chain[: depth + 1])
    with open(os.path.join(shadow, CATALOG_REL), "w", encoding="utf-8") as handle:
        json.dump(catalog_document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def main():
    if len(sys.argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    taxonomy = read_json(TAXONOMY)
    catalog = read_json(os.path.join(ROOT, CATALOG_REL))
    if taxonomy is None or catalog is None:
        print("[wp1c] taxonomy or catalog unreadable", file=sys.stderr)
        return 1
    document, stats = project(taxonomy, catalog)
    mount(sys.argv[1], document)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as handle:
        json.dump({"stats": stats, "shadowRoot": sys.argv[1], "liveCatalogExportsShape": "object" if isinstance(next(iter(catalog["scopes"].values()))["exports"], dict) else "array"}, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print(json.dumps({"stats": stats, "shadowRoot": sys.argv[1]}, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
