#!/usr/bin/env python3
# 🧿️ Orphan-rule scan: `impl Trait for Type` headers of a carved crate in which neither the trait nor the type (nor a generic argument that makes it local, nor a local `dyn`) is defined by the crate itself.
# usage: orphan_scan.py [crate ...]   (default: all WP1 crates)
import re, sys
from lib import *
import wp1files, defs
from rs import blank, item_names
from crates_wp1 import ROOT, ORDER

IMPL = re.compile(r"\bimpl\b(?P<gen><(?:[^<>]|<(?:[^<>]|<[^<>]*>)*>)*>)?\s*(?P<head>[^{;]*?)\{", re.S)


def local_names(files):
    names = set()
    for f in files:
        t = blank(read(f))
        names |= set(re.findall(r"\b(?:struct|enum|trait|union|type)\s+([A-Za-z_][A-Za-z0-9_]*)", t))
        names |= set(re.findall(r"\bmacro_rules!\s+([A-Za-z_0-9]+)", t))
    return names


def scan(crate):
    root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
    files = [f for f, _ in wp1files.tree(root)]
    local = local_names(files)
    out = []
    for f in files:
        text = read(f)
        clean = blank(text)
        for m in IMPL.finditer(clean):
            head = m.group("head")
            if not re.search(r"\bfor\b", head):
                continue
            trait, typ = re.split(r"\bfor\b", head, 1)
            gen = m.group("gen") or ""
            gparams = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", gen))
            idents = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", trait + " " + typ)) - gparams - {"dyn", "for", "where", "mut", "const", "impl", "Send", "Sync", "static", "Self", "Box", "Vec", "Option", "Arc", "Rc", "str", "String", "std", "crate", "super", "self", "ref"}
            if not (idents & local):
                out.append((f.replace(PLUG + "/", ""), clean.count("\n", 0, m.start()) + 1, " ".join((trait + " for " + typ).split())[:110]))
    return out

if __name__ == "__main__":
    total = 0
    for c in (sys.argv[1:] or ORDER):
        rows = scan(c)
        total += len(rows)
        print(f"== {c}: {len(rows)} impl(s) with no local trait or type")
        for r in rows:
            print("   ", *r)
    print("total", total)
