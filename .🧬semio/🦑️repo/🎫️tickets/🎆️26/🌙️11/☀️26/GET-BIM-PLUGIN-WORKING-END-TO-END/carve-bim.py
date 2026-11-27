import os,re,sys
R="/Users/ueli/Documents/semio"
M=R+"/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
A=M+"/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED=A+"/✏️editor"; SN=A+"/🧬️schema/📸️snapshot"
PK="/📦️packages/🦀️rust"
rel=lambda p,d: os.path.relpath(p,d)
def rd(p): return open(p,encoding="utf8").read()
def wr(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True); open(p,"w",encoding="utf8").write(t)
def rebase(text,old,new,name,desc,extra=(),testkit=()):
    def f(m): return '{ path = "'+rel(os.path.normpath(os.path.join(old,m.group(1))),new)+'"'
    text=re.sub(r'\{ path = "([^"]*)"',f,text)
    text=re.sub(r'(?m)^workspace = "[^"]*"','workspace = "'+rel(M,new)+'"',text)
    text=re.sub(r'(?m)^name = "[^"]*"','name = "'+name+'"',text,count=1)
    text=re.sub(r'(?m)^description = "[^"]*"','description = "'+desc+'"',text,count=1)
    if extra:
        rows="".join(f'{n} = {{ path = "{rel(d,new)}" }}\n' for n,d in extra)
        text=re.sub(r'(?m)^\[features\]',lambda m:rows+"\n[features]",text,count=1)
        drows="".join(f'{n} = {{ path = "{rel(d,new)}", features = ["testkit"] }}\n' for n,d in extra)
        if "[dev-dependencies]" in text: text=text.rstrip("\n")+"\n"+drows
    if testkit:
        text=re.sub(r'(?m)^(testkit = \[.*?)\]$',lambda m:m.group(1)+"".join(f', "{t}/testkit"' for t in testkit)+"]",text,count=1)
    return text
EXT=lambda *a:"".join(f"extern crate {c} as {n};\n" for c,n in a)
K=("semio_os_kernel","dsl"),("semio_os_kernel","protocol"),("semio_os_kernel","store"),("semio_framework_schema","schema"),("semio_framework_value_derive","value_derive")
def nest(mods,body,base):
    out="";ind=""
    for m in mods:
        out+=f'{ind}#[path = "."]\n{ind}pub mod {m} {{\n';ind+="    "
    out+="".join(ind+l+"\n" for l in body.split("\n"))
    for m in mods[::-1]:
        ind=ind[:-4];out+=f"{ind}}}\n"
    return out
def cname(n): return n.replace("-","_")
def mount(libdir,file,mod,pub="pub ",attrs=""):
    return f'{attrs}#[path = "{rel(file,libdir)}"]\n{pub}mod {mod};'

def newcrate(src,srcdir,name,dirp,desc,lib,extra=(),testkit=(),drop_dev=False):
    t=rebase(rd(src+"/Cargo.toml"),srcdir,dirp,name,desc,extra,testkit)
    if drop_dev: t=t[:t.index("\n[dev-dependencies]")].rstrip("\n")+"\n" if "[dev-dependencies]" in t else t
    wr(dirp+"/Cargo.toml",t); wr(dirp+"/🦀️.rs",lib)

