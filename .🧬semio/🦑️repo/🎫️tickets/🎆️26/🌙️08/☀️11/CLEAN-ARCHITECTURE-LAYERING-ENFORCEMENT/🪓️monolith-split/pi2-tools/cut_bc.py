#!/usr/bin/env python3
"""✂️ Cuts the builder-contract test module (and the completion / composition fragments it includes) into pieces.
usage: cut_bc.py <piece budget bytes> [--apply]"""
import sys, collections, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tsplit import *
import ed
BUDGET = int(sys.argv[1])
MAIN = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
COMP = PLUG + "/🧪️tests/⏳️completion/🦀️.rs"
COMPS = PLUG + "/🧪️tests/🧩️composition/🦀️.rs"
RUNTIME = PLUG + "/⚛️reactor/🏃️runtime/🦀️.rs"
INCLUDE_LINE = '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");\n'
M = Module.load(MAIN, "plugin_builder_contract_tests")
M.wrapper = None
assert M.pre.strip() == "use crate::builder::PluginBuilder;", repr(M.pre)
all_files = {}

# --- completion: its single test leaves the fragment -------------------------------------------------------------
Mc = Module.load(COMP)
ct = [i for i in Mc.items if i.is_test]
assert len(ct) == 1
comp_core_text = strip_unbalanced_regions("".join(i.text for i in Mc.items if i is not ct[0]) + Mc.tail)
COMP_TEST = sibling(COMP, "-transient-close")
comp_test_text = strip_unbalanced_regions("// ⏳️ The transient-close law cut out of the completion fixtures: it shares them through the module that includes both.\n" + ct[0].text + Mc.tail[:0])
all_files[COMP] = comp_core_text
all_files[COMP_TEST] = comp_test_text

# --- composition: two parts plus shared fixtures -----------------------------------------------------------------
Mp = Module.load(COMPS)
ptests = [i.idx for i in Mp.items if i.has_tests]
main_ids = set()
for it in M.items:
    if it.src == "main" or it.src_kind != "include":
        main_ids |= it.ids
need = {d for it in Mp.items for d in it.defs if d in main_ids and not it.has_tests}
always = {it.idx for it in Mp.items if it.defs & need and not it.has_tests}
for x in list(always):
    always |= Mp.closure([x])
always = {x for x in always if not Mp.items[x].has_tests and Mp.items[x].kind != "use"}
use = collections.Counter()
for t_ in ptests:
    for x in Mp.closure([t_]):
        if not Mp.items[x].has_tests:
            use[x] += 1
cuse = collections.Counter()
for t_ in ptests:
    for x in Mp.closure([t_]):
        if not Mp.items[x].has_tests:
            cuse[x] += 1
cuniv = {x for x, c in cuse.items() if c >= 0.5 * len(ptests)}
for x in list(cuniv):
    cuniv |= Mp.closure([x])
cuniv = {x for x in cuniv if not Mp.items[x].has_tests}
pieces = cluster(Mp, ptests, cuniv, 24000, max_files=18)
punits = {f"C{i+1}": p["tests"] for i, p in enumerate(pieces)}
pdirs = {u: sibling(COMPS, f"-part-{i+1}") for i, u in enumerate(punits)}
pdocs = {u: f"Part {i+1} of {len(punits)} of the composed-document laws (cut from `🧩️composition`)." for i, u in enumerate(punits)}
cfiles, ccore, cshared, cprivate, cfrag = emit_fragments(Mp, punits, pdirs, "🧩️", pdocs)
all_files.update(cfiles)
print("composition parts", {u: len(t) for u, t in punits.items()}, {rel(p)[-40:]: len(t) for p, t in cfiles.items()})

# --- patch the item list of the main module ----------------------------------------------------------------------
new_items, idx = [], 0
def add(it):
    global idx
    it.idx = idx
    idx += 1
    new_items.append(it)
for it in M.items:
    if it.src_kind == "include" or (it.kind == "other" and it.src == rel(COMP)):
        pass
    if it.src == rel(COMP):
        add(make_include_item(M, 0, COMP)) if False else None
        h = make_include_item(M, 0, COMP_NEW := COMP)
        # helper include with the test removed
        itx = comp_core_text
        icl = blank(itx)
        h.ids = set(IDENT.findall(icl)); h.defs = set()
        for s2 in P.statements(itx, icl, 0, len(itx)):
            if s2.name and s2.kind not in ("use", "impl"):
                h.defs.add(s2.name)
            if s2.kind == "other":
                h.defs |= set(DEF_IN_MACRO.findall(icl[s2.start:s2.end]))
        h.has_tests = False; h.bytes = len(itx.encode()); h.lines = itx.count("\n") + 1
        h.emb = embeds_from_text(itx, COMP)
        h.text = it.text; h.gap = it.gap
        add(h)
        add(make_include_item(M, 0, COMP_TEST))
        continue
    if it.src == rel(COMPS):
        for p in [COMPS] + [cfrag[s] for s in sorted(cfrag, key=lambda s: min(cshared[s]))]:
            h = make_include_item(M, 0, p)
            itx = cfiles[p]; icl = blank(itx)
            h.ids = set(IDENT.findall(icl)); h.defs = set()
            for s2 in P.statements(itx, icl, 0, len(itx)):
                if s2.name and s2.kind not in ("use", "impl"):
                    h.defs.add(s2.name)
                if s2.kind == "other":
                    h.defs |= set(DEF_IN_MACRO.findall(icl[s2.start:s2.end]))
            h.has_tests = False; h.bytes = len(itx.encode()); h.lines = itx.count("\n") + 1
            h.emb = embeds_from_text(itx, p)
            add(h)
        for u in punits:
            h = make_include_item(M, 0, pdirs[u])
            itx = cfiles[pdirs[u]]; icl = blank(itx)
            h.ids = set(IDENT.findall(icl)); h.defs = set()
            for s2 in P.statements(itx, icl, 0, len(itx)):
                if s2.name and s2.kind not in ("use", "impl"):
                    h.defs.add(s2.name)
            h.has_tests = True; h.bytes = len(itx.encode()); h.lines = itx.count("\n") + 1
            h.emb = embeds_from_text(itx, pdirs[u])
            add(h)
        continue
    add(it)
