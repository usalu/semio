import sys,shutil,os,time
def patch(path,pairs):
    s=open(path,encoding="utf-8").read()
    out=s
    for old,new in pairs:
        n=out.count(old)
        if n!=1: raise SystemExit(f"anchor count {n} for: {old[:80]!r}")
        out=out.replace(old,new)
    if out==s: print("unchanged",path); return
    bak=os.path.join("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/stale-impl-guard","backup-"+os.path.basename(os.path.dirname(path))+"-"+os.path.basename(path)+"."+str(int(time.time())))
    shutil.copy2(path,bak)
    tmp=path+".sig.tmp"
    open(tmp,"w",encoding="utf-8").write(out); os.replace(tmp,path)
    print("patched",path)
