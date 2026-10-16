"""🔬 Patched full-resolution differential for the os-flow carve (analysis only, output to /dev/null)."""
import collections,json,os,pickle,re,subprocess,sys,importlib.util
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sandbox import *
TOOLCHAIN="+nightly-2026-07-20"
FLOW=OS+"🌊️flow"
WORKFLOW=OS+"🔁️workflow"
CRATES={
 "flow":dict(lib=FLOW+"/📦️packages/🦀️rust/🦀️.rs",name="semio_framework_os_flow",cfgs={
   "native":[],"test":["--test"],
   "wasm":["--target","wasm32-unknown-unknown"],"wasmtest":["--test","--target","wasm32-unknown-unknown"],
   "p2":["--target","wasm32-wasip2"],"p2test":["--test","--target","wasm32-wasip2"]}),
 "flowflow":dict(lib=FLOW+"/🗿️artifacts/🌊️flow/🦀️.rs",name="semio_framework_artifact_flow_flow",cfgs={
   "native":[],"test":["--test","--cfg",'feature="protocol-laws"'],"wasm":["--target","wasm32-unknown-unknown"],"wasmtest":["--test","--cfg",'feature="protocol-laws"',"--target","wasm32-unknown-unknown"]}),
 "workflow":dict(lib=WORKFLOW+"/🗿️artifacts/🔁️workflow/🦀️.rs",name="semio_framework_artifact_workflow_workflow",cfgs={
   "native":[],"test":["--test"],"wasm":["--target","wasm32-unknown-unknown"],"wasmtest":["--test","--target","wasm32-unknown-unknown"]}),
}
def sandbox(tag,src_flow,src_workflow,transform=None):
    root=SCR+"sb-"+tag
    mk(root,{FLOW:src_flow,WORKFLOW:src_workflow})
    files=[p for p in rsfiles(root+"/"+FLOW)+rsfiles(root+"/"+WORKFLOW)]
    mods=modnames(files)
    if transform:transform(root)
    files=[p for p in rsfiles(root+"/"+FLOW)+rsfiles(root+"/"+WORKFLOW)]
    mods=modnames(files)
    neutralize(files,mods)
    return root
def run(root,lib,name,extra):
    libp=root+"/"+lib
    cmd=["rustc",TOOLCHAIN,"--edition","2021","--crate-type","lib","--crate-name",name,"--emit=metadata","-o","/dev/null","--error-format=json","--json=diagnostic-short",*extra,libp]
    env=dict(os.environ,CARGO_MANIFEST_DIR=os.path.dirname(libp))
    p=subprocess.run(cmd,cwd=os.path.dirname(libp),capture_output=True,text=True,env=env)
    return p.stderr
def norm(root,stderr,level=("error",)):
    out=collections.Counter()
    for ln in stderr.splitlines():
        if not ln.startswith("{"):continue
        try:d=json.loads(ln)
        except Exception:continue
        if d.get("level") not in level:continue
        spans=[s for s in d.get("spans",[]) if s.get("is_primary")]
        txt="";lab=""
        if spans:
            s=spans[0]
            txt=" ".join(" ".join(t["text"] for t in s.get("text",[])).split())
            txt=re.sub(r'\bpub\((?:super|crate|in [\w:]+)\)\s+','',txt)
            txt=re.sub(r'(?<![\w:])pub\s+','',txt)
            lab=s.get("label") or ""
        code=(d.get("code") or {}).get("code") or ""
        msg=re.sub(r'\s+',' ',d["message"])
        if msg.startswith('aborting due to'):continue
        msg=re.sub(r'sb-(?:cur|orig)-\w+','SB',msg)
        out[(code,msg,txt,lab)]+=1
    return out
def baseline(crate):
    root=sandbox("orig-"+crate,SCR+"flow",SCR+"workflow")
    res={}
    for tag,extra in CRATES[crate]["cfgs"].items():
        err=run(root,CRATES[crate]["lib"],CRATES[crate]["name"],extra)
        res[tag]=norm(root,err)
        open(SCR+f"err-orig-{crate}-{tag}.json","w").write(err)
        print("baseline",crate,tag,sum(res[tag].values()),flush=True)
    pickle.dump(res,open(SCR+f"base-{crate}.pkl","wb"))
def compare(crate,src_flow,src_workflow,transform=None,cfgs=None,show=60):
    root=sandbox("cur-"+crate,src_flow,src_workflow,transform)
    base=pickle.load(open(SCR+f"base-{crate}.pkl","rb"))
    bad=0
    for tag,extra in CRATES[crate]["cfgs"].items():
        if cfgs and tag not in cfgs:continue
        err=run(root,CRATES[crate]["lib"],CRATES[crate]["name"],extra)
        open(SCR+f"err-cur-{crate}-{tag}.json","w").write(err)
        cur=norm(root,err)
        b=norm(None,open(SCR+f"err-orig-{crate}-{tag}.json").read())
        a=cur-b;c=b-cur
        print(f"== {crate}/{tag}: baseline {sum(b.values())} split {sum(cur.values())} only-split {sum(a.values())} only-baseline {sum(c.values())}",flush=True)
        for k,v in sorted(a.items())[:show]:print("  +split",v,k[0],k[1][:130],"|",k[2][:90],"|",k[3][:50])
        for k,v in sorted(c.items())[:show]:print("  -base ",v,k[0],k[1][:130],"|",k[2][:90],"|",k[3][:50])
        bad+=sum(a.values())+sum(c.values())
    print("TOTAL DIFF",bad)
    return bad
def renorm(crate,cfgs=None,show=80):
    bad=0
    for tag in CRATES[crate]["cfgs"]:
        if cfgs and tag not in cfgs:continue
        b=norm(None,open(SCR+f"err-orig-{crate}-{tag}.json").read())
        cur=norm(None,open(SCR+f"err-cur-{crate}-{tag}.json").read())
        a=cur-b;c=b-cur
        print(f"== {crate}/{tag}: baseline {sum(b.values())} split {sum(cur.values())} only-split {sum(a.values())} only-baseline {sum(c.values())}",flush=True)
        for k,v in sorted(a.items())[:show]:print("  +split",v,k[0],k[1][:130],"|",k[2][:110],"|",k[3][:50])
        for k,v in sorted(c.items())[:show]:print("  -base ",v,k[0],k[1][:130],"|",k[2][:110],"|",k[3][:50])
        bad+=sum(a.values())+sum(c.values())
    print("TOTAL DIFF",bad)
if __name__=="__main__":
    cmd=sys.argv[1]
    if cmd=="baseline":
        for c in sys.argv[2:]:baseline(c)
    elif cmd=="renorm":
        renorm(sys.argv[2],sys.argv[3].split(",") if len(sys.argv)>3 else None)
    elif cmd=="compare":
        crate=sys.argv[2]
        tf=None
        cfgs=None
        if len(sys.argv)>3 and sys.argv[3]!="-":
            spec=importlib.util.spec_from_file_location("t",sys.argv[3]);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);tf=m.transform
        if len(sys.argv)>4:cfgs=sys.argv[4].split(",")
        compare(crate,REPO+FLOW,REPO+WORKFLOW,tf,cfgs)
