#!/usr/bin/env python3
import hashlib,json,os,shutil,sys
R="/Users/ueli/Documents/semio/"
D=os.path.dirname(os.path.abspath(__file__))
M=D+"/manifest.json"
def load(): return json.load(open(M,encoding="utf-8")) if os.path.exists(M) else {"entries":[]}
def sha(p): return hashlib.sha256(open(p,"rb").read()).hexdigest()
def main():
    cmd,target=sys.argv[1],sys.argv[2]
    m=load(); e=next((x for x in m["entries"] if x["target"]==target),None)
    if cmd=="open":
        if e: print(D+"/staged/"+e["staged"]); return
        n="%03d"%(len(m["entries"])+1); ext=os.path.splitext(target)[1]; name=n+ext
        exists=os.path.exists(R+target)
        if exists: shutil.copyfile(R+target,D+"/staged/"+name)
        else: open(D+"/staged/"+name,"w",encoding="utf-8").write("")
        m["entries"].append({"target":target,"before":sha(R+target) if exists else None,"staged":name,"note":sys.argv[3] if len(sys.argv)>3 else ""})
        json.dump(m,open(M,"w",encoding="utf-8"),ensure_ascii=False,indent=1); print(D+"/staged/"+name)
    elif cmd=="list":
        for x in m["entries"]: print(x["staged"],x["target"],x["note"])
main()
