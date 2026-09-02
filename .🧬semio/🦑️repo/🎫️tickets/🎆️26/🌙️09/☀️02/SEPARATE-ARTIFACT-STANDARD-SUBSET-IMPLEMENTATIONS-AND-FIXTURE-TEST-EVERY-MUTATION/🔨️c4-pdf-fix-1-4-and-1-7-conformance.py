#!/usr/bin/env python3
import os, shutil, hashlib, sys

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)

PDF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf"
TESTS = f"{PDF}/🧪️tests"

def std(v): return f"{PDF}/🏅️standards/🔖️{v}"
def subset_dir(v, s): return f"{std(v)}/🪆️subsets/✳️{s}"

OLD_URI = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf"

RICH_SUBSETS = {
    # subset: (case_name, seed_kind_source)
    "a":  ("mutate-pdf-1-7-a",  "insert-javascript-action"),
    "e":  ("mutate-pdf-1-7-e",  "insert-javascript-action"),
    "h":  ("mutate-pdf-1-7-h",  "insert-javascript-action"),
    "ua": ("mutate-pdf-1-7-ua", "set-info-title"),
    "vt": ("mutate-pdf-1-7-vt", "insert-javascript-action"),
    "x":  ("mutate-pdf-1-7-x",  "insert-javascript-action"),
}

def sha256(path):
    return hashlib.sha256(open(path,"rb").read()).hexdigest()

def new_uri_for(subset_owner_rel_dir_name="🧬️conformance-seed", filename="📄️conformance-seed.pdf"):
    return f"asset://📚️examples/{subset_owner_rel_dir_name}/🖼️assets/{filename}"

NEW_URI = new_uri_for()

OLD_PARA_17 = """  The input is the real, committed 6.3 MB bachelor thesis produced by MiKTeX pdfTeX 1.40.21 — 65
  pages, 3,189 indirect objects, a classic cross-reference table, 70 /Type /Font objects and 23
  /Type /FontDescriptor objects, every one of the 23 carrying an embedded font program (5 /FontFile,
  16 /FontFile2, 2 /FontFile3). Scanning the committed file confirms it carries NO /Encrypt, no
  /S /JavaScript action and no /JS key, no /S /Launch action, no /Subtype /Movie or /Sound
  annotation, no /Type /Filespec, no /OutputIntents, no /MarkInfo, no /StructTreeRoot, no /Lang, no
  /ViewerPreferences, no /AcroForm, no /DPartRoot and no /TrimBox or /ArtBox on any page — it is a
  perfectly ordinary PDF that conforms to no conformance class at all, which is exactly what makes
  it the right input here: every scenario moves the real document along ONE axis of the class and
  then back. It is read where the domain already keeps it; every scenario copies it into the case
  work directory before touching it, and the committed document is never written to."""

NEW_PARA_17 = """  The input is this subset's OWN committed seed document — a one-page PDF built and written by the
  SAME lopdf 0.44 reference this catalog's oracle drives, through this subset's own 🏭️generator
  (../../🏭️generator/🦀️lopdf-engine::build_seed), with two /Type /FontDescriptor objects each
  already carrying a synthetic /FontFile2 embedded program. It is the identical lopdf-verified seed
  already registered as the third-party-generated evidence for this catalog's own per-mutation
  fixture pairs in this subset's own 🧪️oracle/🔣️.json, reused here as the whole-catalog exhaustive
  input. It carries NO /Encrypt, no /S /JavaScript action and no /JS key, no /S /Launch action, no
  /Subtype /Movie or /Sound annotation, no /Type /Filespec, no /OutputIntents, no /MarkInfo, no
  /StructTreeRoot, no /Lang, no /ViewerPreferences, no /AcroForm, no /DPartRoot and no /TrimBox or
  /ArtBox — it is a bare PDF that conforms to no conformance class at all, which is exactly what
  makes it the right input here: every scenario moves it along ONE axis of the class and then back.
  It is read where this subset now keeps it, under its OWN 📚️examples — not a cross-subset
  `asset://` reach into a sibling's directory tree, which the framework's owner-containment guard
  forbids by design (see the C4 shard report of the SEPARATE-ARTIFACT-STANDARD-SUBSET-... ticket).
  Every scenario copies it into the case work directory before touching it, and the committed
  document is never written to."""

