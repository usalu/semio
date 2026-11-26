import re,sys
P='semio-s-artifact-norm-en1995-'
def edit(path):
    t=open(path,encoding='utf8').read()
    head,*blocks=t.split('\n[[package]]\n')
    out={}
    order=[]
    def name_of(b): return re.search(r'^name = "([^"]+)"',b,re.M).group(1)
    def deps(b):
        m=re.search(r'dependencies = \[\n(.*?)\n\]',b,re.S)
        return [x.strip().strip(',').strip('"') for x in m.group(1).split('\n')] if m else []
    def setdeps(b,ds):
        ds=sorted(set(ds))
        body='dependencies = [\n'+'\n'.join(' "%s",'%d for d in ds)+'\n]'
        if re.search(r'dependencies = \[\n.*?\n\]',b,re.S): return re.sub(r'dependencies = \[\n.*?\n\]',lambda m:body,b,count=1,flags=re.S)
        return b.rstrip('\n')+'\n'+body+'\n'
    names=[name_of(b) for b in blocks]
    have_edit=P+'mutations-edit-rules' in names
    nb=[]
    for b in blocks:
        n=name_of(b)
        if n==P+'mutations':
            ds=deps(b); ds=[d for d in ds if d not in(P+'mutations-proto',P+'mutations-json-schema',P+'mutations-graphql',P+'mutations-typescript','semio-framework-schema-registry')]; b=setdeps(b,ds)
        elif n==P+'schema':
            b=setdeps(b,deps(b)+[P+'mutations-proto',P+'mutations-json-schema',P+'mutations-graphql',P+'mutations-typescript'])
        elif n==P+'editor':
            b=setdeps(b,deps(b)+[P+'mutations-edit-rules'])
        elif n==P+'mutations-bridges':
            b=setdeps(b,[d for d in deps(b) if d not in('semio-s-plugin-norm-app-surface','semio-framework-value')])
        nb.append((n,b))
    if not have_edit:
        ver=re.search(r'version = "([^"]+)"',[b for n,b in nb if n==P+'mutations'][0]).group(1)
        new='name = "%smutations-edit-rules"\nversion = "%s"\ndependencies = [\n "%smodel",\n "%smutations",\n "semio-framework-value",\n "semio-s-plugin-norm-app-surface",\n]\n'%(P,ver,P,P)
        idx=[i for i,(n,b) in enumerate(nb) if n==P+'mutations-bridges']
        idx=idx[0] if idx else [i for i,(n,b) in enumerate(nb) if n==P+'mutations'][0]
        nb.insert(idx+1,(P+'mutations-edit-rules',new))
    res=head+''.join('\n[[package]]\n'+b for n,b in nb)
    if not res.endswith('\n'): res+='\n'
    open(path,'w',encoding='utf8').write(res)
for p in sys.argv[1:]: edit(p)