def schema():
    sd=A+"/🧬️schema"; sp=sd+PK
    src=rd(SN+"/💠️values/🦀️.rs").split("\n")
    idx=[i for i,l in enumerate(src) if re.match(r'^pub (struct|enum) ',l)]
    names=[re.match(r'^pub (?:struct|enum) (\w+)',src[i]).group(1) for i in idx]
    st=[]
    for i in idx:
        j=i
        while j>0 and (src[j-1].startswith("///") or src[j-1].startswith("#[")): j-=1
        st.append(j)
    st.append(len(src))
    blocks={n:"\n".join(src[st[k]:st[k+1]]).rstrip("\n")+"\n" for k,n in enumerate(names)}
    G={}
    for n in "Point2 Point3 Vertex Rgb Slope ExprPoint ExprPoint3 Axis Profile ParametricProfile SolidAxis SolidShape SectionBox ViewPlane ViewCrop AnchorEnd WallSide AnnotationAnchor AreaMeasure SpaceBoundary".split(): G[n]="geometry"
    for n in "PropertyKind TemplateTarget PropertyValue PropertyDef ClassificationItem Phase Swing ScheduleCategory ScheduleField ScheduleOp ScheduleKey ScheduleColumn ScheduleSort ScheduleFilter ScheduleGroup".split(): G[n]="properties"
    for n in "ElementClass RuleKind RuleSeverity RuleScope ElementSelector ClashRef IssueStatus IssuePriority ViewCamera IssueViewpoint".split(): G[n]="issues"
    for n in names: G.setdefault(n,"values")
    order=["geometry","properties","issues","values"]
    up={"geometry":[],"properties":[],"issues":["geometry","properties"],"values":["geometry","properties","issues"]}
    cr={"geometry":"semio-s-artifact-bim-model-geometry-values","properties":"semio-s-artifact-bim-model-properties-values","issues":"semio-s-artifact-bim-model-issues-values","values":"semio-s-artifact-bim-model-values"}
    top={"geometry":SN+"/📐️geometry-values","properties":SN+"/🔍️properties-values","issues":SN+"/🚨️issues-values","values":SN+"/💠️values"}
    impls={"anchors-values":("🧲️anchors-values","geometry","anchors_values"),"phases-values":("🧮️phases-values","properties","phases_values"),"schedule-kit-values":("🔩️schedule-kit-values","properties","schedule_kit_values"),"property-kit-values":("🪛️property-kit-values","properties","property_kit_values"),"coordination-values":("⚙️coordination-values","issues","coordination_values"),"views-values":("🧷️views-values","values","views_values"),"sheets-values":("🔧️sheets-values","values","sheets_values")}
    rank={g:i for i,g in enumerate(order)}
    for k,(d,g,m) in impls.items():
        body=rd(SN+"/"+d+"/🦀️.rs")
        for n in names:
            if re.search(r'\b'+n+r'\b',body) and rank[G[n]]>rank[g]: sys.exit(f"{k} uses {n} of later group {G[n]}")
    for n in names:
        for m in names:
            if m!=n and re.search(r'\b'+m+r'\b',re.sub(r'///.*','',blocks[n])) and rank[G[m]]>rank[G[n]]: sys.exit(f"{n} uses later {m}")
    doc={"geometry":"📐️ Geometry value types of `s.bim.model@1`: points, vertices, profiles, solid shapes, anchors and view frames.","properties":"🔍️ Property and schedule value types of `s.bim.model@1`: property definitions and values, phases and schedule keys.","issues":"🚨️ Rule and issue value types of `s.bim.model@1`: element classes, rules, selectors, issues and camera viewpoints."}
    sf="standards::v1::subsets::any::schema::snapshot"
    for g in order:
        file=top[g]+"/🦀️.rs"
        head=(src[0]+"\n" if g=="values" else "//! "+doc[g]+"\n")+"\n"
        for u in up[g]: head+=f"pub use {cname(cr[u])}::{sf}::values::*;\n"
        if up[g]: head+="\n"
        body="\n".join(blocks[n] for n in names if G[n]==g)
        if g=="values": wr(file,head+body)
        else: wr(file,head+body)
        libdir=top[g]+PK
        mods=[mount(libdir,file,"values")]
        for k,(d,og,m) in impls.items():
            if og==g: mods.append(mount(libdir,SN+"/"+d+"/🦀️.rs",m))
        lib=f"//! {doc.get(g,'💠️ Value types of `s.bim.model@1`: the building elements, views, sheets and families.')}\n\n"+EXT(("semio_os_kernel","dsl"),("semio_framework_schema","schema"),("semio_framework_value_derive","value_derive"))+"\n"+nest(["standards","v1","subsets","any","schema","snapshot"],"\n".join(mods),libdir)
        extra=[(cr[u],top[u]+PK) for u in up[g]]
        sdesc=f"Component crate of semio-s-artifact-bim-model: {cr[g].split('bim-model-')[1]}"
        newcrate(sp,sp,cr[g],libdir,sdesc,lib,extra,[cr[u] for u in up[g]],True)
    return cr,top,impls

