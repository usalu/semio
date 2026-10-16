"""🧪 Analysis sandbox: real copies of selected repo directories, symlinks for everything else (never touches the repo)."""
import os,re,shutil,sys
REPO="/Users/ueli/Documents/semio/"
OS="🧰️framework/🛍️products/💻️os/🔨️modules/"
SCR="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osflow/"
IGN=shutil.ignore_patterns("node_modules","dist","target",".nx","*.log","🕸️bindings")
def mk(root,real):
    shutil.rmtree(root,ignore_errors=True)
    os.makedirs(root)
    realset={k.rstrip("/"):v for k,v in real.items()}
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
            if r in realset:
                shutil.copytree(realset[r],os.path.join(root,r),symlinks=True,ignore=IGN)
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
def neutralize(files,keep_mods):
    rx=re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$',re.M)
    ex=re.compile(r'^\s*(?:#\[[^\]]*\]\s*)*extern crate (?!self)[^\n]*;[ \t]*$',re.M)
    keep={'crate','super','self'}|keep_mods
    n=0
    for p in files:
        if os.path.islink(p) or not os.path.realpath(p).startswith(SCR):continue
        t=open(p,encoding='utf-8',errors='ignore').read()
        t2=rx.sub(lambda m:(m.group(1)+'const _: () = ();') if m.group(2) not in keep else m.group(0),t)
        t2=ex.sub('',t2)
        if t2!=t:
            open(p,'w',encoding='utf-8').write(t2);n+=1
    return n
