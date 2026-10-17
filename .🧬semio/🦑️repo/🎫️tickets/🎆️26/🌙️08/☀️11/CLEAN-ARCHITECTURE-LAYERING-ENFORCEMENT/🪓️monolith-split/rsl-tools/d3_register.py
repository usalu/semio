"""🗂️ Registers `os-store-engine-bundle`: manifest (dependencies from the moved code, paths re-based from the engine manifest), `📋️project.json`, root workspace member (byte-sorted), owner row, six launch routes in both launch files (right behind the stage routes), the crate-boundary corpus row and the engine's dependency edge (unused engine dependencies are dropped). Idempotent; every file is re-read right before its write.
usage: d3_register.py dry|write"""
import json, os, re, shutil, sys
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
S="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
write=sys.argv[1]=="write"
NAME="semio-framework-os-store-engine-bundle"; ID="os-store-engine-bundle"; PROJECT="@semio-tech/os-store-engine-bundle-rs"
CRATE=S+"/⚙️engine/🧳️bundle"; PKGDIR=CRATE+"/📦️packages/🦀️rust"
ENGINE_MAN=S+"/⚙️engine/📦️packages/🦀️rust/Cargo.toml"
def b(s): return s.encode("utf-8")
def w(path,text):
    if not write: return
    if os.path.exists(path):
        d=PRE+path
        if not os.path.exists(d): os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(path,d)
    if os.path.dirname(path): os.makedirs(os.path.dirname(path),exist_ok=True)
    open(path,"w",encoding="utf8").write(text)
src=open(CRATE+"/🦀️.rs",encoding="utf8").read() if os.path.exists(CRATE+"/🦀️.rs") else open(G+"/rsl-d3/out/bundle.rs",encoding="utf8").read()
# --- dependencies
heads=set(re.findall(r'(?<![A-Za-z0-9_])(semio_framework_[a-z0-9_]+)::',re.sub(r'(?m)^\s*//.*$','',src)))
for m in re.finditer(r'(?m)^use ([a-z_][a-z0-9_]*)::',src): heads.add(m.group(1))
for lib,full in (("protocol","semio_framework_replication"),("pack","semio_framework_pack")):
    if re.search(r'(?<![A-Za-z0-9_:])'+lib+r'::',src): heads.add(full)
heads-= {"std","self","crate","super","backbone","core","alloc","protocol","pack"}
pk={h:h.replace("_","-") for h in heads}
eng=open(ENGINE_MAN,encoding="utf8").read()
lines=eng.split("\n")
dep_lines={}
sec=None
for l in lines:
    m=re.match(r'^\[([^\]]+)\]\s*$',l)
    if m: sec=m.group(1); continue
    if sec=="dependencies":
        m=re.match(r'^([A-Za-z0-9_\-]+)\s*=\s*(.*)$',l)
        if m:
            key,val=m.group(1),m.group(2)
            pm=re.search(r'package\s*=\s*"([^"]+)"',val)
            dep_lines[pm.group(1) if pm else key]=(key,val)
missing=[h for h,p in pk.items() if p not in dep_lines]
assert not missing,("dependency not in the engine manifest",missing)
new_dir=os.path.abspath(PKGDIR)
eng_dir=os.path.dirname(os.path.abspath(ENGINE_MAN))
out_deps=[]
for h,p in sorted(pk.items(),key=lambda x:b(x[1])):
    key,val=dep_lines[p]
    def fix(m):
        target=os.path.normpath(os.path.join(eng_dir,m.group(1)))
        return 'path = "'+os.path.relpath(target,new_dir)+'"'
    val=re.sub(r'path\s*=\s*"([^"]+)"',fix,val)
    out_deps.append(f"{key} = {val}")
# order: first-party store crates first like the stage manifest (byte-sorted overall is acceptable) -> keep byte-sorted by key
out_deps.sort(key=lambda l: b(l.split("=")[0].strip()))
manifest=f'''[package]
workspace = "../../../../../../../../.."
name = "{NAME}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Use cases of the store composer over the StoreParts bundle: batched one-item publication advance and remote-merge ingest"

[package.metadata.semio]
role = "framework"
id = "{ID}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
'''+"\n".join(out_deps)+"\n"
project=json.dumps({"name":PROJECT,"tags":["scope:framework","language:rust"]},indent=2,ensure_ascii=False)+"\n"
# workspace pointer check: the stage manifest uses the same string
stage=open(S+"/⚙️engine/🗄️stage/📦️packages/🦀️rust/Cargo.toml",encoding="utf8").read()
assert 'workspace = "../../../../../../../../.."' in stage
man_path=PKGDIR+"/Cargo.toml"; proj_path=PKGDIR+"/📋️project.json"
if os.path.exists(man_path): assert open(man_path,encoding="utf8").read()==manifest,"manifest exists with other content"
w(man_path,manifest); w(proj_path,project)
print("manifest deps",len(out_deps))
# --- root members
t=open("Cargo.toml",encoding="utf8").read()
i=t.index("members = [\n"); j=t.index("\n]",i)
head=t[:i+len("members = [\n")]; body=t[i+len("members = [\n"):j]; tail=t[j:]
old=[l.strip().rstrip(",").strip('"') for l in body.split("\n") if l.strip()]
member=PKGDIR
if member in old: print("member present")
else:
    merged=sorted(old+[member],key=b)
    out=head+"\n".join('    "'+p+'",' for p in merged)+tail
    cur=open("Cargo.toml",encoding="utf8").read(); assert cur==t
    w("Cargo.toml",out); print("member added")
