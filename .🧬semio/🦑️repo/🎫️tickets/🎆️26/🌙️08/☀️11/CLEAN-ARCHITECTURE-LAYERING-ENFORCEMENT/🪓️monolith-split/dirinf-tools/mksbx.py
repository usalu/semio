import os,sys,shutil
REPO="/Users/ueli/Documents/semio/"
HUB=REPO+"🌎️hub/"
TOOLS=os.path.dirname(os.path.abspath(__file__))
def mk(root,dstage=None,rstage=None):
    shutil.rmtree(root,ignore_errors=True)
    os.makedirs(root)
    sh=os.path.join(root,"🌎️hub");os.makedirs(sh)
    for e in os.listdir(HUB):
        if e in("node_modules",):continue
        if e in("📇️directory","💡️inference","📦️packages"):continue
        os.symlink(HUB+e,os.path.join(sh,e))
    d=os.path.join(sh,"📇️directory");os.makedirs(d)
    for e in os.listdir(HUB+"📇️directory"):
        if e=="🦀️.rs":shutil.copy((dstage+"/🦀️.rs") if dstage else TOOLS+"/orig-directory.rs.txt",os.path.join(d,e))
        else:os.symlink(HUB+"📇️directory/"+e,os.path.join(d,e))
    if dstage:
        for e in os.listdir(dstage):
            if e!="🦀️.rs":shutil.copytree(dstage+"/"+e,os.path.join(d,e))
    i=os.path.join(sh,"💡️inference");os.makedirs(i)
    for e in os.listdir(HUB+"💡️inference"):
        if e=="🏃️runtime":
            r=os.path.join(i,e);os.makedirs(r)
            for f in os.listdir(HUB+"💡️inference/🏃️runtime"):
                if f=="🦀️.rs":shutil.copy((rstage+"/🦀️.rs") if rstage else TOOLS+"/orig-runtime.rs.txt",os.path.join(r,f))
                else:os.symlink(HUB+"💡️inference/🏃️runtime/"+f,os.path.join(r,f))
            if rstage:
                for f in os.listdir(rstage):
                    if f!="🦀️.rs":shutil.copytree(rstage+"/"+f,os.path.join(r,f))
        else:os.symlink(HUB+"💡️inference/"+e,os.path.join(i,e))
    p=os.path.join(sh,"📦️packages");os.makedirs(p)
    for e in os.listdir(HUB+"📦️packages"):
        if e=="🦀️rust":
            r=os.path.join(p,e);os.makedirs(r)
            for f in os.listdir(HUB+"📦️packages/🦀️rust"):
                if f in("node_modules",):continue
                src=HUB+"📦️packages/🦀️rust/"+f
                if f=="🦀️.rs":shutil.copy(src,os.path.join(r,f))
                else:os.symlink(src,os.path.join(r,f))
        elif e!="node_modules":os.symlink(HUB+"📦️packages/"+e,os.path.join(p,e))
    # repo siblings referenced by ../../../ from hub packages
    for e in os.listdir(REPO):
        if e in("🌎️hub",".git"):continue
        try:os.symlink(REPO+e,os.path.join(root,e))
        except FileExistsError:pass
if __name__=="__main__":mk(sys.argv[1],*(sys.argv[2:4] if len(sys.argv)>2 else []))
