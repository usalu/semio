#!/usr/bin/env python3
"""✂️ Cuts the builder-contract test module into pieces plus support modules (the fixtures that more than one piece uses become real modules the pieces import). usage: cut_bc4.py [--apply] [--budget N] [--support N]"""
import sys, collections, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tsplit
from tsplit import *
import ed
LIMIT_BYTES, LIMIT_LINES, LIMIT_FILES = 248000, 3700, 30
MAIN = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
RUNTIME = PLUG + "/⚛️reactor/🏃️runtime/🦀️.rs"
INCLUDE_LINE = '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");\n'


def prepare():
    M = Module.load(MAIN, "plugin_builder_contract_tests")
    M.wrapper = None
    assert M.pre.strip() == "use crate::builder::PluginBuilder;", repr(M.pre)
    flat = sorted({it.src for it in M.items if it.src_kind == "include"})
    flatten(M)
    return M, flat


def build(M, B, supp_limit):
    its = M.items
    tests = [i.idx for i in its if i.has_tests]
    use = collections.Counter()
    for t_ in tests:
        for x in M.closure([t_]):
            if not its[x].has_tests:
                use[x] += 1
    universal = {x for x, c in use.items() if c >= 0.5 * len(tests)}
    for x in list(universal):
        universal |= M.closure([x])
    universal = {x for x in universal if not its[x].has_tests}
    lp = cluster(M, tests, universal, B, max_files=18)
    units = {f"U{i+1}": list(p["tests"]) for i, p in enumerate(lp)}
    names = list(units)
    dirs = {u: sibling(MAIN, f"-part-{i+1}") for i, u in enumerate(names)}
    docs = {u: f"Part {i+1} of {len(names)} of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules)." for i, u in enumerate(names)}
    macros = [i.idx for i in its if i.kind == "macro_rules!"]
    files, spaths, smods, groups, private = emit3(M, units, dirs, "🔬️", docs, None, supp_limit, always_core=macros)
    sizes = {}
    for u in names:
        # a piece alone: its own files plus its -x; the support modules are separate crates after the move
        n, b, l, texts = closure_size(dirs[u], files)
        sup = set(spaths) | {sibling(p, "-x") for p in spaths}
        own = {f: t for f, t in texts.items() if f not in sup}
        nf = len(own); nb = sum(len(t.encode()) for t in own.values()); nl = sum(sum(1 for z in blank(t).split("\n") if z.strip()) for f, t in own.items() if f.endswith(".rs"))
        sizes[u] = (nf, nb, nl)
    ssizes = []
    for p in spaths:
        n, b, l, texts = closure_size(p, files)
        ssizes.append((n, b, l))
    return units, dirs, files, sizes, ssizes, (spaths, smods, groups, private)


if __name__ == "__main__":
    args = sys.argv[1:]
    opt = lambda n, d: int(args[args.index(n) + 1]) if n in args else d
    M, flat = prepare()
    B, S = opt("--budget", 215000), opt("--support", 215000)
    units, dirs, files, sizes, ssizes, (spaths, smods, groups, private) = build(M, B, S)
    for u in units:
        print(u, dirs[u].split("/")[-2][-12:], "items", len(units[u]), "own closure files/bytes/lines", sizes[u])
    print("support modules", [(rel(p)[-50:], s) for p, s in zip(spaths, ssizes)])
    probs = verify(M, files)
    print("verify problems", probs[:6], len(probs))
    perr = parse_check(files)
    print("parse errors", perr)
    its = M.items
    law_ok = any("struct TestApp<const RETAINED" in files[p] for p in spaths)
    print("TestApp support:", [rel(p)[-40:] for p in spaths if "struct TestApp<const RETAINED" in files[p]])
    names = list(units)
    mounts = "#[cfg(test)]\nuse crate::builder::PluginBuilder;\n" + "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(p, os.path.dirname(RUNTIME))}"]\npub(crate) mod {m};\n' for p, m in zip(spaths, smods)) + "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(dirs[u], os.path.dirname(RUNTIME))}"]\nmod plugin_builder_contract_tests_{i+1};\n' for i, u in enumerate(names))
    rt = read(RUNTIME)
    assert rt.count(INCLUDE_LINE) == 1
    json.dump({"names": names, "dirs": {u: rel(p) for u, p in dirs.items()}, "supports": [rel(p) for p in spaths], "flat": flat}, open(GEN + "/pi2w7-cut-bc.json", "w"), ensure_ascii=False, indent=1)
    if "--apply" in args and not probs and not perr:
        for p, tx in files.items():
            ed.put(p, tx)
        for s in flat:
            ed.delete(PLUG + "/" + s)
        ed.put(RUNTIME, rt.replace(INCLUDE_LINE, mounts))
        print("applied", len(files), "files; deleted", len(flat))
