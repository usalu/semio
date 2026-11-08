import re,sys,collections
p=sys.argv[1];t=open(p).read()
parts=re.split(r'(?m)^(?=\[\[package\]\]\n)',t)
head,pk=parts[0],parts[1:]
g=collections.defaultdict(list)
for b in pk:
    n=re.search(r'name = "([^"]+)"',b).group(1);v=re.search(r'version = "([^"]+)"',b).group(1)
    if 'source = "registry' not in b:continue
    ma,mi=v.split('.')[:2]
    g[(n,ma if ma!='0' else '0.'+mi)].append(v)
drop={}
for (n,_),vs in g.items():
    if len(vs)>1:
        vs.sort(key=lambda v:tuple(int(x) for x in v.split('.')))
        for v in vs[1:]:drop[(n,v)]=vs[0]
out=[b for b in pk if (re.search(r'name = "([^"]+)"',b).group(1),re.search(r'version = "([^"]+)"',b).group(1)) not in drop]
res=head+''.join(out)
for (n,v),to in drop.items():res=res.replace('"%s %s"'%(n,v),'"%s %s"'%(n,to))
print("dropped",sorted(drop.items()))
open(p,'w').write(res)
