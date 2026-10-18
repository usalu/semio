"""🧮 OHH family writer: hub core, hub compositions and teaching crates (from the manifest index) plus optional direct-dependency context crates, written to families/. Read only except for those files.
usage: mkfam.py write [--ctx-depth N]"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos")
from poslib import *
m = Manifests(Source(None))
m.build_index()
idx = m.by_name
GROUPS = collections.OrderedDict()
GROUPS["hub-core"] = sorted(n for n, p in idx.items() if p.startswith("🌎️hub/") and "🧩️compositions" not in p)
GROUPS["hub-comp"] = sorted(n for n, p in idx.items() if p.startswith("🌎️hub/🧩️compositions"))
GROUPS["teaching"] = sorted(n for n, p in idx.items() if p.startswith("🎓️teaching/"))


def ctx(crates, depth):
    own = set(crates)
    cur = set(crates)
    out = set()
    for _ in range(depth):
        nxt = set()
        for x in cur:
            man = m.manifest(x)
            if not man:
                continue
            for d in m.deps(man):
                if d[2] in idx and d[2] not in own and d[2] not in out:
                    nxt.add(d[2])
        out |= nxt
        cur = nxt
    return sorted(out)


import posunion as U
from posstub import mask
PUBUSE = re.compile(r"\bpub(?:\([^)]*\))?\s+use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)\b")
BY_IDENT = {n.replace("-", "_"): n for n in idx}


def reexport_closure(crates):
    """Crates reachable from the family's direct dependencies through `pub use <crate>` re-export edges (to a fixpoint), so facade paths resolve against real definitions."""
    own = set(crates)
    frontier = ctx(crates, 1)
    got = set(frontier)
    todo = list(frontier)
    while todo:
        batch, todo = todo[:60], todo[60:]
        for c in U.prepare(Source(None), m, batch, False):
            for rel in c.code:
                for h in PUBUSE.findall(mask(c.files[rel].decode("utf-8", "replace"))):
                    ex = c.externs.get(h) or c.aliases.get(h)
                    n = ex[1] if ex else BY_IDENT.get(h)
                    if n and n.startswith("semio-") and n not in own and n not in got:
                        got.add(n)
                        todo.append(n)
    return sorted(got)


if __name__ == "__main__":
    mode = sys.argv[sys.argv.index("--mode") + 1] if "--mode" in sys.argv else "depth"
    depth = int(sys.argv[sys.argv.index("--ctx-depth") + 1]) if "--ctx-depth" in sys.argv else 1
    os.makedirs(HERE + "/families", exist_ok=True)
    for k, v in GROUPS.items():
        json.dump(dict(name=k, crates=v, note="OHH group, own crates only"), open(f"{HERE}/families/{k}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        c = reexport_closure(v) if mode == "reexport" else ctx(v, depth)
        json.dump(dict(name=k + "-ctx", crates=sorted(set(v) | set(c)), own=v, note=f"OHH group plus direct-dependency context depth {depth}"), open(f"{HERE}/families/{k}-ctx.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(k, len(v), "ctx", len(c))
