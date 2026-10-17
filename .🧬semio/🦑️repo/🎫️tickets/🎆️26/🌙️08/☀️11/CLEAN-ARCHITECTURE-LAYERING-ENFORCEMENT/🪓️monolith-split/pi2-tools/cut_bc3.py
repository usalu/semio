#!/usr/bin/env python3
"""✂️ Cuts the builder-contract test module into pieces: the included flat fixture files are flattened into the item list, every test (own or from an included file) is clustered with the helpers it needs; shared helpers become fragment files, items that came from included files go to `-x` files so the original file keeps only its own text.
usage: cut_bc3.py [--apply] [--budget N] [--merge N] [--search]"""
import sys, collections, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tsplit
from tsplit import *
import ed
LIMIT_BYTES, LIMIT_LINES, LIMIT_FILES = 250000, 3700, 30
MAIN = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
RUNTIME = PLUG + "/⚛️reactor/🏃️runtime/🦀️.rs"
INCLUDE_LINE = '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");\n'


def prepare():
    M = Module.load(MAIN, "plugin_builder_contract_tests")
    M.wrapper = None
    assert M.pre.strip() == "use crate::builder::PluginBuilder;", repr(M.pre)
    flat_sources = sorted({it.src for it in M.items if it.src_kind == "include"})
    flatten(M)
    return M, flat_sources


def build(M, B, merge, solo=frozenset()):
    tsplit.MERGE_BELOW = merge
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
    ub = sum(its[x].bytes for x in universal)
    ue = {}
    for x in universal:
        ue.update(its[x].emb)
    lp = cluster(M, tests, universal, B - ub - sum(ue.values()), max_files=16)
    units = {f"U{i+1}": list(p["tests"]) for i, p in enumerate(lp)}
    names = list(units)
    dirs = {u: sibling(MAIN, f"-part-{i+1}") for i, u in enumerate(names)}
    docs = {u: f"Part {i+1} of {len(names)} of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the shared fixtures stay in the original file)." for i, u in enumerate(names)}
    macros = [i.idx for i in its if i.kind == "macro_rules!"]
    files, core, shared, private = emit2(M, units, dirs, "🔬️", docs, always_core=macros)
    sizes = {u: closure_size(dirs[u], files)[:3] for u in names}
    return units, dirs, files, sizes, (core, shared, private)


if __name__ == "__main__":
    args = sys.argv[1:]
    opt = lambda n, d: int(args[args.index(n) + 1]) if n in args else d
    M, flat_sources = prepare()
    its = M.items
    if "--search" in args:
        for merge in (3000, 6000):
            for B in range(220000, 120000, -10000):
                units, dirs, files, sizes, _ = build(M, B, merge)
                ok = all(s[1] <= LIMIT_BYTES and s[2] <= LIMIT_LINES and s[0] <= LIMIT_FILES for s in sizes.values())
                print("merge", merge, "B", B, "units", len(units), "max bytes", max(s[1] for s in sizes.values()), "max files", max(s[0] for s in sizes.values()), "max lines", max(s[2] for s in sizes.values()), "OK" if ok else "")
                if ok:
                    break
        sys.exit(0)
    B, merge = opt("--budget", 190000), opt("--merge", 3000)
    units, dirs, files, sizes, (core, shared, private) = build(M, B, merge)
    for u in units:
        print(u, dirs[u].split("/")[-2][-12:], "items", len(units[u]), "closure files/bytes/lines", sizes[u])
    probs = verify(M, files)
    print("verify problems", probs[:6], len(probs))
    perr = parse_check(files)
    print("parse errors", perr)
    print("flat sources flattened:", [s[-50:] for s in flat_sources])
    names = list(units)
    mounts = "#[cfg(test)]\nuse crate::builder::PluginBuilder;\n" + "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(dirs[u], os.path.dirname(RUNTIME))}"]\nmod plugin_builder_contract_tests_{i+1};\n' for i, u in enumerate(names))
    rt = read(RUNTIME)
    assert rt.count(INCLUDE_LINE) == 1
    json.dump({"names": names, "dirs": {u: rel(p) for u, p in dirs.items()}, "flat": flat_sources}, open(GEN + "/pi2w7-cut-bc.json", "w"), ensure_ascii=False, indent=1)
    if "--apply" in args and not probs and not perr:
        for p, tx in files.items():
            ed.put(p, tx)
        for s in flat_sources:
            ed.delete(PLUG + "/" + s)
        ed.put(RUNTIME, rt.replace(INCLUDE_LINE, mounts))
        print("applied", len(files), "files; deleted", len(flat_sources))
