#!/usr/bin/env python3
"""🧷 Law of derive-conformant mutation kinds: every `SEMANTICS.kind` of a mutation leaf equals the dsl::Mutations derive's to_kebab(variant), otherwise the derive's compile-time assert fails. Usage: python3 -I 🐍️.py"""
import re, glob, os, sys

def to_kebab(name):
    out = ""; ch = list(name)
    for i, c in enumerate(ch):
        if c in "_-":
            if out and not out.endswith("-"): out += "-"
            continue
        if c.isupper():
            p = ch[i-1] if i else None; n = ch[i+1] if i + 1 < len(ch) else None
            b = p is not None and (p.islower() or p.isdigit()) or (p is not None and p.isupper() and n is not None and n.islower())
            if b and not out.endswith("-"): out += "-"
            out += c.lower()
        else: out += c
    return out

import pathlib
base = str(pathlib.Path(__file__).resolve().parents[3] / "🗿️artifacts")
bad = []
checked = 0
for art in sorted(os.listdir(base)):
    roots = glob.glob(f"{base}/{art}/**/🧬️schema/🧬️mutations", recursive=True)
    if not roots: continue
    for f in glob.glob(f"{roots[0]}/**/🦀️.rs", recursive=True):
        t = open(f, encoding="utf-8").read()
        for m in re.finditer(r'for (\w+)[^{]*\{\s*const SEMANTICS[^}]*kind: "([^"]+)"', t):
            checked += 1
            if m.group(2) != to_kebab(m.group(1)):
                bad.append(f"{art}: {m.group(1)} kind {m.group(2)} must be {to_kebab(m.group(1))}")

for b in bad:
    print(b)
print(f"kind-kebab: checked {checked} leaves, {len(bad)} mismatches")
sys.exit(1 if bad or not checked else 0)
