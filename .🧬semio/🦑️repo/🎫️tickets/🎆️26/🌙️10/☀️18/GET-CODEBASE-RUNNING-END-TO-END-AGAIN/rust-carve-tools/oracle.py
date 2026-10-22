import sys,subprocess,re,os,collections,json
S="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad"
def run(root,crate,cfgs,test=False,edition="2021"):
    out=S+"/o"
    os.makedirs(out,exist_ok=True)
    cmd=["rustc","+nightly-2026-07-20","--edition",edition,"--crate-type","lib","--crate-name",crate,"--emit=metadata","--out-dir",out,"--error-format=short"]
    if test:cmd.append("--test")
    for c in cfgs:cmd+=["--cfg",c]
    cmd.append(root)
    r=subprocess.run(cmd,capture_output=True,text=True,cwd=os.path.dirname(root))
    return r.stderr
def norm(stderr,base):
    rows=[]
    for l in stderr.split("\n"):
        m=re.match(r'^(.*?):(\d+):(\d+): (error(?:\[E\d+\])?|warning): (.*)$',l)
        if not m:continue
        path,line,col,kind,msg=m.groups()
        if kind=="warning":continue
        rel=path.replace(base,"")
        rows.append((kind,msg,rel))
    return rows
def summarize(rows,ext_re):
    c=collections.Counter()
    for kind,msg,rel in rows:
        if re.search(ext_re,msg) and kind.startswith("error[E043"):continue
        c[(kind,msg)]+=1
    return c
if __name__=="__main__":
    root=sys.argv[1];crate=sys.argv[2];cfgs=sys.argv[3].split(",") if len(sys.argv)>3 and sys.argv[3] else []
    test="--test" in sys.argv
    err=run(root,crate,cfgs,test)
    rows=norm(err,os.path.dirname(root)+"/")
    ext=r'(semio_[a-z_0-9]+|serde|serde_json|blake3|protocol|pack|tokio|js_sys|wasm_bindgen|[a-z_0-9]+_derive)'
    c=summarize(rows,ext)
    for (k,m),n in sorted(c.items()):print(n,k,m[:200])
    print("total",sum(c.values()))
