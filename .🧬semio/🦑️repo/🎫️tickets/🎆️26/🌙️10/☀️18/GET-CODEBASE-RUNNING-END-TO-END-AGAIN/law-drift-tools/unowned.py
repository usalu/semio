import re,os,json
owner="🧰️framework/🔨️modules/📡️replication"
corpus=json.load(open(owner+"/🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json"))
def mf(entry):
    seen=set();st=[entry]
    while st:
        f=st.pop()
        if f in seen: continue
        seen.add(f)
        t=open(f,encoding='utf-8').read()
        for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]|\binclude!\(\s*"([^"]*\.rs)"\s*\)',t):
            raw=m.group(1) or m.group(2)
            if raw: st.append(os.path.normpath(os.path.join(os.path.dirname(f),raw)))
    return seen
owned={}
for r in corpus['crates']+[dict(corpus['facade'])]:
    for f in mf(os.path.normpath(os.path.join(owner,r['entry']))): owned.setdefault(f,[]).append(r['package'])
for k,v in owned.items():
    if len(v)>1: print("MULTI",k,v)
for root,ds,fs in os.walk(owner):
    ds[:]=[d for d in ds if d not in('node_modules','dist')]
    for fn in fs:
        if fn.endswith('.rs'):
            p=os.path.normpath(os.path.join(root,fn)); rel=os.path.relpath(p,owner)
            if p in owned: continue
            if any(rel.startswith(x) for x in corpus['excluded']) or rel in corpus['unmounted']: continue
            print("UNOWNED",rel)
