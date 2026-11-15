import json,pickle,subprocess,os,re,collections,sys
R='/Users/Shared/semio/'
BUN='/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun'
meta,keys,tf=pickle.load(open('keys.pkl','rb'))
ns_tok,chk_dep,chk_own=pickle.load(open('keys2.pkl','rb'))
def git(*a):return subprocess.run(['git','--no-optional-locks','-c','core.quotepath=false','-C',R[:-1]]+list(a),capture_output=True).stdout
commits=[l.split() for l in git('log','--since=14 days ago','--format=%H %P').decode().splitlines()][::-1]
print('commits',len(commits))
file2tok=collections.defaultdict(list)
for t,fs in tf.items():
    for f in fs:file2tok[f].append(t)
ofiles={}
for o,toks in ns_tok.items():
    fs=set();ok=True
    for t in toks:
        if t in tf:fs.update(tf[t])
        else:ok=False
    ofiles[o]=(fs,ok)
print('owners with complete ns files',sum(1 for v in ofiles.values() if v[1]),'of',len(ofiles))
fileowners=collections.defaultdict(list)
for o,(fs,ok) in ofiles.items():
    for f in fs:fileowners[f].append(o)
globs=collections.defaultdict(set);paths=collections.defaultdict(set)
for o in chk_own:
    a,b,c=chk_own[o]
    for g in c:globs[g].add(o)
    for p in b:paths[p].add(o)
def g2re(g):
    out='';i=0
    while i<len(g):
        ch=g[i]
        if g[i:i+3]=='**/':out+='(?:.*/)?';i+=3;continue
        if g[i:i+2]=='**':out+='.*';i+=2;continue
        if ch=='*':out+='[^/]*'
        elif ch=='?':out+='[^/]'
        elif ch=='{':out+='('
        elif ch=='}':out+=')'
        elif ch==',':out+='|'
        else:out+=re.escape(ch)
        i+=1
    return re.compile('^'+out+'$')
glist=[(re.split(r'[*?\[{]',g)[0],g2re(g),os_) for g,os_ in globs.items()]
tok_owners_own=collections.defaultdict(set)   # token -> owners whose check OWN part has it
for o,(a,b,c) in chk_own.items():
    for t in a:tok_owners_own[t].add(o)
tok_owners_dep=collections.defaultdict(set)   # for cross-check: token -> owners reaching via deps
for o,deps in chk_dep.items():
    for d in deps:
        for t in ns_tok.get(d,()):tok_owners_dep[t].add(o)
dep_rev=collections.defaultdict(set)   # D -> owners with D in dep list
for o,deps in chk_dep.items():
    for d in deps:dep_rev[d].add(o)

res=json.load(open('jobs.out'))
cnt=collections.Counter(len(v) for v in fileowners.values())
HELP={f for f,v in fileowners.items() if len(v)>=1000}
print('helper files',len(HELP))
meta_jobs={}
perc=[]
tot=collections.Counter()
for p_c in commits:
    c=p_c[0];par=p_c[1] if len(p_c)>1 else None
    if par is None:continue
    names=[x for x in git('diff-tree','-r','--no-commit-id','--name-only','-z','--no-renames',par,c).decode().split('\0') if x]
    CH=set(names)
    CHR={f for f in CH if f in fileowners and f not in HELP}
    cand=collections.defaultdict(list)
    for f in CHR:
        for o in fileowners[f]:cand[o].append(f)
    reason=collections.Counter();ic=set()
    for o,chf in cand.items():
        if any(not f.endswith('.rs') for f in chf):ic.add(o);reason['manifest-or-asset']+=1;continue
        jid=c[:10]+'|'+o
        r=res.get(jid)
        if r is None:
            # job not in file (should not happen): conservative
            ic.add(o);reason['no-digest']+=1;continue
        if r['same']:reason['body-only']+=1
        else:ic.add(o);reason['rs-interface']+=1
    toks={t for f in CHR for t in file2tok.get(f,())}
    own=set();dep=set()
    for t in toks:own|=tok_owners_own.get(t,set());dep|=tok_owners_dep.get(t,set())
    after_dep=set()
    for d in ic:after_dep|=dep_rev.get(d,set())
    before=own|dep;after=own|after_dep
    sb=sa=0
    for o in cand:
        ow=set()
        for t in ns_tok.get(o,()):ow|=tok_owners_own.get(t,set())
        d=dep_rev.get(o,set())
        sb+=len(ow|d);sa+=len(ow|(d if o in ic else set()))
    tot['per_crate_before']+=sb;tot['per_crate_after']+=sa
    row={'per_crate_before':sb,'per_crate_after':sa,'commit':c[:10],'rust_files':len(CHR),'crates':len(cand),**dict(reason),'check_before':len(before),'check_after':len(after)}
    perc.append(row)
    for k in('rust_files','crates','body-only','rs-interface','manifest-or-asset','check_before','check_after'):tot[k]+=row.get(k,0)
json.dump(perc,open('replay2.json','w'))
print(dict(tot))
print('crate edits: body-only %.1f%%'%(100*tot['body-only']/max(1,tot['crates'])),'rs-interface %.1f%%'%(100*tot['rs-interface']/max(1,tot['crates'])),'manifest-or-asset %.1f%%'%(100*tot['manifest-or-asset']/max(1,tot['crates'])))
print('per-crate-edit owner-stales (check): before',tot['per_crate_before'],'after',tot['per_crate_after'],'ratio %.3f'%(tot['per_crate_after']/max(1,tot['per_crate_before'])))
print('Rust-source owner-stales (check): before',tot['check_before'],'after',tot['check_after'],'ratio %.3f'%(tot['check_after']/max(1,tot['check_before'])))
for r in perc:print(r)
