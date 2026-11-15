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
jobs=[];meta_jobs={}
perc=[]
for p_c in commits:
    c=p_c[0];par=p_c[1] if len(p_c)>1 else None
    if par is None:continue
    names=[x for x in git('diff-tree','-r','--no-commit-id','--name-only','-z','--no-renames',par,c).decode().split('\0') if x]
    CH=set(names)
    rec={'commit':c[:10],'changed':len(CH)}
    changed_tokens=set()
    for f in CH:
        for t in file2tok.get(f,()):changed_tokens.add(t)
    own_hit=set()
    for t in changed_tokens:own_hit|=tok_owners_own.get(t,set())
    for f in CH:
        own_hit|=paths.get(f,set())
        for pre,rx,os_ in glist:
            if f.startswith(pre) and rx.match(f):own_hit|=os_
    before_dep=set()
    for t in changed_tokens:before_dep|=tok_owners_dep.get(t,set())
    before=own_hit|before_dep
    cand=collections.defaultdict(list)
    for f in CH:
        for o in fileowners.get(f,()):cand[o].append(f)
    iface_changed=set();pending=[]
    for o,chf in cand.items():
        if any(not f.endswith('.rs') for f in chf):iface_changed.add(o);continue
        pending.append(o)
    for o in pending:
        files=sorted(ofiles[o][0])
        jid=c[:10]+'|'+o
        jobs.append({'id':jid,'files':[{'name':f,'before':(None if f in CH and False else (par+':'+f)),'after':c+':'+f} for f in files]})
        meta_jobs[jid]=(c[:10],o)
    rec['own_hit']=len(own_hit);rec['before_check']=len(before);rec['cand_owners']=len(cand);rec['pending']=len(pending);rec['iface_changed_nonrs']=len(iface_changed)
    rec['_iface']=iface_changed
    perc.append(rec)
print('digest jobs',len(jobs))
json.dump(jobs,open('jobs.json','w'))
r=subprocess.run([BUN,'ifacejobs.ts','jobs.json','jobs.out'],capture_output=True,text=True)
print(r.returncode,r.stderr[:300])
res=json.load(open('jobs.out'))
tot_before=tot_after=0;rows=[]
for rec in perc:
    c=rec['commit'];iface=set(rec.pop('_iface'))
    for jid,(cc,o) in meta_jobs.items():
        pass
    rows.append(rec)
bycommit=collections.defaultdict(set)
for jid,(cc,o) in meta_jobs.items():
    if not res[jid]['same']:bycommit[cc].add(o)
out=[]
for rec,p_c in zip(perc,[x for x in commits if len(x)>1]):
    pass
# second pass to compute after (needs iface sets) -> recompute cheaply
perc2=[]
for p_c in commits:
    c=p_c[0];par=p_c[1] if len(p_c)>1 else None
    if par is None:continue
    names=[x for x in git('diff-tree','-r','--no-commit-id','--name-only','-z','--no-renames',par,c).decode().split('\0') if x]
    CH=set(names)
    changed_tokens={t for f in CH for t in file2tok.get(f,())}
    own_hit=set()
    for t in changed_tokens:own_hit|=tok_owners_own.get(t,set())
    for f in CH:
        own_hit|=paths.get(f,set())
        for pre,rx,os_ in glist:
            if f.startswith(pre) and rx.match(f):own_hit|=os_
    before_dep=set()
    for t in changed_tokens:before_dep|=tok_owners_dep.get(t,set())
    cand=collections.defaultdict(list)
    for f in CH:
        for o in fileowners.get(f,()):cand[o].append(f)
    ic={o for o,chf in cand.items() if any(not f.endswith('.rs') for f in chf)}|bycommit[c[:10]]
    after_dep=set()
    for d in ic:after_dep|=dep_rev.get(d,set())
    before=own_hit|before_dep;after=own_hit|after_dep
    bodyonly=len(cand)-len(ic)
    perc2.append({'commit':c[:10],'files':len(CH),'crates_with_changed_files':len(cand),'body_only_crates':bodyonly,'iface_changed_crates':len(ic),'check_before':len(before),'check_after':len(after)})
json.dump(perc2,open('replay.json','w'))
sb=sum(r['check_before'] for r in perc2);sa=sum(r['check_after'] for r in perc2)
print('sum check tasks re-keyed before',sb,'after',sa,'ratio %.3f'%(sa/sb if sb else 0))
for r in perc2:print(r)
