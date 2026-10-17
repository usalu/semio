"""🧵 c4p22 order proof: splices every child back into its root at the mount (exactly as the Rust `read_world_tree` helper and `readModuleTreeSource` do) and compares the stripped non-empty line SEQUENCE with the pre-edit blob.
Removed from the spliced text first: the wiring (`#[path]`, `mod`, `use x::*;`, `include!`), each child's `//!`/`///` header and `use super::*;`, and the `pub(super) ` prefix of widened lines.
usage: c4pseq.py [key...]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

keys = sys.argv[1:] or ORDER
ok = True


def splice(path):
    d = os.path.dirname(path)
    ls = read(path).split("\n")
    out = []
    i = 0
    while i < len(ls):
        l = ls[i]
        m = re.match(r'^#\[path = "([^"]+)"\]$', l)
        inc = re.match(r'^include!\("([^"]+)"\);$', l)
        if m and i + 2 < len(ls) and re.match(r"^mod \w+;$", ls[i + 1]) and re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", ls[i + 2]):
            out.extend(splice(os.path.join(d, m.group(1)), ) if False else [("CHILD", os.path.join(d, m.group(1)))])
            i += 3
            continue
        if inc:
            out.append(("CHILD", os.path.join(d, inc.group(1))))
            i += 1
            continue
        out.append(l)
        i += 1
    return out


def flatten(path, child=False):
    res = []
    for item in splice(path):
        if isinstance(item, tuple):
            res.extend(flatten(item[1], True))
        else:
            res.append(item)
    if child:
        while res and res[0].strip() == "":
            res.pop(0)
        if res and (res[0].startswith("//!") or res[0].startswith("///") or res[0].startswith("// ")):
            res.pop(0)
        res = [l for l in res if l.strip() != "use super::*;"]
    return res


for key in keys:
    spec = PLANS[key]
    orig = [l.strip() for l in lines_of(base_text(spec["rel"])) if l.strip()]
    new = [l.strip() for l in flatten(ROOT + "/" + spec["rel"]) if l.strip()]
    new = [re.sub(r"\bpub\(super\) ", "", l, count=1) if "pub(super) " in l else l for l in new]
    # a widened original line `pub(super) x` had no pub in the original: compare after removing the prefix on both sides
    orig_n = [re.sub(r"\bpub\(super\) ", "", l, count=1) for l in orig]
    skip = lambda l: l == "#[macro_use]" or re.match(r"^//\s?#(end)?region\b", l)
    orig_n = [l for l in orig_n if not skip(l)]
    new = [l for l in new if not skip(l)]
    same = orig_n == new
    if not same:
        ok = False
        for k, (a, b) in enumerate(zip(orig_n, new)):
            if a != b:
                print(f"[{key}] first difference at stripped line {k}: orig={a[:120]!r} new={b[:120]!r}")
                break
        else:
            print(f"[{key}] length differs: orig={len(orig_n)} new={len(new)}")
    print(f"[{key}] sequence identical={same} lines={len(orig)}")
print("SEQUENCE", "OK" if ok else "DIFFERENT")
