#!/usr/bin/env python3
# 📦️ Dependency check of one or more crates: external and first-party crate identifiers their source closure names against the manifest.
import sys, os, re, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank
HERE = os.path.dirname(os.path.abspath(__file__))
PK = T + "/📦️packages/🦀️rust"

def toml(path):
    return json.loads(subprocess.check_output(["bun", HERE + "/toml.ts", path]))

def idents(manifest):
    d = toml(manifest)
    out = {}
    tables = [d.get("dependencies", {}), d.get("dev-dependencies", {}), d.get("build-dependencies", {})]
    for v in d.get("target", {}).values():
        tables += [v.get("dependencies", {}), v.get("dev-dependencies", {})]
    for tb in tables:
        for k, v in tb.items():
            out[k.replace("-", "_")] = (k, v)
    return out

def check(suffix, table):
    root = T + "/" + table[suffix] + "/🦀️.rs"
    man = T + "/" + table[suffix] + "/📦️packages/🦀️rust/Cargo.toml"
    have = idents(man)
    top = idents(PK + "/Cargo.toml")
    files = modtree.tree(root)
    local, used = set(), collections.Counter()
    for f in files:
        code = blank(read(f))
        for m in re.finditer(r"\bmod\s+(\w+)", code): local.add(m.group(1))
        for m in re.finditer(r"\bas\s+(\w+)", code): local.add(m.group(1))
        for m in re.finditer(r"\b(?:enum|struct|trait|type|fn|static|const)\s+(\w+)", code): local.add(m.group(1))
        for m in re.finditer(r"(?<![\w:.$])([a-z_][a-z0-9_]*)::", code): used[m.group(1)] += 1
        for m in re.finditer(r"\buse\s+([a-z_][a-z0-9_]*)\b", code): used[m.group(1)] += 1
        for m in re.finditer(r"extern\s+crate\s+(\w+)", code): used[m.group(1)] += 1
        for m in re.finditer(r"#\[\s*(wasm_bindgen)\b", code): used[m.group(1)] += 1
    missing = sorted(n for n in used if n in top and n not in have and n not in local and not n.startswith("semio_framework_os_renderer_wgpu_"))
    first = sorted({n[len(PREFIX):].replace("_", "-") for n in used if n.startswith(PREFIX)})
    declared = sorted(k[len(PKG_PREFIX):] for k, _ in have.values() if k.startswith(PKG_PREFIX))
    return files, missing, top, first, declared

if __name__ == "__main__":
    table = crate_table()
    for c in sys.argv[1:]:
        files, missing, top, first, declared = check(c, table)
        print(c, len(files), "files")
        print("   missing external:", [(n, top[n][1]) for n in missing])
        print("   first-party undeclared:", [x for x in first if x not in declared and x != c], " declared-unused:", [x for x in declared if x not in first])
