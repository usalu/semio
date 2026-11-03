import os,re,json,sys
R="/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUB="🏅️standards/🔖️1/🪆️subsets/✳️any"
ARTS=["⚖️en1990","🏋️en1991","🏛️en1992","🔩️en1993","🧩️en1994","🪵️en1995","🪨️en1996","🌍️en1997","🫨️en1998","🪶️en1999"]
def norm(s): return re.sub(r'[^a-z0-9]','',s.lower())
struct_re=re.compile(r'pub struct (\w+)\s*\{(.*?)\n\}',re.S)
for a in ARTS:
    base=f"{R}/{a}/{SUB}"
    schema=json.load(open(f"{base}/🧬️schema/📸️snapshot/🔣️.json"))
    structs={}
    for root,ds,fs in os.walk(base):
        if "node_modules" in root or "/🧪️tests" in root or "🧪️tests" in root.split("/")[-1]: continue
        for f in fs:
            if f=="🦀️.rs":
                try: t=open(os.path.join(root,f)).read()
                except: continue
                for m in struct_re.finditer(t):
                    fields=re.findall(r'^\s*pub (\w+):',m.group(2),re.M)
                    if fields: structs.setdefault(m.group(1),[]).append((os.path.join(root,f),fields))
    defs=dict(schema["$defs"]); defs[schema["title"]]=schema
    bad=0
    for name,d in defs.items():
        props=list(d.get("properties",{}))
        c=structs.get(name)
        if not c: print(f"{a} {name}: no rust struct found"); continue
        best=None
        for p,fields in c:
            if [norm(x) for x in fields]==[norm(x) for x in props]: best=p
        if not best:
            fs=c[0][1]
            miss=[x for x in props if norm(x) not in map(norm,fs)]
            extra=[x for x in fs if norm(x) not in map(norm,props)]
            print(f"{a} {name}: DRIFT json-only={miss} rust-only={extra}"); bad+=1
    print(a,"defs",len(defs),"drift",bad)