def schema_facade(cr,top,impls):
    sp=A+"/🧬️schema"+PK
    lib=rd(sp+"/🦀️.rs")
    a=lib.index('                    #[path = "."]\n                    pub mod snapshot {')
    b=lib.index('                    #[path = "."]\n                    pub mod inferences {')
    sf="standards::v1::subsets::any::schema::snapshot"
    by={}
    for k,(d,g,m) in impls.items(): by.setdefault(g,[]).append(m)
    by["values"]=by.get("values",[])+["values"]
    rows=""
    for g in ["geometry","properties","issues","values"]:
        ms=sorted(by.get(g,[]));
        if ms: rows+=f"                        pub use {cname(cr[g])}::{sf}::"+(ms[0] if len(ms)==1 else "{"+", ".join(ms)+"}")+";\n"
    new='                    #[path = "."]\n                    pub mod snapshot {\n'+rows+'                    }\n'
    lib=lib[:a]+new+lib[b:]
    lib=lib.replace("a path-preserving carve of the artifact root mounts.","the artifact schema descriptor; the snapshot value types are carved into the `*-values` crates and re-exported here.")
    wr(sp+"/🦀️.rs",lib)
    t=rd(sp+"/Cargo.toml")
    rows="".join(f'{n} = {{ path = "{rel(top[g]+PK,sp)}" }}\n' for g,n in cr.items())
    t=re.sub(r'(?m)^\[features\]',lambda m:rows+"\n[features]",t,count=1)
    t=re.sub(r'(?m)^(testkit = \[.*?)\]$',lambda m:m.group(1)+"".join(f', "{n}/testkit"' for n in cr.values())+"]",t,count=1)
    wr(sp+"/Cargo.toml",t)

if __name__=="__main__" and "schema" in sys.argv:
    cr,top,impls=schema(); schema_facade(cr,top,impls)

