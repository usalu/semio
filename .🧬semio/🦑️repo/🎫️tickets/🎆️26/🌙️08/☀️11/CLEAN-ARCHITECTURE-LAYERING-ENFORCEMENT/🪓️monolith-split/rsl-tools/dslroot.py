"""🧬 F-11(a) scan: every bare `dsl::` path (derive attribute or expression) in a Rust file of the repo must have a binding for `dsl`: a dependency key `dsl` in an owner manifest, a `use ... as dsl` / `extern crate ... as dsl` / `use ...::dsl;` / `mod dsl` in the file or in the owner's crate-root files. Read-only. usage: dslroot.py [path-prefix]"""
import json, os, re, subprocess, sys, collections
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
reach=json.load(open(G+"/rsl-reach.json"))
prefix=sys.argv[1] if len(sys.argv)>1 else ""
BARE=re.compile(r'(?<![A-Za-z0-9_:])dsl::([A-Za-z_][A-Za-z0-9_]*)')
BIND=re.compile(r'(?:use\s[^;]*\bas\s+dsl\s*;|extern\s+crate\s+[^;]*\bas\s+dsl\s*;|use\s+[A-Za-z0-9_:{} ,\n]*\bdsl\s*[;,}]|\bmod\s+dsl\b|\bpub(?:\([a-z]+\))?\s+use\s[^;]*\bas\s+dsl\s*;)')
mcache={}
def manifest_keys(m):
    if m not in mcache:
        t=open(m,encoding="utf8").read()
        mcache[m]=bool(re.search(r'^\s*dsl\s*(=|\.)',t,re.M))
    return mcache[m]
def strip(t): return re.sub(r'(?m)^\s*//.*$','',t)
files=[f for f in reach if f.startswith(prefix) and f.endswith(".rs") and not f.startswith(".🧬semio")]
roots=collections.defaultdict(list)
for f,v in reach.items():
    for o in v: roots[(o["manifest"],o["lib"])].append(f)
bad=[];total=0;seen=0
for f in files:
    try: t=strip(open(f,encoding="utf8").read())
    except Exception: continue
    ms=list(BARE.finditer(t))
    if not ms: continue
    seen+=1; total+=len(ms)
    if BIND.search(t): continue
    for o in reach[f]:
        if manifest_keys(o["manifest"]): continue
        rootfile=o["root"]
        try: rt=strip(open(rootfile,encoding="utf8").read())
        except Exception: rt=""
        if BIND.search(rt): continue
        bad.append((f,o["manifest"],o["lib"],sorted({m.group(1) for m in ms})))
print("files with bare dsl::",seen,"uses",total,"unbound (file,owner) pairs",len(bad))
for b in bad[:60]: print(b[0][-100:],"|",b[2],"|",b[3][:4])
