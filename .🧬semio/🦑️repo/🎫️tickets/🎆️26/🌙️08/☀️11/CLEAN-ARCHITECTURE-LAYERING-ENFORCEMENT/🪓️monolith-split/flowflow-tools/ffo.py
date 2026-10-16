"""🔬 Patched full-resolution differential for the flow-flow carve (analysis only; output to /dev/null; writes only inside the scratch sandbox).
usage: ffo.py baseline | compare [cfg,cfg] [--union=file.json]
"""
import collections,json,os,pickle,re,shutil,subprocess,sys
REPO="/Users/ueli/Documents/semio/"
OS="🧰️framework/🛍️products/💻️os/🔨️modules/"
FF=OS+"🌊️flow/🗿️artifacts/🌊️flow"
SCR="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/ff/"
TOOLCHAIN="+nightly-2026-07-20"
NAME="semio_framework_artifact_flow_flow"
CFGS={
 "native":[],
 "test":["--test","--cfg",'feature="protocol-laws"'],
 "wasm":["--target","wasm32-unknown-unknown"],
 "wasmtest":["--test","--cfg",'feature="protocol-laws"',"--target","wasm32-unknown-unknown"],
 "p2":["--target","wasm32-wasip2"],
 "p2test":["--test","--cfg",'feature="protocol-laws"',"--target","wasm32-wasip2"],
}
IGN=shutil.ignore_patterns("node_modules","dist","target",".nx","*.log","🕸️bindings")
def mk(root,realdir):
    assert os.path.realpath(root).startswith(os.path.realpath(SCR)),root
    shutil.rmtree(root,ignore_errors=True)
    os.makedirs(root)
    rel=FF
    parts=rel.split("/")
    chain={"/".join(parts[:n]) for n in range(1,len(parts))}
    def build(relpath):
        d=os.path.join(root,relpath) if relpath else root
        os.makedirs(d,exist_ok=True)
        for e in os.listdir(REPO+relpath):
            r=(relpath+"/"+e) if relpath else e
            if r==rel:
                shutil.copytree(realdir,os.path.join(root,r),symlinks=True,ignore=IGN)
            elif r in chain:
                build(r)
            else:
                os.symlink(REPO+r,os.path.join(root,r))
    build("")
def rsfiles(tree):
    out=[]
    for dp,dn,fn in os.walk(tree):
        for f in fn:
            if f.endswith('.rs'):out.append(os.path.join(dp,f))
    return out
def modnames(files):
    s=set()
    for p in files:
        for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)',open(p,encoding='utf-8',errors='ignore').read(),re.M):s.add(m.group(1))
    return s
def neutralize(files,keep_mods,keep_crates=()):
    rx=re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$',re.M)
    ex=re.compile(r'^\s*(?:#\[[^\]]*\]\s*)*extern crate (?!self)[^\n]*;[ \t]*$',re.M)
    keep={'crate','super','self'}|keep_mods|set(keep_crates)
    n=0
    for p in files:
        if os.path.islink(p) or not os.path.realpath(p).startswith(os.path.realpath(SCR)):continue
        t=open(p,encoding='utf-8',errors='ignore').read()
        t2=rx.sub(lambda m:(m.group(1)+'const _: () = ();') if m.group(2) not in keep else m.group(0),t)
        t2=ex.sub('',t2)
        if t2!=t:
            open(p,'w',encoding='utf-8').write(t2);n+=1
    return n
def sandbox(tag,src,transform=None):
    root=SCR+"sb-"+tag
    mk(root,src)
    if transform:transform(root+"/"+FF)
    files=rsfiles(root+"/"+FF)
    neutralize(files,modnames(files))
    return root
def run(libp,extra,name=NAME):
    cmd=["rustc",TOOLCHAIN,"--edition","2021","--crate-type","lib","--crate-name",name,"--emit=metadata","-o","/dev/null","--error-format=json","--json=diagnostic-short",*extra,libp]
    env=dict(os.environ,CARGO_MANIFEST_DIR=os.path.dirname(libp))
    return subprocess.run(cmd,cwd=os.path.dirname(libp),capture_output=True,text=True,env=env).stderr
def norm(stderr,level=("error",),post=None):
    out=collections.Counter()
    for ln in stderr.splitlines():
        if not ln.startswith("{"):continue
        try:d=json.loads(ln)
        except Exception:continue
        if d.get("level") not in level:continue
        spans=[s for s in d.get("spans",[]) if s.get("is_primary")]
        txt="";lab="";file=""
        if spans:
            s=spans[0]
            txt=" ".join(" ".join(t["text"] for t in s.get("text",[])).split())
            txt=re.sub(r'\bpub\((?:super|crate|in [\w:]+)\)\s+','',txt)
            txt=re.sub(r'(?<![\w:])pub\s+','',txt)
            lab=s.get("label") or ""
            file=s.get("file_name") or ""
        code=(d.get("code") or {}).get("code") or ""
        msg=re.sub(r'\s+',' ',d["message"])
        if msg.startswith('aborting due to'):continue
        msg=re.sub(r'sb-\w+','SB',msg)
        key=(code,msg,txt,lab)
        if post:
            key=post(key,file)
            if key is None:continue
        out[key]+=1
    return out
def baseline():
    root=sandbox("orig",SCR+"orig")
    res={}
    for tag,extra in CFGS.items():
        err=run(root+"/"+FF+"/🦀️.rs",extra)
        open(SCR+f"err-orig-{tag}.json","w").write(err)
        res[tag]=norm(err)
        print("baseline",tag,sum(res[tag].values()),flush=True)
    pickle.dump(res,open(SCR+"base.pkl","wb"))
if __name__=="__main__":
    if sys.argv[1]=="baseline":baseline()
