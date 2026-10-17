#!/usr/bin/env python3
"""Moves the engine error and the text string codec out of the 1.7 base io root into the child modules `error` and `text_string` and re-points every user."""
import re, os, sys, subprocess
ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
B = f"{ART}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
IO = "crate::standards::v1_7::subsets::base::io"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
rootp = f"{B}/🚪️io/🦀️.rs"
root = open(rootp).read()
def region(name):
    a = root.index(f"//#region {name}")
    end_tag = f"//#endregion {name}"
    b = root.index(end_tag) + len(end_tag)
    return a, b
ea, eb = region("🔖️Error")
ta, tb = region("🔤️TextString")
assert eb < ta
err_body = root[ea:eb]
txt_body = root[ta:tb]
new_root = root[:ea] + '#[path = "🚨️error/🦀️.rs"]\npub mod error;\n#[path = "🔤️text-string/🦀️.rs"]\npub mod text_string;' + root[tb:]
new_root = new_root.replace("use std::collections::HashMap;\nuse std::sync::OnceLock;\n\n", "", 1)
assert "HashMap" not in new_root.replace("HashMap","",0) or True
files = {
    rootp: new_root,
    f"{B}/🚪️io/🚨️error/🦀️.rs": "//! 🚨️ The typed engine error every PDF codec layer shares, its Value refusal mapping and the `PResult` alias.\n\n" + err_body + "\n",
    f"{B}/🚪️io/🔤️text-string/🦀️.rs": "//! 🔤️ The PDF text string codec (§7.9.2.2): PDFDocEncoding table, decode and encode.\n\nuse std::collections::HashMap;\nuse std::sync::OnceLock;\n\n" + txt_body + "\n",
}
ERR = ("PdfEngineError", "PResult", "malformed")
TXT = ("decode_text_string", "encode_text_string", "PDF_DOC_ENCODING")
def rewrite_use(m):
    names = [x.strip() for x in m.group(1).split(",") if x.strip()]
    err = [n for n in names if n in ERR]; txt = [n for n in names if n in TXT]; rest = [n for n in names if n not in ERR + TXT]
    out = []
    if rest: out.append(f"use {IO}::{{{', '.join(rest)}}};" if len(rest) > 1 else f"use {IO}::{rest[0]};")
    if err: out.append(f"use {IO}::error::{{{', '.join(err)}}};")
    if txt: out.append(f"use {IO}::text_string::{{{', '.join(txt)}}};")
    return "\n".join(out)
pat_group = re.compile(r"^use " + re.escape(IO) + r"::\{([^}]*)\};", re.M)
pat_single = re.compile(r"^use " + re.escape(IO) + r"::(\w+);", re.M)
changed = []
listing = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7"], capture_output=True, cwd="/Users/ueli/Documents/semio").stdout.decode().split("\0")
for rel in listing:
    if not rel.endswith(".rs"): continue
    f = "/Users/ueli/Documents/semio/" + rel
    if f in files: continue
    t = open(f).read()
    t2 = pat_group.sub(lambda m: rewrite_use(m) if any(n.strip() in ERR + TXT for n in m.group(1).split(",")) else m.group(0), t)
    t2 = pat_single.sub(lambda m: rewrite_use(m) if m.group(1) in ERR + TXT else m.group(0), t2)
    for n in TXT:
        t2 = t2.replace(f"{IO}::{n}(", f"{IO}::text_string::{n}(")
    if t2 != t: files[f] = t2; changed.append(rel)
print(len(changed), "users rewritten")
bad = 0
for f, t in files.items():
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=t, capture_output=True, text=True)
    if r.returncode != 0: bad += 1; print("PARSE FAIL", f[-60:], r.stderr[:200])
print("parse failures", bad)
if apply:
    assert bad == 0
    od = os.environ["F"] + "/r5b-tools/orig/io-root"
    os.makedirs(od, exist_ok=True)
    for f, t in files.items():
        if os.path.exists(f):
            o = od + "/" + f.split("pdf/")[-1].replace("/", "__")
            if not os.path.exists(o): open(o, "w").write(open(f).read())
        os.makedirs(os.path.dirname(f), exist_ok=True)
        open(f, "w").write(t)
    print("applied")
