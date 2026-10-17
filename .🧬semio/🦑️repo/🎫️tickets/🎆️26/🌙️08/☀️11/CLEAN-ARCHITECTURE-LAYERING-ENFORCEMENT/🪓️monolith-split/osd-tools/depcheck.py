"""🔎 OSD: compares the crate heads each new crate's lib closure names (plus the implicit heads of derive macros and attributes) with its declared dependencies."""
import os, re, sys
from depscan import closure, heads, STD
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
CR = {
 "facade": ("📦️packages/🦀️rust", "🦀️.rs"),
 "schema": ("🧬️schema/📦️packages/🦀️rust", "🧬️schema/🦀️.rs"),
 "browser-actor": ("🧬️schema/🌐️browser-actor/📦️packages/🦀️rust", "🧬️schema/🌐️browser-actor/🦀️.rs"),
 "check-in": ("🧬️schema/📌️document-check-in-v1/📦️packages/🦀️rust", "🧬️schema/📌️document-check-in-v1/🦀️.rs"),
 "binary": ("🚪️io/🧱️binary/📦️packages/🦀️rust", "🚪️io/🧱️binary/🦀️.rs"),
 "access-policy": ("🛡️access-policy/📦️packages/🦀️rust", "🛡️access-policy/🦀️.rs"),
}
def declared(path):
    sect, out = None, set()
    for line in open(path, encoding="utf-8"):
        s = line.strip()
        if s.startswith("["):
            sect = s; continue
        if sect in ("[dependencies]", "[dev-dependencies]") and "=" in s and not s.startswith("#"):
            out.add(s.split("=")[0].strip().replace("-", "_"))
    return out
EXT = {"serde", "serde_json", "protocol"}
for key, (pkg, lib) in CR.items():
    dec = declared(f"{M}/{pkg}/Cargo.toml")
    files = closure(f"{M}/{lib}")
    h, local = heads(files)
    texts = "\n".join(open(f, encoding="utf-8").read() for f in files)
    named = {k for k in h if (k.startswith("semio_") or k in EXT) and k not in local}
    if re.search(r"derive\([^)]*\b(ToValue|FromValue)\b", texts):
        named |= {"semio_framework_value", "semio_framework_value_derive"}
    if re.search(r"#\[serde\(|Serialize|Deserialize", texts):
        named.add("serde")
    named.discard("semio_framework_os_directory") if key != "facade" else None
    print(f"== {key}: {len(files)} files; named-but-undeclared {sorted(named - dec)}; declared-but-unnamed {sorted(dec - named)}")
