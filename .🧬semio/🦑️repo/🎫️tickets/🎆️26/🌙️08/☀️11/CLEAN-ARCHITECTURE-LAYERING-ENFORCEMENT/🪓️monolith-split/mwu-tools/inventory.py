import os,re,sys,json
R='/Users/ueli/Documents/semio'
files=[l.rstrip('\n') for l in open('/Users/ueli/.cache/mwu/mirror-tracked.txt') if l.rstrip('\n').endswith('/🪞️mirror/🦀️.rs')]
rows=[]
for f in files:
    p=os.path.join(R,f)
    if not os.path.exists(p): continue
    d=os.path.dirname(p)
    while d!=R and not os.path.exists(os.path.join(d,'Cargo.toml')): d=os.path.dirname(d)
    name=None
    for l in open(os.path.join(d,'Cargo.toml'),encoding='utf8'):
        m=re.match(r'name\s*=\s*"([^"]+)"',l)
        if m: name=m.group(1);break
    t=open(p,encoding='utf8').read()
    lines=t.split('\n')
    pu=[l.strip() for l in lines if re.match(r'\s*(pub(\([^)]*\))?\s+)?use\s',l)]
    ext=[l for l in pu if re.search(r'use\s+semio_',l)]
    pm=len([l for l in lines if '#[path' in l])
    rows.append(dict(file=f,pkg=os.path.relpath(d,R),crate=name,lines=len(lines),uses=len(pu),ext=len(ext),paths=pm))
json.dump(rows,open('/Users/ueli/.cache/mwu/inventory.json','w'),ensure_ascii=False,indent=0)
print(len(rows),sum(r['ext'] for r in rows),sum(r['uses'] for r in rows))
import collections
c=collections.Counter()
for r in rows:
    c['ext0' if r['ext']==0 else 'ext+']+=1
print(c)
for r in sorted(rows,key=lambda r:-r['ext'])[:15]: print(r['crate'],r['lines'],r['ext'],r['paths'])
