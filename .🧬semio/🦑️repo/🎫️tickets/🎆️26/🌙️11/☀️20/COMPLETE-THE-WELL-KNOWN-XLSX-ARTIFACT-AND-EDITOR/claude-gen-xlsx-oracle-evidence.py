#!/usr/bin/env python3
"""Authors the insert-cell before/after pair, the language-agnostic third-party pair table and the insert-cell evidence row of the base subset.

The pair is HANDCRAFTED, and registered as such: its `before` is the committed openpyxl-3.1.5-written package of `set-cell-applied` (openpyxl itself is not installable
offline), and its `after` is that package with one `<row>` added to `sheetData` and the `dimension` widened. The pair table maps every committed third-party-generated pair
(openpyxl for the grid kinds, jszip + fast-xml-parser for the four plumbing kinds) to the wire mutation that turns its `before` into its `after`.
Idempotent: safe to re-run.
"""
import hashlib, importlib.util, io, json, zipfile
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
FIXTURES = SUBSET / "🧪️tests/🧫️fixtures"
MANIFEST = SUBSET / "🔮️oracles/🔣️.json"
SHEET = "xl/worksheets/sheet1.xml"
NS = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
HYPERLINK = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
CUSTOM = "http://example.invalid/relationships/oracle-retyped"

spec = importlib.util.spec_from_file_location("revision", Path(__file__).with_name("claude-xlsx-address-revision.py"))
revision = importlib.util.module_from_spec(spec)
spec.loader.exec_module(revision)

def sha(data): return "sha256:" + hashlib.sha256(data).hexdigest()

def patched(source: bytes) -> bytes:
    zin = zipfile.ZipFile(io.BytesIO(source))
    out = io.BytesIO()
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as zout:
        for info in zin.infolist():
            data = zin.read(info.filename)
            if info.filename == SHEET:
                text = data.decode("utf-8")
                assert '<dimension ref="A1:B1"/>' in text and "</sheetData>" in text
                text = text.replace('<dimension ref="A1:B1"/>', '<dimension ref="A1:B2"/>').replace("</sheetData>", '<row r="2"><c r="A2" t="n"><v>42</v></c></row></sheetData>')
                data = text.encode("utf-8")
            clone = zipfile.ZipInfo(info.filename, info.date_time)
            clone.compress_type = zipfile.ZIP_DEFLATED
            clone.external_attr = info.external_attr
            zout.writestr(clone, data)
    return out.getvalue()

def insert_cell_pair():
    before = (FIXTURES / "✍️set-cell-applied/⬅️before.xlsx").read_bytes()
    after = patched(before)
    directory = FIXTURES / "➕️insert-cell-applied"
    directory.mkdir(exist_ok=True)
    (directory / "⬅️before.xlsx").write_bytes(before)
    (directory / "➡️after.xlsx").write_bytes(after)
    root = revision.parse(zipfile.ZipFile(io.BytesIO(before)).read(SHEET))
    return before, after, revision.revision(root, [4])

def evidence(before, after):
    media = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    return {
        "id": "insert-cell-applied",
        "class": "handcrafted",
        "target": {"artifact": "s.stdio.xlsx", "standard": "ecma-376", "subset": "base"},
        "mutation": "insert-cell",
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "degree"},
        "files": [
            {"role": "expected-before-xlsx", "path": "../🧪️tests/🧫️fixtures/➕️insert-cell-applied/⬅️before.xlsx", "mediaType": media, "sha256": sha(before), "bytes": len(before)},
            {"role": "expected-after-xlsx", "path": "../🧪️tests/🧫️fixtures/➕️insert-cell-applied/➡️after.xlsx", "mediaType": media, "sha256": sha(after), "bytes": len(after)},
        ],
        "provenance": {"source": "handcrafted", "license": "MIT (openpyxl package bytes of set-cell-applied)", "attribution": "The before package is the committed openpyxl 3.1.5 output of set-cell-applied; the after package adds the row <row r=\"2\"><c r=\"A2\"><v>42</v></c></row> to its sheetData and widens its dimension, by the zip-part patch of claude-gen-xlsx-oracle-evidence.py", "security": "scanned-clean", "privacy": "no-personal-data"},
        "comparisonProfile": "exact-bytes-v1",
        "reproducible": True,
        "family": "structural",
        "notes": "A vacant A2 of Sheet is filled with the number 42, in a row the sheet did not have.",
    }

def rel(id_, target="https://example.invalid/oracle"): return {"owner": "", "id": id_, "relType": HYPERLINK, "target": target, "external": True}

def plumbing(id_, directory, kind, params, outcome="applied"):
    return {"id": id_, "generator": "jszip@3.10.1 + fast-xml-parser@5.11.1", "fixture": directory, "compare": "plumbing", "outcome": outcome, "mutation": {"kind": kind, "params": params}}

def grid(id_, directory, generator, kind, params):
    return {"id": id_, "generator": generator, "fixture": directory, "compare": "grid", "outcome": "applied", "mutation": {"kind": kind, "params": params}}

def cell(row_index, col_index, part=SHEET):
    return {"partPath": part, "nodePath": [4, row_index, col_index], "namespaceUri": NS, "localName": "c", "revision": "0000000000000000"}

