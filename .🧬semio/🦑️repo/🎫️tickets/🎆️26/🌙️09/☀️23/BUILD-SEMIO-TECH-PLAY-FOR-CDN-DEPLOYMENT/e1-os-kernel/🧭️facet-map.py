#!/usr/bin/env python3
"""🧭️ Facet dependency map of `semio-framework-os-kernel` from the exact HIR owner graph.

Joins `🔬️hir-owner-graph.py`'s owners JSON with the mounted module tree of `🕸️kernel-module-graph.py`.
Each production owner is placed in a node: its module, except inside the store root file, where the
node is the `//#region` the owner starts in. Nodes map to facets (`FACET_RULES`, `REGION_FACETS`).

Usage:
  facet-map.py OWNERS_JSON summary            facet lines, facet edges, facet SCCs, node SCCs
  facet-map.py OWNERS_JSON edge FROM TO       owner-level references behind one facet edge
  facet-map.py OWNERS_JSON nodes [FACET]      nodes with lines and facet
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kmg", os.path.join(HERE, "🕸️kernel-module-graph.py"))
kmg = importlib.util.module_from_spec(spec); spec.loader.exec_module(kmg)
REPO = kmg.REPO
STORE_ROOT = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
STORE_MOD = "os_store::component"

FACET_RULES = [
    ("io-base", r"🧰️framework/🔨️modules/🚪️io/"),
    ("store-base", r"🏪️store/🎮️one-item/🦀️.rs$"),
    ("canonical-edit", r"🏪️store/🧵️canonical-edit"),
    ("history", r"🏪️store/(🕰️history|📜️space-history|🔁️replay|🧾️document/📜️history)|🪐️space/🪶️sqlite"),
    ("retirement", r"🏪️store/(♻️retirement|🔗️read|📦️prepared)|🏪️store/.*/♻️retirement/"),
    ("store-composition", r"🏪️store/🧩️composition"),
    ("store-sync", r"🏪️store/(🔄️sync|👷️worker)"),
    ("store", r"🏪️store"),
    ("spr", r"📡️spr"),
    ("dsl", r"🗣️dsl"),
    ("pack", r"🎒️pack"),
    ("vcs", r"🌿️vcs"),
    ("directory", r"📇️directory|🔌️plugin/📇️registry"),
    ("io", r"💻️os/🔨️modules/🚪️io"),
    ("inference", r"💡️inference"),
    ("semio", r"🧬️semio"),
    ("extension", r"🧩️extension"),
    ("identity", r"🪪️identity"),
    ("list-delta", r"🪡️list-delta"),
    ("root", r"📦️packages/🦀️rust/🦀️.rs$"),
]

_ri = importlib.util.spec_from_file_location("ri", os.path.join(HERE, "🧱️rust_items.py"))
RI = importlib.util.module_from_spec(_ri); _ri.loader.exec_module(RI)

REGION_FACETS = {}
if os.path.exists(os.path.join(HERE, "🧭️region-facets.json")):
    REGION_FACETS = json.load(open(os.path.join(HERE, "🧭️region-facets.json")))


def regions_of(path):
    lines = open(path, encoding="utf-8").read().split("\n")
    stack, out = [], ["(head)"] * (len(lines) + 2)
    for i, l in enumerate(lines, 1):
        t = l.strip()
        if t.startswith("//#region"):
            stack.append(t[9:].strip())
        elif t.startswith("//#endregion"):
            name = t[12:].strip()
            if name in stack:
                while stack and stack.pop() != name: pass
        out[i] = stack[-1] if stack else "(top)"
    return out


def load():
    kmg.walk_file(kmg.ROOT, "crate", "prod", os.path.dirname(kmg.ROOT))
    modinfo = {}
    for m in kmg.mods:
        p = m.path.replace("crate::", "", 1) if m.path != "crate" else ""
        modinfo[p] = m
    return modinfo


def module_of(path, modinfo):
    segs = path.split("::")
    for k in range(len(segs), -1, -1):
        cand = "::".join(segs[:k])
        if cand in modinfo: return cand
    return ""


def facet_for_file(f, kind):
    if kind in ("test", "laws"): return "laws"
    if re.search(r"🧪️tests|🧪️testing|🔮️oracles", f): return "laws"
    for name, rx in FACET_RULES:
        if re.search(rx, f): return name
    return "other"


def build(owners_path):
    owners = json.load(open(owners_path))
    modinfo = load()
    regions = regions_of(STORE_ROOT)
    node_of, facet_of = {}, {}

    def placed(p, seen=()):
        if p in node_of: return node_of[p]
        segs = p.split("::")
        for k in range(len(segs), 0, -1):
            if segs[k - 1].startswith("{impl#"):
                imp = "::".join(segs[:k])
                own = owners.get(imp, {}).get("self")
                trait = owners.get(imp, {}).get("trait")
                if trait and (not trait.startswith("extern::") or trait.split("::")[-1] in ("From", "TryFrom", "PartialEq", "PartialOrd", "AsRef", "Extend", "FromIterator")):
                    break
                if own and not own.startswith("extern::") and not own.startswith(imp + "::") and own in owners and own not in seen:
                    return placed(own, seen + (p,))
                break
        o = owners.get(p, {"spans": {}, "xspans": {}})
        mod = module_of(p, modinfo)
        m = modinfo[mod]
        if mod == STORE_MOD:
            spans = [v for k, v in o["spans"].items() if k.endswith("🏪️store/🦀️.rs")] or [v for k, v in o.get("xspans", {}).items() if k.endswith("🏪️store/🦀️.rs")]
            start = min((s[0] for s in spans), default=0)
            reg = regions[min(start, len(regions) - 1)] if start else "(nospan)"
            node = f"store-root/{reg}"
            facet_of[node] = REGION_FACETS.get(reg, "store")
        else:
            node = mod
            facet_of[node] = facet_for_file(m.file, m.kind)
        return node

    for p in owners:
        node_of[p] = placed(p)
    return owners, modinfo, node_of, facet_of


def node_lines(modinfo, facet_of):
    """📏️ Non-blank lines per node; inside the store root every top-level item counts toward the node its owners land in
    (inherent and foreign-trait impls with their self type, everything else at the region of its head line)."""
    lines = defaultdict(int)
    regions = regions_of(STORE_ROOT)
    raw = open(STORE_ROOT, encoding="utf-8").read()
    cl = RI.clean(raw)
    its, _ = RI.items(raw, cl, 0, len(cl))
    head_of = {}
    for it in its:
        if it["name"] and it["kind"] in ("struct", "enum", "trait", "union", "type"): head_of.setdefault(it["name"], RI.line_of(raw, it["head"]))
    local_traits = {it["name"] for it in its if it["kind"] == "trait"}
    for it in its:
        if it["kind"] == "mod": continue
        at = RI.line_of(raw, it["head"])
        if it["kind"] == "impl" and it.get("self") in head_of and not (it.get("trait") in local_traits):
            at = head_of[it["self"]]
        n = len([x for x in raw[it["start"]:it["end"]].split("\n") if x.strip()])
        lines[f"store-root/{regions[at]}"] += n
    for p, m in modinfo.items():
        if p == STORE_MOD: continue
        txt = kmg.load(m.file)[1]
        spans = kmg.own_spans(m)
        n = sum(len([x for x in txt[a:b].split("\n") if x.strip()]) for a, b in spans)
        lines[p] += n
    return lines


def eades(nodes, w):
    """🪜️ Greedy feedback-arc-set ordering (Eades, Lin, Smyth 1993): sinks to the bottom, sources to the top."""
    out_w, in_w = defaultdict(int), defaultdict(int)
    succ, pred = defaultdict(set), defaultdict(set)
    for (a, b), c in w.items():
        succ[a].add(b); pred[b].add(a); out_w[a] += c; in_w[b] += c
    alive = set(nodes); s1, s2 = [], []
    while alive:
        changed = True
        while changed:
            changed = False
            for n in sorted(alive):
                if not (succ[n] & alive):
                    s2.append(n); alive.discard(n); changed = True
            for n in sorted(alive):
                if not (pred[n] & alive):
                    s1.append(n); alive.discard(n); changed = True
        if alive:
            def delta(n):
                return sum(w[(n, b)] for b in succ[n] if b in alive) - sum(w[(a, n)] for a in pred[n] if a in alive)
            n = max(sorted(alive), key=delta)
            s1.append(n); alive.discard(n)
    return list(reversed(s1 + list(reversed(s2))))


def scc(nodes, adj):
    return kmg.scc(nodes, adj)


def main():
    owners_path, cmd = sys.argv[1], sys.argv[2]
    owners, modinfo, node_of, facet_of = build(owners_path)
    lines = node_lines(modinfo, facet_of)
    for n in lines:
        if n not in facet_of:
            if n.startswith("store-root/"):
                facet_of[n] = REGION_FACETS.get(n[11:], "store")
            else:
                m = modinfo[n]; facet_of[n] = facet_for_file(m.file, m.kind)
    nedges = defaultdict(lambda: defaultdict(set))
    for p, o in owners.items():
        a = node_of[p]
        for t in o["refs"]:
            if t not in node_of:
                tm = module_of(t, modinfo)
                b = node_of.get("::".join(t.split("::")[:len(tm.split('::')) + 1])) if tm else None
                if b is None:
                    for k in range(len(t.split("::")), 0, -1):
                        b = node_of.get("::".join(t.split("::")[:k]))
                        if b: break
                if b is None: continue
            else:
                b = node_of[t]
            if a != b: nedges[a][b].add((p, t))
    prod = lambda n: facet_of.get(n) != "laws"
    if cmd == "summary":
        fl = defaultdict(int)
        for n, c in lines.items(): fl[facet_of[n]] += c
        print("## facet non-blank lines")
        for f_, c in sorted(fl.items(), key=lambda x: -x[1]): print(f"{f_:20} {c:7}")
        fe = defaultdict(lambda: defaultdict(int))
        for a, tos in nedges.items():
            for b, refs in tos.items():
                if facet_of[a] != facet_of[b]: fe[facet_of[a]][facet_of[b]] += len(refs)
        print("\n## facet -> facet (owner-level references, production only)")
        for a in sorted(fe):
            if a == "laws": continue
            row = {b: c for b, c in fe[a].items() if b != "laws"}
            if row: print(f"{a:20} -> " + ", ".join(f"{b}:{c}" for b, c in sorted(row.items(), key=lambda x: -x[1])))
        adj = defaultdict(set)
        for a, tos in nedges.items():
            for b in tos:
                if prod(a) and prod(b) and facet_of[a] != facet_of[b]: adj[facet_of[a]].add(facet_of[b])
        print("\n## facet SCCs")
        for c in scc(sorted(set(facet_of.values()) - {"laws"}), adj):
            if len(c) > 1: print("cycle:", ", ".join(sorted(c)))
        nadj = defaultdict(set)
        for a, tos in nedges.items():
            for b in tos:
                if prod(a) and prod(b): nadj[a].add(b)
        print("\n## node SCCs (production, size > 1)")
        for c in sorted(scc([n for n in lines if prod(n)], nadj), key=len, reverse=True):
            if len(c) > 1:
                fs = defaultdict(int)
                for n in c: fs[facet_of[n]] += lines[n]
                print(f"SCC {len(c)} nodes, {sum(lines[n] for n in c)} lines: " + ", ".join(f"{k}:{v}" for k, v in sorted(fs.items(), key=lambda x: -x[1])))
    elif cmd == "edge":
        A, B = sys.argv[3], sys.argv[4]
        rows = defaultdict(set)
        for a, tos in nedges.items():
            for b, refs in tos.items():
                if (facet_of[a] == A or a == A) and (facet_of[b] == B or b == B):
                    for p, t in refs: rows[(a, b)].add(t)
        for (a, b), ts in sorted(rows.items(), key=lambda x: -len(x[1])):
            print(f"{a} -> {b} ({len(ts)}): " + ", ".join(sorted(ts)[:25]))
    elif cmd == "nodes":
        flt = sys.argv[3] if len(sys.argv) > 3 else None
        for n in sorted(lines, key=lambda n: (facet_of[n], -lines[n])):
            if flt and facet_of[n] != flt: continue
            outs = sorted({facet_of[b] for b in nedges.get(n, {}) if facet_of[b] != facet_of[n] and prod(b)})
            print(f"{facet_of[n]:18} {lines[n]:6} {n}   -> {','.join(outs)}")
    elif cmd == "scc":
        nadj = defaultdict(set)
        for a, tos in nedges.items():
            for b in tos:
                if prod(a) and prod(b): nadj[a].add(b)
        comps = sorted(scc([n for n in lines if prod(n)], nadj), key=len, reverse=True)
        big = set(comps[0])
        print(f"## giant SCC: {len(big)} nodes, {sum(lines[n] for n in big)} lines")
        for n in sorted(big, key=lambda n: -lines[n]): print(f"  {facet_of[n]:18} {lines[n]:6} {n}")
        print("## store-layer nodes outside the giant SCC")
        for n in sorted(lines, key=lambda n: -lines[n]):
            if n not in big and prod(n) and facet_of[n] in ("store", "store-base", "canonical-edit", "history", "retirement", "store-composition", "io", "store-sync"):
                print(f"  {facet_of[n]:18} {lines[n]:6} {n}")
    elif cmd == "fas":
        nadj = defaultdict(set)
        for a, tos in nedges.items():
            for b in tos:
                if prod(a) and prod(b): nadj[a].add(b)
        big = set(sorted(scc([n for n in lines if prod(n)], nadj), key=len, reverse=True)[0])
        w = {(a, b): len({t for _, t in nedges[a][b]}) for a in big for b in nedges.get(a, {}) if b in big}
        order = eades(big, w)
        pos = {n: i for i, n in enumerate(order)}
        back = sorted(((a, b) for (a, b) in w if pos[a] < pos[b]), key=lambda e: -w[e])
        print(f"## layering (bottom first): {len(order)} nodes; back edges: {len(back)}, item refs: {sum(w[e] for e in back)}")
        for n in reversed(order): print(f"  {pos[n]:3} {facet_of[n]:18} {lines[n]:6} {n}")
        print("## back edges (lower node referencing a higher node) = seams to cut")
        for a, b in back:
            ts = sorted({t.split('::')[-1] for _, t in nedges[a][b]})
            print(f"  {a}  =>  {b}  ({w[(a, b)]}): {', '.join(ts[:12])}")
    elif cmd == "layers":
        k = int(sys.argv[3])
        layer_facets = {"store", "store-base", "canonical-edit", "history", "retirement", "store-composition", "io"}
        nodes = [n for n in lines if prod(n) and facet_of[n] in layer_facets]
        nset = set(nodes)
        nadj = defaultdict(set)
        w = {}
        for a, tos in nedges.items():
            if a not in nset: continue
            for b in tos:
                if b in nset and a != b:
                    nadj[a].add(b); w[(a, b)] = len({t for _, t in nedges[a][b]})
        comps = scc(nodes, nadj)
        comp_of = {n: i for i, c in enumerate(comps) for n in c}
        order = []
        for c in comps:
            order.extend(eades(set(c), {e: v for e, v in w.items() if e[0] in c and e[1] in c}) if len(c) > 1 else c)
        pos = {n: i for i, n in enumerate(order)}
        assert all(pos[a] >= pos[b] or comp_of[a] == comp_of[b] for (a, b) in w), "tarjan order is not reverse topological"
        total = sum(lines[n] for n in order)
        layer, acc, cut = {}, 0, []
        for n in order:
            layer[n] = min(k - 1, int(acc * k / total)); acc += lines[n]
        back = defaultdict(lambda: defaultdict(set))
        for (a, b), c in w.items():
            if layer[a] < layer[b]:
                for _, t in nedges[a][b]: back[(layer[a], layer[b])][(a, b)].add(t.split("::")[-1])
        for i in range(k):
            ns = [n for n in order if layer[n] == i]
            print(f"## layer {i}: {sum(lines[n] for n in ns)} lines, {len(ns)} nodes")
            print("   " + ", ".join(f"{n.replace('os_store::component::', '').replace('store-root/', '#')}({lines[n]})" for n in sorted(ns, key=lambda n: -lines[n])))
        if "--emit" in sys.argv:
            json.dump({str(i): sorted(n for n in order if layer[n] == i) for i in range(k)}, open(sys.argv[sys.argv.index("--emit") + 1], "w"), ensure_ascii=False, indent=1)
        print(f"## upward references across layers: {sum(len(v) for v in back.values())} node pairs, {sum(len(t) for v in back.values() for t in v.values())} items")
        for (la, lb), pairs in sorted(back.items()):
            for (a, b), ts in sorted(pairs.items(), key=lambda x: -len(x[1])):
                print(f"  L{la}->L{lb} {a.replace('os_store::component::', '')} => {b.replace('os_store::component::', '')}: {', '.join(sorted(ts)[:10])}")
    elif cmd == "json":
        json.dump({"lines": lines, "facet": facet_of, "edges": {a: {b: sorted(t for _, t in r) for b, r in tos.items()} for a, tos in nedges.items()}}, open(sys.argv[3], "w"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
