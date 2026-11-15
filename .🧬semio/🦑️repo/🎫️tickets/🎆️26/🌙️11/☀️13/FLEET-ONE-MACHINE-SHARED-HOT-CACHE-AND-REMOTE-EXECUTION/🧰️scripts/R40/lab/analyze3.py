import json,pickle,os,collections,fnmatch,sys
R='/Users/Shared/semio/'
meta,keys,tf=pickle.load(open('keys.pkl','rb'))
own,helpers,res=pickle.load(open('own.pkl','rb'))
pk=json.load(open('oracle-meta.json'))
byroot={os.path.dirname(m)[len(R):]:v for m,v in pk.items()}
name2root={o:d['root'] for o,d in meta.items()}
for mp,v in pk.items():
    n=v['name']
    if n not in name2root:name2root[n]=os.path.dirname(mp)[len(R):]
root2owner={d['root']:o for o,d in meta.items()}
# oracle graph
def edges(root,kinds):
    out=set()
    for d in byroot[root]['deps']:
        if d['kind'] in kinds:
            p=d['path'][len(R):] if d['path'].startswith(R) else None
            if p in byroot:out.add(p)
    return out
nb={r:edges(r,(None,'build')) for r in byroot}
ndev={r:edges(r,('dev',)) for r in byroot}
clo={}
def closure_nb(r):
    if r in clo:return clo[r]
    clo[r]=set()  # cycle guard
    s=set()
    for x in nb[r]:s.add(x);s|=closure_nb(x)
    clo[r]=s;return s
for r in byroot:closure_nb(r)
dep_b=collections.defaultdict(set)   # c -> roots whose build closure contains c
dep_t=collections.defaultdict(set)   # c -> roots whose test closure contains c
for r in byroot:
    for c in clo[r]:dep_b[c].add(r)
    tset=set(clo[r])
    for x in ndev[r]:tset.add(x);tset|=clo[x]
    for c in tset:dep_t[c].add(r)
# key index
kl=list(keys);kidx={k:i for i,k in enumerate(kl)}
reach=collections.defaultdict(int)
import re
gm=collections.defaultdict(int)
for k,(a,b,c) in keys.items():
    bit=1<<kidx[k]
    for t in a:reach[t]|=bit
    for g in c:gm[g]|=bit
def g2re(g):
    out='';i=0
    while i<len(g):
        ch=g[i]
        if g[i:i+3]=='**/':out+='(?:.*/)?';i+=3;continue
        if g[i:i+2]=='**':out+='.*';i+=2;continue
        if ch=='*':out+='[^/]*'
        elif ch=='?':out+='[^/]'
        elif ch in '{}':out+='(' if ch=='{' else ')'
        elif ch==',':out+='|'
        else:out+=re.escape(ch)
        i+=1
    return re.compile('^'+out+'$')
globkeys=[]
for g,bit in gm.items():
    pre=re.split(r'[*?\[{]',g)[0]
    try:globkeys.append((pre,g2re(g),bit))
    except re.error:globkeys.append((pre,None,bit))
print('distinct globs',len(globkeys))
pathkeys=collections.defaultdict(int)
for k,(a,b,c) in keys.items():
    for p in b:pathkeys[p]|=1<<kidx[k]
file2tok=collections.defaultdict(list)
for t,fs in tf.items():
    for f in fs:file2tok[f].append(t)
def stale(f):
    m=0
    for t in file2tok.get(f,()):m|=reach.get(t,0)
    m|=pathkeys.get(f,0)
    for pre,rx,bit in globkeys:
        if f.startswith(pre) and (rx is None or rx.match(f)):m|=bit
    return m
maskt={t:0 for t in('build','check','test','lint')}
for k,i in kidx.items():maskt[k[1]]|=1<<i
def owners_of(m,t):
    m&=maskt[t];out=set();i=0
    while m:
        low=m&-m;i=low.bit_length()-1;out.add(kl[i][0]);m^=low
    return out

ownsrc={}   # file -> owners whose own nativeSources token contains it
for o,fs in res.items():
    for f in fs:ownsrc.setdefault(f,[]).append(o)
missing_t={t for k,(a,b,c) in keys.items() for t in a if t not in tf}
incomplete={k for k,(a,b,c) in keys.items() if a&missing_t}
inc_owner_t=collections.defaultdict(set)
for (o,t) in incomplete:inc_owner_t[t].add(name2root[o])
print('incomplete keys',len(incomplete),'of',len(keys),'missing tokens',len(missing_t))
rows=[];cat=collections.Counter();examples=collections.defaultdict(list)
skipped=collections.Counter()
for o,d in meta.items():
    r=d['root']
    if r not in byroot or r=='.':skipped['root']+=1;continue
    libs=[t['src'][len(R):] for t in byroot[r]['targets'] if any(k in('lib','proc-macro','rlib','cdylib','dylib','staticlib') for k in t['kind'])]
    if not libs:skipped['nolib']+=1;continue
    f=os.path.normpath(libs[0])
    if f not in file2tok:skipped['file-not-in-any-token']+=1;continue
    m=stale(f)
    sharers=ownsrc.get(f,[o])
    sroots={name2root[s] for s in sharers}
    for t,mk in(('build',dep_b),('check',dep_b),('test',dep_t)):
        orc=set()
        for sr in sroots:orc|=mk[sr]|{sr}
        got={name2root.get(x) for x in owners_of(m,t)}-{'.'}-inc_owner_t[t]
        orc-={'.'};orc-=inc_owner_t[t]
        if got==orc:c='exact'
        else:c=('under' if orc-got else '')+('over' if got-orc else '')
        cat[(t,c)]+=1
        if c!='exact' and len(examples[(t,c)])<4:examples[(t,c)].append((o,len(got),len(orc),[x[-40:] for x in list(orc-got)[:2]],[x[-40:] for x in list(got-orc)[:2]]))
    rows.append((o,r,f,len(owners_of(m,'check')),len(owners_of(m,'build')),len(owners_of(m,'test')),len(dep_b[r])))
print(skipped);print(sorted(cat.items()))
for k,v in examples.items():print(k,v)
pickle.dump(rows,open('rows1.pkl','wb'))

print('---- check under cases detail')
import time
GEN=os.path.getmtime(R+'.cache/b/g/m/generations/m/9be866174b9d192d9fa2bfe31d8ce19a49e3b3709c22a4d140680fb1e20890a2.json')
print('generation file mtime',time.strftime('%m-%d %H:%M',time.localtime(GEN)))
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
    miss=(orc-{'.'})-got
    miss={x for x in miss if x not in inc_owner_t['check']}
    for x in miss:
        xo=root2owner.get(x)
        mt=max(os.path.getmtime(R+x+'/Cargo.toml'),os.path.getmtime(R+r+'/Cargo.toml'))
        print(o[-50:],'->',x[-60:],'owner',xo,'edited after gen:',mt>GEN,'(cargo.toml mtime %s)'%time.strftime('%m-%d %H:%M',time.localtime(mt)))