def kit():
    kd=ED+"/🧰️kit"; kp=kd+PK
    W=ED+"/🎭️modes/✏️edit/🪟️windows"
    wins={"plan":"🗺️plan","world":"🧊️world","section":"📐️section","schedule":"🧮️schedule","sheet":"📄️sheet"}
    P="semio-s-artifact-bim-model-"
    core=(P+"kit-core",kd+"/🧱️core"+PK); util=(P+"utilities",ED+"/🪛️utilities"+PK)
    # source edits
    f=kd+"/🦀️.rs"; s=rd(f); s=s.replace("pub(crate) use semio_s_artifact_bim_model_window_config::window_config;","pub use semio_s_artifact_bim_model_window_config::window_config;"); wr(f,s)
    f=ED+"/🪛️utilities/🦀️.rs"; s=rd(f)
    s=s.replace("use crate::editor::bim::modes::edit::windows::{plan, section, sheet, world};\n","")
    s=s.replace("//#region 🔖️Registry","//#region 🔖️WindowKinds\n/// 🪟️ The ids of the window kinds the registry addresses; each window crate publishes its own `WINDOW_KIND_ID` from here.\npub mod window_kinds {\n    pub const PLAN: &str = \"bim-edit-plan\";\n    pub const SECTION: &str = \"bim-edit-section\";\n    pub const SHEET: &str = \"bim-edit-sheet\";\n    pub use semio_s_artifact_bim_model_window_config::world::WINDOW_KIND_ID as WORLD;\n}\n//#endregion 🔖️WindowKinds\n\n//#region 🔖️Registry",1)
    s=s.replace("use semio_plugin::UtilityRef;\n","use semio_plugin::UtilityRef;\nuse window_kinds::{PLAN, SECTION, SHEET, WORLD};\n",1)
    s=re.sub(r'\b(plan|world|section|sheet)::WINDOW_KIND_ID',lambda m:m.group(1).upper(),s)
    wr(f,s)
    for w,k in (("plan","PLAN"),("section","SECTION"),("sheet","SHEET")):
        f=W+"/"+wins[w]+"/🦀️.rs"; s=rd(f)
        s=re.sub(r'pub const WINDOW_KIND_ID: &str = "bim-edit-\w+";',f'pub const WINDOW_KIND_ID: &str = crate::editor::bim::utilities::window_kinds::{k};',s); wr(f,s)
    # kit-core
    e=EXT(*K)
    lib="//! 🧱️ BIM editor kit core: the UI admission helpers every taxonomy node shares, the id mint, and the window-config declaration macro.\n\n"+e+"\n"+nest(["editor","bim","kit"],mount(core[1],kd+"/🦀️.rs","component","","")+"\npub use component::*;",core[1])
    newcrate(kp,kp,core[0],core[1],"Component crate of semio-s-artifact-bim-model: kit-core",lib)
    # utilities
    lib="//! 🪛️ BIM utility registry crate: the tools a window arms and the window kind ids they address.\n\n"+e+"\n"+nest(["editor","bim","utilities"],mount(util[1],ED+"/🪛️utilities/🦀️.rs","component","","")+"\npub use component::*;",util[1])
    newcrate(kp,kp,util[0],util[1],"Component crate of semio-s-artifact-bim-model: utilities",lib)
    crates=[core,util]
    for w,d in wins.items():
        dirp=W+"/"+d+PK; name=P+"window-"+w
        src=rd(W+"/"+d+"/🦀️.rs")+"".join(rd(p) for p in [])
        uses=[("kit",core)]+([("utilities",util)] if "crate::editor::bim::utilities" in src else [])
        pre="".join(f"        use {cname(c[0])}::editor::bim::{n};\n" for n,c in uses)
        body=pre+nest(["modes","edit","windows"],"#[path = \".\"]\npub mod "+w+" {\n    "+mount(dirp,W+"/"+d+"/🦀️.rs","component","","").replace("\n","\n    ")+"\n    pub use component::*;\n}",dirp)
        lib=f"//! {d.split('/')[-1][:2]} BIM `{w}` window crate: the window kind definition, its configuration and its records.\n\n"+e+"\n"+nest(["editor","bim"],body,dirp)
        newcrate(kp,kp,name,dirp,f"Component crate of semio-s-artifact-bim-model: window-{w}",lib,[(c[0],c[1]) for _,c in uses],[c[0] for _,c in uses])
        crates.append((name,dirp))
    return crates

def kit_facade(crates):
    kp=ED+"/🧰️kit"+PK
    P="semio-s-artifact-bim-model-"
    by={n:d for n,d in crates}
    pre="standards" 
    rows="    pub use %s::editor::bim::kit;\n    pub use %s::editor::bim::utilities;\n"%(cname(P+"kit-core"),cname(P+"utilities"))
    win="".join(f"            pub use {cname(P+'window-'+w)}::editor::bim::modes::edit::windows::{w};\n" for w in ["plan","schedule","section","sheet","world"])
    lib=("//! 🧰️ kit of `semio-s-artifact-bim-model`: the facade that re-exports the carved kit core, the utility registry and the five window crates under their original paths.\n\n"
         "pub mod editor {\n    pub mod bim {\n"+"".join("        "+l+"\n" for l in rows.strip("\n").split("\n"))+
         "        pub mod modes {\n            pub mod edit {\n                pub mod windows {\n"+"".join("    "+l+"\n" for l in win.strip("\n").split("\n"))+"                }\n            }\n        }\n    }\n}\n")
    wr(kp+"/🦀️.rs",lib)
    head='[package]\nworkspace = "%s"\nname = "semio-s-artifact-bim-model-kit"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Component crate of semio-s-artifact-bim-model: kit"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "🦀️.rs"\n\n[dependencies]\n'%rel(M,kp)
    deps="".join(f'{n} = {{ path = "{rel(d,kp)}" }}\n' for n,d in crates)
    feat="\n[features]\ntestkit = ["+", ".join(f'"{n}/testkit"' for n,_ in crates)+"]\n"
    wr(kp+"/Cargo.toml",head+deps+feat)

