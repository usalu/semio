#!/usr/bin/env python3
# 🧿️ Inherent impls and foreign-trait impls that name a type of a carved crate from OUTSIDE that crate (E0116 / E0117 after the split).
import re, subprocess, os, sys
from lib import *
import wp1files, defs
from rs import blank
from crates_wp1 import ROOT, ORDER

carved_files = {}
carved_types = {}
for crate in ORDER:
    root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
    prod, test = wp1files.tree_split(root)
    for f in prod + test:
        carved_files[f] = crate
    for f in prod:
        for d in defs.scan(read(f)):
            if d.kind == "item" and d.ctx in ("struct", "enum", "union", "type", "trait") and d.vis == "pub":
                carved_types.setdefault(d.name, crate)

# lifted / hub-hosted files count as monolith
r = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", "*.rs"], cwd=PLUG, capture_output=True, text=True)
mono = [PLUG + "/" + p for p in r.stdout.split("\n") if p and not p.startswith(("🖥️host/", "📦️packages/", "🧷️companions/", "🏛️ownership/", "🏇️mounted-owner/", "🧠️interpreter/", "🖨️describe/", "🧬️guest-contract/"))]
mono = [f for f in mono if f not in carved_files and os.path.exists(f)]
mono_defs = set()
for f in mono:
    for d in defs.scan(read(f)):
        if d.kind == "item" and d.ctx in ("struct", "enum", "union", "type", "trait"):
            mono_defs.add(d.name)
ambiguous = sorted(set(carved_types) & mono_defs)
for n in ambiguous:
    carved_types.pop(n, None)
IMPL = re.compile(r"\bimpl\b(?P<gen><(?:[^<>]|<(?:[^<>]|<[^<>]*>)*>)*>)?\s*(?P<head>[^{;]*?)\{", re.S)
bad = []
for f in mono:
    t = read(f)
    clean = blank(t)
    for m in IMPL.finditer(clean):
        head = m.group("head").strip()
        if not head:
            continue
        gen = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", m.group("gen") or ""))
        if re.search(r"\bfor\b", head):
            trait, typ = re.split(r"\bfor\b", head, 1)
            tm = re.match(r"\s*(?:&\s*(?:mut\s+)?)?(?:dyn\s+)?(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)", typ)
            trm = re.match(r"\s*(?:!?\s*)(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)", trait)
            tname = tm.group(1) if tm else None
            trname = trm.group(1) if trm else None
            if tname in carved_types and tname not in gen and trname not in mono_defs:
                bad.append((f.replace(PLUG + "/", ""), clean.count("\n", 0, m.start()) + 1, f"trait impl {trname} for {tname} ({carved_types[tname]})"))
        else:
            tm = re.match(r"\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)", head)
            tname = tm.group(1) if tm else None
            if tname in carved_types and tname not in gen:
                bad.append((f.replace(PLUG + "/", ""), clean.count("\n", 0, m.start()) + 1, f"INHERENT impl {tname} ({carved_types[tname]})"))
print("monolith files scanned", len(mono), "carved pub types", len(carved_types), "same-named in both (skipped)", ambiguous)
for b in bad:
    print(*b)
print("violations", len(bad))
