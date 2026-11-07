import re,os,sys
root=sys.argv[1]; lib=os.path.join(root,'📦️packages/🦀️.rs') if False else None
seen={}
def walk(f):
    f=os.path.normpath(f)
    if f in seen or not os.path.isfile(f): return
    t=open(f,encoding='utf8').read(); seen[f]=len(t.encode())
    d=os.path.dirname(f)
    for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)#\[path\s*=\s*"([^"]+)"\]((?:\s*#\[[^\]]*\])*)',t):
        if 'cfg(test)' in m.group(1)+m.group(3) or 'cfg(all(test' in m.group(1)+m.group(3): continue
        walk(os.path.join(d,m.group(2)))
    for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',t,re.M):
        for c in (os.path.join(d,m.group(1)+'.rs'),os.path.join(d,m.group(1),'mod.rs')): walk(c)
walk(sys.argv[2])
print(len(seen),sum(seen.values()))
for f,b in sorted(seen.items(),key=lambda x:-x[1])[:40]: print(b,f.split('editor/')[-1])
