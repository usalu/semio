import json,re,sys,os
ROOT="/Users/ueli/Documents/semio"
nx=json.load(open(ROOT+"/nx.json"))["namedInputs"]
LIT={"Cargo.toml","Cargo.lock","rust-toolchain.toml",".cargo/config.toml"}
TAILS={"Cargo.toml","Cargo.lock","*.rs","🧬️schema/**/*"}
def bare(p): return re.sub(r'^\./','',re.sub(r'^!?\{(?:workspaceRoot|projectRoot)\}/','',p))
def braces(p):
    m=re.search(r'\{([^{}]*)\}',p)
    if not m: return [p]
    return [x for c in m.group(1).split(',') for x in braces(p[:m.start()]+c+p[m.end():])]
def viol(leaf):
    if not isinstance(leaf,str) or leaf.startswith('!') or not leaf.startswith('{workspaceRoot}/'): return None
    b=bare(leaf)
    if b in LIT: return b
    for a in braces(b):
        for t in TAILS:
            if a=='**/'+t: return a
    return None
WORD=re.compile(r'^[A-Za-z][A-Za-z0-9_-]*$')
def expand(inputs,names,seen=()):
    out=[]
    for i in inputs:
        if not isinstance(i,str): out.append(i); continue
        w=i[1:] if i.startswith('^') else i
        if not WORD.match(w): out.append(i); continue
        if i.startswith('^') or w in seen or w not in names: continue
        out+=expand(names[w],names,seen+(w,))
    return out
rows=[]
for m in open('/private/tmp/gk/manifests.txt').read().split('\n'):
    if not m: continue
    try: j=json.load(open(ROOT+'/'+m))
    except Exception as e: continue
    names={**nx,**(j.get('namedInputs') or {})}
    for name,t in (j.get('targets') or {}).items():
        ins=t.get('inputs')
        if ins is None: ins=['default','^default']
        leaves=expand(ins,names)
        v=sorted({viol(l) for l in leaves if viol(l)})
        if v: rows.append((j.get('name',m),name,t.get('cache'),v))
    # named inputs defined with violations (even if no target references)
    for n,vals in (j.get('namedInputs') or {}).items():
        v=sorted({viol(l) for l in vals if viol(l)})
        if v: rows.append((j.get('name',m),'<named:%s>'%n,None,v))
json.dump(rows,open('/private/tmp/gk/census_authored.json','w'),ensure_ascii=False,indent=1)
print(len(rows))
from collections import Counter
c=Counter(r[0] for r in rows)
for k,v in c.most_common(60): print(v,k)
