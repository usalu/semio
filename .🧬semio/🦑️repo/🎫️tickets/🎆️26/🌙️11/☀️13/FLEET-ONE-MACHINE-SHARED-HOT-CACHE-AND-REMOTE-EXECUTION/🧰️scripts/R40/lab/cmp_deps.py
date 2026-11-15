import json,collections,os,pickle,sys
R='/Users/Shared/semio/'
pk=json.load(open('oracle-meta.json'))
byroot={os.path.dirname(m)[len(R):]:v for m,v in pk.items()}
# oracle direct edges by kind
def edges(root,kinds):
    out=set()
    for d in byroot[root]['deps']:
        k=d['kind']
        if k in kinds:
            p=d['path'][len(R):] if d['path'].startswith(R) else None
            if p in byroot:out.add(p)
            else:out.add('?'+d['path'])
    return out
cl={}
def closure(root,kinds_first,kinds_rest=(None,'build')):
    seen=set();st=list(edges(root,kinds_first))
    while st:
        x=st.pop()
        if x in seen or x.startswith('?'):
            seen.add(x);continue
        seen.add(x)
        st+=list(edges(x,kinds_rest))
    seen.discard(root)
    return seen
owners={}
projroot={}
for line in open('model.jsonl'):
    d=json.loads(line);owners[d['name']]=d;projroot[d['name']]=d['root']
byname={}
for mp,v in pk.items():byname.setdefault(v['name'],[]).append(os.path.dirname(mp)[len(R):])
for n_,r_ in byname.items():
    if n_ not in projroot and len(r_)==1:projroot[n_]=r_[0]
print('owners',len(owners),'distinct roots',len(set(projroot.values())))
print('oracle roots missing from records',len(set(byroot)-set(projroot.values())),'records missing in oracle',len(set(projroot.values())-set(byroot)))
res=collections.Counter();ex=collections.defaultdict(list)
for n,d in owners.items():
    r=d['root']
    if r not in byroot:res['no-oracle']+=1;continue
    for t in('build','check','test'):
        ins=d['targets'].get(t)
        if ins is None:continue
        projs=set()
        for i in ins:
            if isinstance(i,dict) and i.get('input')=='nativeSources' and 'projects' in i:projs|=set(i['projects'])
        rec={projroot[p] for p in projs if p in projroot}
        unk=[p for p in projs if p not in projroot]
        orc=closure(r,(None,'build','dev') if t=='test' else (None,'build'))
        if t=='test':orc=closure(r,(None,'build','dev'))
        if rec==orc and not unk:res[t+':equal']+=1
        else:
            missing=orc-rec;extra=rec-orc
            key=t+(':under' if missing else '')+(':over' if extra else '')
            res[key]+=1
            if len(ex[key])<4:ex[key].append((n,len(rec),len(orc),list(missing)[:2],list(extra)[:2],unk[:2]))
print(res)
for k,v in ex.items():print(k,v)
