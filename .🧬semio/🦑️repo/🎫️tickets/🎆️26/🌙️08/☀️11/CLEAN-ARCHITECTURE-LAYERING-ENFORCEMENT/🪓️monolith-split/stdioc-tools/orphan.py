#!/usr/bin/env python3
# 🛡️ Orphan scan: every `impl` header whose implemented type or trait is not defined in its own producer crate (foreign-trait-on-foreign-type is the E0117/E0116 risk).
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
DEF = re.compile(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|type|union)\s+([A-Za-z_][A-Za-z0-9_]*)")
IMPL = re.compile(r"(?m)^\s*(?:unsafe\s+)?impl(?:<[^{]*?>)?\s+(?:(?P<trait>[A-Za-z_:][A-Za-z0-9_:<>, ']*?)\s+for\s+)?(?P<ty>[&A-Za-z_][A-Za-z0-9_:<>, '\[\]()&]*?)\s*(?:where\b[^{]*)?\{")
problems = 0
for crate in CRATES:
    files = closure(crate["root"])
    text = "\n".join(strip_comments(read(p)) for p in files)
    local = set(DEF.findall(text))
    for match in IMPL.finditer(text):
        trait = (match.group("trait") or "").strip()
        ty = match.group("ty").strip()
        tyname = re.match(r"&?(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)", ty)
        tyname = tyname.group(1) if tyname else ty
        traitname = re.match(r"([A-Za-z_][A-Za-z0-9_:]*)", trait).group(1).split("::")[-1] if trait else ""
        if tyname in local or (traitname and traitname in local) or tyname in {"T", "S", "E", "P", "M", "C"} and traitname in local:
            continue
        problems += 1
        print(crate["key"], "|", (trait + " for " if trait else "") + ty)
print("flagged", problems)
