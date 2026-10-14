#!/usr/bin/env python3
"""✂️ Fills every new artifact crate root's `@@ALIASES@@` block and writes its manifest, workspace member and rows (lane P1).

A crate's aliases and dependencies are what its closure names (`✂️artifact_lib.deps_for`): lib files give
`[dependencies]`, `🧪️tests` files and `[[test]]` roots give `[dev-dependencies]` and `#[cfg(test)]` aliases.
`[[test]]` roots (files under `🧬️mutations/🧪️tests/🔬️<target>/`) get their own alias block and feed the fixture crate.
usage: ✂️finish-crates.py <spec.json>   spec: [{"owner": <dir>, "package": <name>, "alias": <alias>, "description": <text>,
       "tests": [[<target>, <root file>]...]}]
"""
import importlib.util, json, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
DEV_FEATURES = {"semio-framework-os-kernel": ["protocol-laws", "mutation-testing"], "semio-framework-plugin": ["artifact-app-testing"]}
crates = json.load(open(sys.argv[1], encoding="utf-8"))
for c in crates:
    owner, package, alias = c["owner"], c["package"], c.get("alias")
    root = f"{owner}/🦀️.rs"
    files = L.closure_files(root)
    lib = [f for f in files if "🧪️tests" not in os.path.relpath(f, owner)]
    test = [f for f in files if "🧪️tests" in os.path.relpath(f, owner)]
    used_l, deps = L.deps_for(lib, package)
    used_t, dev = L.deps_for(test, package)
    for x in (used_l, used_t): x.discard(alias)
    text = L.rd(root)
    if "@@ALIASES@@" in text:
        block = L.alias_block(used_l)
        test_only = sorted(used_t - used_l - {"pack"})
        if test_only: block += ("\n" if block else "") + "\n".join(f"#[cfg(test)]\nextern crate {L.ALIASES[a].replace('-', '_')} as {a};" for a in test_only)
        L.wr(root, text.replace("@@ALIASES@@", block))
    tests = []
    for name, troot in c.get("tests", []):
        tfiles = L.closure_files(troot)
        used_x, tdeps = L.deps_for(tfiles, package)
        t = L.rd(troot)
        if "@@ALIASES@@" in t: L.wr(troot, t.replace("@@ALIASES@@", L.alias_block(used_x)))
        dev |= tdeps
        test += tfiles
        tests.append((name, os.path.relpath(troot, os.path.join(owner, "📦️packages", "🦀️rust"))))
    dev -= {package}
    if any("async_test" in L.rd(f) for f in test): dev.add("semio-framework-async-macros")
    feats = {k: v for k, v in DEV_FEATURES.items() if (k in deps or k in dev) and test}
    dev_rows = (dev - deps) | set(feats)
    L.manifest(owner, package, c["description"], deps, sorted(dev_rows), feats, tests)
    L.ensure_workspace_rows(deps | dev)
    L.add_energy_workspace_row(package, owner)
    L.add_workspace_member(owner)
    print(package, len(deps), "deps", len(dev_rows), "dev", len(tests), "tests")
