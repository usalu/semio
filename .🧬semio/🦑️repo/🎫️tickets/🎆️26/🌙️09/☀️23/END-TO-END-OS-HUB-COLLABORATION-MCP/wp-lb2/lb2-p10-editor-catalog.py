#!/usr/bin/env python3
"""✏️ LB2 p10 (rule 22, test-only): `editor_catalog` law shape + fixture drift.

- window roster: the law rendered the details window under `window_id: "editor-catalog-details"` with an EMPTY
  `window_instances` roster, so every editor whose details window owns window config was refused
  `window-config.window-context` ("target window is absent from the exact ViewModel window instance roster") — a real shell
  always sends the roster it addresses (`ViewModel::window_instances`, the SDK's own `addressed_preview_view` shape).
- identity law: every editor edits the document schema of its own snapshot model (`E::DOCUMENT_SCHEMA` == the initial
  snapshot's `schema`, `s.`-normalized) — the rule p12 establishes for pdf/gif; an avi/pdf/gif-89a violation fails by name here
  instead of as a `snapshot-edit.schema-identity` refusal deep in the edit chain.
- fixture rows (`PATHS`): restated against each editor's CURRENT initial snapshot (probe `lb2_probe_initial_snapshots`, scratch
  only): rows whose path vanished (gltf, obj, pdf 1.4, xlsx), json's `/value` written as the snapshot's own tagged `JsonValue`
  (`{kind: bool, value}`, not a raw JSON boolean), txt edits `/lines` (a trailing newline on a document with NO line is not a native
  text shape — the contract correctly refuses it `publication-mismatch`); csv/mp3 edits that a native file cannot carry on an empty
  document (a header flag with no row, a 3-byte "ID3v1" tag) become representable ones (a record, a complete 128-byte ID3v1 tag);
  docx no longer rewrites the WordprocessingML namespace (publication refused, correctly) but adds an empty paragraph; xlsx adds a
  shared string with its counts; pdf's family row addresses the canonical 1.7 model every pdf editor edits (p12); png/xml/gif
  rows address fields a NEW document carries (p11: an absent optional chunk is omitted, not `null`; an empty xml document has no
  root; gif 87a has no loop count); jpg edits a JFIF APP0 density the file stores (a re-encode quality hint is not stored).
- rows a saved document keeps exactly (the law reopens the pack): pdf edits its declared version — the one detail that keeps the
  retained COS graph canonical (typed-lane edits re-lower the graph, so the reopened `objects` differ); pptx edits the package
  comment (a typed slide without its parts is not the document the writer saves); ifc 2x3 edits its STEP `FILE_NAME` in the
  canonical Part-21 projection (p16; the EDM preamble is absent in a new document, p11); semio brep adds a vertex (`nextLabel` is an allocator the diff does not carry).
- xml valid gets its own row: its new document is the minimal VALID document (p11), and the edit keeps it valid (text in the
  document element); the family row's root-only document would be well-formed only — refused by the subset (§5.1).
- png edits its pixels: the writer emits interlace method 0 only (encode scope note), so an `interlace` edit never saves.
- retired rows: the docx per-subset rows rewrote the WordprocessingML namespace (publication refused, correctly) and the pdf 1.7
  per-subset rows repeated the family row — every docx/pdf editor now uses its family row.

usage: python3 lb2-p10-editor-catalog.py --dry-run | --write | --revert [--root <tree>]
"""
import hashlib, json, os, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p10/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
LAW = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/✏️editor-catalog/🦀️.rs"
FIXTURE = "✏️s/🔌️plugins/🗄️stdio/🧫️fixtures/✏️editor-catalog/🔣️.json"
ID3V1_TITLE = [84, 65, 71] + list(b"Edited") + [0] * 119
SST_EDITED = {
    "kind": "element",
    "name": "sst",
    "attrs": [{"name": "xmlns", "value": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}, {"name": "count", "value": "1"}, {"name": "uniqueCount", "value": "1"}],
    "children": [{"kind": "element", "name": "si", "attrs": [], "children": [{"kind": "element", "name": "t", "attrs": [], "children": [{"kind": "text", "text": "Edited"}]}]}],
}
ROOT = {"kind": "element", "name": "root", "attrs": [], "children": []}
PATHS = {
    "json": {"path": "/value", "value": {"kind": "bool", "value": True}},
    "txt": {"path": "/lines", "value": ["Edited"]},
    "csv": {"path": "/records", "value": [{"fields": [{"value": "Edited", "quoted": False}]}]},
    "obj": {"path": "/vertices", "value": [{"x": 1.5, "y": 2.25, "z": -3.5}]},
    "gltf": {"path": "/sourceForm", "value": "glb"},
    "pdf": {"path": "/declaredVersion", "value": "1.6"},
    "pptx": {"path": "/opc/comment", "value": "Edited"},
    "ifc": {"path": "/document/header/fileName/0", "value": {"kind": "str", "value": "edited.ifc"}},
    "s.stdio.semio@v1/brep#editor": {"path": "/vertices", "value": [{"id": "v1", "point": {"x": 1.0, "y": 2.0, "z": 3.0}, "tol": 0.125}]},
    "docx": {"path": "/xmlParts/0/document/root/children/0/children", "value": [{"kind": "element", "name": "w:p", "attrs": [], "children": []}]},
    "xlsx": {"path": "/xmlParts/0/document/root", "value": SST_EDITED},
    "mp3": {"path": "/id3v1", "value": {"raw": ID3V1_TITLE}},
    "png": {"path": "/pixels", "value": [0, 0, 0, 255]},
    "xml": {"path": "/doc", "value": {"root": ROOT}},
    "gif": {"path": "/backgroundColorIndex", "value": 1},
    "jpg": {"path": "/jfifXDensity", "value": 72},
}

