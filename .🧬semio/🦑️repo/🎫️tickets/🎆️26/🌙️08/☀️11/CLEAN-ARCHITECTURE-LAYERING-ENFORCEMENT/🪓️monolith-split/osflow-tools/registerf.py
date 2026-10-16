"""📋️ Registers the new crates: root workspace members, owner rows, launch routes (+seed), wasm-prototype profile entries. Idempotent. usage: register.py [--apply]"""
import os,re,sys,json,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import importlib
FAMCFG={"workflow":("build_wf","@semio-tech/framework-workflow-workflow-rs"),"flowflow":("build_ff","@semio-tech/framework-flow-flow-rs")}
FAMKEY=os.environ["OSFAM"]
CR=importlib.import_module(FAMCFG[FAMKEY][0]).CR
FACADE_ROW=FAMCFG[FAMKEY][1]
REPO="/Users/ueli/Documents/semio/"
apply="--apply" in sys.argv
def rel(c):return os.path.relpath(c.pkgdir,REPO)
def members():
    p=REPO+"Cargo.toml";t=open(p,encoding="utf-8").read()
    i=t.index("members = [");j=t.index("]",i)
    lines=t[i:j].split("\n")
    ms=[l.strip().strip(",").strip('"') for l in lines[1:] if l.strip()]
    assert ms==sorted(ms,key=lambda s:s.encode())
    new=[rel(c) for c in CR.values() if rel(c) not in ms]
    allm=sorted(ms+new,key=lambda s:s.encode())
    body="members = [\n"+"".join(f'    "{m}",\n' for m in allm)
    t2=t[:i]+body+t[j:]
    print("members +",len(new))
    if apply and new:open(p,"w",encoding="utf-8").write(t2)
def owners():
    p=REPO+"📋️build-owners.json";t=open(p,encoding="utf-8").read()
    names=set(re.findall(r'"name": "([^"]+)"',t))
    new=[c for c in CR.values() if c.project not in names]
    for c in sorted(new,key=lambda c:c.project.encode()):
        row=f'    {{\n      "name": "{c.project}",\n      "root": "{rel(c)}",\n      "manifest": "{rel(c)}/📋️project.json",\n      "packages": []\n    }},\n'
        blocks=list(re.finditer(r'    \{\n      "name": "([^"]+)",\n(?:.*\n)*?    \},?\n',t))
        idx=next(k for k,b in enumerate(blocks) if b.group(1)==FACADE_ROW)
        pos=blocks[idx].end()
        k=idx+1
        while k<len(blocks) and blocks[k].group(1).startswith(FACADE_ROW[:-3]+"-") and blocks[k].group(1).encode()<c.project.encode():
            pos=blocks[k].end();k+=1
        t=t[:pos]+row+t[pos:]
    json.loads(t)
    print("owners +",len(new))
    if apply and new:open(p,"w",encoding="utf-8").write(t)
def routes():
    used=set()
    for f in [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]:
        used|=set(re.findall(r'"order": ([0-9.]+)',open(REPO+f,encoding="utf-8").read()))
    def alloc(base,n):
        out=[];k=0
        while len(out)<n:
            v=f"{base+k*0.00001:.5f}".rstrip("0")
            k+=1
            if v not in used:out.append(v);used.add(v)
        return out
    cr=list(CR.values())
    prot=[]
    spec=[]
    for c in cr:spec.append(c)
    ob=alloc(900.09500,2*len(spec));og=alloc(922.06000,2*len(spec));ot=alloc(903.60000,2*len(spec))
    blocks=[]
    for i,c in enumerate(spec):
        for j,(mode,cfg,icon) in enumerate([("prototype","prototype","🛠️"),("release","release","🚀️")]):
            for kind,tag,grp,orders,tg in [("build","🏗️build","4_build",ob,"build"),("check","🔎️check","4_gate",og,"check"),("test","🧪️test","4_test",ot,"test")]:
                name=f"🦀️ {c.name} {tag} {icon}{mode}"
                cmd=f"bun repo run {c.project}:{tg} --configuration={cfg} --parallel=1 -- --lib --offline"
                blocks.append((name,f'    {{\n      "name": "{name}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "{cmd}",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{grp}",\n        "order": {orders[2*i+j]}\n      }}\n    }},\n'))
    return blocks
def launch():
    blocks=routes()
    for f in [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]:
        p=REPO+f;t=open(p,encoding="utf-8").read()
        missing=[b for n,b in blocks if f'"name": "{n}"' not in t]
        if not missing:print(f,"routes present");continue
        anchor='"name": "🦀️ semio-framework-pets-staging 🧪️test 🚀️release"'
        k=t.index(anchor);e=t.index("\n    },\n",k)+len("\n    },\n")
        t2=t[:e]+"".join(missing)+t[e:]
        print(f,"routes +",len(missing))
        if apply:open(p,"w",encoding="utf-8").write(t2)
if __name__=="__main__":
    members();owners();launch()
