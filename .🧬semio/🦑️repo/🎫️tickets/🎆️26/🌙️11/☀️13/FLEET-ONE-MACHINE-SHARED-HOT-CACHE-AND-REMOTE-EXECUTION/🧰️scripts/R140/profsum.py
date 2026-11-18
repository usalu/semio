import json,sys,collections
p=json.load(open(sys.argv[1]));nodes={n['id']:n for n in p['nodes']}
dt=p['timeDeltas'];samples=p['samples']
self_t=collections.Counter()
for s,d in zip(samples,dt):self_t[s]+=d
parent={}
for n in p['nodes']:
  for c in n.get('children',[]):parent[c]=n['id']
def name(i):
  f=nodes[i]['callFrame'];u=f['url'].split('/')[-1] if f['url'] else ''
  return f"{f['functionName'] or '(anon)'} {u}:{f['lineNumber']+1}"
selfby=collections.Counter();incl=collections.Counter()
for i,t in self_t.items():
  selfby[name(i)]+=t
  seen=set();j=i
  while j is not None:
    nm=name(j)
    if nm not in seen:incl[nm]+=t;seen.add(nm)
    j=parent.get(j)
tot=sum(self_t.values())
print("total ms",tot/1000)
print("--- self");[print(f"{t/1000:9.0f} {n}") for n,t in selfby.most_common(int(sys.argv[2]) if len(sys.argv)>2 else 25)]
print("--- inclusive");[print(f"{t/1000:9.0f} {n}") for n,t in incl.most_common(int(sys.argv[3]) if len(sys.argv)>3 else 45)]