def pair_table(insert_revision):
    openpyxl = "openpyxl@3.1.5"
    root_last = {"owner": "", "id": "rId3", "relType": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties", "target": "docProps/app.xml", "external": False}
    return [
        grid("insert-sheet-applied", "➕️insert-sheet-applied", openpyxl, "insert-sheet", {"sheet": {"name": "Sheet2", "cells": []}}),
        grid("remove-sheet-applied", "➖️remove-sheet-applied", openpyxl, "remove-sheet", {"name": "Sheet2"}),
        grid("rename-sheet-applied", "🏷️rename-sheet-applied", openpyxl, "rename-sheet", {"name": "Sheet1", "newName": "Renamed"}),
        grid("set-cell-applied", "✍️set-cell-applied", openpyxl, "set-cell", {"address": cell(0, 1), "value": {"kind": "number", "value": 2}}),
        grid("remove-cell-applied", "🧽️remove-cell-applied", openpyxl, "remove-cell", {"address": cell(0, 1)}),
        grid("insert-cell-applied", "➕️insert-cell-applied", "handcrafted from openpyxl@3.1.5 bytes", "insert-cell", {"address": {"worksheet": {"partPath": SHEET, "nodePath": [4], "namespaceUri": NS, "localName": "sheetData", "revision": insert_revision}, "row": 2, "column": 0}, "value": {"kind": "number", "value": 42}}),
        grid("set-shared-string-applied", "🔤️set-shared-string-applied", "handcrafted from openpyxl@3.1.5 bytes", "set-shared-string", {"index": 0, "value": "World"}),
        plumbing("set-relationship-inserts-an-external-hyperlink-first", "📎️set-relationship-inserts-an-external-hyperlink-first", "set-relationship", {**rel("rIdOracleLink"), "index": 0}),
        plumbing("set-relationship-appends-an-external-hyperlink", "📎️set-relationship-appends-an-external-hyperlink", "set-relationship", rel("rIdOracleLink")),
        plumbing("set-relationship-retypes-the-last-root-relationship", "📎️set-relationship-retypes-the-last-root-relationship", "set-relationship", {**root_last, "relType": CUSTOM}),
        plumbing("set-relationship-no-op-identical-relationship", "📎️set-relationship-no-op-identical-relationship", "set-relationship", dict(root_last)),
        plumbing("remove-relationship-drops-the-last-root-relationship", "🧷️remove-relationship-drops-the-last-root-relationship", "remove-relationship", {"owner": "", "id": "rId3"}),
        plumbing("remove-relationship-drops-an-added-hyperlink", "🧷️remove-relationship-drops-an-added-hyperlink", "remove-relationship", {"owner": "", "id": "rIdOracleLink"}),
        plumbing("remove-relationship-rejected-missing-id", "🧷️remove-relationship-rejected-missing-id", "remove-relationship", {"owner": "", "id": "rIdNothing"}, "rejected"),
        plumbing("set-content-type-inserts-a-default-first", "📇️set-content-type-inserts-a-default-first", "set-content-type", {"isOverride": False, "name": "zzoracle", "contentType": "application/x-oracle", "index": 0}),
        plumbing("set-content-type-retypes-an-existing-default", "📇️set-content-type-retypes-an-existing-default", "set-content-type", {"isOverride": False, "name": "xml", "contentType": "application/x-oracle-retyped"}),
        plumbing("set-content-type-no-op-identical-entry", "📇️set-content-type-no-op-identical-entry", "set-content-type", {"isOverride": False, "name": "rels", "contentType": "application/vnd.openxmlformats-package.relationships+xml"}),
        plumbing("set-content-type-rejected-default-in-another-case", "📇️set-content-type-rejected-default-in-another-case", "set-content-type", {"isOverride": False, "name": "ZZORACLE", "contentType": "application/x-oracle-other"}, "rejected"),
        plumbing("remove-content-type-drops-an-added-default", "🧺️remove-content-type-drops-an-added-default", "remove-content-type", {"isOverride": False, "name": "zzoracle"}),
        plumbing("remove-content-type-rejected-missing-entry", "🧺️remove-content-type-rejected-missing-entry", "remove-content-type", {"isOverride": False, "name": "nothing"}, "rejected"),
    ]

def main():
    before, after, insert_revision = insert_cell_pair()
    table = {
        "schema": "semio.xlsx.third-party-pairs/v1",
        "description": "Every committed before/after pair of the base subset that a third-party tool produced (openpyxl for the grid kinds, jszip + fast-xml-parser for the four OPC plumbing kinds), with the wire mutation that turns its before into its after. Language-agnostic: the Rust oracle judges its own computation of each mutation against the committed after; the same table drives a python or bun verifier.",
        "pairs": pair_table(insert_revision),
    }
    directory = FIXTURES / "🔭️third-party-pairs"
    directory.mkdir(exist_ok=True)
    (directory / "🔣️.json").write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    original = MANIFEST.read_text(encoding="utf-8")
    manifest = json.loads(original)
    entry = evidence(before, after)
    at = next((i for i, e in enumerate(manifest["testEvidence"]) if e["id"] == entry["id"]), None)
    if at is None: manifest["testEvidence"].append(entry)
    else: manifest["testEvidence"][at] = entry
    text = json.dumps(manifest, indent=2, ensure_ascii=False) + ("\n" if original.endswith("\n") else "")
    text = text.replace('"tolerance": 1e-09', '"tolerance": 1e-9').replace('"features": [\n        "testkit"\n      ]', '"features": ["testkit"]')
    MANIFEST.write_text(text, encoding="utf-8")
    print("insert-cell sheetData revision", insert_revision)

main()
