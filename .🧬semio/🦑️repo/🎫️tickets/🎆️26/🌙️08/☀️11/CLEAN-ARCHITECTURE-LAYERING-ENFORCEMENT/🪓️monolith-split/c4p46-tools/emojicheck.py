import json,sys,re
T="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
t=json.load(open(T,encoding="utf8"))
names=set()
def walk(o):
    if isinstance(o,dict):
        for k,v in o.items():
            if k=="memberNames" and isinstance(v,list): names.update(x for x in v if isinstance(x,str))
            else: walk(v)
    elif isinstance(o,list):
        for x in o: walk(x)
walk(t)
# also kind folder names
text=open(T,encoding="utf8").read()
prefixes={}
for n in names:
    m=re.match(r"^(\S+?)️",n)
    if m: prefixes.setdefault(m.group(1),[]).append(n)
for c in sys.argv[1:]:
    e=c.split("️")[0]
    print(c, "prefix-in-taxonomy:", len(prefixes.get(e,[])), ("e.g. "+prefixes[e][0]) if e in prefixes else "")
