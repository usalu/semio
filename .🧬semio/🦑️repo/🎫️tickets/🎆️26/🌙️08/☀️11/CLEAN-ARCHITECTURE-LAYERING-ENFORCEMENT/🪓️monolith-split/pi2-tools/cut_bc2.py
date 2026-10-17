#!/usr/bin/env python3
"""✂️ Cuts the builder-contract test module (and the completion / composition fragments it includes) into pieces; searches the piece budget so that every piece's exact closure stays inside the source boundary.
usage: cut_bc2.py [--apply] [--search] [--budget N] [--merge N]"""
import sys, collections, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tsplit
from tsplit import *
import ed
LIMIT_BYTES, LIMIT_LINES, LIMIT_FILES = 250000, 3700, 30
MAIN = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
COMP = PLUG + "/🧪️tests/⏳️completion/🦀️.rs"
COMPS = PLUG + "/🧪️tests/🧩️composition/🦀️.rs"
RUNTIME = PLUG + "/⚛️reactor/🏃️runtime/🦀️.rs"
INCLUDE_LINE = '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");\n'


def text_item(M, path, text, has_tests):
    h = make_include_item(M, 0, path)
    icl = blank(text)
    h.ids = set(IDENT.findall(icl)); h.defs = set()
    for s2 in P.statements(text, icl, 0, len(text)):
        if s2.name and s2.kind not in ("use", "impl"):
            h.defs.add(s2.name)
        if s2.kind == "other":
            h.defs |= set(DEF_IN_MACRO.findall(icl[s2.start:s2.end]))
    h.has_tests = has_tests
    h.bytes = len(text.encode()); h.lines = text.count("\n") + 1
    h.emb = embeds_from_text(text, path)
    return h


def prepare(comp_budget):
    M = Module.load(MAIN, "plugin_builder_contract_tests")
    M.wrapper = None
    assert M.pre.strip() == "use crate::builder::PluginBuilder;", repr(M.pre)
    base = {}
    Mc = Module.load(COMP)
    ct = [i for i in Mc.items if i.is_test]
    assert len(ct) == 1
    comp_core = strip_unbalanced_regions("".join(i.text for i in Mc.items if i is not ct[0]) + Mc.tail)
    COMP_TEST = sibling(COMP, "-transient-close")
    base[COMP] = comp_core
    base[COMP_TEST] = strip_unbalanced_regions("// ⏳️ The transient-close law cut out of the completion fixtures: the module that includes both shares them.\n" + ct[0].text)
    Mp = Module.load(COMPS)
    ptests = [i.idx for i in Mp.items if i.has_tests]
    cuse = collections.Counter()
    for t_ in ptests:
        for x in Mp.closure([t_]):
            if not Mp.items[x].has_tests:
                cuse[x] += 1
    cuniv = {x for x, c in cuse.items() if c >= 0.5 * len(ptests)}
    for x in list(cuniv):
        cuniv |= Mp.closure([x])
    cuniv = {x for x in cuniv if not Mp.items[x].has_tests}
    pieces = cluster(Mp, ptests, cuniv, comp_budget, max_files=18)
    punits = {f"C{i+1}": p["tests"] for i, p in enumerate(pieces)}
    pdirs = {u: sibling(COMPS, f"-part-{i+1}") for i, u in enumerate(punits)}
    pdocs = {u: f"Part {i+1} of {len(punits)} of the composed-document laws (cut from `🧩️composition`)." for i, u in enumerate(punits)}
    cfiles, ccore, cshared, cprivate, cfrag = emit_fragments(Mp, punits, pdirs, "🧩️", pdocs)
    base.update(cfiles)
    new_items, idx = [], [0]
    def add(it):
        it.idx = idx[0]; idx[0] += 1; new_items.append(it)
    for it in M.items:
        if it.src == rel(COMP):
            h = text_item(M, COMP, comp_core, False); h.text = it.text; h.gap = it.gap
            add(h); add(text_item(M, COMP_TEST, base[COMP_TEST], True)); continue
        if it.src == rel(COMPS):
            for p in [COMPS] + [cfrag[s] for s in sorted(cfrag, key=lambda s: min(cshared[s]))]:
                add(text_item(M, p, cfiles[p], False))
            for u in punits:
                add(text_item(M, pdirs[u], cfiles[pdirs[u]], True))
            continue
        add(it)
    M.items = new_items
    M.link()
    return M, base


