#!/usr/bin/env python3
"""Moves the cfg(test) fixtures of the 1.7 base conformance support and the cfg(test) appliers of the io mutation bridge into the ungated sibling module `schema::testkit` and re-points every user."""
import re, os, sys, subprocess
ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
B = f"{ART}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
BASE = "crate::standards::v1_7::subsets::base"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
files = {}
cp = f"{B}/🧬️schema/🏅️conformance-support/🦀️.rs"
c = open(cp).read()
ra = c.index("//#region 🧪️Fixtures"); rb = c.index("//#endregion 🧪️Fixtures") + len("//#endregion 🧪️Fixtures")
fixtures = c[ra:rb]
new_c = c[:ra] + c[rb:].lstrip("\n")
new_c = new_c.replace("#[cfg(test)]\nuse crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;\n", "")
names = re.findall(r"^pub fn (\w+)", fixtures, re.M)
fx = fixtures.replace("#[cfg(test)]\n", "").replace("//#region 🧪️Fixtures\n", "").replace("//#endregion 🧪️Fixtures", "")
fx = fx.replace(f"{BASE}::io::mutation_bridge::after_rows", "after_rows")
fx = re.sub(r"^    use crate::standards::v1_7::subsets::base::(io::import::decode_pdf|io::export::encode_pdf|schema::snapshot::PdfPage);\n", "", fx, flags=re.M)
bridge = f"{B}/🚪️io/🦀️.rs"
b = open(bridge).read()
m = re.search(r"\n    /// 🧪️ `base` after the `rows` of one edit.*?\n    #\[cfg\(test\)\]\n    pub fn after_rows.*?\n    }\n\n    /// 🧪️ `base` after `mutation`.*?\n    #\[cfg\(test\)\]\n    pub fn applied.*?\n    }\n", b, re.S)
assert m, "bridge helpers"
helpers = m.group(0)
new_b = b.replace(helpers, "")
dedent = "\n".join(l[4:] if l.startswith("    ") else l for l in helpers.split("\n")).replace("#[cfg(test)]\n", "")
dedent = dedent.replace(f"{BASE}::schema::diff::graph_edit", "graph_edit")
head = f"""//! 🧪️ Fixtures and appliers of the PDF 1.7 conformance mutation tests: documents built from objects or read back from their own encoding, the
//! committed `lopdf` pairs, edit extractors, and the central applier. A regular module of the schema so a test of any crate can name it without
//! reaching into another crate's `cfg(test)` items.

use {BASE}::io::export::encode_pdf;
use {BASE}::io::import::decode_pdf;
use {BASE}::schema::conformance_support::*;
use {BASE}::schema::diff::{{graph_edit, PdfDiff}};
use {BASE}::schema::snapshot::{{ObjRef, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot}};

"""
testkit = head + fx.strip("\n") + "\n" + dedent.rstrip() + "\n"
testkit = testkit.replace("encode_pdf(&typed)", "encode_pdf(&typed)")
files[cp] = new_c
files[bridge] = new_b
files[f"{B}/🧬️schema/🧪️testkit/🦀️.rs"] = testkit
schema_root = f"{B}/🧬️schema/🦀️.rs"
sr = open(schema_root).read()
mount = "#[path = \"🏅️conformance-support/🦀️.rs\"]\npub mod conformance_support;"
assert mount in sr, "mount"
files[schema_root] = sr.replace(mount, mount + "\n#[path = \"🧪️testkit/🦀️.rs\"]\npub mod testkit;", 1)
HELP = set(names) | {"applied", "after_rows"}
listing = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"], capture_output=True, cwd="/Users/ueli/Documents/semio").stdout.decode().split("\0")
SUPPORT_USE = f"use {BASE}::schema::conformance_support as support;"
TESTKIT_USE = f"use {BASE}::schema::testkit;"
touched = []
for rel in listing:
    if not rel.endswith(".rs"): continue
    f = "/Users/ueli/Documents/semio/" + rel
    if f in files: continue
    t = open(f).read()
    o = t
    t = re.sub(r"\b%s::io::mutation_bridge::(applied|after_rows)\b" % re.escape(BASE), lambda m: f"{BASE}::schema::testkit::{m.group(1)}", t)
    t = re.sub(r"^(\s*)use %s::schema::testkit::(applied|after_rows);" % re.escape(BASE), lambda m: f"{m.group(1)}use {BASE}::schema::testkit::{m.group(2)};", t, flags=re.M)
    if re.search(r"\bsupport::(%s)\b" % "|".join(sorted(names)), t) and SUPPORT_USE in t:
        t = re.sub(r"\bsupport::(%s)\b" % "|".join(sorted(names)), r"testkit::\1", t)
        if TESTKIT_USE not in t:
            t = t.replace(SUPPORT_USE, SUPPORT_USE + "\n" + TESTKIT_USE, 1)
    if t != o: files[f] = t; touched.append(rel)
print(len(touched), "users rewritten; helpers", sorted(HELP))
bad = 0
for f, txt in files.items():
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=txt, capture_output=True, text=True)
    if r.returncode: bad += 1; print("PARSE FAIL", f[-70:], r.stderr[:200])
print("parse failures", bad)
if apply:
    assert not bad
    od = os.environ["F"] + "/r5b-tools/orig/testkit"
    os.makedirs(od, exist_ok=True)
    for f, txt in files.items():
        if os.path.exists(f):
            o = od + "/" + f.split("pdf/")[-1].replace("/", "__")[-180:]
            if not os.path.exists(o): open(o, "w").write(open(f).read())
        os.makedirs(os.path.dirname(f), exist_ok=True); open(f, "w").write(txt)
    print("applied")