def interaction():
    ip=ED+"/🕹️interaction"+PK
    P="semio-s-artifact-bim-model-"
    specs=[("entities-tests",ED+"/🧩️entities/🧪️tests/🔬️unit",["editor","bim","entities"]),("entities-families-tests",ED+"/🧩️entities/🧬️families/🧪️tests/🔬️unit",["editor","bim","entities","families"])]
    out=[]
    for n,d,mods in specs:
        dirp=d+PK
        lib=f"//! 🔬️ {n} of `semio-s-artifact-bim-model`: the entity table tests, carved out of the interaction crate.\n\n"+EXT(("semio_os_kernel","protocol"),("semio_framework_schema","schema"))+"\n"
        mods2=["editor","bim","entities"]+(["families"] if "families" in mods else [])
        lib+=nest(mods2,"#[cfg(test)]\n"+mount(dirp,d+"/🦀️.rs","tests"),dirp)
        newcrate(ip,ip,P+n,dirp,f"Component crate of semio-s-artifact-bim-model: {n}",lib)
        out.append((P+n,dirp))
    f=ip+"/🦀️.rs"; s=rd(f)
    a=s.index('        #[path = "."]\n        pub mod entities {'); b=s.index('        #[path = "."]\n        pub mod interaction {')
    s=s[:a]+s[b:]
    s=s.replace("extern crate semio_os_kernel as protocol;\n","")
    wr(f,s)
    return out

if __name__=="__main__" and "kit" in sys.argv:
    cs=kit(); kit_facade(cs); print(cs)
if __name__=="__main__" and "interaction" in sys.argv:
    print(interaction())

