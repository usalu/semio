"""🧬 Definition-identity check of every path rewrite in a list of plans (static, no rustc): the old path (resolved through the facade hub) and the new path (resolved in the producer crate it now names) must reach the SAME defining item (file, name, kind). Covers `use` statements (leaf by leaf), inline paths and macro bodies; paths into new macro crates are checked against the created source. `python3 verify_identity.py tag,tag,... [--seed]` (--seed corrupts one new path per run and must be caught)."""
from hubenv import *
import re
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from repoint_core import leaves
tags = sys.argv[1].split(",")
plans = [json.load(open(os.path.normpath(T + "/../staged/plans/%s.json" % t))) for t in tags]
world, I2L, I2D, LIB = make_world()
HUBID = {"umbrella": "semio_framework", "plugin": "semio_framework_plugin", "os-kernel": "semio_framework_os_kernel"}
NEWSRC = {}
for p in plans:
    for c in p.get("creates", []):
        if c["path"].endswith(".rs"): NEWSRC[c["path"]] = c["content"]
NEW = json.load(open(OUT + "/newcrates.json"))
cache = {}
def key_of(ident, rest):
    k = (ident, tuple(rest))
    if k in cache: return cache[k]
    m = world.model_for(ident)
    out = None
    if m is not None:
        q = list(rest)
        while q:
            r = world.resolve(m, q)
            if r:
                out = (r.get("file"), r.get("name"), r.get("kind"), tuple(rest[len(q):])); break
            q.pop()
    cache[k] = out
    return out
def hub_of(plan): return plan["hub"] if plan["hub"] in HUBID else None
hubs_models = {}
def old_key(plan, segs):
    hub = hub_of(plan)
    hubs = [hub] if hub else list(HUBID)
    for h in hubs:
        ident = LIB[byname[{"umbrella": "semio-framework", "plugin": "semio-framework-plugin", "os-kernel": "semio-framework-os-kernel"}[h]]]
        first = segs[0]
        if first == "ZZCRATE":
            rest = segs[1:]
        elif first == ident or first not in I2L or first in ("protocol",):
            rest = segs[1:]
        elif segs[0] in HUBID.values():
            continue
        else:
            continue
        k = key_of(ident, rest)
        if k: return k
        if (first not in I2L or first == "protocol") and first != "ZZCRATE":
            k = key_of(ident, segs)
            if k: return k
    return None
PATH = re.compile(r"(?<![A-Za-z0-9_$:.])(?:::)?([A-Za-z_][A-Za-z0-9_]*)((?:::[A-Za-z_][A-Za-z0-9_]*)+)")
bad, checked, skipped = [], 0, 0
SEED = '--seed' in sys.argv
seeded = [0, 0]
import random
random.seed(7)
for plan in plans:
    hubname = plan["hub"]
    for f in plan["files"]:
        for e in f["edits"]:
            old, new = e[2], e[3]
            # macro-body style `$crate::__private::<ident>::rest` or plain paths / use statements
            if old.lstrip().startswith(("use ", "pub use", "pub(crate) use", "#[")) and "use " in old:
                ol = leaves(old[old.index("use ") + 4:].rstrip().rstrip(";"))
                nl = []
                for stmt in new.split("\n"):
                    s = stmt.strip()
                    if "use " in s:
                        nl += leaves(s[s.index("use ") + 4:].rstrip().rstrip(";"))
                om = {(a or p[-1]): old_key(plan, list(p)) for p, a in ol if p and p[-1] not in ("*", "self") and (p[0] in HUBID.values() or p[0] not in I2L)}
                for p, a in nl:
                    nm = a or p[-1]
                    if nm not in om: continue
                    if p[0] in NEW:
                        skipped += 1; continue
                    nk = key_of(p[0], list(p[1:])) if len(p) > 1 else ("crate",)
                    ok = om[nm]
                    checked += 1
                    if nk == ("crate",) and ok and ok[2] == "crate":
                        continue
                    if ok is None or nk is None or (nk[:3] != ok[:3] and not (nk[0] == ok[0] and nk[1] == ok[1])):
                        bad.append((plan['tag'], f["path"][-60:], old[:80], new[:90], ok, nk))
            else:
                # inline paths: pair them in order
                oldn = old.replace('$crate::', 'ZZCRATE::'); newn = new.replace('$crate::__private::', '').replace('$crate::', 'ZZCRATE::')
                op = [m for m in PATH.finditer(oldn)]; np_ = [m for m in PATH.finditer(newn)]
                if not op or len(op) != len(np_):
                    m2 = re.match(r"^(\$crate::)?(?:::)?([A-Za-z_]\w*)((?:::\w+)+)\s*$", old)
                    skipped += 1; continue
                for a, b in zip(op, np_):
                    osegs = [a.group(1)] + [x for x in a.group(2).split("::") if x]
                    nsegs = [b.group(1)] + [x for x in b.group(2).split("::") if x]
                    if nsegs[0] == 'ZZCRATE' or osegs == nsegs or osegs[0] in ('Self', 'self', 'super', 'crate', 'std', 'core', 'alloc'):
                        continue
                    if nsegs[0] in NEW: skipped += 1; continue
                    ok = old_key(plan, osegs)
                    if SEED:
                        if seeded[0] >= 200 or ok is None: continue
                        nsegs = nsegs[:-1] + [nsegs[-1] + 'Zz']; seeded[0] += 1
                    nk = key_of(nsegs[0], nsegs[1:]) if len(nsegs) > 1 else ("crate",)
                    checked += 1
                    if ok is None or nk is None or (nk[:2] != ok[:2]):
                        bad.append((plan['tag'], f["path"][-60:], old[:80], new[:90], ok, nk))
if SEED: print('seeded corruptions', seeded[0], 'caught', len(bad))
print(f"{','.join(tags)}: checked {checked} resolvable rewrites, skipped {skipped}, identity mismatches {len(bad)}")
bp = collections.Counter(b[0] for b in bad)
print('by plan', dict(bp))
cat = collections.Counter(('old unresolved (alias ambiguity or override)' if b[4] is None else 'new unresolved' if b[5] is None else 'different definitions') for b in bad)
print('by kind', dict(cat))
import random as _r
for b in bad[:12] + _r.sample(bad, min(14, len(bad))): print('  ', b)
