import os,re,sys
R='✏️s/🔌️plugins'
names=['animate','architect','bim','block','cad','dag','demonstrator','draw','energy','fem','flow','forms','gis','imperative','layout','lowpoly']
dirs=[d for d in os.listdir(R) if any(d.endswith(n) for n in names) and os.path.isdir(os.path.join(R,d))]
print(len(dirs),file=sys.stderr)
out=[];skip={'node_modules','target','.git'}
for d in dirs:
    for root,ds,fs in os.walk(os.path.join(R,d)):
        ds[:]=[x for x in ds if x not in skip]
        for f in fs:
            if f.endswith('.rs'): out.append(os.path.join(root,f))
hdr=re.compile(r'^impl[^\n{]*\b(MutationDiff|DiffAlgebra|Mutation|MutationKind)<',re.M)
cand=[]
for f in out:
    try:s=open(f,encoding='utf-8').read()
    except Exception as e: continue
    if hdr.search(s): cand.append(f)
open(sys.argv[1]+'/rs-all.txt','w').write('\n'.join(out)+'\n')
open(sys.argv[1]+'/cand.txt','w').write('\n'.join(cand)+'\n')
print(len(out),len(cand))
