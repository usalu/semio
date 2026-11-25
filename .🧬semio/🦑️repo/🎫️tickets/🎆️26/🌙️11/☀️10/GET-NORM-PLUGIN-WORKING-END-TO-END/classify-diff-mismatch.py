import json,os,sys,collections
produced,fixtures=sys.argv[1:3]
def walk(a,b,p,out):
    if isinstance(a,dict) and isinstance(b,dict):
        for k in sorted(set(a)|set(b)):
            if k not in a: out.append((p+"/"+k,"absent in committed",b[k]))
            elif k not in b: out.append((p+"/"+k,"absent in produced",a[k]))
            else: walk(a[k],b[k],p+"/"+k,out)
    elif isinstance(a,list) and isinstance(b,list) and len(a)==len(b):
        for i,(x,y) in enumerate(zip(a,b)): walk(x,y,p+"[]",out)
    elif a!=b: out.append((p,a,b))
classes=collections.defaultdict(list)
for n in sorted(os.listdir(produced)):
    leaf,case=n[:-5].split("__")
    c=json.load(open(os.path.join(fixtures,leaf,case,"🔣️.diff.json"),encoding="utf-8"))
    pr=json.load(open(os.path.join(produced,n),encoding="utf-8"))
    out=[];walk(c,pr,"",out)
    key=tuple(sorted({(o[0].replace("/members[]/modified[]/patch","/m.patch"),json.dumps(o[1])[:40],json.dumps(o[2])[:40]) for o in out}))
    classes[key].append(leaf)
for k,v in sorted(classes.items(),key=lambda x:-len(x[1])):
    print(len(v),v[:3]);[print("   ",x) for x in k[:6]]
