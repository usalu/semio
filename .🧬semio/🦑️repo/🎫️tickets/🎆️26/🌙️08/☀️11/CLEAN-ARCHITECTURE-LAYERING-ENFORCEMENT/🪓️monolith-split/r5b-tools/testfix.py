#!/usr/bin/env python3
"""Source fixes of the pdf carve that follow the testkit move: leaf tests reach the fixtures through `testkit` (also where they used the host's private `support` import), conformance support unit tests import the bare fixtures, and the native codec publication test moves to the root tests because it names the artifact declaration."""
import re, subprocess
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
R = "/Users/ueli/Documents/semio/"
BASE = "crate::standards::v1_7::subsets::base"
H = ["document_of", "document", "snapshots", "added", "removed", "content_edit", "catalog_object", "with_tail", "with_trailing_entry", "placed"]
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", ART + "/🏅️standards/7️⃣1.7"], capture_output=True, cwd=R).stdout.decode().split("\0")
pat = re.compile(r"\bsupport::(%s)\b" % "|".join(H))
USE = f"use {BASE}::schema::testkit;"
n = 0
for rel in files:
    if not rel.endswith(".rs") or "tests" not in rel: continue
    t = open(R + rel).read()
    if not pat.search(t): continue
    t2 = pat.sub(r"testkit::\1", t)
    if USE not in t2:
        lines = t2.split("\n"); i = 0
        while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")): i += 1
        lines.insert(i, USE); t2 = "\n".join(lines)
    open(R + rel, "w").write(t2); n += 1
print(n, "leaf test files")
cu = R + ART + "/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🏅️conformance-support/🧪️tests/🔬️unit/🦀️.rs"
t = open(cu).read()
used = [h for h in H if re.search(r"(?<![:\w])%s\(" % h, t)]
line = f"use {BASE}::schema::testkit::{{{', '.join(used)}}};"
if used and line not in t:
    lines = t.split("\n"); i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")): i += 1
    lines.insert(i, line); open(cu, "w").write("\n".join(lines))
print("bare", used)
T = R + ART + "/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs"
t = open(T).read()
key = "/// 📖️ Proves PDF 1.7 logical carriers, independent content oracles and exact protocol custody.\n#[test]\nfn owned_fixture_publication_reports_canonical_logical_carriers()"
if key in t:
    a = t.index(key); end = t.find("\n}\n", a); b = end + 3 if end >= 0 else len(t)
    fn = t[a:b]
    open(T, "w").write(t[:a].rstrip("\n") + "\n" + t[b:])
    fn = fn.replace("    logical_character_codes_bind_complete_native_fragments_and_preserve_word_precision();\n    retained_text_dates_admit_owned_values_and_keep_indirect_identity();\n", "")
    fn = fn.replace("let snapshot = text_document(", "let snapshot = crate::standards::v1_7::subsets::base::io::export::text_document(")
    fn = fn.replace('let compiled = runtime_fixture_bytes("../../../../../💾️binary/📸️snapshot/📡️.protocol.semio");', 'let protocol = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio");\n    let compiled: &[u8] = Box::leak(std::fs::read(&protocol).unwrap().into_boxed_slice());')
    fn = re.sub(r'let current = std::fs::read\(std::path::Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\("[^"]*"\)\)\.unwrap\(\);', 'let current = std::fs::read(&protocol).unwrap();', fn)
    RT = R + ART + "/🧪️tests/🔬️unit/🦀️.rs"
    r = open(RT).read()
    if "owned_fixture_publication_reports_canonical_logical_carriers" not in r:
        open(RT, "w").write(r.rstrip() + "\n\n" + fn.rstrip() + "\n")
    print("moved native codec publication test")
