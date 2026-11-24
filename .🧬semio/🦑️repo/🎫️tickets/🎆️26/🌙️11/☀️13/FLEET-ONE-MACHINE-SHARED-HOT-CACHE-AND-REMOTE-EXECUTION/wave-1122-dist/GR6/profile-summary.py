import json,sys,collections
p=json.load(open(sys.argv[1]));nodes={n['id']:n for n in p['nodes']}
dt=p['timeDeltas'];self_=collections.Counter()
for s,d in zip(p['samples'],dt):self_[s]+=d
tot=sum(self_.values());parent={}
for n in p['nodes']:
  for c in n.get('children',[]):parent[c]=n['id']
bf=collections.Counter();incl=collections.Counter()
for nid,t in self_.items():
  cf=nodes[nid]['callFrame'];bf[(cf['functionName'] or '(anon)')+' '+cf['url'].split('/')[-1][:40]+':'+str(cf['lineNumber'])]+=t
  seen=set();x=nid
  while x:
    cf=nodes[x]['callFrame'];k=(cf['functionName'] or '(anon)')+' '+cf['url'].split('/')[-1][:40]+':'+str(cf['lineNumber'])
    if k not in seen:seen.add(k);incl[k]+=t
    x=parent.get(x)
print('total s',tot/1e6)
print('--self');[print(round(t/1e6,1),k) for k,t in bf.most_common(22)]
print('--incl');[print(round(t/1e6,1),k) for k,t in incl.most_common(45)]
