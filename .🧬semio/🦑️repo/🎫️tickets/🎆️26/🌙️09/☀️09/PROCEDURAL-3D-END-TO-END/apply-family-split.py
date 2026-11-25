#!/usr/bin/env python3
"""Family split without facade or re-exports of moved modules. Dry run by default; --apply writes. usage: apply-family-split.py <spec>... [--apply]
Every moved module group becomes one crate (flat root: `mod x; pub use x::*;`), the remaining root crate drops the mounts, every consumer is repointed to the owning crate. Aborts on any unresolved reference (dry run lists them)."""
import json,os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from carve_lib import *
os.chdir(R)
def S(pkg,parent,groups,kind="paths",icon="🧱️",provides=None): return dict(pkg=pkg,parent=parent,groups=groups,kind=kind,icon=icon,provides=provides or {})
RASTER_PROVIDES={"panels":"semio_s_artifact_raster_raster_editor_panels::panels","modes":"semio_s_artifact_raster_raster_editor_modes::modes","commands":"semio_s_artifact_raster_raster_editor_commands::commands","RasterCommand":"semio_s_artifact_raster_raster_editor_commands::RasterCommand","config":"semio_s_artifact_raster_raster_editor_session::config","presence":"semio_s_artifact_raster_raster_editor_session::presence","terminology":"semio_s_artifact_raster_raster_editor_session::terminology"}
SPECS={
"pdf":S("semio-s-artifact-stdio-pdf-model","model",{"core":["object_model","document_metadata","navigation"],"content":["content_operators","colour_and_graphics_state","xobjects"],"font":["font_model"],"annotations":["annotations_and_forms"],"page":["page_model"]},kind="types"),
"dwg-objects":S("semio-s-artifact-stdio-dwg-snapshot-objects","objects",{"metadata":["metadata"],"xrecord":["xrecord"],"tables":["tables"],"entities":["entities"],"associativity":["associativity"],"styles":["styles"],"blocks":["blocks"],"constraints":["constraints"]}),
"gltf":S("semio-s-artifact-stdio-gltf-model-snapshot",None,{"base":["attribute_map_and_defaults","source_form_and_json"],"scene":["scene_graph","cameras"],"materials":["materials_and_textures"],"animation":["skins_and_animations"],"document":["document_and_decoded_input","accessors_and_buffers"]}),
"raster-editor":S("semio-s-artifact-raster-raster-editor",None,{"ingress":["base64_ingress"],"work":["original_work"],"assembly":["app_assembly"],"store":["store_preparation"],"core":["app","file_import","media_export","args_bridge","retained_commands"]},provides=RASTER_PROVIDES),
"raster-session":S("semio-s-artifact-raster-raster-editor-session",None,{"selection":["selection","layer_selection"],"config":["config"],"presence":["presence"],"terminology":["terminology"]}),
}
WARN=[]
def ident(n): return n.replace("-","_")
def run(key,apply):
    sp=SPECS[key];pkg=sp["pkg"];E=Edits();log=[f"# {key}: {pkg}"];unres=[];warn=[]
    idx=index();mf=idx[pkg];mt=open(mf,encoding="utf8").read();entry=libroot(mf,mt);T=tree(entry)
    names={f:m["chain"][-1] if m["chain"] else None for f,m in T.items()}
    parent_of={c:f for f,m in T.items() for _,c in m["children"]}
    want={n:g for g,ns in sp["groups"].items() for n in ns};found={}
    for f,m in T.items():
        n=names[f]
        if n in want:
            par=names.get(parent_of.get(f))
            if sp["parent"] is None or par==sp["parent"]: found.setdefault(n,[]).append(f)
    for n in want:
        if len(found.get(n,[]))!=1: unres.append(f"module {n}: {len(found.get(n,[]))} matches {found.get(n,[])}")
    if unres: return finish(key,log,unres,E,False)
    fmod={n:found[n][0] for n in want};gmods=collections.defaultdict(list)
    for n,g in want.items(): gmods[g].append(n)
    gfirst={g:ns[0] for g,ns in gmods.items()}
    def subtree(f):
        out=[f]
        for _,c in T[f]["children"]: out+=subtree(c)
        return out
    moved_files={f for n in fmod.values() for f in subtree(n)}
    parent_file={n:parent_of[fmod[n]] for n in want}
    crate=lambda g:f"{pkg}-{g}"
    ptext=mt;depkeys=dep_lines(mt);orig_dir=os.path.dirname(mf)
    wsroot=os.path.join(os.path.normpath(os.path.join(orig_dir,re.search(r'^workspace\s*=\s*"([^"]+)"',mt,re.M).group(1))),'Cargo.toml');wst=open(wsroot,encoding='utf8').read();wsd=dep_lines(wst,'workspace.dependencies');wss=sections(wst)
    def realpkg(k,v):
        if 'package' in v: return pkg_of(k,v)
        if 'workspace = true' in v.replace('  ',' '):
            if k in wsd: return pkg_of(k,wsd[k])
            tbl=wss.get('workspace.dependencies.'+k)
            if tbl:
                m=re.search(r'^package\s*=\s*"([^"]+)"',"\n".join(tbl),re.M)
                if m: return m.group(1)
        return k
    def gdir(g): return os.path.join(os.path.dirname(parent_file[gfirst[g]]),f"{sp['icon']}{g}")
    tests_free=lambda fs:[f for f in fs if "🧪️tests" not in f]
    macros={};
    for n,f in fmod.items():
        for m in re.findall(r'macro_rules!\s*(\w+)',open(f,encoding="utf8").read()): macros[m]=want[n]
    def provide(text):
        pv=sp["provides"]
        def brace(m):
            items=[x.strip() for x in m.group(1).split(",") if x.strip()];out=[];rest=[]
            for it in items:
                nm=it.split(" as ")[0].strip();al=it[len(nm):]
                (out.append(f"use {pv[nm]}{al};") if nm in pv and "::" in pv[nm] and pv[nm].rsplit("::",1)[1]==nm else rest.append(it))
            if rest: out.append("use crate::{"+", ".join(rest)+"};")
            return "\n".join(out)
        text=re.sub(r'(?<![\w:])use crate::\{([^}]*)\};',brace,text)
        for h,path in pv.items(): text=re.sub(r'\bcrate::'+h+r'\b(?!\w)',path,text)
        return text
    def rewrite(text,self_g,self_file=None,consumer=False):
        for n,g in want.items():
            P=names.get(parent_of[fmod[n]]) or ""
            lead=(r'\b'+ident(pkg)+r'::(?:\w+::)*?') if consumer else (r'(?:\w+::)*' if P else r'(?:\w+::)+')
            seg=(re.escape(P)+'::' if P else '')+n
            pat=re.compile(lead+seg+r'::')
            if g==self_g and not consumer: text=pat.sub(f"crate::{n}::",text)
            else: text=pat.sub(ident(crate(g))+"::",text)
            if not consumer and (g!=self_g): text=re.sub(r'\bsuper::'+n+r'::',ident(crate(g))+"::",text)
            if not consumer and re.search(lead+seg+r'\s*[;,}]',text): unres.append(f"{self_file}: bare module use of {n}")
        return text
    used_dep={};macro_user={}
    # moved module files
    for g,ns in gmods.items():
        gtext="";files=[]
        for n in ns:
            for f in subtree(fmod[n]):
                if "🧪️tests" in f: continue
                t=open(f,encoding="utf8").read()
                if f==fmod[n]:
                    t=rewrite(t,g,f)
                    t=provide(t)
                    t=re.sub(r'(?<!#\[macro_export\]\n)(\s*)macro_rules!',lambda m:"\n#[macro_export]\nmacro_rules!",t) if macros else t
                    code="\n".join(l for l in t.split("\n") if not l.lstrip().startswith("//"))
                    graw="\n".join(open(fmod[x],encoding="utf8").read() for x in ns)
                    for mm in re.findall(r'\bcrate::(\w+)',code)+[x.strip().split(" as ")[0] for b in re.findall(r'use crate::\{([^}]*)\};',code) for x in b.split(",") if x.strip()]:
                        if mm not in ns and mm not in want and not re.search(r'\b(struct|enum|fn|type|const|static|trait)\s+'+mm+r'\b',graw): unres.append(f"{f}: crate::{mm}")
                    E.set(f,t)
                gtext+=t;files.append(f)
        used_dep[g]=set()
        for n,gg in want.items():
            if gg!=g and re.search(r'\b'+ident(crate(gg))+r'::',gtext): used_dep[g].add(gg)
        for mname,pg in macros.items():
            if pg!=g and re.search(r'\b'+mname+r'!',gtext): used_dep[g].add(pg);macro_user.setdefault(g,set()).add(pg)
        gmods[g]=(ns,gtext)
    order=[];seen=set()
    def visit(g,st=()):
        if g in st: unres.append(f"group cycle {st+(g,)}");return
        if g in seen:return
        for d in used_dep[g]:visit(d,st+(g,))
        seen.add(g);order.append(g)
    for g in gmods:visit(g)
    # remaining root: files of tree not moved
    for f in list(T):
        if f in moved_files: continue
        t=E.get(f);n=rewrite(t,None,f)
        if n!=t: E.set(f,n)
    for n,f in parent_file.items(): pass
    # parent mount lines and (types) re-exports
    tmap={}
    for pf in set(parent_file.values()):
        t=E.get(pf)
        for n in [x for x in want if parent_file[x]==pf]:
            m=re.compile(r'(?:[ \t]*#\[[^\]\n]*\]\s*\n)*[ \t]*(?:pub(?:\([a-z]+\))?\s+)?mod\s+'+n+r'\s*;\n?').search(t)
            if m: t=t[:m.start()]+t[m.end():]
            else: unres.append(f"{pf}: mount of {n} not found")
            if sp["kind"]=="types":
                for mm in re.finditer(r'pub use '+n+r'::(\{[^}]*\}|\w+);\n',t):
                    for nm in re.findall(r'\b\w+\b',mm.group(1)): tmap[nm]=want[n]
                t=re.sub(r'pub use '+n+r'::(\{[^}]*\}|\w+);\n','',t)
        E.set(pf,t)
    # generated crates
    new_manifest={}
    for g in order:
        ns,gtext=gmods[g];d=gdir(g);cdir=f"{d}/📦️packages/🦀️rust"
        orig_root=open(entry,encoding="utf8").read()
        inner=[l for l in orig_root.split("\n") if l.startswith("#![")]
        ext=[l for l in orig_root.split("\n") if l.startswith("extern crate ")]
        lines=[f"//! 🧱️ `{crate(g)}`: the `{'`, `'.join(ns)}` module(s) of `{pkg}`, one compilation unit of their own."]+inner
        keep={}
        for k,v in depkeys.items():
            if re.search(r'\b'+ident(k)+r'\b',gtext): keep[k]=v
        for l in ext:
            m=re.match(r'extern crate (\w+) as (\w+);',l)
            if m and re.search(r'\b'+m.group(2)+r'\b',gtext):
                lines.append(l)
                for k in depkeys:
                    if ident(k)==m.group(1): keep[k]=depkeys[k]
        for pg in sorted(macro_user.get(g,())): lines.append(f"#[macro_use]\nextern crate {ident(crate(pg))};")
        for n in ns:
            attr="#[macro_use]\n" if any(mm in open(fmod[n],encoding='utf8').read() for mm in macros if macros[mm]==g) else ""
            lines.append(f'{attr}#[path = "{rel(d,fmod[n])}"]\nmod {n};\npub use {n}::*;')
        E.new(f"{d}/🦀️.rs","\n".join(lines)+"\n")
        wsrel=os.path.relpath(os.path.normpath(os.path.join(orig_dir,re.search(r'^workspace\s*=\s*"([^"]+)"',mt,re.M).group(1))),cdir)
        deps=[]
        for k,v in sorted(keep.items()):
            pm=re.search(r'path\s*=\s*"([^"]+)"',v)
            if pm: v=v.replace(pm.group(0),f'path = "{rel(cdir,os.path.normpath(os.path.join(orig_dir,pm.group(1))))}"')
            deps.append(f"{k} = {v}")
        for dg in sorted(used_dep[g]): deps.append(f'{crate(dg)} = {{ path = "{rel(cdir,gdir(dg)+"/📦️packages/🦀️rust")}" }}')
        carve=""
        if "[package.metadata.semio.carve]" in mt:
            carve="\n[package.metadata.semio.carve]\nowned = [\n"+"".join(f'  "{"::".join(x for x in T[fmod[n]]["chain"])}",\n' for n in ns)+"]\n"
        E.new(f"{cdir}/Cargo.toml",f'[package]\nworkspace = "{wsrel}"\nname = "{crate(g)}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Component crate of {pkg}: {", ".join(ns)}"\n\n[package.metadata.semio]\nrole = "library"\n{carve}\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n'+"\n".join(deps)+"\n")
        new_manifest[g]=(f"{cdir}/Cargo.toml",[realpkg(k,v) for k,v in keep.items()]+[crate(x) for x in used_dep[g]])
    # root manifest + consumers
    libroots,mounted,mfs=world(idx);mdirs={m:p for m,p in mfs.items()}
    cons=collections.defaultdict(set);cons_changes=[]
    pats=["pdf_model","snapshot::model"] if sp["kind"]=="types" else [ (T[parent_of[fmod[n]]]["chain"] or ["x"])[-1] for n in want][:1]
    import subprocess
    files=[f[2:] for f in subprocess.run(["grep","-rIlE","--include=*.rs","pdf_model|snapshot::model" if sp["kind"]=="types" else ident(pkg),"."]+[f"--exclude-dir={x}" for x in SKIP],capture_output=True,text=True).stdout.split("\n") if f]
    for f in files:
        if f in moved_files or f in E.t: continue
        t=open(f,encoding="utf8").read();n=t
        if sp["kind"]=="types":
            def use(m):
                vis,pre,body=m.group(1) or "",m.group(2),m.group(3).strip()
                if body=="*": gs=sorted(set(tmap.values()));return "".join(f"{vis}use {ident(crate(g))}::*;\n" for g in gs)+f"{vis}use {pre}model::*;"
                items=[x.strip() for x in body.strip("{}").split(",") if x.strip()] if body.startswith("{") else [body]
                by=collections.defaultdict(list);rest=[]
                for it in items:
                    nm=it.split(" as ")[0].strip();(by[tmap[nm]] if nm in tmap else rest).append(it)
                out=[f"{vis}use {ident(crate(g))}::{{{', '.join(v)}}};" for g,v in by.items()]
                if rest: out.append(f"{vis}use {pre}model::{{{', '.join(rest)}}};")
                return "\n".join(out)
            n=re.sub(r'(pub\s+)?use\s+((?:\w+::)*snapshot::)model::(\{[^;]*?\}|\*|\w+(?:\s+as\s+\w+)?)\s*;',use,n,flags=re.S)
            n=re.sub(r'((?:\w+::)*snapshot::)model::(\w+)\b',lambda m:f"{ident(crate(tmap[m.group(2)]))}::{m.group(2)}" if m.group(2) in tmap else m.group(0),n)
        else: n=rewrite(n,None,f,True)
        if n!=t:
            E.set(f,n);ow=crate_of(f,libroots,mounted,mdirs)
            if ow is None: unres.append(f"{f}: no owning crate");continue
            used={g for g in gmods if re.search(r'\b'+ident(crate(g))+r'::',n)}
            cons[ow]|=used;cons_changes.append((f,sorted(used)))
    cons[pkg]|=set(order)
    for ow,gs in cons.items():
        mfp=idx[ow];t=E.get(mfp);cd=os.path.dirname(mfp)
        add="".join(f'{crate(g)} = {{ path = "{rel(cd,gdir(g)+"/📦️packages/🦀️rust")}" }}\n' for g in sorted(gs) if f"{crate(g)} =" not in t)
        if not add: continue
        t=re.sub(r'(^\[dependencies\]\n)',lambda m:m.group(1)+add,t,count=1,flags=re.M) if re.search(r'^\[dependencies\]',t,re.M) else t.rstrip("\n")+"\n\n[dependencies]\n"+add
        E.set(mfp,t)
    if sp["kind"]=="types" and "[package.metadata.semio.carve]" in mt:
        t=E.get(mf);moved_chain={"::".join(T[fmod[n]]["chain"]) for n in want}
        for c in moved_chain: t=t.replace(f'  "{c}",\n','')
        E.set(mf,t)
    # workspaces
    pdir=os.path.dirname(mf);wsm=[l for l in ls("*Cargo.toml") if os.path.exists(l) and "[workspace]" in open(l,encoding="utf8").read()]
    for w in wsm:
        t=E.get(w);wd=os.path.dirname(w) or ".";me=os.path.relpath(pdir,wd)
        if f'"{me}"' in t:
            add="".join(f'    "{os.path.relpath(gdir(g)+"/📦️packages/🦀️rust",wd)}",\n' for g in order)
            E.set(w,t.replace(f'    "{me}",\n',add+f'    "{me}",\n',1))
    # build owners
    bo="📋️build-owners.json";t=E.get(bo);names_o=sorted(re.findall(r'^      "name": "([^"]+)",$',t,re.M))
    for g in order:
        c=crate(g);root=f"{gdir(g)}/📦️packages/🦀️rust";blk=f'    {{\n      "name": "{c}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
        nx=next(x for x in names_o if x>c);a=f'    {{\n      "name": "{nx}",\n'
        if t.count(a)!=1: unres.append(f"build-owners anchor {nx}");continue
        t=t.replace(a,blk+a);names_o=sorted(names_o+[c])
    E.set(bo,t)
    # locks
    nlock=0
    for lk in ls("*Cargo.lock"):
        lt=open(lk,encoding="utf8").read()
        if f'\nname = "{pkg}"\n' not in lt: continue
        head,blocks=lock_blocks(lt);bd=dict(blocks);_,odeps=lock_deps(bd[pkg]);byname={}
        for e in odeps: byname.setdefault(lockname(e),e)
        out=[]
        for nm,b in blocks:
            if nm==pkg: b=set_lock_deps(b,odeps+[crate(g) for g in order])
            elif nm in cons and nm!=pkg: b=set_lock_deps(b,lock_deps(b)[1]+[crate(g) for g in sorted(cons[nm])])
            out.append((nm,b))
        ver=re.search(r'version = "([^"]+)"',bd[pkg]).group(1)
        for g in order:
            _,pk=new_manifest[g];ds=[]
            for p_ in pk:
                if p_ in byname: ds.append(byname[p_])
                elif p_.startswith(pkg+"-"): ds.append(p_)
                else: WARN.append(f"{lk}: dependency {p_} of {crate(g)} absent from the lock block of {pkg}; left out")
            blk=f'name = "{crate(g)}"\nversion = "{ver}"\ndependencies = [\n'+"".join(f' "{d}",\n' for d in sorted(set(ds)))+']\n'
            k=0
            while k<len(out) and out[k][0]<crate(g):k+=1
            out.insert(k,(crate(g),blk))
        E.set(lk,lock_join(head,out));nlock+=1
    log+=[f"groups in build order: {order}",f"new crates: {[crate(g) for g in order]}",f"files edited or created: {len(E.t)}",f"consumer files repointed: {len(cons_changes)}",f"lock files: {nlock}"]+[f"  - {f}: {u}" for f,u in cons_changes]
    return finish(key,log,unres,E,apply)
def finish(key,log,unres,E,apply):
    log+=["","WARNINGS:"]+WARN+["","UNRESOLVED (apply refuses):"]+unres if unres else ["","WARNINGS:"]+WARN+["","no unresolved reference"]
    out=f"{os.path.dirname(os.path.abspath(__file__))}/📓️apply-{key}-report.md";open(out,"w",encoding="utf8").write("\n".join(log)+"\n");print(out,"unresolved:",len(unres),"edits:",len(E.t))
    if os.environ.get("DUMP"): E.dump_new(os.environ["DUMP"])
    if apply:
        if unres: sys.exit("refusing to apply: unresolved references")
        E.write();print("applied",key)
if __name__=="__main__":
    for k in [a for a in sys.argv[1:] if a!="--apply"] or SPECS: run(k,"--apply" in sys.argv)
