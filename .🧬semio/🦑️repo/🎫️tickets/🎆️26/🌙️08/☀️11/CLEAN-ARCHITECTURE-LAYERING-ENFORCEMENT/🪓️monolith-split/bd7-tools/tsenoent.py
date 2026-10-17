import re,os,sys
root="/Users/ueli/Documents/semio"
F=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
files=[l for l in open(F+"/🗑️generated/bd7-ts-tests.txt",encoding="utf-8").read().split("\n") if l]
pats=[
 re.compile(r'new URL\(\s*"([^"]+)"\s*,\s*import\.meta\.url\s*\)'),
 re.compile(r'(?:join|resolve)\(\s*import\.meta\.(?:dir|dirname)\s*,\s*((?:"[^"]+"\s*,?\s*)+)\)'),
]
out=[]
for f in files:
    p=os.path.join(root,f)
    try: t=open(p,encoding="utf-8").read()
    except Exception: continue
    d=os.path.dirname(p)
    cands=[]
    for m in pats[0].finditer(t): cands.append((m.group(1),m.start()))
    for m in pats[1].finditer(t):
        parts=re.findall(r'"([^"]+)"',m.group(1))
        cands.append((os.path.join(*parts),m.start()))
    for rel,pos in cands:
        if rel.startswith(("http:","https:","node:","data:","file:","/")): continue
        if not re.search(r'\.(json|ts|rs|md|toml|tsx|txt|semio|graphql|proto|sql|png|glb|js|mjs|cjs|css|html|yaml|yml|jsonc|wasm|svg)$',rel): continue
        tgt=os.path.normpath(os.path.join(d,rel))
        if not os.path.exists(tgt):
            line=t.count("\n",0,pos)+1
            out.append((f,line,rel,tgt.replace(root+"/","")))
for o in out: print("\t".join(str(x) for x in o))
