#!/usr/bin/env python3
"""🪪️ LB2 p12 (window 3 T6+, stdio pdf + gif): an app's document schema is its snapshot model's own schema.

Rule (decided with the coordinator, 2026-09-29): ONE document schema per document MODEL — each standard version that has its own
snapshot type has its own document schema, declared where the kind is declared (the artifact root's declaration binds
`schema id → codec → snapshot type`), and every editor/viewer's `DOCUMENT_SCHEMA` is the schema of the snapshot type it edits.
Measured violations (`editor_catalog`, scratch 07:3x, after p9): every snapshot edit refused `snapshot-edit.schema-identity`.
- pdf: all 10 editors + 10 viewers edit the canonical 1.7 object graph (`crate::PdfSnapshot`, schema `stdio.pdf.1.7`, root
  S-6 note: "1.7 is the real object-graph engine … now canonical") but declared `stdio.pdf` — the 1.4 `PageDoc` stub's schema
  (`standards::v1_4`, the native `stdio.native.pdf.v1` codec). → `STDIO_PDF17_DOCUMENT_SCHEMA`.
- gif: the 89a editor + viewer edit `standards::v89a` (`stdio.gif.89a`, root S-6: 89a canonical) but declared `stdio.gif` (the 87a
  model's schema). → `STDIO_GIF89A_DOCUMENT_SCHEMA`.
- the gif 89a editor's tool proofs declared `artifact_schema: "stdio.gif"` → `interactive-job.catalog-authority` (schema_eq=false)
  once its DOCUMENT_SCHEMA was right → `stdio.gif.89a` (pdf editors: `stdio.pdf.1.7`).
- a new pdf document is the model's own new document: `blank_pdf_snapshot()` = `decode_pdf(encode_pdf(default))` (the retained
  object graph a fresh write produces, read back — `demo_pdf17_snapshot`'s precedent); the graph-less `Default` saved as a
  document that reopened as a different one. All 20 pdf apps' `initial_snapshot()` return it.
- the pdf 1.7 snapshot contract (+ its TS twin and the TS-embedded schema) describes `PdfObject`'s `real`/`ref` arms as
  `{kind, value: …}`, but `#[value(tag = "kind")]` projects an OBJECT payload flattened beside the tag (the value derive's
  documented internally-tagged rule, serde's too) — `{kind: "ref", num, gen}`; every retained COS graph (a new document's
  first) failed `constraint-invalid … objects[0].value: expected exactly one matching oneOf branch` (python-jsonschema agrees) →
  the arms describe the declared encoding (`allOf` the payload + the tag; TS `{ kind: "ref" } & ObjRef`). `PdfValueDiff` is
  unaffected (struct variants carrying `value`).
- roots name the re-exported document schema (`STDIO_PDF17_DOCUMENT_SCHEMA`; gif: the export p9 adds, added here only if absent).
Laws: `editor_catalog` asserts the rule for every editor (p10), the edit/undo/redo/save/reopen chain proves it end to end.

usage: python3 lb2-p12-document-schema-identity.py --dry-run | --write | --revert [--root <tree>]
"""
import glob, hashlib, json, os, re, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p12/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
PDF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
GIF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif"
PDF17_EDITOR_TESTS = f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
PDF17_SNAPSHOT = f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"
PDF17_CONTRACT = f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"
PDF17_CONTRACT_TS = f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts"
FLATTENED = {"real": "PdfDecimal", "ref": "ObjRef"}
R9 = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"
GIF89A = [f"{GIF}/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs", f"{GIF}/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]

problems = []


def once(text, old, new, label):
    if text.count(old) != 1:
        problems.append(f"{label}: anchor x{text.count(old)}")
        return text
    return text.replace(old, new)


def pdf_root(text):
    text = once(text, "pub use schema::snapshot::PdfSnapshot;\n", "pub use schema::snapshot::PdfSnapshot;\npub use schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA;\n", "pdf root")
    return once(text, "document_codec_bare::<PdfSnapshot, PdfMutation>(standards::v1_7::subsets::base::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA);\n", "document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA);\n", "pdf root codec")


def pdf_blank(text):
    old = f"/// 🦑 Pure snapshot constructors, no codec/IO concern.\n{R9}pub fn empty_pdf_snapshot() -> PdfSnapshot {{\n    PdfSnapshot::default()\n}}\n"
    new = (
        "/// 🆕️ A new pdf document: the empty 1.7 document as the real codec round-trips it — its retained object graph\n"
        "/// (`objects`/`trailer`) is what a fresh write produced, read back, exactly like [`demo_pdf17_snapshot`]; the empty\n"
        "/// `Default` carries no graph, so it saved as a document that reopened as a different one.\n"
        f"{R9}pub fn blank_pdf_snapshot() -> PdfSnapshot {{\n"
        "    use crate::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};\n"
        "    encode_pdf(&PdfSnapshot::default()).and_then(|bytes| decode_pdf(&bytes)).expect(\"blank_pdf_snapshot: the empty document round-trips through the real codec\")\n"
        "}\n"
    )
    return once(text, old, new, "pdf blank")


def flattened_arms(text, label):
    for kind, payload in FLATTENED.items():
        arm = re.compile(r'( *)\{\n\1  "type": "object",\n\1  "properties": \{\n\1    "kind": \{\n\1      "const": "' + kind + r'"\n\1    \},\n\1    "value": \{\n\1      "\$ref": "#/\$defs/' + payload + r'"\n\1    \}\n\1  \},\n\1  "required": \[\n\1    "kind",\n\1    "value"\n\1  \]\n\1\}')
        found = arm.findall(text)
        if len(found) != 1:
            problems.append(f"{label}: {kind} arm x{len(found)}")
            continue
        s = found[0]
        flat = "\n".join(s + line for line in ["{", '  "allOf": [', "    {", f'      "$ref": "#/$defs/{payload}"', "    },", "    {", '      "type": "object",', '      "properties": {', '        "kind": {', f'          "const": "{kind}"', "        }", "      },", '      "required": [', '        "kind"', "      ]", "    }", "  ]", "}"])
        text = arm.sub(lambda _: flat, text, count=1)
    return text


def pdf_contract(text):
    text = flattened_arms(text, "pdf contract")
    json.loads(text)
    return text


def pdf_contract_ts(text):
    text = once(text, '  | { kind: "real"; value: PdfDecimal }\n', '  | ({ kind: "real" } & PdfDecimal)\n', "pdf contract ts: real")
    text = once(text, '  | { kind: "ref"; value: ObjRef }\n', '  | ({ kind: "ref" } & ObjRef)\n', "pdf contract ts: ref")
    return flattened_arms(text, "pdf contract ts")


def pdf_app(text, label):
    initial = list(re.finditer(r"    fn initial_snapshot\(\) -> ([A-Za-z:]+) \{\n        PdfSnapshot::default\(\)\n    \}\n", text))
    if len(initial) != 1:
        problems.append(f"{label} initial_snapshot: anchor x{len(initial)}")
    else:
        text = text[: initial[0].start()] + f"    fn initial_snapshot() -> {initial[0].group(1)} {{\n        crate::standards::v1_7::subsets::base::schema::snapshot::blank_pdf_snapshot()\n    }}\n" + text[initial[0].end() :]
    text = once(text, "PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};\n", "PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF17_DOCUMENT_SCHEMA};\n", f"{label} import")
    text = once(text, "const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;\n", "const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;\n", f"{label} DOCUMENT_SCHEMA")
    if "/✏️editor/" in label:
        text = once(text, '        artifact_schema: "stdio.pdf",\n', '        artifact_schema: "stdio.pdf.1.7",\n', f"{label} details schema")
    if re.search(r"\bSTDIO_PDF_DOCUMENT_SCHEMA\b", text):
        problems.append(f"{label}: STDIO_PDF_DOCUMENT_SCHEMA still used")
    return text


def pdf17_editor_tests(text):
    count = len(re.findall(r"\bSTDIO_PDF_DOCUMENT_SCHEMA\b", text))
    if count != 2:
        problems.append(f"pdf 1.7 editor tests: STDIO_PDF_DOCUMENT_SCHEMA x{count}")
    return re.sub(r"\bSTDIO_PDF_DOCUMENT_SCHEMA\b", "STDIO_PDF17_DOCUMENT_SCHEMA", text)


def gif_root(text):
    if "pub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;\n" in text:
        return text
    return once(text, "pub use schema::snapshot::GifSnapshot;\n", "pub use schema::snapshot::GifSnapshot;\npub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;\n", "gif root")


def gif89a_app(text, label):
    text = once(text, "use crate::{GIF_89A_DIALECT, STDIO_GIF_DOCUMENT_SCHEMA};\n", "use crate::{GIF_89A_DIALECT, STDIO_GIF89A_DOCUMENT_SCHEMA};\n", f"{label} import")
    count = len(re.findall(r"\bSTDIO_GIF_DOCUMENT_SCHEMA\b", text))
    if count == 0:
        problems.append(f"{label}: no document schema use")
    if "/✏️editor/" in label:
        text = once(text, '        artifact_schema: "stdio.gif",\n', '        artifact_schema: "stdio.gif.89a",\n', f"{label} tool proof schema")
    return re.sub(r"\bSTDIO_GIF_DOCUMENT_SCHEMA\b", "STDIO_GIF89A_DOCUMENT_SCHEMA", text)


def plan():
    edits = {f"{PDF}/🦀️.rs": pdf_root, f"{GIF}/🦀️.rs": gif_root}
    for role in ("✏️editor", "👁️viewer"):
        for path in sorted(glob.glob(os.path.join(TREE, PDF, "🏅️standards", "*", "🪆️subsets", "*", role, "🦀️.rs"))):
            relative = os.path.relpath(path, TREE)
            edits[relative] = lambda text, relative=relative: pdf_app(text, relative)
    for path in GIF89A:
        edits[path] = lambda text, path=path: gif89a_app(text, path)
    edits[PDF17_EDITOR_TESTS] = pdf17_editor_tests
    edits[PDF17_SNAPSHOT] = pdf_blank
    edits[PDF17_CONTRACT] = pdf_contract
    edits[PDF17_CONTRACT_TS] = pdf_contract_ts
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    edits = plan()
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
        if after == before and path != f"{GIF}/🦀️.rs":
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    staged = {path: pair for path, pair in staged.items() if pair[0] != pair[1]}
    pdf_apps = sum(1 for path in staged if path.startswith(PDF) and path not in (f"{PDF}/🦀️.rs", PDF17_EDITOR_TESTS, PDF17_SNAPSHOT, PDF17_CONTRACT, PDF17_CONTRACT_TS))
    if pdf_apps != 20:
        problems.append(f"pdf: {pdf_apps} editor/viewer files, expected 20")
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