# --- owner row
o=open("📋️build-owners.json",encoding="utf8").read()
jo=json.loads(o); names=[r["name"] for r in jo["owners"]]
if PROJECT in names: print("owner present")
else:
    after=[n for n in names if b(n)>b(PROJECT)][0]
    anchor='    {\n      "name": "'+after+'",'
    assert o.count(anchor)==1
    row=f'''    {{
      "name": "{PROJECT}",
      "root": "{PKGDIR}",
      "manifest": "{PKGDIR}/📋️project.json",
      "packages": []
    }},
'''
    o2=o.replace(anchor,row+anchor); json.loads(o2)
    cur=open("📋️build-owners.json",encoding="utf8").read(); assert cur==o
    w("📋️build-owners.json",o2); print("owner added")
# --- launch routes behind the stage routes
LAUNCH=[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]
SPEC=[("build","🏗️build","prototype","🛠️prototype"),("build","🏗️build","release","🚀️release"),("check","🔎️check","prototype","🛠️prototype"),("check","🔎️check","release","🚀️release"),("test","🧪️test","prototype","🛠️prototype"),("test","🧪️test","release","🚀️release")]
for path in LAUNCH:
    t=open(path,encoding="utf8").read(); t0=t
    used={round(float(x),5) for x in re.findall(r'"order": ([0-9.]+)',t)}
    if f'"name": "🦀️ {NAME} 🏗️build 🛠️prototype"' in t: print("launch present",path); continue
    for tgt,label,conf,cl in SPEC:
        stage_name=f'"name": "🦀️ semio-framework-os-store-engine-stage {label} {cl}"'
        assert t.count(stage_name)==1,(path,stage_name)
        a=t.index(stage_name); a=t.rfind("\n    {\n",0,a)+1
        e=t.index("\n    },\n",a)+len("\n    },\n")
        blk=t[a:e]
        order=float(re.search(r'"order": ([0-9.]+)',blk).group(1))
        n=1
        while round(order+n/100000,5) in used: n+=1
        new_order=round(order+n/100000,5); used.add(new_order)
        new=blk.replace("os-store-engine-stage",ID if False else "os-store-engine-bundle").replace(repr(order) if repr(order) in blk else str(order),str(new_order))
        new=re.sub(r'"order": [0-9.]+','"order": '+str(new_order),new)
        assert "bundle" in new
        t=t[:e]+new+t[e:]
    if path.endswith(".json"): json.loads(t)
    cur=open(path,encoding="utf8").read(); assert cur==t0
    w(path,t); print("launch routes added",path)
# --- boundary corpus
B="🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
t=open(B,encoding="utf8").read()
if NAME in t: print("corpus present")
else:
    anchor='''    {
      "package": "semio-framework-os-store-engine",
      "manifest": "🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust/Cargo.toml",'''
    assert t.count(anchor)==1
    row='''    {
      "package": "semio-framework-os-store-engine-bundle",
      "manifest": "🔨️modules/🏪️store/⚙️engine/🧳️bundle/📦️packages/🦀️rust/Cargo.toml",
      "entry": "🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs",
      "maximumActiveLines": 1500,
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework"
      ]
    },
'''
    t2=t.replace(anchor,row+anchor); json.loads(t2)
    cur=open(B,encoding="utf8").read(); assert cur==t
    w(B,t2); print("corpus row added")
# --- engine manifest: add the bundle edge
e=open(ENGINE_MAN,encoding="utf8").read()
if NAME in e: print("engine edge present")
else:
    rel=os.path.relpath(os.path.abspath(PKGDIR),os.path.dirname(os.path.abspath(ENGINE_MAN)))
    anchor="semio-framework-os-store-engine-core = "
    assert e.count(anchor)==1
    line=f'{NAME} = {{ path = "{rel}" }}\n'
    e2=e.replace(anchor,line+anchor)
    cur=open(ENGINE_MAN,encoding="utf8").read(); assert cur==e
    w(ENGINE_MAN,e2); print("engine edge added")
