"""🧮 c4p11 lossless proof. usage: c4plossless.py stage|live [key...]
Stage 1: the baseline against the prepared text (the declared function extractions): nothing may be removed, only the head, the call and the closing brace are added.
Stage 2: the prepared text against the union of the staged (or live) files of each target: the stripped non-empty line multiset may lose only the lines that have a `pub(super)` twin, and gains only wiring, child docstrings and those twins."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
keys = sys.argv[2:] or ORDER
tot_rm = tot_add = 0
ok = True


def files_of(key):
    spec = PLANS[key]
    base = os.path.dirname(spec["rel"])
    return [spec["rel"]] + [f"{base}/{c['folder']}/🦀️.rs" for c in spec["children"]]


def cls(l):
    if l.startswith("#[path = ") or re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", l) or re.match(r"^mod \w+;$", l) or l.startswith("include!("):
        return "wiring"
    if l.startswith("//!") or (l.startswith("//") and not l.startswith("//#")) or l.startswith("///"):
        return "child docstring"
    if "pub(super)" in l:
        return "widened"
    return "other"


for key in keys:
    spec = PLANS[key]
    orig = lines_of(base_text(spec["rel"]))
    prep = prep_lines(spec["rel"], spec)
    e_gone, e_added = diff_multiset(orig, prep)
    if e_gone:
        ok = False
        print(f"[{key}] !! extraction removed lines: {dict(e_gone)}")
    if spec.get("extract"):
        print(f"[{key}] extraction: baseline {len(orig)} lines -> prepared {len(prep)}; removed {sum(e_gone.values())}, added {sum(e_added.values())}")
        for l, n in sorted(e_added.items()):
            print(f"   e+ {n}x {l[:200]}")
    new = []
    for rel in files_of(key):
        p = (STAGE + f"/{key}/" + rel) if which == "stage" else (ROOT + "/" + rel)
        new.extend(lines_of(read(p)))
    gone, added = diff_multiset(prep, new)
    ga, ad = collections.Counter(), collections.Counter()
    for l, n in gone.items():
        ga[cls(l)] += n
    for l, n in added.items():
        ad[cls(l)] += n
    print(f"[{key}] prepared={len(prep)} lines, files={len(files_of(key))}, new total={len(new)}")
    print(f"   removed (stripped non-empty multiset): {sum(gone.values())} {dict(ga)}")
    print(f"   added   (stripped non-empty multiset): {sum(added.values())} {dict(ad)}")
    for l, n in sorted(gone.items()):
        print(f"   - {n}x {l[:170]}")
    for l, n in sorted(added.items()):
        print(f"   + {n}x {l[:170]}")
    if spec["mode"] == "mod":
        for l, n in gone.items():
            twin = [a for a in added if "pub(super)" in a and a.replace("pub(super) ", "", 1) == l]
            if not twin:
                ok = False
                print("   !! removed line without a widened twin:", l[:150])
        if ad.get("other"):
            ok = False
            print("   !! added lines of class other")
    else:
        if gone:
            ok = False
            print("   !! fragment mode must remove nothing")
    tot_rm += sum(gone.values()); tot_add += sum(added.values())
print("LOSSLESS", "OK" if ok else "FAILED", f"removed={tot_rm} added={tot_add}")
