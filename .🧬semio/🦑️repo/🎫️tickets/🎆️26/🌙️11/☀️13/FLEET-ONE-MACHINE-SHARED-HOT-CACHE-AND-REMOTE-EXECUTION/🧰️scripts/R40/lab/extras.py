import collections,os,json,pickle
src=open('analyze3.py').read().split("rows=[];cat=")[0]
exec(src)
ownsrc={}
for o,fs in res.items():
    for f in fs:ownsrc.setdefault(f,[]).append(o)
ex=collections.Counter();n_over=0;tot=0;stats=collections.Counter()
for o,d in meta.items():
    r=d['root']
    if r not in byroot or r=='.':continue
    libs=[t['src'] for t in byroot[r]['targets'] if any(k in('lib','proc-macro','rlib','cdylib','dylib','staticlib') for k in t['kind'])]
    if not libs:continue
    f=os.path.normpath(libs[0])[len(R):]
    if f not in file2tok:continue
    m=stale(f);sharers=ownsrc.get(f,[o]);sroots={name2root[s] for s in sharers}
    orc=set()
    for sr in sroots:orc|=dep_b[sr]|{sr}
    got={name2root.get(x) for x in owners_of(m,'check')}-{'.'}
    extra=got-(orc-{'.'})
    tot+=1
    stats['exact-or-over' if (orc-{'.'})<=got else 'under']+=1
    if extra:
        n_over+=1
        for x in extra:ex[x]+=1
print('crates',tot,'with extra owners',n_over,stats)
for k,v in ex.most_common(8):print(v,k)
print('mean extra',sum(ex.values())/max(1,tot))