def registries():
    P="semio-s-artifact-bim-model-"
    W=ED+"/🎭️modes/✏️edit/🪟️windows"
    new=[("geometry-values",SN+"/📐️geometry-values"),("properties-values",SN+"/🔍️properties-values"),("issues-values",SN+"/🚨️issues-values"),("values",SN+"/💠️values"),
     ("kit-core",ED+"/🧰️kit/🧱️core"),("utilities",ED+"/🪛️utilities"),
     ("window-plan",W+"/🗺️plan"),("window-world",W+"/🧊️world"),("window-section",W+"/📐️section"),("window-schedule",W+"/🧮️schedule"),("window-sheet",W+"/📄️sheet"),
     ("entities-tests",ED+"/🧩️entities/🧪️tests/🔬️unit"),("entities-families-tests",ED+"/🧩️entities/🧬️families/🧪️tests/🔬️unit")]
    new=[(P+n,d+PK) for n,d in new]
    # workspace members
    f=M+"/Cargo.toml"; t=rd(f)
    m=re.search(r'members = \[(.*?)\n\]',t,re.S)
    items=re.findall(r'"([^"]+)"',m.group(1))
    items=sorted(set(items)|{rel(d,M) for _,d in new})
    t=t[:m.start()]+"members = [\n"+"".join(f'    "{i}",\n' for i in items).rstrip(",\n").join(["",""])+"\n]"+t[m.end():]
    wr(f,t)
    # artifact root manifest
    f=M+"/📦️packages/🦀️rust/Cargo.toml"; t=rd(f); r=M+"/📦️packages/🦀️rust"
    done=new[0][0]+" = " in t
    deps="".join(f'{n} = {{ path = "{rel(d,r)}" }}\n' for n,d in new)
    ddeps="".join(f'{n} = {{ path = "{rel(d,r)}", features = ["testkit"] }}\n' for n,d in new)
    t=re.sub(r'(?m)^\[features\]',lambda x:deps+"\n[features]",t,count=1)
    t=re.sub(r'(?m)^(testkit = \[.*?)\]$',lambda x:x.group(1)+"".join(f', "{n}/testkit"' for n,_ in new)+"]",t,count=1)
    t=t.rstrip("\n")+"\n"+ddeps
    if not done: wr(f,t)
    # script.ts
    f=M+"/📦️packages/🦀️rust/📜️script.ts"; t=rd(f)
    mm=re.search(r'packages: \[(.*?)\]',t,re.S); names=re.findall(r'"([^"]+)"',mm.group(1))
    names=sorted(set(names)|{n for n,_ in new})
    t=t[:mm.start()]+"packages: ["+", ".join(f'"{n}"' for n in names)+"]"+t[mm.end():]
    wr(f,t)
    # build owners
    f=R+"/📋️build-owners.json"; t=rd(f)
    for n,d in new:
        if f'"name": "{n}"' in t: continue
        row=f'    {{\n      "name": "{n}",\n      "root": "{rel(d,R)}",\n      "manifest": "{rel(d,R)}/Cargo.toml",\n      "packages": []\n    }},\n'
        ms=list(re.finditer(r'    \{\n      "name": "([^"]+)",\n',t))
        pos=None
        for x in ms:
            if x.group(1).startswith("semio-s-artifact-bim-model") and x.group(1)>n: pos=x.start(); break
        if pos is None:
            last=[x for x in ms if x.group(1).startswith("semio-s-artifact-bim-model")][-1]
            pos=t.index("    },\n",last.start())+len("    },\n")
        t=t[:pos]+row+t[pos:]
    wr(f,t)
    # cargo config
    f=R+"/.cargo/config.toml"; t=rd(f)
    old='# 🪟️ Compile the five BIM window owners within the library worker\'s memory grant.\n[profile.dev.package.semio-s-artifact-bim-model-kit]\nopt-level = 1\ncodegen-units = 64\n'
    assert old in t
    new_s="# 🪟️ Compile the five BIM window owners within the library worker's memory grant.\n"+"".join(f"[profile.dev.package.{P}window-{w}]\nopt-level = 1\ncodegen-units = 64\n\n" for w in ["plan","world","section","schedule","sheet"]).rstrip("\n")+"\n"
    t=t.replace(old,new_s)
    old2='[profile.dev.package.semio-s-artifact-bim-model-schema]\nopt-level = 1\ncodegen-units = 64\n'
    assert old2 in t
    t=t.replace(old2,"".join(f"[profile.dev.package.{P}{n}]\nopt-level = 1\ncodegen-units = 64\n\n" for n in ["geometry-values","properties-values","issues-values","values"]).rstrip("\n")+"\n")
    wr(f,t)
    # oracle subject hosts
    f=A+"/🔮️oracles/🔣️.json"; t=rd(f)
    anchor='    {\n      "implementation": "rust",\n      "package": "semio-s-artifact-bim-model-schema",'
    assert anchor in t
    rows="".join(f'    {{\n      "implementation": "rust",\n      "package": "{n}",\n      "path": "{rel(d,R)}",\n      "rationale": "🧱️ The subject half of the cases of this subset drives the production code this crate owns."\n    }},\n' for n,d in new[:4])
    t=t.replace(anchor,rows+anchor,1); wr(f,t)

if __name__=="__main__" and "registries" in sys.argv: registries()
