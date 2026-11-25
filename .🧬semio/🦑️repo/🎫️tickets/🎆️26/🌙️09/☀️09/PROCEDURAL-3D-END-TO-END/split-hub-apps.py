import json,re,os,sys
R="/Users/ueli/Documents/semio"
os.chdir(R)
H="🌎️hub"
SPECS={
"flow":dict(dir="🌊️flow",member="🧩️compositions/🌊️flow/📦️packages/🦀️rust",pkg="semio-hub-flow",base="Flow",
  variants=[dict(slug="flow-editor",icon="✏️",enum="FlowEditorApps",variant="FlowEditor",ty="VcsArtifactApp<EditorApp<semio_s_artifact_flow_flow::editor::flow::FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>",deps=["semio-s-artifact-flow-flow","semio-s-artifact-stdio-semio"],doc="the flow editor surface"),
            dict(slug="flow-viewer",icon="👁️",enum="FlowViewerApps",variant="FlowViewer",ty="VcsArtifactApp<ViewerApp<semio_s_artifact_flow_flow::viewer::flow::FlowViewer>, semio_s_artifact_stdio_semio::SemioMembers>",deps=["semio-s-artifact-flow-flow","semio-s-artifact-stdio-semio"],doc="the flow viewer surface")],
  common=["semio-plugin","semio-framework-dispatch-macros","semio-framework-tool-run","semio-framework-job","semio-framework-value","semio-framework-hash","semio-framework","semio-os-kernel"],
  features={"semio-plugin":["canvas-runtime-assets"]}),
}
def apply(name):
    s=SPECS[name]; comp=f"{H}/🧩️compositions/{s['dir']}"
    names=[]
    for v in s["variants"]:
        crate=f"semio-hub-{v['slug']}-app"; names.append((crate,v))
        d=f"{comp}/🗃️apps/{v['icon']}{v['slug']}"; os.makedirs(d+"/📦️packages/🦀️rust",exist_ok=True)
        v["extra_use"]="use semio_plugin_instance_construction::OriginalVcsApp;\n" if "OriginalVcsApp" in v["ty"] else ""
        if v["extra_use"]: v["deps"]=sorted(set(v["deps"])|{"semio-plugin-instance-construction"})
        open(d+"/🦀️.rs","w",encoding="utf8").write(f'''//! 🗃️ The closed runtime app fleet of {v['doc']}: one crate per app so the app machinery of the composition is never instantiated in a single compilation unit.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
{v["extra_use"]}
semio_framework_dispatch_macros::dyn_enum_close! {{
    /// 🗃️ The one-app fleet of {v['doc']}, routed by the composition's own fleet enum.
    pub enum {v['enum']}: PluginApp {{
        {v['variant']}({v['ty']}),
    }}
}}
''')
        deps=sorted(set(s["common"]+v["deps"]))
        def dep(n):
            f=s["features"].get(n)
            return f'{n} = {{ workspace = true, features = {json.dumps(f)} }}\n' if f else f'{n} = {{ workspace = true }}\n'
        open(d+"/📦️packages/🦀️rust/Cargo.toml","w",encoding="utf8").write(f'''[package]
workspace = "../../../../../.."
name = "{crate}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "App fleet of {v['doc']} of semio-hub-{name}"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
'''+"".join(dep(n) for n in deps))
    # workspace manifest
    p=f"{H}/Cargo.toml"; t=open(p,encoding="utf8").read()
    a=f'    "{s["member"]}",\n'; assert t.count(a)==1
    t=t.replace(a,a+"".join(f'    "🧩️compositions/{s["dir"]}/🗃️apps/{v["icon"]}{v["slug"]}/📦️packages/🦀️rust",\n' for _,v in names))
    b='[workspace.dependencies]\n'; assert t.count(b)==1
    if any(v["extra_use"] for _,v in names) and "semio-plugin-instance-construction = " not in t:
        b2=b+'semio-plugin-instance-construction = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧱️construction/📦️packages/🦀️rust" }\n'; t=t.replace(b,b2); b=b2
    t=t.replace(b,b+"".join(f'{c} = {{ path = "🧩️compositions/{s["dir"]}/🗃️apps/{v["icon"]}{v["slug"]}/📦️packages/🦀️rust" }}\n' for c,v in names))
    open(p,"w",encoding="utf8").write(t)
    # hub package manifest
    p=f"{comp}/📦️packages/🦀️rust/Cargo.toml"; t=open(p,encoding="utf8").read()
    a="[dependencies]\n"; assert t.count(a)==1
    t=t.replace(a,a+"".join(f"{c} = {{ workspace = true }}\n" for c,_ in names)); open(p,"w",encoding="utf8").write(t)
    # lock
    p=f"{H}/Cargo.lock"; t=open(p,encoding="utf8").read()
    head,*blocks=t.split("\n[[package]]\n"); out=[]
    for b in blocks:
        nm=re.match(r'name = "([^"]+)"',b).group(1)
        if nm==s["pkg"]:
            m=re.search(r'dependencies = \[\n((?: "[^"]+",?\n)*)\]',b)
            ds=sorted(set(re.findall(r'"([^"]+)"',m.group(1)))|{c for c,_ in names})
            b=b[:m.start(1)]+"".join(f' "{d}",\n' for d in ds)+b[m.end(1):]
        out.append((nm,b))
    have={n for n,_ in out}
    for c,v in names:
        ds=sorted(set(s["common"]+v["deps"])); assert all(x in have for x in ds),[x for x in ds if x not in have]
        blk=f'name = "{c}"\nversion = "0.1.0"\ndependencies = [\n'+"".join(f' "{d}",\n' for d in ds)+']\n'
        k=0
        while k<len(out) and out[k][0]<c: k+=1
        out.insert(k,(c,blk))
    open(p,"w",encoding="utf8").write(head+"".join("\n[[package]]\n"+b for _,b in out))
    # build owners
    p="📋️build-owners.json"; t=open(p,encoding="utf8").read()
    on=re.findall(r'^      "name": "([^"]+)",$',t,re.M)
    for c,v in names:
        root=f"{comp}/🗃️apps/{v['icon']}{v['slug']}/📦️packages/🦀️rust"
        blk=f'    {{\n      "name": "{c}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
        nxt=next(x for x in sorted(on) if x>c); a=f'    {{\n      "name": "{nxt}",\n'; assert t.count(a)==1
        t=t.replace(a,blk+a); on.append(c)
    json.loads(t); open(p,"w",encoding="utf8").write(t)
    print(name,"ok",[c for c,_ in names])
