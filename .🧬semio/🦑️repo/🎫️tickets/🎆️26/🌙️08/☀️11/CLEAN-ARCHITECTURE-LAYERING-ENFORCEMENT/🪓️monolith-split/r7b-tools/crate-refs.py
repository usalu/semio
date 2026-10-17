# 🔭️ Lists first segments of `crate::` paths (including `use crate::{..}` groups) per artifact file, skipping the graph-query package: usage `python3 crate-refs.py <artifact dir> [--files NAME]`.
import re,sys,os,collections
root=sys.argv[1]
want=sys.argv[sys.argv.index('--files')+1] if '--files' in sys.argv else None
cnt=collections.defaultdict(set)
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in('node_modules','🪆️graph-query')]
    for f in fs:
        if not f.endswith('.rs'): continue
        p=os.path.join(d,f); t=open(p,encoding='utf8').read()
        rel=os.path.relpath(p,root)
        for m in re.finditer(r'\bcrate::\{',t):
            i=m.end();depth=1;cur='';items=[]
            while depth:
                ch=t[i]
                if ch=='{':depth+=1
                if ch=='}':
                    depth-=1
                    if depth==0:break
                if ch==',' and depth==1: items.append(cur.strip());cur=''
                else: cur+=ch
                i+=1
            items.append(cur.strip())
            for it in items:
                mm=re.match(r'([A-Za-z_][A-Za-z0-9_]*)',it)
                if mm: cnt[mm.group(1)].add(rel)
        for m in re.finditer(r'\bcrate::([A-Za-z_][A-Za-z0-9_]*)',t):
            cnt[m.group(1)].add(rel)
if want:
    for f in sorted(cnt.get(want,[])): print(f)
else:
    for k,v in sorted(cnt.items(),key=lambda kv:-len(kv[1])): print(len(v),k)
