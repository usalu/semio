"""🗂️ Registry wiring of the ten new producers: workspace members, owner rows and the six launch routes per crate in both launch files (idempotent)."""
import json,os,re,sys
R="/Users/ueli/Documents/semio/"
FW="🧰️framework/🛍️products/💻️os/"
CRATES=[ # (crate, package dir, project)
 ("semio-framework-os-host-codec-abi",FW+"🖥️host/📡️codec-abi/📦️packages/🦀️rust","@semio-tech/framework-os-host-codec-abi-rs"),
 ("semio-framework-os-host-instance",FW+"🖥️host/🧱️instance/📦️packages/🦀️rust","@semio-tech/framework-os-host-instance-rs"),
 ("semio-framework-os-host-media-raster",FW+"🖥️host/🖼️media-export-raster/📦️packages/🦀️rust","@semio-tech/framework-os-host-media-raster-rs"),
 ("semio-framework-os-host-registry",FW+"🖥️host/🗂️registry/📦️packages/🦀️rust","@semio-tech/framework-os-host-registry-rs"),
 ("semio-framework-os-host-space",FW+"🔨️modules/🪐️space/📦️packages/🦀️rust","@semio-tech/framework-os-host-space-rs"),
 ("semio-framework-os-host-workflow",FW+"🖥️host/🔀️workflow/📦️packages/🦀️rust","@semio-tech/framework-os-host-workflow-rs"),
 ("semio-framework-os-kernel-neural-engine-budgeted",FW+"🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/📦️packages/🦀️rust","@semio-tech/neural-engine-budgeted-rs"),
 ("semio-framework-os-kernel-neural-engine-model",FW+"🔨️modules/🧠️neural/⚙️engine/🧱️model/📦️packages/🦀️rust","@semio-tech/neural-engine-model-rs"),
 ("semio-framework-os-kernel-neural-engine-registry",FW+"🔨️modules/🧠️neural/⚙️engine/📔️registry/📦️packages/🦀️rust","@semio-tech/neural-engine-registry-rs"),
 ("semio-framework-artifact-infinite-dag-vcs",FW+"🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/📦️packages/🦀️rust","@semio-tech/framework-infinite-dag-vcs-rs"),
]
def rd(p):return open(R+p,encoding="utf-8").read()
def wr(p,t):open(R+p,"w",encoding="utf-8").write(t)
# --- workspace members
t=rd("Cargo.toml")
a=t.index("members = [");b=t.index("]",a)
lines=[l for l in t[a:b].split("\n")[1:] if l.strip()]
cur=[l.strip().rstrip(",").strip('"') for l in lines]
for _,pkg,_ in CRATES:
    if pkg not in cur:cur.append(pkg)
cur=sorted(set(cur),key=lambda s:s.encode())
new="members = [\n"+"".join(f'    "{m}",\n' for m in cur)
t=t[:a]+new+t[b:]
wr("Cargo.toml",t)
# --- owner rows
d=json.loads(rd("📋️build-owners.json"))
names={o["name"] for o in d["owners"]}
raw=rd("📋️build-owners.json")
rows=[]
for crate,pkg,proj in CRATES:
    if proj in names:continue
    rows.append((proj,{"name":proj,"root":pkg,"manifest":pkg+"/📋️project.json","packages":[]}))
if rows:
    # insert each row at the end of the owners array keeping existing order untouched; find insertion by name order among neighbours
    owners=d["owners"]
    for proj,row in rows:
        idx=len(owners)
        for i,o in enumerate(owners):
            if o["name"]>proj and (i==0 or owners[i-1]["name"]<=proj):idx=i;break
        owners.insert(idx,row)
    indent=2
    out=json.dumps(d,ensure_ascii=False,indent=indent)+"\n"
    wr("📋️build-owners.json",out)
print("members",len(cur),"owner rows added",len(rows))
# --- launch routes
ROUTES=[("🏗️build","build","4_build",900),("🔎️check","check","4_gate",922),("🧪️test","test","4_test",903)]
CONFIGS=[("🛠️prototype","prototype"),("🚀️release","release")]
def block(crate,proj,idx):
    out=""
    for emoji,target,group,band in ROUTES:
        for k,(label,conf) in enumerate(CONFIGS):
            order=f"{band}.2{idx:02d}{k+1:02d}"
            out+=('    {\n'
                  f'      "name": "🦀️ {crate} {emoji} {label}",\n'
                  '      "type": "node-terminal",\n'
                  '      "request": "launch",\n'
                  f'      "command": "bun repo run {proj}:{target} --configuration={conf} --parallel=1 -- --lib --offline",\n'
                  '      "cwd": "${workspaceFolder}",\n'
                  '      "presentation": {\n'
                  f'        "group": "{group}",\n'
                  f'        "order": {order}\n'
                  '      }\n'
                  '    },\n')
    return out
for path,anchor_prefix in [(".vscode/launch.json",'"name": "🦀️ semio-framework-server-throttle '),(".vscode/🧩️launch.seed.jsonc",'"name": "🦀️ semio-framework-pets-')]:
    t=rd(path)
    add=""
    for i,(crate,pkg,proj) in enumerate(CRATES,1):
        if f'"name": "🦀️ {crate} 🏗️build 🛠️prototype"' in t:continue
        add+=block(crate,proj,i)
    if not add:print(path,"already wired");continue
    last=t.rindex(anchor_prefix)
    end=t.index("\n    },\n",last)+len("\n    },\n")
    t=t[:end]+add+t[end:]
    wr(path,t)
    print(path,"routes added:",add.count('"name"'))
