import sys,os,re,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from roots import *
from rs import *

NONROOT = {'sc', 'pub', 'dc', 'dgd'}
EXT = {}
DROP = {('ex', 'frame_id')}
FORCE_CFG = {('hs', 'OwnerKey'): '#[cfg(any(test, feature = "testkit"))]'}
FMT = re.compile(r'\{([A-Za-z_]\w*)(?::[^{}"]*)?\}')
EXCLUDE = {'dd': {'lifted_app'}, 'dc': {'execution_target_module'}}
GLOB = re.compile(r'^((?:[ \t]*#\[[^\n]*\]\n)*)([ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+(\w+)::\*;[ \t]*$', re.M)

def analyze(key):
    d = find(roots[key]); rootp = os.path.join(d, "🦀️.rs")
    t = tree(rootp)
    text = read(rootp)
    stext = strip(text)
    toks = {m: set(IDENT.findall(strip(x))) | set(FMT.findall(x)) for m, (p, x) in t.items()}
    rootitems = top_items(text)
    plans = []
    for g in GLOB.finditer(text):
        attrs, ind, vis, mod = g.group(1), g.group(2), (g.group(3) or "").strip(), g.group(4)
        if (mod,) not in t or mod in EXCLUDE.get(key, ()): continue
        items = [i for i in top_items(t[(mod,)][1]) if i["vis"] and i["name"] != "*"]
        plans.append(dict(span=(g.start(), g.end()), attrs=attrs, ind=ind, vis=vis, mod=mod, items=items))
    return d, rootp, t, text, toks, rootitems, plans

def gates_of(path, mod=(), chain=(), acc=None):
    acc = {} if acc is None else acc
    acc[mod] = chain
    text = read(path)
    for name, vis, f, attrs in mod_children(path, text):
        cf = tuple(a.strip() for a in attrs.split("\n") if a.strip().startswith("#[cfg("))
        if os.path.exists(f): gates_of(f, mod + (name,), chain + cf, acc)
    return acc

def uses_of(name, mod, toks, gates):
    gs = set()
    for m, ts in toks.items():
        if m[:1] == (mod,): continue
        if name in ts: gs.add(gates.get(m, ()))
    return gs

def build(key):
    d, rootp, t, text, toks, rootitems, plans = analyze(key)
    own = {}
    for i in rootitems:
        if i["name"] != "*": own.setdefault(i["name"], []).append(i)
    gates = gates_of(rootp)
    out = []; warn = []; seen = {}
    for p in plans:
        mod = p["mod"]; byn = {}
        for it in p["items"]:
            byn.setdefault(it["name"], []).append(it)
        pub_lines = {}; priv_lines = {}
        for n, its in byn.items():
            if (key, n) in DROP: continue
            if n in own:
                warn.append(f"SKIP {mod}::{n} shadowed by root item/import {[ (x['kind'],x['vis']) for x in own[n]]}")
                continue
            for it in its:
                cfg = tuple(it["cfg"])
                is_pub = p["vis"] == "pub" and it["vis"] == "pub"
                if is_pub and key in NONROOT:
                    EXT.setdefault(key, {}).setdefault((mod, cfg), set()).add(n)
                    gs0 = uses_of(n, mod, toks, gates)
                    if gs0:
                        c2 = tuple(dict.fromkeys(cfg + (next(iter(gs0)) if len(gs0) == 1 else ())))
                        priv_lines.setdefault(c2 if () not in gs0 else cfg, set()).add(n)
                elif is_pub:
                    pub_lines.setdefault(cfg, set()).add(n)
                elif p["vis"] in ("pub", "pub(crate)", "") or p["vis"].startswith("pub"):
                    gs = uses_of(n, mod, toks, gates)
                    if it["kind"] == "trait" and not gs: gs = {()}
                    if (key, n) in FORCE_CFG: gs = {(FORCE_CFG[(key, n)],)}
                    if not gs:
                        warn.append(f"drop-unused {mod}::{n} ({it['vis']} {it['kind']})")
                    elif () in gs or len(gs) > 1 and not all(g == next(iter(gs)) for g in gs):
                        if () not in gs: warn.append(f"MIXED-GATES {mod}::{n} {sorted(gs)}")
                        priv_lines.setdefault(cfg, set()).add(n)
                    else:
                        g = next(iter(gs))
                        c2 = tuple(dict.fromkeys(cfg + g))
                        priv_lines.setdefault(c2, set()).add(n)
            seen.setdefault(n, []).append(mod)
        lines = []
        pcfg = tuple(a.strip() for a in p["attrs"].strip().split("\n") if a.strip())
        def emit(prefix, groups):
            for cfg, names in sorted(groups.items()):
                if not names: continue
                lst = ", ".join(sorted(names))
                body = f"{prefix}use {mod}::{{{lst}}};" if len(names) > 1 else f"{prefix}use {mod}::{lst};"
                allc = list(dict.fromkeys(list(pcfg) + [c for c in cfg]))
                lines.append("".join(f"{p['ind']}{c}\n" for c in allc) + p["ind"] + body)
        emit("pub ", pub_lines); emit("", priv_lines)
        out.append((p, lines))
    dups = {n: m for n, m in seen.items() if len(set(m)) > 1}
    return d, rootp, text, out, warn, dups, t, toks

if __name__ == "__main__":
    for key in sys.argv[1:]:
        d, rootp, text, out, warn, dups, t, toks = build(key)
        print("=====", key, os.path.relpath(rootp, R)[-60:])
        for p, lines in out:
            print("  --", p["vis"] or "priv", p["mod"], len(p["items"]), "items")
            for l in lines: print("     ", l.replace("\n", "\n      ")[:400])
        for w in warn: print("  !", w)
        for n, m in dups.items(): print("  DUP", n, m)
