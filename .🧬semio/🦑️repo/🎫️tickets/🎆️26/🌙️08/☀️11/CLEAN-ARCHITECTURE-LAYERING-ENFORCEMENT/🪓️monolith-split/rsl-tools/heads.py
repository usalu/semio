"""🔎️ Read-only unbound-head scan: lower-case path heads `x::` of a Rust file that are bound by no dependency key of the owner manifests, no `use`/`mod`/item definition of the same file and no glob import (globs are listed as possible). CAVEAT: the heads of the dependency libraries are their [lib] names (`protocol` is semio-framework-replication, `pack` is semio-framework-pack), not the dependency keys, so these two are reported unbound although they resolve. usage: heads.py <file.rs>... ; owner manifests from rsl-reach.json (regenerate with reach.ts when files are new) or --manifest <Cargo.toml>"""
import json, os, re, sys
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade_lib import dep_entries
args=sys.argv[1:]; mans=[]
if "--manifest" in args:
    i=args.index("--manifest"); mans=[args[i+1]]; args=args[:i]+args[i+2:]
reach=json.load(open(G+"/rsl-reach.json"))
PRIM={"std","core","alloc","crate","self","super","u8","u16","u32","u64","u128","usize","i8","i16","i32","i64","i128","isize","f32","f64","bool","char","str"}
def strip(t):
    t=re.sub(r'/\*.*?\*/',lambda m:" "*len(m.group(0)),t,flags=re.S)
    t=re.sub(r'(?m)//.*$','',t)
    t=re.sub(r'"(?:\\.|[^"\\])*"','""',t)
    return t
def deps(m):
    t=open(m,encoding="utf8").read()
    return {e["key"].replace("-","_") for e in dep_entries(t)}
for f in args:
    t=strip(open(f,encoding="utf8").read())
    own=mans or [o["manifest"] for o in reach.get(f,[])]
    keys=set()
    for m in own: keys|=deps(m)
    bound=set(PRIM)|keys
    for m in re.finditer(r'\buse\s+([^;]*);',t,re.S):
        body=m.group(1)
        for a in re.finditer(r'\bas\s+([A-Za-z_][A-Za-z0-9_]*)',body): bound.add(a.group(1))
        # leaf names and path heads of use groups
        for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body): bound.add(n) if n not in ("as","self","super","crate") else None
    for m in re.finditer(r'\b(?:mod|fn|struct|enum|type|const|static|trait|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)',t): bound.add(m.group(1))
    globs=re.findall(r'\buse\s+([^;{]*)::\*\s*;',t)
    heads={}
    for m in re.finditer(r'(?<![A-Za-z0-9_:.$#])([a-z_][a-z0-9_]*)::',t):
        h=m.group(1)
        if h in bound: continue
        heads[h]=heads.get(h,0)+1
    print(f[-80:],"owners",[m[-60:] for m in own][:2],"globs",len(globs))
    for h,n in sorted(heads.items(),key=lambda x:-x[1]): print("   UNBOUND",h,n)
