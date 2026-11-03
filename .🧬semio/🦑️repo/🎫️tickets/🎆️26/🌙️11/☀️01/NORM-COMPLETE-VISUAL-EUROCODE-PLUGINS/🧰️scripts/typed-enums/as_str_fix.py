#!/usr/bin/env python3
"""Typed enums v4: gives the model enums of a spec an `as_str` (the first legacy token of each variant) and `Display`, and rewrites the residual lines the converter listed (`&x.f` -> `x.f.as_str()`, string-armed `match x.f {` -> `match x.f.as_str() {`, `.is_empty()` of an `Unset` variant, `.to_uppercase()`). Idempotent.
usage: as_str_fix.py ART RESIDUAL_LOG [ENUM,ENUM…]"""
import json, re, sys
sys.path.insert(0, ".")
import convert as C
art, log = sys.argv[1], open(sys.argv[2], encoding="utf-8").read()
only = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
spec = json.load(open(f"specs/{art}.json", encoding="utf-8"))
root = C.artifact_root(art)
crate = f"semio_s_artifact_norm_{art}_model"
snap = root + "/🧬️schema/📸️snapshot/🦀️.rs"
text = open(snap, encoding="utf-8").read()
fields = []
add = ""
for c in spec["conversions"]:
    if only and c["enum"] not in only:
        continue
    for h in c["hosts"]:
        fields.append(h.split(".")[1])
    if re.search(r"impl %s \{\s*/// 🔤" % c["enum"], text) or f"impl std::fmt::Display for {c['enum']}" in text or f"impl {c['enum']} {{\n    pub fn as_str" in text or f"impl {c['enum']} {{\n    pub const fn as_str" in text:
        continue
    arms = "".join(f'            Self::{v[0]} => {json.dumps(v[1][0] if v[1] else v[0].lower())},\n' for v in c["variants"])
    add += f"""
impl {c['enum']} {{
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {{
        match self {{
{arms}        }}
    }}
}}

impl std::fmt::Display for {c['enum']} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        f.write_str(self.as_str())
    }}
}}
"""
if add:
    open(snap, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + add)
F = "|".join(sorted(set(fields), key=len, reverse=True))
n = 0
for m in re.finditer(r"^\s*residual: (.+?): (\d+): ", log, re.M):
    rel, ln = m.group(1), int(m.group(2))
    if rel.endswith(".py") or "mutations/" in rel or "/inverse/" in rel:
        continue
    p = root + "/" + rel
    lines = open(p, encoding="utf-8").read().split("\n")
    old = lines[ln - 1]
    new = re.sub(r"&(\w+(?:\.\w+)*)\.(%s)\b(?![\w(]|\.as_str)" % F, r"\1.\2.as_str()", old)
    new = re.sub(r"\b(\w+(?:\.\w+)*)\.(%s)\.to_(upper|lower)case\(\)" % F, r"\1.\2.as_str().to_\3case()", new)
    new = re.sub(r"\b(\w+(?:\.\w+)*)\.(%s)\.to_ascii_(upper|lower)case\(\)" % F, r"\1.\2.as_str().to_ascii_\3case()", new)
    mm = re.search(r"match (\w+(?:\.\w+)*)\.(%s) \{" % F, new)
    if mm and ln < len(lines) and lines[ln].lstrip().startswith('"'):
        new = new.replace(mm.group(0), f"match {mm.group(1)}.{mm.group(2)}.as_str() {{")
    if new != old:
        lines[ln - 1] = new
        open(p, "w", encoding="utf-8").write("\n".join(lines))
        n += 1
print(art, "enums with as_str:", add.count("pub const fn as_str"), "lines rewritten:", n)
