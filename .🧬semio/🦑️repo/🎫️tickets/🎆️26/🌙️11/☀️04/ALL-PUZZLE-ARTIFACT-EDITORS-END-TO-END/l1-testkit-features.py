import os,re,sys
root=sys.argv[1]
members=re.findall(r'"([^"]+)"',open(os.path.join(root,'Cargo.toml'),encoding='utf8').read().split('members = [')[1].split(']')[0])
dirs={os.path.realpath(os.path.join(root,m)):m for m in members}
info={}
for d in dirs:
    p=os.path.join(d,'Cargo.toml');c=open(p,encoding='utf8').read()
    name=re.search(r'^name\s*=\s*"([^"]+)"',c,re.M).group(1)
    info[d]=(name,c)
def section(c,name):
    m=re.search(r'^\['+re.escape(name)+r'\]\n(.*?)(?=^\[|\Z)',c,re.M|re.S)
    return m
changed=0
for d,(name,c) in info.items():
    dm=section(c,'dependencies')
    deps=[]
    if dm:
        for line in dm.group(1).split('\n'):
            m=re.match(r'^([A-Za-z0-9_-]+)\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"',line)
            if m:
                t=os.path.realpath(os.path.join(d,m.group(2)))
                if t in dirs and t!=d: deps.append((m.group(1),t,line))
    feats=[f'{n}/testkit' for n,t,l in deps]
    if not deps: continue
    new=c
    fm=section(new,'features')
    if fm:
        tl=re.search(r'^testkit\s*=\s*\[(.*?)\]\s*$',fm.group(1),re.M)
        if tl:
            have=re.findall(r'"([^"]+)"',tl.group(1))
            add=[f for f in feats if f not in have]
            if add:
                line='testkit = ['+', '.join('"%s"'%x for x in have+add)+']'
                new=new.replace(tl.group(0),line,1)
        else:
            new=new.replace(fm.group(0),fm.group(0).rstrip('\n')+'\ntestkit = ['+', '.join('"%s"'%x for x in feats)+']\n\n',1)
    else:
        dmm=section(new,'dependencies')
        new=new.replace('[dependencies]\n','[features]\ntestkit = ['+', '.join('"%s"'%x for x in feats)+']\n\n[dependencies]\n',1)
    dv=section(new,'dev-dependencies')
    devlines=dv.group(1) if dv else ''
    adds=''
    for n,t,l in deps:
        if re.search(r'^'+re.escape(n)+r'\s*=\s*\{[^}]*testkit',devlines,re.M): continue
        m=re.match(r'^[A-Za-z0-9_-]+\s*=\s*\{\s*(.*?)\s*\}\s*$',l)
        body=m.group(1)
        adds+=f'{n} = {{ {body}, features = ["testkit"] }}\n' if 'features' not in body else ''
    if adds:
        if dv: new=new.replace(dv.group(0),'[dev-dependencies]\n'+dv.group(1).rstrip('\n')+'\n'+adds+'\n',1)
        else: new=new.rstrip('\n')+'\n\n[dev-dependencies]\n'+adds
    if new!=c:
        open(os.path.join(d,'Cargo.toml'),'w',encoding='utf8').write(new);changed+=1
print('changed',changed)
