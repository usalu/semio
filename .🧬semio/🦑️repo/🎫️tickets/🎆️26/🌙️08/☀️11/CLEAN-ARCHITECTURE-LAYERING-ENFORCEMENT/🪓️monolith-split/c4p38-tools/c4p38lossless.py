"""🧮 c4p38 independent lossless proof on the LIVE tree: stripped non-blank line multiset of each snapshot versus its live root plus live children, after reversing the two declared rewrites (`pub(super) ` widening prefix, one extra `../` on relative include paths).
Everything still missing is REMOVED; everything extra must be wiring (`#[path]`, `mod`, `use <child>::*`, `include!`, child docstrings, `use super::*;`). usage: c4p38lossless.py"""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p38lib import *
from plans38 import PLANS, ORDER

WIRE = re.compile(r'^(#\[path = ".*"\]|mod \w+;|(pub(\(crate\))? )?use \w+::\*;|use super::\*;|//! .*|include!\(".*"\);|// .*)$')
bad = False
tot_removed = tot_added = tot_wire = 0
for key in ORDER:
    spec = PLANS[key]
    base_dir = os.path.dirname(spec["rel"])
    files = [spec["rel"]] + [f"{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    new = collections.Counter()
    for rel in files:
        for l in lines_of(read(ROOT + "/" + rel)):
            s = l.strip()
            if not s:
                continue
            s = re.sub(r"\bpub\(super\) ", "", s)
            s = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', s)
            new[s] += 1
    old = multiset(lines_of(orig_text(spec["n"])))
    gone, extra = old - new, new - old
    wire = sum(n for l, n in extra.items() if WIRE.match(l))
    other = {l: n for l, n in extra.items() if not WIRE.match(l)}
    tot_removed += sum(gone.values())
    tot_added += sum(extra.values())
    tot_wire += wire
    print(f"[{key}] orig non-blank {sum(old.values())} live {sum(new.values())} removed {sum(gone.values())} added {sum(extra.values())} (wiring {wire}, other {sum(other.values())})")
    for l, n in list(gone.items())[:10]:
        print("    REMOVED", n, l[:120])
    for l, n in list(other.items())[:10]:
        print("    OTHER  ", n, l[:120])
    if gone or other:
        bad = True
print("LOSSLESS", "OK" if not bad else "FAILED", "removed", tot_removed, "added", tot_added, "wiring", tot_wire)
