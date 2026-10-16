"""🏖️ Builds a symlink sandbox of the repo with file overrides, creating override files the current tree no longer has (deleted children of the baseline)."""
import os,sys,shutil
REPO="/Users/ueli/Documents/semio"
def mk(sbx, overrides):
    shutil.rmtree(sbx, ignore_errors=True)
    os.makedirs(sbx)
    chain=set()
    for rel in overrides:
        parts=rel.split("/")
        for i in range(len(parts)):
            chain.add("/".join(parts[:i]))
    def fill(rel):
        real=os.path.join(REPO,rel) if rel else REPO
        dst=os.path.join(sbx,rel) if rel else sbx
        os.makedirs(dst,exist_ok=True)
        if not os.path.isdir(real):
            return
        for e in os.listdir(real):
            r=(rel+"/"+e) if rel else e
            if r in overrides:
                shutil.copy(overrides[r], os.path.join(dst,e))
            elif r in chain:
                fill(r)
            else:
                os.symlink(os.path.join(real,e), os.path.join(dst,e))
    fill("")
    for rel,src in overrides.items():
        dst=os.path.join(sbx,rel)
        if not os.path.exists(dst):
            os.makedirs(os.path.dirname(dst),exist_ok=True)
            shutil.copy(src,dst)
if __name__=="__main__":
    sbx=sys.argv[1]
    ov={}
    for a in sys.argv[2:]:
        k,v=a.split("=",1); ov[k]=v
    mk(sbx,ov)
