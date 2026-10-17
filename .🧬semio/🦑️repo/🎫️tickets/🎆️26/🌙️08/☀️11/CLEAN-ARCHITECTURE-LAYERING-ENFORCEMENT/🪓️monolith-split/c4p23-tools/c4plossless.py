"""🧮 c4p23 lossless proof. usage: c4plossless.py stage|live [key...]
Compares the stripped non-empty line multiset of the pre-edit blob with the union of the staged (or live) files of each target, and prints every differing line class."""
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
    rels = [spec["rel"]] + [f"{base}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    return rels


for key in keys:
    spec = PLANS[key]
    orig = lines_of(base_text(spec["rel"]))
    new = []
    for rel in files_of(key):
        p = (STAGE + f"/{key}/" + rel) if which == "stage" else (ROOT + "/" + rel)
        new.extend(lines_of(read(p)))
    gone, added = diff_multiset(orig, new)

    def cls(l):
        if l.startswith("#[path = ") or re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", l) or re.match(r"^mod \w+;$", l) or l.startswith("include!("):
            return "wiring"
        if l.startswith("//!") or (l.startswith("//") and not l.startswith("//#")) or l.startswith("///"):
            return "child docstring"
        if "pub(super)" in l:
            return "widened"
        return "other"

    ga, ad = collections.Counter(), collections.Counter()
    for l, n in gone.items():
        ga[cls(l)] += n
    for l, n in added.items():
        ad[cls(l)] += n
    print(f"[{key}] original={len(orig)} lines, files={len(files_of(key))}, new total={len(new)}")
    print(f"   removed (stripped non-empty multiset): {sum(gone.values())} {dict(ga)}")
    print(f"   added   (stripped non-empty multiset): {sum(added.values())} {dict(ad)}")
    for l, n in sorted(gone.items()):
        print(f"   - {n}x {l[:170]}")
    for l, n in sorted(added.items()):
        print(f"   + {n}x {l[:170]}")
    # every removed line must be the pre-edit twin of an added line: widened (`pub(super) `), re-based include (`"../`), or a declared root substitution
    def twin_of(l):
        for a in added:
            if "pub(super)" in a and a.replace("pub(super) ", "", 1) == l:
                return "widened"
            if re.sub(r'(\binclude(?:_str|_bytes)?!\(\s*")\.\./', r"\1", a) == l and a != l:
                return "rebased"
        for x, y in spec.get("subs", []):
            if x in l and l.replace(x, y) in added:
                return "descriptor"
        return None

    kinds = collections.Counter()
    for l, n in gone.items():
        k = twin_of(l)
        if k is None:
            ok = False
            print("   !! removed line without a twin:", l[:150])
        else:
            kinds[k] += n
    print("   removed lines by reason:", dict(kinds))
    tot_rm += sum(gone.values()); tot_add += sum(added.values())
print("LOSSLESS", "OK" if ok else "FAILED", f"removed={tot_rm} added={tot_add}")
