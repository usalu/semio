import re,sys,collections
L="/Users/ueli/Documents/semio/.cache/f/graph-server/read.log"
if len(sys.argv)>1: L=sys.argv[1]
rows=collections.Counter();own=collections.defaultdict(set);first=collections.Counter();firstown=collections.defaultdict(set)
for l in open(L,encoding="utf8",errors="replace"):
    m=re.search(r"owner (\S+): the first row of the record from the fleet that does not hold here is (\S+) (.*?) \(fact .*\)(?:; reason: (.*))?$",l.rstrip())
    if not m: continue
    name,kind,path,why=m.groups()
    key=kind
    if why:
        w=why.split(";")[0]
        w=re.sub(r"build:([a-z0-9-]+):v1:[0-9a-f]{64}",r"build:\1:v1:#",w)
        w=re.sub(r"[0-9a-f]{10,12}",r"#",w)
        key=kind+" / "+w[:110]
    rows[key]+=1;own[key].add(name);first[kind]+=1;firstown[kind].add(name)
print("lines  owners  kind / reason")
for k,c in rows.most_common(): print("%5d  %6d  %s"%(c,len(own[k]),k))
print("per kind:",{k:(c,len(firstown[k])) for k,c in first.items()})