def camel_of(slug):
    return "".join(x.capitalize() for x in slug.split("-"))
def parse_and_register(name,dirname,pkg,root_rel):
    src=open(f"{H}/🧩️compositions/{dirname}/{root_rel}",encoding="utf8").read()
    i=src.index("semio_framework_dispatch_macros::dyn_enum_close! {"); j=src.index("\n}\n",i)+3
    block=src[i:j]
    vs=re.findall(r'^\s{8}(\w+)\((.*)\),$',block,re.M)
    assert vs,block
    variants=[];hub_enum=re.search(r'pub enum (\w+): PluginApp',block).group(1)
    for var,ty in vs:
        slug=re.sub(r'(?<!^)(?=[A-Z])','-',var).lower().replace("generation-","generation")
        crates=sorted(set(c.replace("_","-") for c in re.findall(r'\b(semio_[a-z0-9_]+)::',ty)))
        variants.append(dict(slug=f"{name}-{slug}" if not slug.startswith(name) else slug,icon="✏️" if var.endswith("Editor") else "👁️",enum=var+"Apps",variant=var,ty=ty,deps=crates,doc=f"the {slug.replace('-',' ')} surface"))
    return dict(dir=dirname,member=f"🧩️compositions/{dirname}/📦️packages/🦀️rust",pkg=pkg,base=name,variants=variants,hub_enum=hub_enum,block=block,src=src,
        common=["semio-plugin","semio-framework-dispatch-macros","semio-framework-tool-run","semio-framework-job","semio-framework-value","semio-framework-hash","semio-framework","semio-os-kernel"],features={})
def rewrite_root(s,root_rel):
    comp=f"{H}/🧩️compositions/{s['dir']}"
    pre="semio_hub_"+s["base"].replace("-","_")+"_"
    new=s["block"]
    for v in s["variants"]:
        crate="semio_hub_"+v["slug"].replace("-","_")+"_app"
        new=new.replace(f"        {v['variant']}({v['ty']}),",f"        {v['variant']}({crate}::{v['enum']}),")
    froms="".join(f"\nimpl From<{v['ty']}> for {s['hub_enum']} {{\n    fn from(app: {v['ty']}) -> Self {{\n        Self::{v['variant']}(app.into())\n    }}\n}}\n" for v in s["variants"])
    open(f"{comp}/{root_rel}","w",encoding="utf8").write(s["src"].replace(s["block"],new+froms))
def apply_auto(name,dirname,pkg,root_rel="🦀️.rs",extra_paths=None):
    s=parse_and_register(name,dirname,pkg,root_rel)
    SPECS[name]=s
    if extra_paths:
        s["extra_paths"]=extra_paths
    apply(name)
    rewrite_root(s,root_rel)
if __name__=="__main__":
    if sys.argv[1]=="procedural": apply_auto("procedural","🌀️procedural","semio-hub-procedural")
    elif sys.argv[1]=="raster": apply_auto("raster","🖨️raster","semio-hub-raster")
    else: apply(sys.argv[1])
