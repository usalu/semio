"""🧮 c4p29 lossless proof. usage: c4plossless.py stage|live [key...]
Compares the stripped non-empty line multiset of the pre-edit blob with the union of the staged (or live) files of each target as a SIGNED delta (new minus old) and subtracts the delta the plan itself causes:
wrapper heads and closing braces of depth-1 impl ranges, literal region marker lines, dropped impl head and close lines, rebased include literals. What remains may only be wiring, child docstrings, `use super::*;`
and `pub(super) ` widenings (a removed line paired with its widened twin). Anything else is printed and fails."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
keys = sys.argv[2:] or ORDER
ok = True
tot_rm = tot_add = 0


def files_of(key):
    spec = PLANS[key]
    base = os.path.dirname(spec["rel"])
    return [spec["rel"]] + [f"{base}/{c['folder']}/🦀️.rs" for c in spec["children"]]


def strip_counter(ls):
    return collections.Counter(norm_line(l) for l in ls if l.strip())


def expected(spec, orig):
    exp = collections.Counter()
    for c in spec["children"]:
        for r in c["ranges"]:
            if isinstance(r, list):
                for l in r:
                    exp[l.strip()] += 1
            elif len(r) > 2:
                exp[r[2].strip()] += 1
                exp["}"] += 1
        if c.get("tree"):
            attr, head, close = tree_wrap(orig, c["ranges"][0])
            exp[attr] -= 1
            exp[head] -= 1
            exp[close] -= 1
        for l in c.get("pre", []) + c.get("post", []):
            exp[l.strip()] += 1
        for old, new in c.get("replace", []):
            hits = [l for l in orig if old in l]
            assert len(hits) == 1
            exp[hits[0].replace(old, new).strip()] += 1
            exp[hits[0].strip()] -= 1
    for a, b in spec.get("drop", []):
        for l in orig[a - 1:b]:
            if l.strip():
                exp[l.strip()] -= 1
    return exp


def cls(l):
    if l.startswith("#[path = ") or re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", l) or re.match(r"^(pub(\([^)]*\))? )?mod \w+;$", l) or l.startswith("include!(") or l == "#[macro_use]":
        return "wiring"
    if l.startswith("//!") or (l.startswith("//") and not l.startswith("//#")) or l.startswith("///"):
        return "child docstring"
    return "other"


for key in keys:
    spec = PLANS[key]
    orig = lines_of(base_text(spec["rel"]))
    new = []
    for rel in files_of(key):
        p = (STAGE + "/" + rel) if which == "stage" else (ROOT + "/" + rel)
        new.extend(lines_of(read(p)))
    delta = collections.Counter()
    for l, n in strip_counter(new).items():
        delta[l] += n
    for l, n in strip_counter(orig).items():
        delta[l] -= n
    for l, n in expected(spec, orig).items():
        delta[l] -= n
    delta = {l: n for l, n in delta.items() if n != 0}
    plus = {l: n for l, n in delta.items() if n > 0}
    minus = {l: -n for l, n in delta.items() if n < 0}
    widened = collections.Counter()
    for pl in list(plus):
        if plus[pl] <= 0 or "pub(super) " not in pl:
            continue
        base = pl.replace("pub(super) ", "")
        if minus.get(base, 0) > 0:
            k = min(minus[base], plus[pl])
            widened["widened"] += k
            minus[base] -= k
            plus[pl] -= k
    rest_plus = collections.Counter()
    for l, n in plus.items():
        if n > 0:
            rest_plus[cls(l)] += n
    rest_minus = {l: n for l, n in minus.items() if n > 0}
    bad = {l: n for l, n in plus.items() if n > 0 and cls(l) == "other"}
    print(f"[{key}] original={len(orig)} lines, files={len(files_of(key))}, new total={len(new)}")
    print(f"   widened pairs: {widened['widened']}; remaining added classes: {dict(rest_plus)}; remaining removed: {sum(rest_minus.values())}")
    for l, n in sorted(rest_minus.items()):
        print(f"   !! removed {n}x {l[:170]}")
    for l, n in sorted(bad.items()):
        print(f"   !! added {n}x {l[:170]}")
    if rest_minus or bad:
        ok = False
    tot_rm += sum(rest_minus.values())
    tot_add += sum(plus.values())
print("LOSSLESS", "OK" if ok else "FAILED", f"unexplained-removed={tot_rm} added(wiring/doc/widening)={tot_add}")
