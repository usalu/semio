import os,sys,json,subprocess,collections,re,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sbx import *
HOSTDIR=OS+"🖥️host"
MCPDIR=OS+"🔨️modules/🌉️mcp"
CRATES={
 "host":dict(real=HOSTDIR,snap="orig-host",lib=HOSTDIR+"/📦️packages/🦀️rust/🦀️.rs",name="semio_framework_os",mono=HOSTDIR+"/🦀️.rs",children=["🔌️plugin-host","🗃️document-backbone","🧱️instance","🖼️media-export-raster","🧾️media-export-simple","🔀️workflow","📡️codec-abi","🗂️registry"],cfgs={"native":[],"test":["--test"],"wasm":["--target","wasm32-unknown-unknown"],"wasmtest":["--test","--target","wasm32-unknown-unknown"],"p2":["--target","wasm32-wasip2"],"p2test":["--test","--target","wasm32-wasip2"]}),
 "mcp":dict(real=MCPDIR,snap="orig-mcp",lib=MCPDIR+"/📦️packages/🦀️rust/🦀️.rs",name="semio_framework_os_mcp",mono=MCPDIR+"/🏠️workspace/🦀️.rs",children=["🏠️workspace/"+x for x in ["🧭️plugin-paths-and-descriptor","🔬️probe-document","🔖️plugin-binding-and-activation","🧊️compiled-components","📨️plugin-artifact-channel","🧩️artifact-app-frames","🔀️routing-and-shell-routed","🗂️guest-document-codec","🤝️hub-relay","🪪️headless-workspace-state","🛂️gateway-backend","💡️inference-facade"]],cfgs={"native":[],"test":["--test"],"wasm":["--target","wasm32-unknown-unknown"],"wasmtest":["--test","--target","wasm32-unknown-unknown"],"p2":["--target","wasm32-wasip2"]}),
}
def build(crate,which):
    c=CRATES[crate]
    root=SCR+f"sbx-{crate}-{which}"
    mk(root,{c["real"]:(SCR+"snap/"+c["snap"]) if which=="orig" else REPO+c["real"]})
    if which=="split" and crate=="mcp" and os.environ.get("STAGE_MCP"):
        st=os.environ["STAGE_MCP"];wsd=root+"/"+c["real"]+"/🏠️workspace"
        for e in os.listdir(st):
            dst=os.path.join(wsd,e)
            if os.path.isdir(dst):shutil.rmtree(dst)
            if e=="🦀️.rs":shutil.copy(os.path.join(st,e),dst)
            else:shutil.copytree(os.path.join(st,e),dst)
    files=rsfiles(root+"/"+c["real"])
    mods=modnames(files)
    n=neutralize(files,mods)
    if which=="split" and os.environ.get("NEG"):
        for old,new in [x.split("=>") for x in os.environ["NEG"].split("||")]:
            for p in files:
                s=open(p,encoding="utf-8").read()
                if old in s:
                    open(p,"w",encoding="utf-8").write(s.replace(old,new,1));print("NEG patched",p[-60:]);break
    return root,n
def run(crate,root,cfg,extra):
    c=CRATES[crate]
    lib=root+"/"+c["lib"]
    p=subprocess.run(["rustc","+nightly-2026-07-20","--edition","2021","--crate-type","lib","--crate-name",c["name"],"--emit=metadata","-o","/dev/null","--error-format=json","--json=diagnostic-short",*extra,lib],capture_output=True,text=True,cwd=os.path.dirname(lib))
    return p.stderr
def norm(crate,root,stderr):
    c=CRATES[crate]
    libdir=os.path.dirname(root+"/"+c["lib"])
    mono=c["mono"]
    child_dirs=[c["real"]+"/"+x+"/" for x in c["children"]]
    out=collections.Counter()
    for ln in stderr.splitlines():
        if not ln.startswith("{"):continue
        try:d=json.loads(ln)
        except Exception:continue
        if "message" not in d or "level" not in d:continue
        lvl=d["level"]
        if lvl not in("error","warning"):continue
        spans=[s for s in d.get("spans",[]) if s.get("is_primary")]
        f="";txt="";lab=""
        if spans:
            s=spans[0]
            fn=os.path.normpath(os.path.join(libdir,s["file_name"]))
            f=os.path.relpath(fn,root) if fn.startswith(root) else fn
            for cd in child_dirs:
                if f.startswith(cd):f=mono
            txt=" ".join(" ".join(t["text"] for t in s.get("text",[])).split()).replace("pub(super) ","")
            lab=s.get("label") or ""
        code=(d.get("code") or {}).get("code") or ""
        if os.environ.get("DROP_TRAIT_USES") and txt.strip() in ("use semio_framework_plugin_host::GuestRuntime as _;","use store::SpaceMember as _;","use super::ArtifactChannel as _;"):continue
        msg=re.sub(r'\s+',' ',d["message"])
        out[(lvl,code,msg,f,txt,lab)]+=1
    return out
def main():
    crate=sys.argv[1];only=sys.argv[2:] 
    c=CRATES[crate]
    ro,no=build(crate,"orig");rs,ns=build(crate,"split")
    print(f"neutralized files orig={no} split={ns}")
    bad=0
    for cfg,extra in c["cfgs"].items():
        if only and cfg not in only:continue
        eo=run(crate,ro,cfg,extra);es=run(crate,rs,cfg,extra)
        open(SCR+f"{crate}-{cfg}-orig.json","w").write(eo);open(SCR+f"{crate}-{cfg}-split.json","w").write(es)
        a=norm(crate,ro,eo);b=norm(crate,rs,es)
        ea=sum(v for k,v in a.items() if k[0]=="error");eb=sum(v for k,v in b.items() if k[0]=="error")
        wa=sum(v for k,v in a.items() if k[0]=="warning");wb=sum(v for k,v in b.items() if k[0]=="warning")
        da=a-b;db=b-a
        print(f"== {crate}/{cfg}: orig errors={ea} warnings={wa} | split errors={eb} warnings={wb} | only-orig={sum(da.values())} only-split={sum(db.values())}")
        for k,v in sorted(da.items()):print("  -orig",v,k[0],k[1],k[2][:110],"|",k[3][-60:],"|",k[4][:70])
        for k,v in sorted(db.items()):print("  +split",v,k[0],k[1],k[2][:110],"|",k[3][-60:],"|",k[4][:70])
        bad+=sum(da.values())+sum(db.values())
    print("TOTAL DIFF",bad)
main()
