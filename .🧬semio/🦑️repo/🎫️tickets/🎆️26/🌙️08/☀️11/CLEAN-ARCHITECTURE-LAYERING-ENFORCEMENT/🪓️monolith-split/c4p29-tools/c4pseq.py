"""🧵 c4p29 order proof: splices every child back into its root at the mount (exactly as the Rust `read_world_tree` helper and `readModuleTreeSource` do) and compares the stripped non-empty line SEQUENCE with the pre-edit blob.
Removed from the spliced text first: the wiring (`#[path]`, `mod`, `use x::*;`, `include!`), each child's `//!`/`///`/`//` header and `use super::*;`, the plan's wrapper heads and their closing braces, every region marker line (compared by name in c4pcheck.py), the `pub(super) ` prefix of widened lines.
Removed from the pre-edit blob: the plan's dropped lines and every region marker line. A `replace` rebase is applied to the pre-edit blob before comparing.
usage: c4pseq.py [live|stage] [key...]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

which = "live"
args = sys.argv[1:]
if args and args[0] in ("live", "stage"):
    which = args.pop(0)
keys = args or ORDER
ok = True
BASE = ROOT if which == "live" else STAGE
FOLDERS = {c["folder"] for s in PLANS.values() for c in s["children"]}
TREES = {}
for _k, _s in PLANS.items():
    _orig = lines_of(base_text(_s["rel"]))
    for _c in _s["children"]:
        if _c.get("tree"):
            TREES[_c["folder"]] = tree_wrap(_orig, _c["ranges"][0])
REGION = re.compile(r"^\s*//\s?#(end)?region\b")


def splice(path):
    d = os.path.dirname(path)
    ls = read(path).split("\n")
    out = []
    i = 0
    while i < len(ls):
        l = ls[i]
        m = re.match(r'^#\[path = "([^"]+)"\]$', l.strip())
        inc = re.match(r'^include!\("([^"]+)"\);$', l.strip())
        if m and i + 1 < len(ls) and re.match(r"^(pub(\([^)]*\))? )?mod \w+;$", ls[i + 1].strip()) and os.path.basename(os.path.dirname(m.group(1))) in FOLDERS:
            out.append(("CHILD", os.path.join(d, m.group(1))))
            i += 2
            if i < len(ls) and re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", ls[i].strip()):
                i += 1
            continue
        if inc and os.path.basename(os.path.dirname(inc.group(1))) in FOLDERS:
            out.append(("CHILD", os.path.join(d, inc.group(1))))
            i += 1
            continue
        out.append(l)
        i += 1
    return out


def unwrap(res, heads):
    out, i = [], 0
    while i < len(res):
        if res[i] in heads:
            j = i + 1
            while j < len(res) and res[j] != "}":
                j += 1
            out.extend(res[i + 1:j])
            i = j + 1
            continue
        out.append(res[i])
        i += 1
    return out


def flatten(path, heads, child=False):
    res = []
    for item in splice(path):
        if isinstance(item, tuple):
            sub = flatten(item[1], heads, True)
            folder = os.path.basename(os.path.dirname(item[1]))
            if folder in TREES:
                attr, head, close = TREES[folder]
                sub = [attr, head] + sub + [close]
            res.extend(sub)
        else:
            res.append(item)
    if child:
        while res and res[0].strip() == "":
            res.pop(0)
        if res and res[0].startswith("//"):
            res.pop(0)
        res = [l for l in res if l.strip() != "use super::*;"]
        res = unwrap(res, heads)
    return res


def norm(ls):
    return [re.sub(r"\bpub\(super\) ", "", norm_line(l)) for l in ls if l.strip() and not REGION.match(l)]


for key in keys:
    spec = PLANS[key]
    orig_raw = lines_of(base_text(spec["rel"]))
    dropped = set()
    for a, b in spec.get("drop", []):
        dropped.update(range(a - 1, b))
    for c in spec["children"]:
        for old, new in c.get("replace", []):
            orig_raw = [l.replace(old, new) if old in l else l for l in orig_raw]
    heads = {r[2] for c in spec["children"] for r in c["ranges"] if not isinstance(r, list) and len(r) > 2}
    orig = norm([l for i, l in enumerate(orig_raw) if i not in dropped])
    new = norm(flatten(BASE + "/" + spec["rel"], heads))
    if spec.get("container"):
        head = orig_raw[spec["container"][0] - 1].strip()
        at = [i for i in range(len(new) - 1) if new[i] == '#[path = "."]' and new[i + 1] == head]
        assert len(at) == 1, (key, "container path attribute", at)
        del new[at[0]]
    same = orig == new
    moved = []
    if not same:
        first = {}
        owned = {}
        for c in spec["children"]:
            real = [r for r in c["ranges"] if not isinstance(r, list)]
            first[real[0][0]] = c
            for r in real:
                for n in range(r[0], r[1] + 1):
                    owned[n] = c["folder"]
            if len(real) > 1:
                moved.extend(f"{c['folder'].split('/')[-1]}:{r[0]}-{r[1]}" for r in real[1:])
        for a, b in spec.get("drop", []):
            for n in range(a, b + 1):
                owned[n] = "DROP"
        rel = []
        for n, l in enumerate(orig_raw, 1):
            if n in first:
                c = first[n]
                for r in c["ranges"]:
                    if not isinstance(r, list):
                        rel.extend(orig_raw[r[0] - 1:r[1]])
            elif n not in owned:
                rel.append(l)
        rel = norm(rel)
        if rel == new:
            print(f"[{key}] sequence identical after relocating {len(moved)} non-contiguous block(s): {moved}")
            same = True
    if not same:
        ok = False
        for k, (a, b) in enumerate(zip(orig, new)):
            if a != b:
                print(f"[{key}] first difference at stripped line {k}: orig={a[:120]!r} new={b[:120]!r}")
                break
        else:
            print(f"[{key}] length differs: orig={len(orig)} new={len(new)}")
    print(f"[{key}] sequence identical={same} lines={len(orig)}")
print("SEQUENCE", "OK" if ok else "DIFFERENT")
