"""🔎 First path segments `ident::` in each crate that are neither a dependency, extern alias, local module, imported name, primitive nor std."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *

S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
PRIMS = set("u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char str std core alloc crate self super".split())
FIRST = re.compile(r'(?<![\w:.$#])([a-z_]\w*)::')
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    deps = set(c["deps"]) | {lib}
    al = S.aliases_of(files)
    mods = set()
    for src in files.values():
        mods |= set(re.findall(r'\bmod\s+(\w+)', src))
        mods |= set(re.findall(r'\b(?:fn|static|const)\s+(\w+)', src))
    for f, src in files.items():
        for si, (own, par, full) in enumerate(S.scopes(f, files)):
            vis, opaque = S.vis_scope(f, si, files, parent, lib)
            body = re.sub(r'#!?\[[^\]]*\]', ' ', own)
            body = re.sub(r'(?:^|(?<=[;{}\s]))(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+;', lambda m: " " * len(m.group(0)), body)
            seen = set()
            for m in FIRST.finditer(body):
                n = m.group(1)
                if n in PRIMS or n in deps or n in al or n in mods or n in vis or n in seen: continue
                if opaque and n in S.crate_names_all(): continue
                seen.add(n)
                print(f"{rel(f)}: path root `{n}::` unresolved in {c['name']}{' (opaque)' if opaque else ''}")
