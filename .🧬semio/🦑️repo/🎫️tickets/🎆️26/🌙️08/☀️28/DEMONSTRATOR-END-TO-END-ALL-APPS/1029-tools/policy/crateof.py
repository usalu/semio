import os,sys,re
def crate_of(f):
    d=os.path.dirname(os.path.abspath(f))
    root="/Users/ueli/Documents/semio"
    while d.startswith(root) and d!=root:
        cm=os.path.join(d,"📦️packages","🦀️rust","Cargo.toml")
        if os.path.exists(cm):
            t=open(cm,encoding="utf-8").read()
            m=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M)
            return cm,m.group(1) if m else None
        d=os.path.dirname(d)
    return None,None
if __name__=="__main__":
    seen={}
    for f in sys.stdin.read().split("\n"):
        f=f.strip()
        if not f: continue
        cm,n=crate_of(f)
        seen.setdefault((cm,n),[]).append(f)
    for (cm,n),fs in seen.items():
        print(n,len(fs),(cm or "")[-110:])
