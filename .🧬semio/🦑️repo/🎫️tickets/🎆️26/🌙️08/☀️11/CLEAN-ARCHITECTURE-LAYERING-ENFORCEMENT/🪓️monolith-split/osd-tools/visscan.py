"""🔭 OSD: lists every pub(crate)/pub(super)/pub(in ...) item of the new crates and reports uses of its name in a different crate of the carve (the union oracle is blind to crate-restricted visibility)."""
import os, re, sys
from depscan import closure
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
CRATES = {
 "facade": M + "/🦀️.rs",
 "schema": M + "/🧬️schema/🦀️.rs",
 "browser-actor": M + "/🧬️schema/🌐️browser-actor/🦀️.rs",
 "check-in": M + "/🧬️schema/📌️document-check-in-v1/🦀️.rs",
 "binary": M + "/🚪️io/🧱️binary/🦀️.rs",
 "access-policy": M + "/🛡️access-policy/🦀️.rs",
}
texts = {k: {f: open(f, encoding="utf-8").read() for f in closure(v)} for k, v in CRATES.items()}
DEF = re.compile(r"\bpub\((?:crate|super|in [^)]*)\)\s+(?:async\s+|const\s+|unsafe\s+)*(?:fn|struct|enum|const|static|type|trait|mod|use)\s+(\w+)")
FIELD = re.compile(r"\bpub\((?:crate|super|in [^)]*)\)\s+(\w+)\s*:")
for k, files in texts.items():
    for f, t in files.items():
        for m in list(DEF.finditer(t)) + list(FIELD.finditer(t)):
            name = m.group(1)
            users = [o for o, of in texts.items() if o != k and any(re.search(r"\b" + re.escape(name) + r"\b", ot) for ot in of.values())]
            print(f"{k}: {os.path.relpath(f, M)}: {m.group(0)[:60]} -> used in other crates: {users}")
