#!/usr/bin/env python3
# 📋️ Compare the external crates a carved crate names against its manifest; --apply adds the missing dependency lines (specs copied from the old top manifest).
# usage: manifest.py <suffix> [--apply] [--dev name,name]
import os
import re
import sys
from common import *
import modtree
from rs import blank, uses

TOP = PKG_REL
TOP_MANIFEST = REPO + "/" + PKG_REL + "/Cargo.toml"
STD = {"crate", "self", "super", "std", "core", "alloc", "Self"}


def top_specs():
    specs = {}
    section = ""
    for line in read(TOP_MANIFEST).split("\n"):
        m = re.match(r"\[(.+)\]\s*$", line.strip())
        if m:
            section = m.group(1)
            continue
        m = re.match(r"([A-Za-z0-9_\-]+)\s*=\s*(.+)$", line.strip())
        if m and "dependencies" in section and not m.group(1).startswith(PKG_PREFIX):
            specs.setdefault(m.group(1), []).append((section, m.group(2)))
    return specs


def lib_of(dep, spec):
    pm = re.search(r'package\s*=\s*"([^"]+)"', spec)
    return dep.replace("-", "_")


def named_externals(files):
    names = set()
    for f in files:
        text = read(f)
        clean = blank(text)
        for m in re.finditer(r"(?<![A-Za-z0-9_$:.])([A-Za-z_][A-Za-z0-9_]*)::", clean):
            names.add(m.group(1))
        for m in re.finditer(r"extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)", clean):
            names.add(m.group(1))
        for m in re.finditer(r"#\[\s*([a-z_]+)\s*\(", clean):
            pass
        if re.search(r"derive\([^)]*\b(Serialize|Deserialize)\b", clean):
            names.add("serde")
        if re.search(r"#\[\s*serde\s*\(", clean):
            names.add("serde")
        if re.search(r"#\[\s*wasm_bindgen\b", clean):
            names.add("wasm_bindgen")
    return names


def rebase_paths(spec, from_dir, to_dir):
    def sub(m):
        absolute = os.path.normpath(os.path.join(from_dir, m.group(1)))
        return 'path = "' + os.path.relpath(absolute, to_dir) + '"'
    return re.sub(r'path\s*=\s*"([^"]+)"', sub, spec)


def main():
    suffix = sys.argv[1]
    apply = "--apply" in sys.argv
    folder = T + "/" + CRATES[suffix]
    mpath = folder + "/📦️packages/🦀️rust/Cargo.toml"
    have = set()
    section = ""
    for line in read(mpath).split("\n"):
        m = re.match(r"\[(.+)\]\s*$", line.strip())
        if m:
            section = m.group(1)
            continue
        m = re.match(r"([A-Za-z0-9_\-]+)\s*=", line.strip())
        if m and "dependencies" in section:
            have.add(m.group(1).replace("-", "_"))
    files = [f for f in modtree.tree(folder + "/🦀️.rs")]
    prod_files = [f for f in files if "/🧪️tests/" not in f and not f.endswith("/tests.rs")]
    used = named_externals(files)
    used_prod = named_externals(prod_files)
    specs = top_specs()
    by_lib = {k.replace("-", "_"): k for k in specs}
    local = set()
    for f in files:
        local |= set(re.findall(r"\bmod\s+([A-Za-z_][A-Za-z0-9_]*)", blank(read(f))))
        local |= set(re.findall(r"\b(?:struct|enum|trait|type|union)\s+([A-Za-z_][A-Za-z0-9_]*)", blank(read(f))))
    missing = []
    for n in sorted(used):
        if n in STD or n in have or n in local:
            continue
        if n in by_lib:
            missing.append(n)
    print("files", len(files), "missing", missing)
    unknown = [n for n in sorted(used) if n not in STD and n not in have and n not in local and n not in by_lib and not n[0].isupper() and n.startswith(("semio", "ui_", "infinite", "flow", "replication", "serde", "dsl", "protocol", "store", "camera", "framework"))]
    if unknown:
        print("first-party/unknown names:", unknown)
    if apply and missing:
        text = read(mpath)
        add = ""
        dev = ""
        for n in missing:
            dep = by_lib[n]
            cands = [c for c in specs[dep] if "entrypoint" not in c[1]] or specs[dep]
            sec, spec = cands[0]
            spec = rebase_paths(spec, os.path.dirname(TOP_MANIFEST), os.path.dirname(mpath))
            if n in used_prod:
                add += f"{dep} = {spec}\n"
            else:
                dev += f"{dep} = {spec}\n"
        assert "[dependencies]\n" in text and text.rstrip().endswith("[dev-dependencies]") or "[dev-dependencies]\n" in text
        text = text.replace("[dependencies]\n", "[dependencies]\n" + add, 1)
        text = text.rstrip("\n") + "\n" + dev if text.rstrip().endswith("[dev-dependencies]") else text.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + dev, 1)
        write(mpath, text)
        print("added", add, "dev", dev)


if __name__ == "__main__":
    main()
