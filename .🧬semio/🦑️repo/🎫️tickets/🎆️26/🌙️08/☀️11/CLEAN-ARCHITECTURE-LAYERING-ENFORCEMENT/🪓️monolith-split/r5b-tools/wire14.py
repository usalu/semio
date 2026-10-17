#!/usr/bin/env python3
"""Hoists the binary primitives and the hex codec of the 1.4 base io mutation aggregates into child `wire` modules the leaves import."""
import re, os, sys, glob, subprocess
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io"
M = "crate::standards::v1_4::subsets::base::io"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
files = {}
# binary
bp = f"{B}/💾️binary/🧬️mutations/🦀️.rs"
b = open(bp).read()
ra = b.index("//#region 🔖️Primitives"); rb = b.index("//#endregion 🔖️Primitives") + len("//#endregion 🔖️Primitives")
prim = b[ra:rb]
wire_b = "//! 🧰️ Index, text and cursor primitives of the PDF 1.4 binary mutation payloads.\n\n" + re.sub(r"pub\(super\) ", "pub ", prim) + "\n"
nb = b[:ra] + '#[path = "🧰️wire/🦀️.rs"]\npub mod wire;' + b[rb:]
files[bp] = nb
files[f"{B}/💾️binary/🧬️mutations/🧰️wire/🦀️.rs"] = wire_b
# text
tp = f"{B}/📝️text/🧬️mutations/🦀️.rs"
t = open(tp).read()
ta = t.index("pub(super) fn hex(")
tb = t.index("impl OpText for PdfMutation")
hexes = t[ta:tb]
wire_t = "//! 🧰️ Hexadecimal payload codec of the PDF 1.4 text mutation payloads.\n\n" + hexes.replace("pub(super) ", "pub ").rstrip() + "\n"
files[tp] = t[:ta] + t[tb:]
files[f"{B}/📝️text/🧬️mutations/🧰️wire/🦀️.rs"] = wire_t
t_new = files[tp].replace('//#region 🔖️Framing\n', '//#region 🔖️Framing\n', 1)
files[tp] = t_new.replace('#[path = "🔀️move-page/🦀️.rs"]\npub mod move_page;', '#[path = "🧰️wire/🦀️.rs"]\npub mod wire;\n\n#[path = "🔀️move-page/🦀️.rs"]\npub mod move_page;', 1)
# aggregates still use helpers? add imports when referenced
for path, kind in ((bp, "binary"), (tp, "text")):
    text = files[path]
    names = ["put_index", "put_text", "Reader"] if kind == "binary" else ["hex", "unhex"]
    used = [n for n in names if re.search(r"\b%s\b" % n, text.split("//#endregion")[-1] if False else text)]
    if used:
        files[path] = re.sub(r"(use protocol::Op(?:Binary|Text);\n)", r"\1use %s::%s::mutations::wire::{%s};\n" % (M, kind, ", ".join(used)), text, count=1)
# users
for kind, names in (("binary", ["put_index", "put_text", "Reader"]), ("text", ["hex", "unhex"])):
    sub = "💾️binary" if kind == "binary" else "📝️text"
    for f in glob.glob(f"{B}/{sub}/🧬️mutations/*/🦀️.rs"):
        s = open(f).read()
        pat = re.compile(r"use %s::%s::mutations::\{([^}]*)\};" % (re.escape(M), kind))
        def rep(m):
            items = [x.strip() for x in m.group(1).split(",") if x.strip()]
            w = [x for x in items if x in names]; o = [x for x in items if x not in names]
            out = []
            if o: out.append("use %s::%s::mutations::{%s};" % (M, kind, ", ".join(o)))
            if w: out.append("use %s::%s::mutations::wire::{%s};" % (M, kind, ", ".join(w)))
            return "\n".join(out)
        s2 = pat.sub(rep, s)
        if s2 != s: files[f] = s2
bad = 0
for f, txt in files.items():
    r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=txt, capture_output=True, text=True)
    if r.returncode: bad += 1; print("PARSE FAIL", f[-70:], r.stderr[:150])
print(len(files), "files", "parse failures", bad)
if apply:
    assert not bad
    od = os.environ["F"] + "/r5b-tools/orig/wire14"
    os.makedirs(od, exist_ok=True)
    for f, txt in files.items():
        if os.path.exists(f):
            o = od + "/" + f.split("1.4/")[-1].replace("/", "__")
            if not os.path.exists(o): open(o, "w").write(open(f).read())
        os.makedirs(os.path.dirname(f), exist_ok=True); open(f, "w").write(txt)
    print("applied")
