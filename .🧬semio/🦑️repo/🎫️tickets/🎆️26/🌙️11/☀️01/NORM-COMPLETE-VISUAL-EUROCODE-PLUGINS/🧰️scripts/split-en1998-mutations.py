import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from crate_split import *
A='/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998'
M=A+'/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations'
MAXLEAVES=7
MOUNT=re.compile(r'#\[path = "\.\./([^/]+)/🦀️\.rs"\]\npub mod (\w+);\n')
def slug(d):return re.sub(r'^[^a-z]+','',d)
def entity(d):
    t=slug(d).split('-')
    if t[0]=='retaining':return 'retaining-wall'
    if len(t)>2 and t[1]=='retaining':return 'retaining-wall'
    return t[1] if len(t)>1 else t[0]
def run(crate_dir,suffix,emoji):
    root_f=M+'/'+crate_dir+'/🦀️.rs';root=open(root_f).read()
    ms=MOUNT.findall(root)
    if len(ms)<=MAXLEAVES:return
    names={m:d for d,m in ms}
    refs={m:set(re.findall(r'crate::(\w+)',''.join(open(os.path.join(M,d,p)).read() for p in ['🦀️.rs','🔺️diff/🦀️.rs','↩️inverse/🦀️.rs'] if os.path.exists(os.path.join(M,d,p)))))&set(names)-{m} for d,m in ms}
    par={m:m for _,m in ms}
    def f(x):
        while par[x]!=x:x=par[x]
        return x
    ent={}
    for d,m in ms:ent.setdefault(entity(d),[]).append(m)
    for e,l in ent.items():
        for x in l[1:]:par[f(x)]=f(l[0])
    for m,r in refs.items():
        for x in r:par[f(x)]=f(m)
    groups={}
    for d,m in ms:groups.setdefault(f(m),[]).append((d,m,entity(d)))
    gl=sorted(groups.values(),key=lambda g:-len(g))
    bins=[]
    for g in gl:
        chunks=[g[i:i+MAXLEAVES] for i in range(0,len(g),MAXLEAVES)] if len(g)>MAXLEAVES else [g]
        for ch in chunks:
            for b in bins:
                if len(b)+len(ch)<=MAXLEAVES:b.extend(ch);break
            else:bins.append(list(ch))
    keep=max(bins,key=lambda b:sum(1 for x in b if x[2]==suffix.rstrip('s')))
    bins.remove(keep)
    base=M+'/'+crate_dir+'/📦️packages/🦀️rust/Cargo.toml'
    pre=root[root.index('#![allow'):root.index('//#region 🪢️Mounts')]
    base_name=re.search(r'name = "([^"]+)"',open(base).read()).group(1)
    moved=[];used=set()
    for b in bins:
        ents=sorted({x[2] for x in b})
        key='-'.join(ents[:2])+('' if len(ents)<=2 else f'-{len(ents)}')
        k=key;i=2
        while k in used:k=f'{key}-{i}';i+=1
        used.add(k)
        cname=f'{base_name}-{k}'
        cdir=M+'/'+f'{emoji}{crate_dir[len(emoji):] if False else ""}'
        d=f'{emoji}{slug(crate_dir)}-{k}'
        body=f'//! {emoji} EN 1998 {suffix} mutation leaves: {", ".join(ents)}.\n\n'+pre+'//#region 🪢️Mounts\n'+''.join(f'#[path = "../{x[0]}/🦀️.rs"]\npub mod {x[1]};\n' for x in b)+'//#endregion 🪢️Mounts\n'
        pkg=mk_crate(A,M+'/'+d,cname,f'EN 1998 {suffix} mutation leaves: {", ".join(ents)}',body,base,drop_deps=tuple(n for n in re.findall(r'^(semio-s-artifact-norm-en1998-mut-[\w-]+) =',open(base).read(),re.M)))
        register(A,pkg,cname)
        add_dep(base,cname)
        moved.append((cname,[x[1] for x in b]))
    for d,m in [(x[0],x[1]) for b in bins for x in b]:
        root=root.replace(f'#[path = "../{d}/🦀️.rs"]\npub mod {m};\n','')
    root=root.rstrip('\n')+'\n'+''.join(f'\npub use {c.replace("-","_")}::{{{", ".join(sorted(ms_))}}};\n' for c,ms_ in moved)
    open(root_f,'w').write(root)
    print(crate_dir,'keeps',len(keep),[ (c,len(m)) for c,m in moved])
for d,s,e in [('🏢️buildings','buildings','🏢️'),('🏭️silos-tanks','silos-tanks','🏭️'),('🧱️members','members','🧱️'),('🗼️bridges-towers','bridges-towers','🗼️'),('🏗️foundations','foundations','🏗️')]:
    run(d,s,e)