OLD_GRAPH = "still on a genuine 3,189-object graph:"
NEW_GRAPH = "still on the subset's own generated seed graph:"

OLD_BULLET = "embed-font-file — descriptor 4's embedded program is REMOVED first — all 23 of the fixture's /FontDescriptor objects already carry one."
NEW_BULLET = "embed-font-file — descriptor 0's embedded program is REMOVED first — both of the seed's two /FontDescriptor objects already carry one."

def edit_feature_rich(path):
    text = open(path, encoding="utf8").read()
    assert OLD_URI in text, f"missing OLD_URI in {path}"
    n_uri = text.count(OLD_URI)
    text = text.replace(OLD_URI, NEW_URI)
    assert OLD_PARA_17 in text, f"missing OLD_PARA_17 in {path}"
    text = text.replace(OLD_PARA_17, NEW_PARA_17)
    assert OLD_GRAPH in text, f"missing OLD_GRAPH in {path}"
    text = text.replace(OLD_GRAPH, NEW_GRAPH)
    assert OLD_BULLET in text, f"missing OLD_BULLET in {path}"
    text = text.replace(OLD_BULLET, NEW_BULLET)
    n_ord = text.count('"descriptorOrdinal": 4')
    text = text.replace('"descriptorOrdinal": 4', '"descriptorOrdinal": 0')
    open(path, "w", encoding="utf8").write(text)
    return n_uri, n_ord

OLD_RS_COMMENT = "//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory\n//! first; the committed asset is never written to."
NEW_RS_COMMENT = "//! Every scenario copies this subset's own committed lopdf-generated seed document (built by\n//! this subset's own 🏭️generator, the same one that produced this catalog's per-mutation fixture\n//! pairs) into the case work directory first; the committed asset is never written to."

def edit_rust_rich(path):
    text = open(path, encoding="utf8").read()
    assert OLD_URI in text, f"missing OLD_URI in {path}"
    n_uri = text.count(OLD_URI)
    text = text.replace(OLD_URI, NEW_URI)
    assert OLD_RS_COMMENT in text, f"missing OLD_RS_COMMENT in {path}"
    text = text.replace(OLD_RS_COMMENT, NEW_RS_COMMENT)
    n_bt = text.count('Some("bachelor-thesis.pdf")')
    text = text.replace('Some("bachelor-thesis.pdf")', 'Some("conformance-seed.pdf")')
    text = text.replace("the whole 3,189-object graph", "the whole seed object graph")
    open(path, "w", encoding="utf8").write(text)
    return n_uri, n_bt

results = []

# --- 1.7 rich conformance subsets ---
for subset, (case, kind) in RICH_SUBSETS.items():
    sub = subset_dir("1.7", subset)
    seed_src = f"{sub}/🧫️fixtures/{kind}/base.pdf"
    seed_dst_dir = f"{sub}/📚️examples/🧬️conformance-seed/🖼️assets"
    seed_dst = f"{seed_dst_dir}/📄️conformance-seed.pdf"
    os.makedirs(seed_dst_dir, exist_ok=True)
    shutil.copyfile(seed_src, seed_dst)
    case_src = f"{TESTS}/{case}"
    feature = f"{case_src}/🥒️.feature"
    rustf = f"{case_src}/🦀️.rs"
    n1 = edit_feature_rich(feature)
    n2 = edit_rust_rich(rustf)
    case_dst = f"{sub}/🧪️tests/{case}"
    os.makedirs(f"{sub}/🧪️tests", exist_ok=True)
    shutil.move(case_src, case_dst)
    results.append((case, "1.7", subset, seed_src, seed_dst, sha256(seed_dst), n1, n2))

print("DONE rich subsets:")
for r in results:
    print(r)
