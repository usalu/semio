#!/usr/bin/env python3
# 🔒️ Adds the three mesh leaf crates to every mesh-family Cargo.lock that lacks them (textual, sorted insertion, idempotent).
import json,re,sys
R="/Users/ueli/Documents/semio/"
corpus=json.load(open(R+"🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🧪️.json",encoding='utf8'))
NEW={"semio-framework-3d-mesh-journal":["semio-framework-3d-mesh-document","semio-framework-value"],
     "semio-framework-3d-mesh-quality":["semio-framework-3d-mesh-document","semio-framework-value","semio-framework-value-derive"],
     "semio-framework-3d-mesh-surface":["semio-framework-3d-mesh-document"]}
def block(name,deps):
    return f'[[package]]\nname = "{name}"\nversion = "0.1.0"\ndependencies = [\n'+"".join(f' "{d}",\n' for d in deps)+']\n'
changed=0
for lk in corpus["locks"]:
    p=R+lk
    s=open(p,encoding='utf8').read()
    if all(f'name = "{n}"' in s for n in NEW): continue
    head,*blocks=re.split(r'(?m)^\[\[package\]\]\n',s)
    names=[re.match(r'name = "([^"]+)"',b).group(1) for b in blocks]
    items=list(zip(names,blocks))
    for n,d in NEW.items():
        if n in names: continue
        text=block(n,d)[len('[[package]]\n'):]+"\n"
        idx=next((i for i,(nm,_) in enumerate(items) if nm>n),len(items))
        if idx==len(items) and items:
            items[-1]=(items[-1][0],items[-1][1].rstrip("\n")+"\n\n")
            text=text.rstrip("\n")+"\n"
        items.insert(idx,(n,text))
    out=head+"".join("[[package]]\n"+b for _,b in items)
    # facade edge
    m=re.search(r'(\[\[package\]\]\nname = "semio-framework-3d"\nversion = "[^"]+"\ndependencies = \[\n)(.*?)(\]\n)',out,re.S)
    if m:
        deps=[x.strip().strip('",') for x in m.group(2).strip().split("\n")]
        for n in NEW:
            if n not in deps: deps.append(n)
        deps.sort(key=lambda x:(x.split(" ")[0],x))
        out=out[:m.start(2)]+"".join(f' "{d}",\n' for d in deps)+out[m.end(2):]
    open(p,'w',encoding='utf8').write(out)
    changed+=1
    print("patched",lk)
print("changed",changed)
