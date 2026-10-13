#!/usr/bin/env python3
"""🧽️ Normalises the import preamble of E1-moved store modules.

- `use crate::os_store::component::X` (rustc's suggestion spelling) becomes `use super::X`;
- imports of sibling-module traits that the preamble copy carried along (`use super::<module>::…` whose names are all
  traits) are dropped: rustc's unused-import lint keeps a trait import alive when it was merely a method-probe
  candidate, so the remaining genuinely needed ones are re-added by `🔁️fix-loop.py` from rustc's suggestions.
Usage: tidy-imports.py FILE [FILE ...]
"""
import re, subprocess, sys

STORE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"


def traits_named(names):
    out = set()
    for n in names:
        r = subprocess.run(["rg", "-l", "--glob", "*.rs", r"^\s*pub(\([^)]*\))?\s+trait\s+" + re.escape(n) + r"\b", STORE], capture_output=True, text=True)
        if r.stdout.strip(): out.add(n)
    return out


for f in sys.argv[1:]:
    src = open(f, encoding="utf-8").read()
    src = src.replace("use crate::os_store::component::", "use super::")
    out, dropped = [], 0
    for m in re.finditer(r"(?ms)^(use super::(\w+)::(\{[^}]*\}|\w+);\n)", src):
        pass
    def drop(m):
        global dropped
        body = m.group(3).strip("{}")
        names = [x.strip().split(" as ")[0] for x in body.split(",") if x.strip()]
        if names and all(n in traits_named(names) for n in names):
            dropped += 1
            return ""
        return m.group(1)
    src = re.sub(r"(?ms)^(use super::(\w+)::(\{[^}]*\}|\w+);\n)", drop, src)
    open(f, "w", encoding="utf-8").write(src)
    print(f"[tidy] {f.split('🏪️store/')[-1]}: dropped {dropped} sibling-trait imports")
