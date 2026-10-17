#!/usr/bin/env python3
"""Decouples the children of the pdf 1.7 base sqlite snapshot from their root: shared helpers move to a `reader` child, every child names exactly what it uses."""
import re, os, sys, subprocess
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot"
SELF = "crate::standards::v1_7::subsets::base::io::sqlite::snapshot"
SCHEMA = "crate::standards::v1_7::subsets::base::schema::snapshot"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
ORIG = os.environ["F"] + "/r5b-tools/orig/sqlite-split"
rootp = f"{B}/🦀️.rs"
root = open(rootp).read()
mods = re.findall(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod (\w+);', root)
mods = [(p, m) for p, m in mods if m != "tests"]
modmap = {m: p for p, m in mods}
start = root.index("fn integer<")
end = root.index("#[cfg(test)]")
helpers = root[start:end].rstrip() + "\n"
helper_names = re.findall(r'^(?:fn|struct) (\w+)', helpers, re.M)
STORE = [("self", "sqlite_snapshot", r"\bsqlite_snapshot\b"), ("SqliteDatabase as Db", "Db", r"\bDb\b"), ("SqliteRow as RawRow", "RawRow", r"\bRawRow\b"), ("SqliteValue as V", "V", r"\bV\s*::|<V>|\bV\b(?=[,)>])"), ("SqliteSnapshotControl as Control", "Control", r"\bControl\b"), ("SqliteSnapshotPhase as Phase", "Phase", r"\bPhase\b"), ("artifact::{Cell as C}", "C", r"\bC\s*::|\bC<")]
def strip(text):
    text = re.sub(r"//[^\n]*", "", text)
    return re.sub(r'"(?:[^"\\]|\\.)*"', '""', text)
def imported(text):
    names = set()
    for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", text, re.M):
        body = m.group(1)
        for part in re.findall(r"(?:as\s+)?(\w+)\s*(?=[,}]|$)", body):
            names.add(part)
    return names
def defined(text):
    return set(re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:fn|struct|enum|type|const|static|trait|mod)\s+(\w+)", text, re.M))
reader = f"""//! 🧰️ Shared cell helpers and the controlled row reader of the PDF 1.7 sqlite snapshot.
use semio_framework_value::{{ValueError,ValueRefusalKind}};
use store::sqlite_snapshot::{{self, SqliteDatabase as Db, SqliteRow as RawRow, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{{Cell as C}}}};
use std::collections::{{BTreeMap, BTreeSet}};
use {SELF}::number::{{Row,Projection}};

"""
body = helpers
body = re.sub(r"^fn ", "pub fn ", body, flags=re.M)
body = re.sub(r"^struct Reader", "pub struct Reader", body, flags=re.M)
body = re.sub(r"^    ((?:db|rows|groups|used|control|total): )", r"    pub \1", body, flags=re.M)
body = re.sub(r"^    fn ", "    pub fn ", body, flags=re.M)
body = body.replace("number::columns(", f"{SELF}::number::columns(")
reader += body
newroot = "//! 📖️ Explicit PDF 1.7 document, COS, rendering and resource relations.\n"
newroot += f'#[path = "🔢️number/🦀️.rs"]\npub mod number;\n#[path = "🧰️reader/🦀️.rs"]\npub mod reader;\n\n'
for p, m in mods:
    if m in ("number", "document", "admission"): continue
    newroot += f'#[path = "{p}/🦀️.rs"]\npub mod {m};\n'
newroot += f'#[path = "📄️document/🦀️.rs"]\npub mod document;\npub use document::{{PdfConformanceProfile,validate_conformance_profile}};\n#[path = "🛂️admission/🦀️.rs"]\npub mod admission;\n\n'
newroot += root[end:]
GLOBS = [f"use {SELF}::*;", "use super::*;"]
def subtree_files(m):
    p = modmap[m]
    out = [f"{B}/{p}/🦀️.rs"]
    if m == "admission":
        for sub in os.listdir(f"{B}/{p}"):
            q = f"{B}/{p}/{sub}/🦀️.rs"
            if os.path.exists(q): out.append(q)
    return out
results = {"reader": (f"{B}/🧰️reader/🦀️.rs", reader), "root": (rootp, newroot)}
for m in modmap:
    files = subtree_files(m)
    main = files[0]
    text = open(main).read()
    glob = next((g for g in GLOBS if g + "\n" in text), None)
    assert glob, m
    ids = set()
    for f in files:
        ids |= set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", strip(open(f).read())))
    code = "\n".join(strip(open(f).read()) for f in files)
    have = imported(text) | defined(text)
    lines = []
    if m != "number" or True:
        lines.append(f"use {SCHEMA}::*;")
    used_helpers = [h for h in helper_names if h in ids and h not in have]
    if used_helpers: lines.append(f"use {SELF}::reader::{{{', '.join(used_helpers)}}};")
    pieces = [piece for piece, name, rx in STORE if re.search(rx, code) and name not in have]
    if pieces: lines.append("use store::sqlite_snapshot::{" + ", ".join(pieces) + "};")
    if re.search(r"\bBTreeMap\b|\bBTreeSet\b", code):
        c = [n for n in ("BTreeMap", "BTreeSet") if re.search(rf"\b{n}\b", code) and n not in have]
        if c: lines.append("use std::collections::{" + ", ".join(c) + "};")
    rp = [n for n in ("Row", "Projection") if re.search(rf"\b{n}\b", code) and n not in have and m != "number"]
    if rp: lines.append(f"use {SELF}::number::{{{', '.join(rp)}}};")
    sibs = [s for s in modmap if s != m and re.search(rf"\b{s}\s*::", code) and s not in have]
    if m == "admission": sibs = [s for s in sibs if s in ("number", "artifact_reference", "color", "font", "annotation", "form") and False]
    if sibs: lines.append(f"use {SELF}::{{{', '.join(sibs)}}};")
    vals = [n for n in ("ValueError", "ValueRefusalKind") if re.search(rf"\b{n}\b", code) and n not in have]
    if vals: lines.append(f"use semio_framework_value::{{{', '.join(vals)}}};")
    new = text.replace(glob + "\n", "\n".join(lines) + "\n", 1)
    new = re.sub(r"pub\(super\)", "pub", new)
    results[m] = (main, new)
    print(f"== {m}\n  " + "\n  ".join(lines))
tp = f"{B}/🧪️tests/🔬️unit/🦀️.rs"
ttext = open(tp).read()
tglob = f"use {SELF}::*;"
assert tglob + "\n" in ttext
tcode = strip(ttext)
thave = imported(ttext) | defined(ttext)
tl = [f"use {SCHEMA}::*;", "use pack::snapshot::ArtifactSqliteSnapshot as _;"]
th = [h for h in helper_names if re.search(rf"\b{h}\b", tcode) and h not in thave]
if th: tl.append(f"use {SELF}::reader::{{{', '.join(th)}}};")
tp_ = [piece for piece, name, rx in STORE if re.search(rx, tcode) and name not in thave]
if tp_: tl.append("use store::sqlite_snapshot::{" + ", ".join(tp_) + "};")
tr = [n for n in ("Row", "Projection") if re.search(rf"\b{n}\b", tcode) and n not in thave]
if tr: tl.append(f"use {SELF}::number::{{{', '.join(tr)}}};")
ts = [s for s in modmap if re.search(rf"\b{s}\s*::", tcode) and s not in thave]
if ts: tl.append(f"use {SELF}::{{{', '.join(ts)}}};")
tv = [n for n in ("ValueError", "ValueRefusalKind") if re.search(rf"\b{n}\b", tcode) and n not in thave]
if tv: tl.append(f"use semio_framework_value::{{{', '.join(tv)}}};")
tc = [n for n in ("BTreeMap", "BTreeSet") if re.search(rf"\b{n}\b", tcode) and n not in thave]
if tc: tl.append("use std::collections::{" + ", ".join(tc) + "};")
results["tests"] = (tp, ttext.replace(tglob + "\n", "\n".join(tl) + "\n", 1))
print("== tests\n  " + "\n  ".join(tl))
bad = 0
for key, (path, text) in results.items():
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=text, capture_output=True, text=True)
    if r.returncode != 0:
        bad += 1
        print("PARSE FAIL", key, r.stderr[:300])
print("parse failures:", bad)
if apply:
    assert bad == 0
    os.makedirs(ORIG, exist_ok=True)
    for key, (path, text) in results.items():
        o = f"{ORIG}/{key}.rs"
        if os.path.exists(path) and not os.path.exists(o): open(o, "w").write(open(path).read())
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w").write(text)
    print("applied")
