import json,re,collections,sys
S='/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad'
idx=json.load(open(S+'/idx.json'))
root=json.load(open('📋️project.json'))['targets']
dom=collections.defaultdict(list)
for t,l in idx.items():
    for n,f,c in l:
        if n=='workspace': continue
        m=re.search(r'📜️script\.ts verify (\S+)',c)
        if m: dom[m.group(1)].append((n,t,f,c))
plan=[]
for t,v in root.items():
    c=(v.get('options') or {}).get('command','')
    m=re.match(r'repo \./📜️script\.ts verify (\S+)(.*)',c)
    if not m or not dom.get(m.group(1)): continue
    name,args=m.group(1),m.group(2).split()
    cands=dom[name]
    base=[x for x in cands if x[3].endswith('verify '+name)]
    plan.append((t,name,args,[(x[0],x[1]) for x in cands]))
if __name__=='__main__':
    multi=[p for p in plan if len({x[0] for x in p[3]})>1]
    print(len(plan),len(multi))
    for p in multi: print(p)
    ex=[(p[0],p[1],p[2],p[3]) for p in plan if p[2]==[] ][:0]
    # which plan entries have no base target in domain whose command is exactly verify name
    nb=[p for p in plan if not any(True for x in p[3])]
    names={}
    for p in plan: names.setdefault(p[1],set()).update(x[0] for x in p[3])
    print(len(names))
