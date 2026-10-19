import json,os,re,sys
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"
d=json.load(open(C+sys.argv[1]))
def nearest(f):
    p=os.path.dirname(os.path.join(ROOT,f))
    while p.startswith(ROOT) and p!=ROOT:
        if os.path.exists(p+"/package.json"): return p
        p=os.path.dirname(p)
    return ROOT if os.path.exists(ROOT+"/package.json") else None
def declared(pj,name):
    try: j=json.load(open(pj+"/package.json"))
    except Exception as e: return "?"
    r=[]
    for k in ("dependencies","devDependencies","peerDependencies","optionalDependencies"):
        if name in j.get(k,{}): r.append(k)
    return ",".join(r)
rows=[]
for s,fs in sorted(d["bad"].items()):
    if not re.match(r"^(@[a-z0-9._-]+/)?[a-z0-9._-]+(/[A-Za-z0-9._/-]+)?$",s): continue
    if s[0].isupper() or s in ("exit","now","poll","probe","succeeds","x","external","dep/package.json","profile-fixture","vscode"): continue
    name="/".join(s.split("/")[:2 if s.startswith("@") else 1])
    owners={}
    for f in fs:
        pj=nearest(f); owners.setdefault(pj,[]).append(f)
    for pj,ff in owners.items():
        rows.append((name,s,os.path.relpath(pj,ROOT) if pj else None,declared(pj,name) if pj else "-",len(ff),ff[0]))
for r in rows: print(" | ".join(map(str,r)))