def build(M, base, B, merge, solo=frozenset()):
    tsplit.MERGE_BELOW = merge
    its = M.items
    laws = [i.idx for i in its if i.is_test]
    inc_items = [i for i in its if i.has_tests and not i.is_test]
    life_dep = {}
    for t_ in laws:
        cl = M.closure([t_])
        for h in inc_items:
            if h.idx in cl and "lifetime" in h.src:
                life_dep.setdefault(h.idx, []).append(t_)
    comp_help = {i.idx for i in its if i.src_kind == "include" and "composition" in i.src and not i.has_tests}
    comp_parts = [i.idx for i in its if i.src_kind == "include" and "composition-part" in i.src]
    comp_bundle = [t_ for t_ in laws if M.closure([t_]) & comp_help]
    bundled = {t_ for v in life_dep.values() for t_ in v} | set(comp_bundle)
    rest = [t_ for t_ in laws if t_ not in bundled]
    use = collections.Counter()
    for t_ in rest:
        for x in M.closure([t_]):
            if not its[x].has_tests:
                use[x] += 1
    universal = {x for x, c in use.items() if c >= 0.5 * len(rest)}
    for x in list(universal):
        universal |= M.closure([x])
    universal = {x for x in universal if not its[x].has_tests}
    ub = sum(its[x].bytes for x in universal)
    ue = {}
    for x in universal:
        ue.update(its[x].emb)
    lp = cluster(M, rest, universal, B - ub - sum(ue.values()), max_files=16)
    units = {}
    for i, p in enumerate(lp):
        units[f"L{i+1}"] = list(p["tests"])
    for h, ts in life_dep.items():
        units["LIFE"] = [h] + ts
    extras = [i.idx for i in inc_items if i.idx not in life_dep and i.idx not in comp_parts[:1]]
    if comp_parts:
        units["COMP"] = [comp_parts[0]] + comp_bundle
    ex_units = []
    for h in sorted(extras, key=lambda h: -(its[h].bytes + sum(its[h].emb.values()))):
        cl = {x for x in M.closure([h]) if not its[x].has_tests}
        for u in ([] if h in solo else ex_units):
            if any(i in solo for i in u["items"]):
                continue
            newc = u["cl"] | cl
            em = {}
            for x in list(newc) + u["items"] + [h]:
                em.update(its[x].emb)
            tot = sum(its[x].bytes for x in newc) + sum(its[t_].bytes for t_ in u["items"] + [h]) + sum(em.values())
            if tot <= B and len(em) <= 16:
                u["items"].append(h); u["cl"] = newc; break
        else:
            ex_units.append(dict(items=[h], cl=cl))
    for k, u in enumerate(ex_units):
        units[f"X{k+1}"] = u["items"]
    names = list(units)
    dirs = {u: sibling(MAIN, f"-part-{i+1}") for i, u in enumerate(names)}
    docs = {u: f"Part {i+1} of {len(names)} of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the shared fixtures stay in the original file)." for i, u in enumerate(names)}
    macros = [i.idx for i in its if i.kind == "macro_rules!"]
    import copy
    files, core, shared, private = emit(M, units, dirs, "🔬️", docs, always_core=macros)
    allf = dict(base); allf.update(files)
    sizes = {u: closure_size(dirs[u], allf)[:3] for u in names}
    return units, dirs, files, allf, sizes, (core, shared, private)


if __name__ == "__main__":
    args = sys.argv[1:]
    def opt(n, d):
        return int(args[args.index(n) + 1]) if n in args else d
    comp_budget = opt("--comp", 24000)
    if "--search" in args:
        M, base = prepare(comp_budget)
        for merge in (0, 3000, 6000):
            for B in range(210000, 100000, -10000):
                units, dirs, files, allf, sizes, _ = build(M, base, B, merge)
                worst = max(sizes.values(), key=lambda s: s[1])
                ok = all(s[1] <= LIMIT_BYTES and s[2] <= LIMIT_LINES and s[0] <= LIMIT_FILES for s in sizes.values())
                print("merge", merge, "B", B, "units", len(units), "worst bytes", worst[1], "max files", max(s[0] for s in sizes.values()), "max lines", max(s[2] for s in sizes.values()), "OK" if ok else "")
                if ok:
                    break
        sys.exit(0)
    B, merge = opt("--budget", 170000), opt("--merge", 3000)
    M, base = prepare(comp_budget)
    its = M.items
    solo = set()
    for rounds in range(12):
        units, dirs, files, allf, sizes, (core, shared, private) = build(M, base, B, merge, frozenset(solo))
        bad = [u for u, s in sizes.items() if s[1] > LIMIT_BYTES or s[2] > LIMIT_LINES or s[0] > LIMIT_FILES]
        print("round", rounds, "units", len(units), "bad", {u: sizes[u] for u in bad})
        grow = False
        for u in bad:
            cands = [h for h in units[u] if its[h].has_tests and not its[h].is_test and h not in solo]
            if len(units[u]) > 1 and cands:
                solo.add(max(cands, key=lambda h: its[h].bytes + sum(its[h].emb.values())))
                grow = True
        if not grow:
            break
    for u in units:
        print(u, dirs[u].split("/")[-2][-12:], "tests", len(units[u]), "closure files/bytes/lines", sizes[u])
    probs = verify(M, files)
    print("verify problems", probs[:6], len(probs))
    perr = parse_check(allf)
    print("parse errors", perr)
    names = list(units)
    mounts = "#[cfg(test)]\nuse crate::builder::PluginBuilder;\n" + "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(dirs[u], os.path.dirname(RUNTIME))}"]\nmod plugin_builder_contract_tests_{i+1};\n' for i, u in enumerate(names))
    rt = read(RUNTIME)
    assert rt.count(INCLUDE_LINE) == 1
    json.dump({"names": names, "dirs": {u: rel(p) for u, p in dirs.items()}}, open(GEN + "/pi2w7-cut-bc.json", "w"), ensure_ascii=False, indent=1)
    if "--apply" in args and not probs and not perr:
        for p, tx in allf.items():
            ed.put(p, tx)
        ed.put(RUNTIME, rt.replace(INCLUDE_LINE, mounts))
        print("applied", len(allf), "files")
