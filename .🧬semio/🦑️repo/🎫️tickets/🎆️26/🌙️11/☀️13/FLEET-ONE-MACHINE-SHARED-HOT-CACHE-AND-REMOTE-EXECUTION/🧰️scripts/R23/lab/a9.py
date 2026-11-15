import json,pickle,collections,re
pairs=pickle.load(open('pairs.pkl','rb'))
VOL={'mtimeMs','ctimeMs','ino','atimeMs','birthtimeMs','dev','nlink','uid','gid','blocks','blksize','mode'}
GEN=re.compile(r'(\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/])([0-9a-f]{16,64}|\*|⟦generation⟧)(?=[\\/]|$)')
OWN=re.compile(r'(build:[a-z-]+:v1:)[0-9a-f]{64}')
def nrm(s,win):
    if not isinstance(s,str):return s
    s=GEN.sub(r'\1G',s); s=OWN.sub(r'\1O',s)
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
        if o['kind']=='lstat' and o.get('fact')=='directory' and not o['path'].startswith('⟦semio-root'):continue
        if o['kind']=='resolve': continue
        d[(o['kind'],nrm(o['path'],win))]=json.dumps(nf(o.get('fact'),win),sort_keys=True,ensure_ascii=False)
    return d
c=collections.Counter();good=[]
for w,p in pairs:
    rw,rp=rows(w,True),rows(p,False)
    iw=json.loads(w['rec']['image'].replace('⟦semio-root-slash⟧','⟦semio-root⟧'));ip=json.loads(p['rec']['image'])
    for d in (iw,ip):d.pop('closureRecipes',None)
    def strip(v):
        if isinstance(v,dict):
            if list(v)==['closureRef']:return 'REF'
            return {k:strip(x) for k,x in v.items()}
        if isinstance(v,list):return [strip(x) for x in v]
        return v
    same_img=strip(iw)==strip(ip)
    same_rows=rw==rp
    c[(same_img,same_rows)]+=1
    if same_img and same_rows: good.append((len(w['rec']['observations'])+len(p['rec']['observations']),w['name'],w['platform'],p['platform'],w['t'],p['t']))
print(c);good.sort();print(good[:15])
