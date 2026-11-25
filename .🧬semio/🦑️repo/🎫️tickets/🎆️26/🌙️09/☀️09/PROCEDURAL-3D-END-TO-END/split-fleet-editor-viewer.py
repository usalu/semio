#!/usr/bin/env python3
"""Separates the artifact app fleet of a `*-tests` / `*-io-registry` carve crate: the editor and viewer sources become crates of their own, each variant gets a one-variant fleet crate (as split-hub-apps.py), the carve crate nests them. `artifact_kind` (pure, needs only the document schema constant and framework types) moves to a kind crate below both, so the moved sources never depend on the carve crate. Dry run by default; --apply writes; aborts on any unresolved head or lock dependency. usage: split-fleet-editor-viewer.py [txt|binary]... [--apply]"""
import json,os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from carve_lib import *
os.chdir(R)
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
SPECS={
"txt":dict(pkg="semio-s-artifact-stdio-txt-tests",artifact="semio-s-artifact-stdio-txt",art_root=f"{A}/🔤️txt/🦀️.rs",std=f"{A}/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any",stdfile=f"{A}/🔤️txt/🏅️standards/🔖️utf-8/🦀️.rs",fleet="TxtApps",doc="STDIO_TXT_DOCUMENT_SCHEMA",schema_crate="semio-s-artifact-stdio-txt-schema",
  sides=dict(
   editor=dict(local="✏️editor",enum="TxtEditorApps",variant="Editor",ty="VcsArtifactApp<EditorApp<editor::txt::TxtEditor>>",mod="editor",
     provides=["pub use semio_s_artifact_stdio_txt_examples::examples;","pub mod schema {\n    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::snapshot;\n    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::diff;\n    pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations;\n}","pub use schema::snapshot::TxtSnapshot;"],
     deps=["semio-s-artifact-stdio-txt-examples","semio-s-artifact-stdio-txt-schema","semio-s-artifact-stdio-txt-inferences","semio-s-artifact-stdio-txt-mutations"]),
   viewer=dict(local="👁️viewer",enum="TxtViewerApps",variant="Viewer",ty="VcsArtifactApp<ViewerApp<viewer::txt::TxtViewer>>",mod="viewer",
     provides=["pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::snapshot::TxtSnapshot;"],deps=["semio-s-artifact-stdio-txt-schema"])),
  kind=True),
"binary":dict(pkg="semio-s-artifact-stdio-binary-io-registry",artifact="semio-s-artifact-stdio-binary",art_root=f"{A}/💾️binary/🦀️.rs",std=f"{A}/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any",stdfile=f"{A}/💾️binary/🏅️standards/🔖️raw/🦀️.rs",fleet="BinaryApps",doc="STDIO_BINARY_DOCUMENT_SCHEMA",schema_crate="semio-s-artifact-stdio-binary-snapshot",
  sides=dict(
   editor=dict(local=None,enum="BinaryEditorApps",variant="Editor",ty="VcsArtifactApp<EditorApp<editor::binary::BinaryEditor>>",mod="editor",external="semio_s_artifact_stdio_binary_mutations",extdeps=["semio-s-artifact-stdio-binary-mutations"]),
   viewer=dict(local="👁️viewer",enum="BinaryViewerApps",variant="Viewer",ty="VcsArtifactApp<ViewerApp<viewer::binary::BinaryViewer>>",mod="viewer",
     provides=["pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;","pub use semio_s_artifact_stdio_binary_snapshot::STDIO_BINARY_DOCUMENT_SCHEMA;","pub use semio_s_artifact_stdio_binary_mutations::standards::v_raw::subsets::any::schema::mutations::BinaryMutation;"],
     deps=["semio-s-artifact-stdio-binary-snapshot","semio-s-artifact-stdio-binary-mutations"])),
  kind=False),
}
ident=lambda n:n.replace("-","_")
def block(text,start_pat):
    m=re.search(start_pat,text)
    if not m: return None
    i=text.index("{",m.end()-1);d=0
    for j in range(i,len(text)):
        d+= (text[j]=="{")-(text[j]=="}")
        if d==0: return m.start(),j+1
