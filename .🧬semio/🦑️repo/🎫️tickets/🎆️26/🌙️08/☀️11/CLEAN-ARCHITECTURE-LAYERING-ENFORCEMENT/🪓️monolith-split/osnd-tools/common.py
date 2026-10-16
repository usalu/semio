"""🧭 Shared paths and sandbox helpers of the OS-HOST-NEURAL-DAG carve tools (analysis only, never writes into the repo)."""
import os,re,shutil,subprocess,sys
REPO="/Users/ueli/Documents/semio/"
FW="🧰️framework/"
OS=FW+"🛍️products/💻️os/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
SNAP=SC+"snap/"
TOOLS=os.path.dirname(os.path.abspath(__file__))
IGN=shutil.ignore_patterns("node_modules","dist","target",".nx","*.log","pkg","*.wasm")
def mk(root,real):
    shutil.rmtree(root,ignore_errors=True);os.makedirs(root)
    realset={rel.rstrip("/"):src for rel,src in real.items()}
    chain=set()
    for rel in realset:
        parts=rel.split("/")
        for n in range(1,len(parts)):chain.add("/".join(parts[:n]))
    def build(relpath):
        d=os.path.join(root,relpath) if relpath else root
        os.makedirs(d,exist_ok=True)
        srcdir=REPO+relpath
        for e in os.listdir(srcdir):
            r=(relpath+"/"+e) if relpath else e
            if r in realset:shutil.copytree(realset[r],os.path.join(root,r),symlinks=True,ignore=IGN)
            elif r in chain:build(r)
            else:os.symlink(REPO+r,os.path.join(root,r))
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
def neutralize(files,keep_mods):
    rx=re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$',re.M)
    ex=re.compile(r'^\s*(?:#\[[^\]]*\]\s*)*extern crate [^\n]*;[ \t]*$',re.M)
    dyn=re.compile(r'dyn_enum_close!\s*\{\s*((?:#\[[^\]]*\]\s*)*pub\s+enum\s+\w+)\s*:\s*[\w:]+\s*\{(.*?)\}\s*\}',re.S)
    keep={'crate','super','self'}|keep_mods
    n=0
    for p in files:
        t=open(p,encoding='utf-8',errors='ignore').read()
        t2=rx.sub(lambda m:(m.group(1)+'const _: () = ();') if m.group(2) not in keep else m.group(0),t)
        t2=ex.sub('',t2)
        t2=dyn.sub(lambda m:m.group(1)+' {'+m.group(2)+'}',t2)
        if t2!=t:
            open(p,'w',encoding='utf-8').write(t2);n+=1
    return n
def closure(entry_rel,pkg_rel,cfg=""):
    """canonical-counter closure (repo-relative paths) of one entry under extra cfg tokens"""
    p=subprocess.run(["bun",TOOLS+"/dump-closure-cfg.ts",REPO+entry_rel,REPO+pkg_rel,cfg],capture_output=True,text=True,cwd=TOOLS)
    if p.returncode!=0:raise SystemExit("closure failed "+p.stderr[-400:])
    return [l.split("\t") for l in p.stdout.splitlines() if l.strip()]