ADDED = {
    "s.stdio.xml@1.0/valid#editor": {"path": "/doc/root/children", "value": [{"kind": "text", "text": "Edited"}]},
}
RETIRED = [
    "s.stdio.docx@ecma-376/*#editor",
    "s.stdio.docx@ecma-376/strict#editor",
    "s.stdio.docx@ecma-376/transitional#editor",
    *(f"s.stdio.pdf@1.7/{subset}#editor" for subset in ("*", "a", "e", "h", "ua", "vt", "x")),
]

problems = []


def law(text):
    old = '            window_id: Some("editor-catalog-details".into()),\n            locale,\n'
    new = '            window_id: Some("editor-catalog-details".into()),\n            focused_window_id: Some("editor-catalog-details".into()),\n            window_instances: vec![semio_framework::ViewWindowInstance { id: "editor-catalog-details".into(), window_kind_id: details.into() }],\n            locale,\n'
    if text.count(old) != 1:
        problems.append(f"law: window view anchor x{text.count(old)}")
        return text
    text = text.replace(old, new)
    old = '    let base = E::initial_snapshot();\n    let before = snapshot_json(&base);\n'
    new = old + '    let schema_identity = |schema: &str| schema.trim_start_matches("s.").to_string();\n    assert_eq!(before["schema"].as_str().map(schema_identity), Some(schema_identity(E::DOCUMENT_SCHEMA)), "{} edits the document schema of its own snapshot model", definition.id);\n'
    if text.count(old) != 1:
        problems.append(f"law: identity anchor x{text.count(old)}")
        return text
    return text.replace(old, new)


def fixture(text):
    document = json.loads(text)
    for key, row in PATHS.items():
        if key not in document["snapshotEdits"]:
            problems.append(f"fixture: no snapshotEdits row {key}")
            continue
        document["snapshotEdits"][key] = row
    for key, row in ADDED.items():
        if key in document["snapshotEdits"]:
            problems.append(f"fixture: snapshotEdits row {key} exists")
            continue
        document["snapshotEdits"][key] = row
    for key in RETIRED:
        if document["snapshotEdits"].pop(key, None) is None:
            problems.append(f"fixture: no snapshotEdits row {key} to retire")
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    edits = {LAW: law, FIXTURE: fixture}
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for path in edits:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after != before:
            staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
