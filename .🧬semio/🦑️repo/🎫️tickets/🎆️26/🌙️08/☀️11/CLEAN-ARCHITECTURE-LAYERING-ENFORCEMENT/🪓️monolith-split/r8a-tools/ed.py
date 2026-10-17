import json,re,sys,collections
# usage: ed.py <log> <blocker-index> : slice table + SCC condensation of the chosen blocker
text=open(sys.argv[1],errors='ignore').read(); idx=int(sys.argv[2])
dec=json.JSONDecoder(); pos=0; objs=[]
while True:
    j=text.find('\n{\n',pos)
    if j<0:break
    obj,end=dec.raw_decode(text[j+1:]); pos=j+1+end; objs.append(obj)
b=objs[idx]
nodes={}
for line in b['groupReport']:
    m=re.match(r'^slice (.*?) \[(.*?)\] files=(\d+) bytes=(\d+) lines=(\d+) -> (.*)$',line)
    if not m: continue
    name=m.group(1); nodes[name]=dict(files=int(m.group(3)),bytes=int(m.group(4)),lines=int(m.group(5)),group=m.group(2),out=[])
    for t in m.group(6).split(' ; '):
        t=t.strip()
        if not t: continue
        mm=re.match(r'^(.*?) \((.*)\)$',t)
        tgt=mm.group(1) if mm else t
        why=mm.group(2) if mm else ''
        nodes[name]['out'].append((tgt,why))
print('slices',len(nodes))
json.dump({k:v for k,v in nodes.items()},open(sys.argv[1]+'.nodes.json','w'),ensure_ascii=False)
tot=sum(v['lines'] for v in nodes.values()); print('lines',tot)
for k,v in sorted(nodes.items(),key=lambda kv:-kv[1]['lines'])[:int(sys.argv[3]) if len(sys.argv)>3 else 30]:
    print(f"{v['lines']:6} {v['bytes']:8} {v['files']:3} {k[:110]}")