M.items = new_items
M.link()
its = M.items
for it in its:
    if it.src_kind == "include" and it.has_tests and "composition-part" not in it.src and "completion-transient" not in it.src:
        pass
for i in its:
    if i.src_kind == "include" and i.src.endswith("completion/🦀️.rs"):
        i.has_tests = False

# --- units ----------------------------------------------------------------------------------------------------------
laws = [i.idx for i in its if i.is_test]
inc_items = [i for i in its if i.has_tests and not i.is_test]
bund = {}
for t_ in laws:
    cl = M.closure([t_])
    for h in inc_items:
        if h.idx in cl and "lifetime" in h.src:
            bund.setdefault(h.idx, []).append(t_)
comp_help = {i.idx for i in its if i.src_kind == "include" and "composition" in i.src and not i.has_tests}
comp_parts = [i.idx for i in its if i.src_kind == "include" and "composition-part" in i.src]
comp_bundle = [t_ for t_ in laws if M.closure([t_]) & comp_help]
print("law tests bundled with the composition part", len(comp_bundle))
bundled = {t_ for v in bund.values() for t_ in v} | set(comp_bundle)
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
lp = cluster(M, rest, universal, BUDGET - ub - sum(sum(its[x].emb.values()) for x in universal), max_files=18)
units = {}
for i, p in enumerate(lp):
    units[f"L{i+1}"] = list(p["tests"])
for h, ts in bund.items():
    units["LIFE"] = [h] + ts
extras = [i.idx for i in inc_items if i.idx not in bund and i.idx not in comp_parts[:1]]
if comp_parts:
    units["COMP"] = [comp_parts[0]] + comp_bundle
ex_units = []
for h in sorted(extras, key=lambda h: -its[h].bytes):
    cl = {x for x in M.closure([h]) if not its[x].has_tests}
    for u in ex_units:
        newc = u["cl"] | cl
        em = {}
        for x in list(newc) + u["items"] + [h]:
            em.update(its[x].emb)
        tot = sum(its[x].bytes for x in newc) + sum(its[t_].bytes for t_ in u["items"] + [h]) + sum(em.values())
        if tot <= BUDGET and len(em) <= 18:
            u["items"].append(h); u["cl"] = newc; break
    else:
        ex_units.append(dict(items=[h], cl=cl))
for k, u in enumerate(ex_units):
    units[f"X{k+1}"] = u["items"]
names = list(units)
dirs = {u: sibling(MAIN, f"-part-{i+1}") for i, u in enumerate(names)}
docs = {u: f"Part {i+1} of {len(names)} of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the shared fixtures stay in the original file)." for i, u in enumerate(names)}
macros = [i.idx for i in its if i.kind == "macro_rules!"]
files, core, shared, private = emit(M, units, dirs, "🔬️", docs, always_core=macros)
all_files.update(files)
for u in names:
    cl = {x for x in M.closure(units[u]) if not its[x].has_tests}
    print(u, dirs[u].split("/")[-2], "tests", len(units[u]), "bytes", sum(its[t_].bytes for t_ in units[u]) + sum(its[x].bytes for x in cl), "lines", sum(its[t_].lines for t_ in units[u]) + sum(its[x].lines for x in cl))
for u in names:
    nf, nb, nl, _ = closure_size(dirs[u], all_files)
    print("   closure of", u, "files", nf, "bytes", nb, "lines", nl)
print("core", len(core), sum(its[x].bytes for x in core), "shared classes", len(shared))
probs = verify(M, {p: t for p, t in files.items()})
print("verify problems", probs[:6], len(probs))
perr = parse_check(all_files)
print("parse errors", perr)
mounts = "#[cfg(test)]\nuse crate::builder::PluginBuilder;\n" + "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(dirs[u], os.path.dirname(RUNTIME))}"]\nmod plugin_builder_contract_tests_{i+1};\n' for i, u in enumerate(names))
rt = read(RUNTIME)
assert rt.count(INCLUDE_LINE) == 1
json.dump({"names": names, "dirs": {u: rel(p) for u, p in dirs.items()}, "tests": {u: [its[t_].name or its[t_].src for t_ in units[u]] for u in names}}, open(GEN + "/pi2w7-cut-bc.json", "w"), ensure_ascii=False, indent=1)
if "--apply" in sys.argv and not probs and not perr:
    for p, tx in all_files.items():
        ed.put(p, tx)
    ed.put(RUNTIME, rt.replace(INCLUDE_LINE, mounts))
    print("applied", len(all_files), "files")
if os.environ.get("SHOW"):
    for u in os.environ["SHOW"].split(","):
        nf, nb, nl, seen = closure_size(dirs[u], all_files)
        print("==", u)
        for f, tx in sorted(seen.items(), key=lambda kv: -len(kv[1])):
            print("   ", len(tx.encode()), rel(f)[-80:])
