#!/usr/bin/env python3
"""🧭️ Test-path index of the plugin SDK partition: every `#[test]`-like fn of the facade lib and of the companion test bins with its crate package and the cargo law path (module path inside that crate).
usage: lawindex.py [out.json]   (library: build() -> {fn name: [(package, law path, file)]})"""
import glob, json, os, re, sys
from lib import *
from rs import blank
import tree as T

FN = re.compile(r"((?:#\[[^\]]*\]\s*)+)(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)")
MODI = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{")


def inline_mods(clean):
    out = []
    for m in MODI.finditer(clean):
        i, d = m.end(), 1
        while i < len(clean) and d:
            c = clean[i]
            d += (c == "{") - (c == "}")
            i += 1
        out.append((m.start(), i, m.group(1)))
    return out


def file_laws(path, prefix):
    text = read(path)
    clean = blank(text)
    mods = inline_mods(clean)
    res = []
    for m in FN.finditer(clean):
        if "test" not in clean[m.start(1):m.end(1)]:
            continue
        enc = [n for s, e, n in sorted(mods) if s < m.start() < e]
        res.append("::".join(prefix + enc + [m.group(2)]))
    return res


def crate_laws(root, package):
    modpath = {}
    out = []
    for p, parent, name in T.tree(root):
        if parent is None:
            mp = []
        elif name == "<include>":
            mp = modpath[parent]
        else:
            mp = modpath[parent] + [name]
        modpath[p] = mp
        for law in file_laws(p, mp):
            out.append((package, law, rel(p)))
    return out


def lib_crates():
    """(package, lib root) of every crate of the plugin tree whose manifest declares an explicit `[lib] path`"""
    out = []
    for dp, dns, fns in os.walk(PLUG):
        dns[:] = [d for d in dns if d not in ("node_modules", "target", ".cache")]
        if "Cargo.toml" in fns and dp.endswith("/" + PKGDIR):
            t = read(dp + "/Cargo.toml")
            name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', t)
            lib = re.search(r'(?ms)^\[lib\]\n(.*?)(?=^\[|\Z)', t)
            pth = lib and re.search(r'(?m)^path\s*=\s*"([^"]+)"', lib.group(1))
            if name and pth:
                out.append((name.group(1), os.path.normpath(dp + "/" + pth.group(1))))
    return out


def roots():
    out = [("semio-framework-plugin", PLUG + "/🦀️.rs")]
    seen = {r for _, r in out}
    for name, r in sorted(lib_crates()):
        if r not in seen and os.path.exists(r) and "🧷️companions" not in r:
            out.append((name, r)); seen.add(r)
    for d in sorted(glob.glob(PLUG + "/🧷️companions/*tests-*")):
        r = d + "/🦀️.rs"
        if os.path.exists(r) and not r.endswith("test-support/🦀️.rs"):
            out.append(("semio-framework-plugin-" + os.path.basename(d).split("️")[-1], r))
    return out


def build():
    idx = {}
    for pkg, r in roots():
        for package, law, f in crate_laws(r, pkg):
            idx.setdefault(law.split("::")[-1], []).append((package, law, f))
    return idx


if __name__ == "__main__":
    idx = build()
    n = sum(len(v) for v in idx.values())
    print(len(idx), "names", n, "laws")
    if len(sys.argv) > 1:
        json.dump(idx, open(sys.argv[1], "w"), ensure_ascii=False, indent=0)