def run(name,apply):
    sp=SPECS[name];pkg=sp["pkg"];E=Edits();log=[f"# fleet separation: {pkg}"];unres=[];warn=[]
    idx=index();mf=idx[pkg];mt=open(mf,encoding="utf8").read();lib=libroot(mf,mt);libt=open(lib,encoding="utf8").read();orig_dir=os.path.dirname(mf)
    mirror=os.path.join(os.path.dirname(lib),"🪞️mirror/🦀️.rs");mir=open(mirror,encoding="utf8").read();mdir=os.path.dirname(mirror)
    depkeys=dep_lines(mt);wsroot=os.path.join(os.path.normpath(os.path.join(orig_dir,re.search(r'^workspace\s*=\s*"([^"]+)"',mt,re.M).group(1))),'Cargo.toml');wst=open(wsroot,encoding="utf8").read();wsd=dep_lines(wst,'workspace.dependencies');wss=sections(wst)
    def realpkg(k,v):
        if 'package' in v: return pkg_of(k,v)
        if 'workspace = true' in v:
            if k in wsd: return pkg_of(k,wsd[k])
            tb=wss.get('workspace.dependencies.'+k)
            if tb:
                m=re.search(r'^package\s*=\s*"([^"]+)"',"\n".join(tb),re.M)
                if m: return m.group(1)
        return k
    inner=[l for l in libt.split("\n") if l.startswith("#![")];ext=[l for l in libt.split("\n") if l.startswith("extern crate ")]
    newcrates=[];fleet_lines=[];newpath={};newm={};tyby={}
    def manifest(cdir,cname,descr,text,extra_paths,extra_names,always=()):
        keep={k:v for k,v in depkeys.items() if re.search(r'\b'+ident(k)+r'\b',text) or k in always}
        externs=[]
        for l in ext:
            m=re.match(r'extern crate (\w+) as (\w+);',l)
            if m and re.search(r'\b'+m.group(2)+r'\b',text):
                externs.append(l)
                for k in depkeys:
                    if ident(k)==m.group(1): keep[k]=depkeys[k]
        wsrel=os.path.relpath(os.path.dirname(wsroot),cdir);deps=[]
        for k,v in sorted(keep.items()):
            pm=re.search(r'path\s*=\s*"([^"]+)"',v)
            if pm: v=v.replace(pm.group(0),f'path = "{rel(cdir,os.path.normpath(os.path.join(orig_dir,pm.group(1))))}"')
            deps.append(f"{k} = {v}")
        for n,p in extra_paths: deps.append(f'{n} = {{ path = "{rel(cdir,p)}" }}')
        E.new(f"{cdir}/Cargo.toml",f'[package]\nworkspace = "{wsrel}"\nname = "{cname}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "{descr}"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "🦀️.rs"\n\n[dependencies]\n'+"\n".join(sorted(deps))+"\n")
        newcrates.append((cname,[realpkg(k,v) for k,v in keep.items()]+extra_names));return externs
    cdirs={}
    def cd(c,base): d=f"{base}/📦️packages/🦀️rust";cdirs[c]=d;return d
    kind=f"{sp['artifact']}-kind";kind_ident=ident(kind)
    if sp["kind"]:
        sf=open(sp["stdfile"],encoding="utf8").read();span=block(sf,r'pub fn artifact_kind\(\)[^{]*\{')
        if not span: unres.append("artifact_kind not found")
        else:
            s0=span[0]
            while sf[:s0].rstrip().split("\n")[-1].lstrip().startswith("//"): s0=sf[:s0].rstrip().rfind("\n",0,len(sf[:s0].rstrip()))+1
            fn=sf[span[0]:span[1]]
            uses=[]
            for m in re.finditer(r'^\s*use\s+[^;]+;\n',sf,re.M): uses.append(m.group(0).strip())
            used=[u for u in uses if any(re.search(r'\b'+w+r'\b',fn) for w in re.findall(r'\b[A-Z]\w+\b',u))]
            kd=f"{sp['std']}/🏷️kind";kcd=cd(kind,kd)
            body="//! 🏷️ The artifact kind of the artifact: pure data below the editor, the viewer and the carve crate.\n\n"+"\n".join(u.replace("crate::{"+sp["doc"]+"}",ident(sp["schema_crate"])+"::"+sp["doc"]).replace("use crate::{"+sp["doc"]+"};","use "+ident(sp["schema_crate"])+"::"+sp["doc"]+";") for u in used)+f"\nuse {ident(sp['schema_crate'])}::{sp['doc']};\n\n"+fn+"\n"
            E.new(f"{kd}/🦀️.rs",body)
            manifest(kcd,kind,f"Artifact kind of {sp['artifact']}",body,[(sp['schema_crate'],None)] if False else [],[sp["schema_crate"]],always=())
            text=E.get(f"{kcd}/Cargo.toml");E.set(f"{kcd}/Cargo.toml",text.replace("\n[dependencies]\n",f'\n[dependencies]\n{sp["schema_crate"]} = {{ path = "{rel(kcd,os.path.dirname(idx[sp["schema_crate"]]))}" }}\n'))
            sf2=sf[:s0]+sf[span[1]:]
            for mm in re.finditer(r'pub mod (artifact_items_\d+) \{',sf2):
                pass
            def addimport(m):
                return m.group(0)
            out="";pos=0
            for mm in list(re.finditer(r'pub mod (artifact_items_\d+) \{',sf2)):
                sp2=block(sf2,r'pub mod '+mm.group(1)+r' \{')
                seg=sf2[sp2[0]:sp2[1]]
                if re.search(r'(?<![.\w])artifact_kind\b(?!\s*:)',seg): sf2=sf2.replace(seg,seg.replace("{\n",f"{{\nuse {kind_ident}::artifact_kind;\n",1),1)
            E.set(sp["stdfile"],sf2)
    for side,sd in sp["sides"].items():
        var=sd["variant"];app=f"{sp['artifact']}-{side}-app";src=f"{sp['artifact']}-{side}";src_id=ident(src)
        if sd["local"]:
            root=f"{sp['std']}/{sd['local']}";cdir=cd(src,root)
            span=block(mir,r'(?:#\[path = "\."\]\s*\n)?pub mod '+sd["mod"]+r' \{')
            if not span: unres.append(f"{side}: mirror block missing");continue
            blk=mir[span[0]:span[1]];E.set(mirror,E.get(mirror).replace(blk,"",1))
            blk=re.sub(r'#\[path = "([^"]+)"\]',lambda m:'#[path = "'+(m.group(1) if m.group(1)=="." else os.path.relpath(os.path.normpath(os.path.join(mdir,m.group(1))),cdir))+'"]',blk)
            files=[os.path.join(dp,f) for dp,_,fs in os.walk(root) for f in fs if f.endswith(".rs") and "🧪️tests" not in dp]
            code="\n".join(open(f,encoding="utf8").read() for f in files)
            heads=collections.Counter(h for h in re.findall(r"crate::(\w+)",code))
            provided=set(re.findall(r'(?:pub (?:use|mod) |pub use [\w:]*::)(\w+)',"\n".join(sd["provides"])))|{sd["mod"]}
            for h in heads:
                if h=="artifact_kind" and sp["kind"]: continue
                if h not in provided and not re.search(r'pub use [\w:]*::'+h+r'\s*;',"\n".join(sd["provides"])): unres.append(f"{side}: crate::{h} not provided")
            for f in files:
                t=open(f,encoding="utf8").read();n=t
                if sp["kind"]: n=n.replace("crate::artifact_kind()",f"{kind_ident}::artifact_kind()")
                if n!=t: E.set(f,n)
            lines=[f"//! 👁️ The {side} surface of `{sp['artifact']}`, one compilation unit of its own."]+inner+[l for l in ext]+sd["provides"]+[blk]
            body="\n".join(lines)+"\n";E.new(f"{cdir}/🦀️.rs",body)
            extra=[(n,os.path.dirname(idx[n])) for n in sd["deps"]]+([(kind,cdirs[kind])] if sp["kind"] else [])
            manifest(cdir,src,f"{side} of {sp['artifact']}",code+body,extra,[n for n,_ in extra])
            tyfull=sd["ty"].replace(sd["mod"]+"::",src_id+"::"+sd["mod"]+"::",1)
            dep_src=[(src,cdirs[src])];newpath[side]=src_id
        else:
            tyfull=sd["ty"].replace(sd["mod"]+"::",sd["external"]+"::"+sd["mod"]+"::",1);dep_src=[(n,os.path.dirname(idx[n])) for n in sd["extdeps"]];newpath[side]=sd["external"]
            span=block(mir,r'(?:#\[path = "\."\]\s*\n)?pub mod '+sd["mod"]+r' \{')
            if span: E.set(mirror,E.get(mirror).replace(mir[span[0]:span[1]],"",1))
        tyby[side]=tyfull
        fd=f"{sp['std']}/🗃️apps/{'✏️' if side=='editor' else '👁️'}{side}-app";fcd=cd(app,fd)
        E.new(f"{fd}/🦀️.rs",f"//! 🗃️ The closed runtime app fleet of the {side} surface of `{sp['artifact']}`: one crate per app so the app machinery is never instantiated in one compilation unit.\n"+"\n".join(inner)+"\n"+"\n".join(ext)+f"\n\nuse semio_framework_dispatch_macros::dyn_enum_close;\nuse semio_plugin::plugin_app_close_prelude::*;\n\ndyn_enum_close! {{\n    pub enum {sd['enum']}: PluginApp {{\n        {var}({tyfull}),\n    }}\n}}\n")
        manifest(fcd,app,f"App fleet of the {side} surface of {sp['artifact']}","semio_plugin semio_framework_dispatch_macros semio_os_kernel",dep_src,[n for n,_ in dep_src],always=("semio-plugin","semio-framework-dispatch-macros","semio-os-kernel"))
        fleet_lines.append((side,sd,app))
    # no compatibility re-exports: every user of the moved modules names the owning crate
    for side,sd in sp["sides"].items():
        L0=E.get(lib);E.set(lib,re.sub(r'^pub use mirror::'+sd["mod"]+r';\n','',L0,flags=re.M))
    moved_prefix=[f"{sp['std']}/{sd['local']}" for sd in sp["sides"].values() if sd["local"]]
    T=tree(lib);own=[f for f in T if not any(f.startswith(m) for m in moved_prefix)]
    def repoint(text):
        for side,sd in sp["sides"].items():
            text=re.sub(r'\bcrate::'+sd["mod"]+r'::',newpath[side]+"::"+sd["mod"]+"::",text)
        return text
    for f in own+[lib,mirror]:
        t0=E.get(f);n=repoint(t0)
        if n!=t0: E.set(f,n)
        for side,sd in sp["sides"].items():
            if re.search(r'\bcrate::'+sd["mod"]+r'\b',n): unres.append(f"{f}: crate::{sd['mod']} left")
    libroots,mounted,mfs=world(idx);mdirs=dict(mfs)
    import subprocess
    users=[f[2:] for f in subprocess.run(["grep","-rIlE","--include=*.rs",ident(pkg)+"::(editor|viewer)::|"+ident(sp["artifact"])+"::artifact_kind|crate::artifact_kind","."]+[f"--exclude-dir={x}" for x in SKIP],capture_output=True,text=True).stdout.split("\n") if f]
    deps_for=collections.defaultdict(set)
    for f in users:
        if f in E.t and f not in (lib,mirror): pass
        t0=E.get(f);n=t0
        for side,sd in sp["sides"].items(): n=re.sub(r'\b'+ident(pkg)+r'::'+sd["mod"]+r'::',newpath[side]+"::"+sd["mod"]+"::",n)
        if sp["kind"]:
            n=re.sub(r'\b'+ident(sp["artifact"])+r'::artifact_kind\b',kind_ident+"::artifact_kind",n)
            if f in {x for x in T} and f.startswith(os.path.dirname(sp["art_root"])) and not f.startswith(sp["std"]): n=re.sub(r'\bcrate::artifact_kind\b',kind_ident+"::artifact_kind",n)
        if n!=t0:
            E.set(f,n);ow=crate_of(f,libroots,mounted,mdirs)
            if ow is None: unres.append(f"{f}: no owning crate");continue
            for c,_ in newcrates:
                if re.search(r'\b'+ident(c)+r'::',n) and ow!=c: deps_for[ow].add(c)
    for ow,cs in deps_for.items():
        mfp=idx.get(ow) or newm.get(ow)
        if not mfp: continue
        tt=E.get(mfp);a="".join(f'{c} = {{ path = "{rel(os.path.dirname(mfp),cdirs[c])}" }}\n' for c in sorted(cs) if f"{c} =" not in tt)
        if a: E.set(mfp,re.sub(r'(^\[dependencies\]\n)',lambda m:m.group(1)+a,tt,count=1,flags=re.M))
    extra_users=deps_for
    # carve crate lib: replace the dyn_enum_close block
    libt=E.get(lib);span=block(libt,r'dyn_enum_close!\s*\{');
    if span:
        L=libt;s0=span[0];vs=dict(re.findall(r'^\s{8}(\w+)\((.*)\),$',L[span[0]:span[1]],re.M))
        nest="dyn_enum_close! {\n    pub enum "+sp["fleet"]+": PluginApp {\n"+"".join(f"        {sd['variant']}({ident(app)}::{sd['enum']}),\n" for _,sd,app in fleet_lines)+"    }\n}\n"
        froms="".join(f"\nimpl From<{tyby[s_]}> for {sp['fleet']} {{\n    fn from(app: {tyby[s_]}) -> Self {{\n        Self::{sd['variant']}(app.into())\n    }}\n}}\n" for s_,sd,_ in fleet_lines)
        L=L[:s0]+nest+froms+L[span[1]:]
        if sp["kind"]: L=re.sub(r'artifact_items_\d+::\{([^}]*)\}',lambda m:m.group(0).replace("artifact_kind, ","").replace(", artifact_kind","").replace("artifact_kind",""),L)
        E.set(lib,L)
    else: unres.append("dyn_enum_close block not found")
    # carve crate manifest + artifact root
    t=E.get(mf);add="".join(f'{c} = {{ path = "{rel(orig_dir,cdirs[c])}" }}\n' for c,_ in newcrates if c.endswith("-app") or c==kind or True)
    E.set(mf,re.sub(r'(^\[dependencies\]\n)',lambda m:m.group(1)+add,t,count=1,flags=re.M))
    if sp["kind"]:
        ar=E.get(sp["art_root"]);ar2=re.sub(r'artifact_kind,\s*','',ar,count=1) if re.search(r'\{[^}]*artifact_kind,',ar) else ar.replace(", artifact_kind","")
        E.set(sp["art_root"],ar2)
        am=idx[sp["artifact"]];at=E.get(am);E.set(am,re.sub(r'(^\[dependencies\]\n)',lambda m:m.group(1)+f'{kind} = {{ path = "{rel(os.path.dirname(am),cdirs[kind])}" }}\n',at,count=1,flags=re.M))
    # workspaces, owners, locks
    wsm=[l for l in ls("*Cargo.toml") if os.path.exists(l) and "[workspace]" in open(l,encoding="utf8").read()];pdir=os.path.dirname(mf)
    for w in wsm:
        t=E.get(w);wd=os.path.dirname(w) or ".";me=os.path.relpath(pdir,wd)
        if f'"{me}"' in t: E.set(w,t.replace(f'    "{me}",\n',"".join(f'    "{os.path.relpath(cdirs[c],wd)}",\n' for c,_ in newcrates)+f'    "{me}",\n',1))
    bo="📋️build-owners.json";t=E.get(bo);on=sorted(re.findall(r'^      "name": "([^"]+)",$',t,re.M))
    for c,_ in newcrates:
        root=cdirs[c];blk=f'    {{\n      "name": "{c}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
        nx=next(x for x in on if x>c);a=f'    {{\n      "name": "{nx}",\n'
        if t.count(a)!=1: unres.append(f"build-owners anchor {nx}");continue
        t=t.replace(a,blk+a);on=sorted(on+[c])
    E.set(bo,t);nl=0
    for lk in ls("*Cargo.lock"):
        lt=open(lk,encoding="utf8").read()
        if f'\nname = "{pkg}"\n' not in lt: continue
        head,blocks=lock_blocks(lt);bd=dict(blocks);_,od=lock_deps(bd[pkg]);byname={lockname(e):e for e in od};out=[]
        names_in={n for n,_ in blocks}
        for nm,b in blocks:
            if nm==pkg: b=set_lock_deps(b,od+[c for c,_ in newcrates if c.endswith("-app") or True])
            if nm in extra_users: b=set_lock_deps(b,lock_deps(b)[1]+sorted(extra_users[nm]))
            out.append((nm,b))
        ver=re.search(r'version = "([^"]+)"',bd[pkg]).group(1)
        for c,pk in newcrates:
            ds=[]
            for p_ in pk:
                if p_ in byname: ds.append(byname[p_])
                elif p_ in names_in or p_ in {x for x,_ in newcrates}: ds.append(p_)
                else: warn.append(f"{lk}: {p_} of {c} absent from the lock (lock already inconsistent for {pkg}); left out")
            blk=f'name = "{c}"\nversion = "{ver}"\ndependencies = [\n'+"".join(f' "{d}",\n' for d in sorted(set(ds)))+']\n';k=0
            while k<len(out) and out[k][0]<c:k+=1
            out.insert(k,(c,blk))
        E.set(lk,lock_join(head,out));nl+=1
    log+=[f"new crates: {[c for c,_ in newcrates]}",f"edits: {len(E.t)}, lock files: {nl}"]
    log+=["","WARNINGS:"]+warn+["","UNRESOLVED (apply refuses):"]+unres if unres else ["","WARNINGS:"]+warn+["","no unresolved reference"]
    out=f"{os.path.dirname(os.path.abspath(__file__))}/📓️dry-run-fleet-{name}.md";open(out,"w",encoding="utf8").write("\n".join(log)+"\n");print(out,"unresolved:",len(unres))
    if os.environ.get("DUMP"): E.dump_new(os.environ["DUMP"])
    if apply:
        if unres: sys.exit("refusing to apply: unresolved references")
        E.write();print("applied",name)
if __name__=="__main__":
    for n in [a for a in sys.argv[1:] if a!="--apply"] or SPECS: run(n,"--apply" in sys.argv)
