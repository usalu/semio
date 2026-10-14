#!/usr/bin/env python3
"""✂️ Re-derives every artifact crate's alias block and dependency rows from what its closure names (lane P1).

After a slice moves items between existing crates, a crate's root may name a crate it does not alias yet. For each
artifact crate (`<owner>/📦️packages/🦀️rust/Cargo.toml` under the artifact workspace) this adds the missing
`extern crate` lines (lib, or `#[cfg(test)]` when only tests name it; `[[test]]` roots get their own), drops aliases
nothing names any more, and rewrites the manifest: energy-family rows are exactly the derived ones, other rows are the
union of the recorded and the derived rows.
usage: ✂️resync-crates.py [--dry]
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
DRY = "--dry" in sys.argv
DEV_FEATURES = {"semio-framework-os-kernel": ["protocol-laws", "mutation-testing"], "semio-framework-plugin": ["artifact-app-testing"]}
PKG_ALIAS = {v: k for k, v in L.ALIASES.items()}
EXTERN = re.compile(r"(?m)^(#\[cfg\(test\)\]\n)?extern crate (\w+) as (\w+);\n")

def manifests():
    for r, dirs, fs in os.walk(L.ART):
        dirs[:] = [x for x in dirs if x not in ("target", "node_modules")]
        if "Cargo.toml" in fs and r.endswith(os.path.join("📦️packages", "🦀️rust")): yield os.path.join(r, "Cargo.toml")

def parse(path):
    t = L.rd(path); sec = None; rows = {"dependencies": set(), "dev-dependencies": set()}; tests = []; cur = {}
    for line in t.splitlines():
        h = re.match(r"^\[\[?([^\]]+)\]\]?$", line)
        if h:
            sec = h.group(1)
            if line.startswith("[[test]]"): cur = {}; tests.append(cur)
            continue
        kv = re.match(r'^([\w-]+)\s*=\s*(.*)$', line)
        if not kv: continue
        if sec in rows: rows[sec].add(L.RENAMED.get(kv.group(1), kv.group(1)))
        elif sec == "test": cur[kv.group(1)] = kv.group(2).strip('"')
        elif sec == "package" and kv.group(1) in ("name", "description"): rows[kv.group(1)] = kv.group(2).strip('"')
    return rows, [(x["name"], x["path"]) for x in tests]

def sync_block(root, lib_used, test_used, own):
    t = L.rd(root); have = {m.group(3): bool(m.group(1)) for m in EXTERN.finditer(t)}
    want = {a: False for a in lib_used} | {a: True for a in test_used - lib_used}
    want = {a: c for a, c in want.items() if a in L.ALIASES and a != own and a != "pack"}
    keep_unknown = {a for a in have if a not in L.ALIASES and a not in PKG_ALIAS.values()}
    if {a: c for a, c in have.items() if a not in keep_unknown} == want: return False
    if DRY: print(os.path.relpath(root, L.ART)[-60:], 'add', sorted((a, c) for a, c in want.items() if have.get(a) != c), 'drop', sorted(a for a in have if a not in want and a not in keep_unknown))
    ms = list(EXTERN.finditer(t))
    lines = [f"extern crate {L.ALIASES[a].replace('-', '_')} as {a};" for a in L.ROOT_ALIAS_ORDER if a in want and not want[a]]
    lines += [f"extern crate {L.ALIASES[a].replace('-', '_')} as {a};" for a in sorted(want) if a not in L.ROOT_ALIAS_ORDER and not want[a]]
    lines += [f"#[cfg(test)]\nextern crate {L.ALIASES[a].replace('-', '_')} as {a};" for a in sorted(want) if want[a]]
    lines += [m.group(0).rstrip("\n") for m in ms if m.group(3) in keep_unknown]
    block = "\n".join(lines) + "\n"
    if ms:
        t = t[:ms[0].start()] + block + "".join(t[a.end():b.start()] for a, b in zip(ms, ms[1:])).lstrip("\n") + t[ms[-1].end():]
    else:
        i = t.index("\n\n") + 2
        while t[i:].startswith("#!["): i = t.index("\n", i) + 1
        t = t[:i] + ("\n" if not t[i - 2:i] == "\n\n" else "") + block + "\n" + t[i:]
    if not DRY: L.wr(root, t)
    return True

changed = []
for mf in sorted(manifests()):
    pkg_dir = os.path.dirname(mf); owner = os.path.normpath(os.path.join(pkg_dir, "..", ".."))
    if owner == os.path.normpath(L.ART) and not os.path.exists(os.path.join(owner, "🦀️.rs")): continue
    rows, tests = parse(mf); package = rows.get("name")
    if not package or not package.startswith("semio-s-artifact-energy-model"): continue
    own = PKG_ALIAS.get(package); root = os.path.join(owner, "🦀️.rs")
    files = L.closure_files(root)
    lib = [f for f in files if "🧪️tests" not in os.path.relpath(f, owner)]
    test = [f for f in files if "🧪️tests" in os.path.relpath(f, owner)]
    local = set(re.findall(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", L.strip(L.rd(root))))
    used_l, deps = L.deps_for(lib, package, local); used_t, dev = L.deps_for(test, package, local)
    touched = sync_block(root, used_l, used_t, own)
    for name, rel in tests:
        troot = os.path.normpath(os.path.join(pkg_dir, rel)); tf = L.closure_files(troot)
        ux, tdeps = L.deps_for(tf, package); dev |= tdeps; test += tf
        touched |= sync_block(troot, ux, set(), None)
    dev -= {package}
    if any("async_test" in L.rd(f) for f in test): dev.add("semio-framework-async-macros")
    family = set(L.ALIASES.values()) - {L.ALIASES[a] for a in L.FRAMEWORK_ALIAS_CRATES}
    deps_all = {d for d in rows["dependencies"] if d not in family} | deps
    dev_all = ({d for d in rows["dev-dependencies"] if d not in family} | dev) - deps_all
    feats = {k: v for k, v in DEV_FEATURES.items() if (k in deps_all or k in dev_all) and test}
    dev_all |= set(feats)
    before = L.rd(mf)
    if not DRY:
        L.manifest(owner, package, rows.get("description", ""), deps_all, sorted(dev_all), feats, tests)
        L.ensure_workspace_rows(deps_all | dev_all)
    if touched or (not DRY and L.rd(mf) != before): changed.append(package)
print(len(changed), "crates resynced:", " ".join(sorted(changed)))
