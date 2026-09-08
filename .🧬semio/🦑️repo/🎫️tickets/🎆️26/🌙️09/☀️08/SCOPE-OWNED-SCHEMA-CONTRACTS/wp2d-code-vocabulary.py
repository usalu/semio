#!/usr/bin/env python3
"""🔠️ Renames the catalog generator's diagnostic codes onto the harness `schema-*` vocabulary (ledger row 97).

Left column: the generator's own name. Right column: the harness `SCHEMA_DIAGNOSTIC_CODE_TABLE` id where one
exists, otherwise the `schema-*` name the harness is asked to register (see the report's cross-partition section).
Only quoted occurrences are rewritten; prose is fixed by hand.
"""
import sys

MAPPING = {
    "catalog-absent": "schema-catalog-missing",
    "catalog-malformed": "schema-catalog-malformed",
    "catalog-stale": "schema-catalog-stale",
    "dependency-uncataloged": "schema-cross-scope-dependency-uncataloged",
    "dependency-undeclared": "schema-cross-scope-dependency-forbidden",
    "document-dialect-unexpected": "schema-dialect-not-draft-07",
    "document-id-duplicate": "schema-module-id-duplicate",
    "document-id-missing": "schema-module-id-missing",
    "document-id-unaddressable": "schema-module-id-unaddressable",
    "document-not-object": "schema-document-not-object",
    "document-unparseable": "schema-document-unparseable",
    "enum-empty": "schema-enum-empty",
    "export-format-missing": "schema-export-incomplete",
    "export-formats-annotation-invalid": "schema-export-formats-annotation-invalid",
    "export-id-duplicate": "schema-export-id-duplicate",
    "export-id-invalid": "schema-export-id-invalid",
    "fixture-defines-schema": "schema-fixture-defines-schema",
    "module-level-ineligible": "schema-owner-ineligible",
    "module-scope-id-inconsistent": "schema-module-id-inconsistent",
    "module-scope-id-missing": "schema-module-id-missing",
    "mutation-aggregate-id-grammar": "schema-mutation-aggregate-id",
    "mutation-aggregate-kinds-redundant": "schema-mutation-aggregate-kinds-redundant",
    "mutation-leaf-id-grammar": "schema-mutation-leaf-id",
    "mutation-leaf-schema-absent": "schema-mutation-leaf-schema-absent",
    "placement-retired-location": "schema-placement-forbidden-filename",
    "ref-not-catalog-addressable": "schema-ref-unresolved",
    "ref-unresolved": "schema-ref-unresolved",
    "scope-id-duplicate": "schema-scope-ambiguous",
}


def main():
    for path in sys.argv[1:]:
        text = original = open(path, encoding="utf-8").read()
        for source, target in sorted(MAPPING.items(), key=lambda row: -len(row[0])):
            text = text.replace(f'"{source}"', f'"{target}"')
        if text != original:
            open(path, "w", encoding="utf-8").write(text)
            print(f"edit {path}")


main()
