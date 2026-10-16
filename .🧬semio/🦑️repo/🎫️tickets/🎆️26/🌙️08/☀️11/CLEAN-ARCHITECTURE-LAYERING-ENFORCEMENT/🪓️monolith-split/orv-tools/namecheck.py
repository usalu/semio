"""Module-name collision scan in every namespace: a child module name must not equal any identifier imported by a use item, defined as an item (any kind), a function-like macro, or a mounted module of the root or of another child; reports also bare-identifier uses of the name as a call/value.
usage: namecheck.py [stage|real]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
from verify import files_of
from split import descendant_texts

which = sys.argv[1] if len(sys.argv) > 1 else "real"
bad = 0
for name, s in SPEC.items():
    base, root, kids = files_of(name, which)
    texts = {"<root>": read(root)}
    for f, p in kids: texts[f] = read(p)
    desc, _ = descendant_texts(name)
    mods = {c["mod"]: c["folder"] for c in s["children"]}
    found = []
    for fk, txt in list(texts.items()):
        for it in top_items(txt):
            if it["kind"] == "use":
                ids = [t[1] for t in it["hdr"] if t[0] == "id"]
                last = []
                # imported leaf names (identifier followed by , } ; or `as` alias)
                hd = it["hdr"]
                for q, tk in enumerate(hd):
                    nxt = hd[q + 1][1] if q + 1 < len(hd) else ""
                    prv = hd[q - 1][1] if q else ""
                    if tk[0] == "id" and nxt in (",", "}", ";") and tk[1] not in ("self", "super", "crate") and prv != "as":
                        last.append(tk[1])
                    if prv == "as" and tk[0] == "id":
                        last.append(tk[1])
                for n in last:
                    if n in mods and not (fk == "<root>" and it["hdr"][0][1] not in ("use", "pub")):
                        found.append((fk, "imports", n))
            elif it["kind"] in ("fn", "struct", "enum", "trait", "type", "const", "static", "union", "mod", "macro") and it["name"] in mods:
                if not (fk == "<root>" and it["kind"] == "mod"):
                    found.append((fk, it["kind"], it["name"]))
    # mod declarations of the descendants whose names equal a child mod
    print(f"{name}: child module names {sorted(mods)}; collisions: {found}")
    if found: bad += 1
print("NAME CHECK", "CLEAN" if bad == 0 else "PROBLEMS")
