"""📋️ Registers the flow-flow producers: root workspace members, owner rows, six launch routes per crate in launch.json and the seed. Idempotent. usage: register_ff.py [--apply]"""
import os,re,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import mkcrates
CR=mkcrates.CR
FACADE_ROW="@semio-tech/framework-flow-flow-rs"
REPO="/Users/ueli/Documents/semio/"
apply="--apply" in sys.argv
def rel(c):return c.pkgdir
def members():
    p=REPO+"Cargo.toml";t=open(p,encoding="utf-8").read()
    i=t.index("members = [");j=t.index("]",i)
    ms=[l.strip().strip(",").strip('"') for l in t[i:j].split("\n")[1:] if l.strip()]
    assert ms==sorted(ms,key=lambda s:s.encode()),"members not byte-sorted"
    new=[rel(c) for c in CR.values() if rel(c) not in ms]
    allm=sorted(ms+new,key=lambda s:s.encode())
    t2=t[:i]+"members = [\n"+"".join(f'    "{m}",\n' for m in allm)+t[j:]
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
        pos=blocks[idx].end();k=idx+1
        prefix=FACADE_ROW[:-3]+"-"
        while k<len(blocks) and blocks[k].group(1).startswith(prefix) and blocks[k].group(1).encode()<c.project.encode():
            pos=blocks[k].end();k+=1
        t=t[:pos]+row+t[pos:]
    json.loads(t)
    print("owners +",len(new))
    if apply and new:open(p,"w",encoding="utf-8").write(t)
def routes(used):
    def alloc(base,n):
        out=[];k=0
        while len(out)<n:
            v=f"{base+k*0.00001:.5f}".rstrip("0")
            k+=1
            if v not in used:out.append(v);used.add(v)
        return out
    spec=list(CR.values())
    ob=alloc(900.09600,2*len(spec));og=alloc(922.06100,2*len(spec));ot=alloc(903.60100,2*len(spec))
    blocks=[]
    for i,c in enumerate(spec):
        for j,(mode,icon) in enumerate([("prototype","🛠️"),("release","🚀️")]):
            for tag,grp,orders,tg in [("🏗️build","4_build",ob,"build"),("🔎️check","4_gate",og,"check"),("🧪️test","4_test",ot,"test")]:
                name=f"🦀️ {c.name} {tag} {icon}{mode}"
                cmd=f"bun repo run {c.project}:{tg} --configuration={mode} --parallel=1 -- --lib --offline"
                blocks.append((name,f'    {{\n      "name": "{name}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "{cmd}",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{grp}",\n        "order": {orders[2*i+j]}\n      }}\n    }},\n'))
    return blocks
def launch():
    used=set()
    texts={}
    for f in [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]:
        texts[f]=open(REPO+f,encoding="utf-8").read()
        used|=set(re.findall(r'"order": ([0-9.]+)',texts[f]))
    blocks=routes(used)
    anchor='"name": "🦀️ semio-framework-artifact-workflow-workflow-schema-laws 🧪️test 🚀️release"'
    for f,t in texts.items():
        missing=[b for n,b in blocks if f'"name": "{n}"' not in t]
        if not missing:print(f,"routes present");continue
        k=t.index(anchor);e=t.index("\n    },\n",k)+len("\n    },\n")
        t2=t[:e]+"".join(missing)+t[e:]
        print(f,"routes +",len(missing))
        if apply:open(REPO+f,"w",encoding="utf-8").write(t2)
if __name__=="__main__":
    members();owners();launch()
