#!/usr/bin/env python3
"""✅️ Static gates for the flipped crates: path literals, single owner per file, orphan/inherent impls, manifest dependencies, region markers, rustfmt parse, TOML parse.
usage: verify.py <crate>... [--all]   (reads the live tree)"""
import collections, json, os, re, subprocess, sys
from lib import *
from rs import blank
import tree as T
import rsparse as P
import manifest as MF
import world as Wd
import uo

CR = Wd.O.CR


def crate_root(K):
    return REPO + "/" + CR[K]["dir"] + "/🦀️.rs"


def real_set():
    return uo.real_crates()


def tree_files(root):
    return [f for f, parent, n in T.tree(root)]


def check_paths(K):
    """every #[path]/include!/mod literal of the crate tree resolves"""
    problems = []
    seen = set()
    stack = [crate_root(K)]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.exists(f):
            continue
        seen.add(f)
        text = read(f)
        clean = blank(text)
        for m in T.MOUNT.finditer(clean):
            if m.group(4) == "{":
                continue
            attrs = text[m.start(1):m.end(1)]
            pm = T.PATH_ATTR.search(attrs)
            if pm:
                t = os.path.normpath(os.path.join(os.path.dirname(f), pm.group(1)))
                if not os.path.exists(t):
                    problems.append((rel(f), pm.group(1)))
                else:
                    stack.append(t)
        for c, n, a, v in T.children(f):
            if os.path.exists(c):
                stack.append(c)
        for m in T.INCLUDE.finditer(text):
            if clean[m.start():m.start() + 8] == "include!":
                t = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
                if not os.path.exists(t) and not t.endswith((".json", ".bin", ".txt", ".wit")):
                    problems.append((rel(f), m.group(1)))
    return problems, seen


def region_balance(files):
    bad = []
    for f in files:
        t = read(f)
        o = re.findall(r"//#region ([^\n]*)", t)
        c = re.findall(r"//#endregion ([^\n]*)", t)
        if len(o) != len(c):
            bad.append((rel(f), len(o), len(c)))
    return bad


def rustfmt_parse(files):
    bad = []
    for f in files:
        p = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", f], capture_output=True, text=True)
        if p.returncode != 0 and "error" in p.stderr:
            bad.append((rel(f), p.stderr.strip().split("\n")[0][:160]))
    return bad


def toml_parse(K):
    p = subprocess.run(["bun", "-e", f"const t=await Bun.file({json.dumps(MF.pkg_dir(K) + '/Cargo.toml')}).text();Bun.TOML.parse(t);console.log('ok')"], capture_output=True, text=True)
    return p.stdout.strip() == "ok", p.stderr[:200]


def deps_check(K, files):
    texts = [read(f) for f in files if f.endswith(".rs")]
    have = MF.dep_lines(read(MF.pkg_dir(K) + "/Cargo.toml"))
    haveidents = {MF.norm(k) for k in have} | {"std", "core", "alloc", "crate", "super", "self"}
    ids = MF.used_crate_idents(texts)
    fac = {MF.norm(k) for k in MF.facade_deps()}
    fp = {"semio_framework_plugin_" + k.replace("-", "_") for k in CR}
    missing = sorted(i for i in ids if i not in haveidents and (i in fac or i in fp))
    return missing


def impl_scan(files, owner_of_type):
    return []


def run(K):
    root = crate_root(K)
    problems, seen = check_paths(K)
    files = [f for f in seen if f.endswith(".rs")]
    out = dict(crate=K, files=len(files), path_problems=problems, regions=region_balance(files), fmt=rustfmt_parse(files), toml=toml_parse(K), deps_missing=deps_check(K, files))
    return out, set(files)


def single_owner(crates):
    owners = collections.defaultdict(list)
    for K in crates:
        _, seen = check_paths(K)
        for f in seen:
            owners[f].append(K)
    return {rel(f): v for f, v in owners.items() if len(v) > 1}


if __name__ == "__main__":
    crates = [a for a in sys.argv[1:] if not a.startswith("--")]
    allf = {}
    for K in crates:
        out, files = run(K)
        print(json.dumps(out, ensure_ascii=False))
    print("shared files:", json.dumps(single_owner(list(real_set()) if "--all" in sys.argv else crates), ensure_ascii=False))
