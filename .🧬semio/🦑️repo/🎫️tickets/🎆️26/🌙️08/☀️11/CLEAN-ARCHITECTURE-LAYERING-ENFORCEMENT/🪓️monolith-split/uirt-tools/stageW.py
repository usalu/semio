"""🔌️ Stage W: workspace members, owner rows, launch routes (launch.json and seed) and corpus rows for the five new ui-runtime crates."""
import json,re,sys
REPO="/Users/ueli/Documents/semio"
UI="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
NEW=[("exchange","📡️exchange"),("presentation","🖼️presentation"),("reconcile-primitives","🧱️primitives"),("reconcile-cursor","⏭️cursor"),("reconcile","♻️reconcile")]
def rd(p): return open(f"{REPO}/{p}",encoding="utf-8").read()
def wr(p,t): open(f"{REPO}/{p}","w",encoding="utf-8").write(t)
# ---- 1. framework workspace members (byte-sorted around the existing runtime member)
p="🧰️framework/Cargo.toml"; t=rd(p)
base='    "🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust",\n'
assert t.count(base)==1
rows={f'    "🔨️modules/🖱️ui/🧠️runtime/{n}/📦️packages/🦀️rust",\n':n for _,n in NEW}
if not any(r in t for r in rows):
    before=[r for r in rows if r.encode()<base.encode()]
    after=[r for r in rows if r.encode()>base.encode()]
    t=t.replace(base,"".join(sorted(before,key=lambda s:s.encode()))+base+"".join(sorted(after,key=lambda s:s.encode())))
    wr(p,t)
members=re.findall(r'^    "(🔨️modules/🖱️ui/🧠️runtime/[^"]+)",$',t,re.M)
assert [m.encode() for m in members]==sorted(m.encode() for m in members),members
assert len(members)==6
# ---- 2. owner rows
p="📋️build-owners.json"; o=json.loads(rd(p))
rowsO=o["owners"]
names=[r["name"] for r in rowsO]
text=rd(p)
for id_,node in NEW:
    n=f"@semio-tech/ui-runtime-{id_}-rs"
    assert n not in names
    root=f"{UI}/{node}/📦️packages/🦀️rust"
    row={"name":n,"root":root,"manifest":root+"/📋️project.json","packages":[]}
    # textual insertion in the file's own formatting: copy the shape of the ui-runtime-rs row
    anchor_name='"name": "@semio-tech/ui-runtime-rs",'
    k=text.index(anchor_name)
    start=text.rindex("    {",0,k)
    end=text.index("    },",k)+len("    },")
    sample=text[start:end]
    new=sample.replace("@semio-tech/ui-runtime-rs",n).replace(f"{UI}/📦️packages/🦀️rust",root)
    assert new.count(root)==2,new
    # insert before the ui-runtime-rs row, keeping lexical order by name
    text=text[:start]+new+"\n"+text[start:] if n<"@semio-tech/ui-runtime-rs" else text[:end]+"\n"+new+text[end:]
wr(p,text)
json.loads(text)
# ---- 3. launch routes
def free_orders(texts,prefix,count,start):
    used=set()
    for t in texts: used|=set(re.findall(r'"order": ('+re.escape(prefix)+r'\d+)',t))
    out=[];k=start
    while len(out)<count:
        v=f"{prefix}{k:05d}"[:len(prefix)+5]
        if v not in used: out.append(v)
        k+=1
    return out
files=[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]
texts=[rd(f) for f in files]
build=free_orders(texts,"900.0",10,21001)
build=[f"900.{x.split('.')[1]}" for x in build]
gate=free_orders(texts,"922.0",10,5001)
test=free_orders(texts,"903.5",10,9001)
print(build,gate,test)
def block(crate,id_,label,group,order,cfg):
    tgt={"build":"build","check":"check","test":"test"}[label[1]]
    return f'''    {{
      "name": "🦀️ {crate} {label[0]} {cfg[1]}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run @semio-tech/{id_}-rs:{tgt} --configuration={cfg[0]} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }}
    }},
'''
allblocks=""
bi=gi=ti=0
for id_,node in NEW:
    crate=f"semio-framework-ui-runtime-{id_}"
    pid=f"ui-runtime-{id_}"
    for cfg in (("prototype","🛠️prototype"),("release","🚀️release")):
        allblocks+=block(crate,pid,("🏗️build","build"),"4_build",build[bi],cfg); bi+=1
    for cfg in (("prototype","🛠️prototype"),("release","🚀️release")):
        allblocks+=block(crate,pid,("🔎️check","check"),"4_gate",gate[gi],cfg); gi+=1
    for cfg in (("prototype","🛠️prototype"),("release","🚀️release")):
        allblocks+=block(crate,pid,("🧪️test","test"),"4_test",test[ti],cfg); ti+=1
for f in files:
    t=rd(f)
    if "semio-framework-ui-runtime-exchange" in t: print("launch already present in",f); continue
    anchor='"name": "🦀️ semio-framework-pets-staging 🧪️test 🚀️release",'
    assert t.count(anchor)==1,(f,t.count(anchor))
    k=t.index(anchor)
    end=t.index("\n    },\n",k)+len("\n    },\n")
    t=t[:end]+allblocks+t[end:]
    wr(f,t)
    if f.endswith(".json"): json.loads(t)
# ---- 4. corpus rows
p="🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
c=json.loads(rd(p))
deps={
 "exchange":["semio-framework-ui-contract"],
 "presentation":["semio-framework-job","semio-framework-ui-contract","semio-framework-ui-runtime-exchange","semio-framework-ui-scene","semio-framework-value"],
 "reconcile-primitives":["semio-framework-job","semio-framework-ui-contract","semio-framework-ui-runtime-presentation"],
 "reconcile-cursor":["semio-framework-job","semio-framework-ui-contract","semio-framework-ui-runtime-presentation","semio-framework-ui-runtime-reconcile-primitives"],
 "reconcile":["semio-framework-job","semio-framework-ui-contract","semio-framework-ui-runtime-presentation","semio-framework-ui-runtime-reconcile-cursor","semio-framework-ui-runtime-reconcile-primitives"],
}
forb=["semio-framework-ui","semio-framework-os-kernel","semio-framework-plugin","wgpu","winit","taffy"]
have={r["package"] for r in c["crates"]}
tx=rd(p)
for id_,node in NEW:
    pk=f"semio-framework-ui-runtime-{id_}"
    if pk in have: continue
    c["crates"].append({"package":pk,"manifest":f"🧠️runtime/{node}/📦️packages/🦀️rust/Cargo.toml","entry":f"🧠️runtime/{node}/🦀️.rs","dependencies":deps[id_],"forbiddenDependencies":forb})
out=json.dumps(c,ensure_ascii=False,indent=2)+"\n"
wr(p,out)
print("ok")
