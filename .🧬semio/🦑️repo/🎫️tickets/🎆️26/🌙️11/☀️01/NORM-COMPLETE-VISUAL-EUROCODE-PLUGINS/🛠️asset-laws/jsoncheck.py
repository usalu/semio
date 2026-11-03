import sys,json,os,re
sys.path.insert(0,".")
from dslmodel import *
def walk(d,sname,structs,enums,path,out):
    fl=getstruct(structs,sname)
    if fl is None or not isinstance(d,dict): return
    names={norm(f.name):f for f in fl}
    for k in d:
        if norm(k) not in names: out.append(f"{path}.{k}: key not in {sname}")
    for f in fl:
        if f.name not in d and norm(f.name) not in {norm(k) for k in d}: out.append(f"{path}.{f.name}: missing")
        else:
            k=[k for k in d if norm(k)==norm(f.name)]
            if not k: continue
            v=d[k[0]];t,x,o=tag(f,structs,enums)
            if t in("TABLE","LIST") and x in structs and isinstance(v,list):
                for i,e in enumerate(v[:50]): walk(e,x,structs,enums,f"{path}.{f.name}[{i}]",out)
            elif t=="REC" and isinstance(v,dict): walk(v,x,structs,enums,f"{path}.{f.name}",out)
for art in ARTS:
    base=f"{R}/{art}/{SUB}";structs,enums=parse_rust(base)
    for p,fl in parse_rust("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance")[0].items(): structs.setdefault(p,[]).extend(fl)
    sch=schema_of(art)
    for root,ds,fs in os.walk(base+"/🖼️assets"):
        for f in fs:
            if f.endswith(".json"):
                p=os.path.join(root,f);out=[]
                walk(json.load(open(p)),sch["title"],structs,enums,"$",out)
                print(art,p.split("🖼️assets/")[1],len(out)); [print("   ",o) for o in out[:6]]
