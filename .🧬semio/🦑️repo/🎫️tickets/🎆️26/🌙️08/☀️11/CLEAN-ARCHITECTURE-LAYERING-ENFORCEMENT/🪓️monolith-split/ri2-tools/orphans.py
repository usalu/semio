#!/usr/bin/env python3
# 🧱️ Orphan and inherent-impl scan: every column-0 `impl` of a crate whose self type (or, for trait impls, trait and self type) is defined only in another crate of the partition.
import sys, os, re, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank

DEF = re.compile(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|union|trait|type)\s+([A-Za-z_]\w*)")
IMPL = re.compile(r"(?m)^[ \t]*(?:unsafe\s+)?impl\b(?:\s*<(?:[^<>]|<[^<>]*>)*>)?\s+(.*?)\s*(?:where\b|\{)")

table = crate_table()
real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
defs = collections.defaultdict(set)
files_of = {}
for c in real:
    files = modtree.tree(T + "/" + table[c] + "/🦀️.rs")
    files_of[c] = files
    for f in files:
        for m in DEF.finditer(blank(read(f))):
            defs[m.group(1)].add(c)
bad = []
for c in real:
    for f in files_of[c]:
        text = blank(read(f))
        for m in IMPL.finditer(text):
            head = m.group(1)
            if " for " in head:
                trait, selfty = head.split(" for ", 1)
            else:
                trait, selfty = None, head
            selfname = re.sub(r"^[&\s]*(?:mut\s+)?(?:dyn\s+)?", "", selfty.strip())
            selfname = re.split(r"[<\s(\[]", selfname)[0].split("::")[-1]
            owners = defs.get(selfname, set())
            tname = re.split(r"[<\s]", trait.strip())[0].split("::")[-1] if trait else None
            towners = defs.get(tname, set()) if tname else set()
            if not owners or c in owners:
                continue
            if trait is None:
                bad.append((c, f.replace(E + "/", "E/"), "inherent", head[:80], sorted(owners)))
            elif c not in towners:
                bad.append((c, f.replace(E + "/", "E/"), "trait", head[:80], sorted(owners)))
print("suspect impls:", len(bad))
for b in bad:
    print("  ", b)
