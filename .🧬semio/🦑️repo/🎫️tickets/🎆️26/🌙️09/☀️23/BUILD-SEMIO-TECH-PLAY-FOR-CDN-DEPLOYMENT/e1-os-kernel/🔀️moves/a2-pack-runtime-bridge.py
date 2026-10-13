#!/usr/bin/env python3
"""🌉️ E1 move A2: the schema-driven pack runtime bridge (`pack_rt`) out of the store root into the os pack facade
(`🎒️pack/🌉️bridge/🦀️.rs`, mounted as `os_pack::pack_rt`). It only forwards to os pack and the replication byte sink,
so it belongs below spr and dsl; this removes the dsl -> store reference. Rewrites every `os_store::pack_rt` path in
the repository to `os_pack::pack_rt` (no re-export is left behind). Anchored on exact text; refuses to run twice."""
import os, re, subprocess, sys
REPO = "/Users/ueli/Documents/semio"
M = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules"
store_p, pack_p, new_p = f"{M}/🏪️store/🦀️.rs", f"{M}/🎒️pack/🦀️.rs", f"{M}/🎒️pack/🌉️bridge/🦀️.rs"
if os.path.exists(new_p):
    sys.exit("pack bridge already exists")
store, pack = (open(p, encoding="utf-8").read() for p in (store_p, pack_p))

doc_start = store.index("/// 🧵️ Thin runtime bridge to `crate::os_pack::{encode_document, decode_document}`")
head = store.index("pub mod pack_rt {\n", doc_start)
depth, i = 0, head
while True:
    c = store[i]
    if c == "{": depth += 1
    elif c == "}":
        depth -= 1
        if depth == 0: break
    i += 1
end = i + 2
docs = store[doc_start:head]
body = store[head + len("pub mod pack_rt {\n"):i]
inner_docs = "".join("//!" + l[3:] + "\n" for l in docs.rstrip("\n").split("\n"))
inner_docs = inner_docs.replace("resolved as\n//! `::crate::os_store::pack_rt::...`", "resolved as\n//! `::semio_framework_os_kernel::os_pack::pack_rt::...`")
lines = [l[4:] if l.startswith("    ") else l for l in body.rstrip("\n").split("\n")]
text = "\n".join(lines) + "\n"
old_use = "use super::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal};"
assert text.count(old_use) == 1
text = text.replace(old_use, "use super::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, PackRefusal};")
assert text.count("crate::os_spr::operation_bytes::OperationByteOutput") == 1
text = text.replace("crate::os_spr::operation_bytes::OperationByteOutput", "protocol::io::binary::operation_bytes::OperationByteOutput")
assert "os_store" not in text and "os_spr" not in text, [l for l in text.split("\n") if "os_store" in l or "os_spr" in l]
os.makedirs(os.path.dirname(new_p), exist_ok=True)
open(new_p, "w", encoding="utf-8").write(inner_docs + "\n" + text)

store = store[:doc_start] + "use crate::os_pack::pack_rt;\n" + store[end:]
store = store.replace("`dsl_derive`'s generated impl calls through `::crate::os_store::pack_rt`", "`dsl_derive`'s generated impl calls through `::semio_framework_os_kernel::os_pack::pack_rt`")
open(store_p, "w", encoding="utf-8").write(store)

anchor = "pub use scalar_witness::{ScalarRecordField, ScalarRecordView, ScalarRecordWireStep, ScalarRecordWireWitness};\n"
assert pack.count(anchor) == 1
pack = pack.replace(anchor, anchor + '\n#[path = "🌉️bridge/🦀️.rs"]\npub mod pack_rt;\n')
open(pack_p, "w", encoding="utf-8").write(pack)

files = subprocess.run(["rg", "-l", r"os_store::pack_rt\b|os_store::\{[^}]*\bpack_rt\b", "--glob", "*.rs", "--glob", "!.🧬semio/**", REPO], capture_output=True, text=True).stdout.split()
for f in files:
    s = open(f, encoding="utf-8").read()
    t = re.sub(r"\bos_store::pack_rt\b", "os_pack::pack_rt", s)
    def strip(m):
        items = [x.strip() for x in m.group(2).split(",") if x.strip() and x.strip() != "pack_rt"]
        return f"{m.group(1)}{{{', '.join(items)}}};\nuse crate::os_pack::pack_rt;"
    t = re.sub(r"(use crate::os_store::)\{([^}]*\bpack_rt\b[^}]*)\};", strip, t)
    if t != s:
        open(f, "w", encoding="utf-8").write(t)
        print("rewrote", os.path.relpath(f, REPO))
print("A2 applied")
