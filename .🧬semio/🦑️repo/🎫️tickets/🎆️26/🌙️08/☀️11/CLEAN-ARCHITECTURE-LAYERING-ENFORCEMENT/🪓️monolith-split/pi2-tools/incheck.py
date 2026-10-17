#!/usr/bin/env python3
"""🔎️ Every `include_str!`, `include_bytes!`, `include!` literal and every `CARGO_MANIFEST_DIR` read of the sources a companion test bin owns must name an existing file from where the bin builds it.
usage: incheck.py"""
import glob, os, re, sys
from lib import *
from rs import blank
import tree as T

INC = re.compile(r'\b(include_str|include_bytes|include)!\(\s*"([^"]+)"\s*\)')
MAN = re.compile(r'\b(include_str|include_bytes|include)!\(\s*concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]+)"\s*(?:,\s*"([^"]+)"\s*)*\)\s*\)')
bad, n = [], 0
for d in sorted(glob.glob(PLUG + "/🧷️companions/*tests-*")):
    root = d + "/🦀️.rs"
    if not os.path.exists(root):
        continue
    pkg = d + "/" + PKGDIR
    for f, _, _ in T.tree(root):
        text = read(f)
        clean = blank(text)
        for m in INC.finditer(text):
            if clean[m.start():m.start() + 6] != text[m.start():m.start() + 6]:
                continue
            n += 1
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(2)))
            if not os.path.exists(p):
                bad.append((rel(f)[-70:], m.group(2)[:90]))
        for m in re.finditer(r'\b(?:include_str|include_bytes|include)!\(\s*concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,', text):
            n += 1
            k, depth, lits = m.end(), 1, []
            while depth and k < len(text):
                c = text[k]
                if c == '"':
                    e = text.index('"', k + 1)
                    lits.append(text[k + 1:e]); k = e
                elif c == "(":
                    depth += 1
                elif c == ")":
                    depth -= 1
                k += 1
            p = os.path.normpath(pkg + "".join(lits))
            if not os.path.exists(p):
                bad.append((rel(f)[-70:], "MANIFEST+" + "".join(lits)[:100]))
print("checked", n, "missing", len(bad))
for b in bad[:30]:
    print(b)
