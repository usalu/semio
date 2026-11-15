import json,pickle,collections,re
pairs=pickle.load(open('pairs.pkl','rb'))
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
GEN=re.compile(r'(\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/])[0-9a-f]{16,64}(?=[\\/]|$)')
def nrm(s,win):
    if not isinstance(s,str):return s
    s=GEN.sub(r'\1*',s)
    if win:
        s=s.replace('⟦semio-root-json⟧','⟦semio-root⟧').replace('⟦semio-root-slash⟧','⟦semio-root⟧')
        s=re.sub(r'\\\\+|\\','/',s)
    return s
def nf(f,win):
    if isinstance(f,dict):return {k:nf(v,win) for k,v in f.items() if k not in VOL}
    if isinstance(f,list):return [nf(x,win) for x in f]
    return nrm(f,win)
def rows(r,win):
    d={}
    for o in r['rec']['observations']:
        if o['kind'] in('closure','stamp'):continue
        d[(o['kind'],nrm(o['path'],win))]=json.dumps(nf(o.get('fact'),win),sort_keys=True,ensure_ascii=False)
    return d
tot=collections.Counter();ex={}
for w,p in pairs:
    rw,rp=rows(w,True),rows(p,False)
    fw={k:v for k,v in rw.items() if k[0]=='file'};fp={k:v for k,v in rp.items() if k[0]=='file'}
    shaw={k:json.loads(v).get('sha256') for k,v in fw.items()};shap={k:json.loads(v).get('sha256') for k,v in fp.items()}
    if shaw!=shap: tot['tree differs']+=1;continue
    tot['same tree']+=1
    for k in set(rw)|set(rp):
        if k not in rw: kind='onlyP';tot[('onlyP',k[0])]+=1;ex.setdefault(('onlyP',k[0]),(k,rp[k],w['name']))
        elif k not in rp: tot[('onlyW',k[0])]+=1;ex.setdefault(('onlyW',k[0]),(k,rw[k],w['name']))
        elif rw[k]!=rp[k]: tot[('diff',k[0])]+=1;ex.setdefault(('diff',k[0]),(k,rw[k][:300],rp[k][:300],w['name']))
print(tot)
for k,v in ex.items():print(k,str(v)[:700])
