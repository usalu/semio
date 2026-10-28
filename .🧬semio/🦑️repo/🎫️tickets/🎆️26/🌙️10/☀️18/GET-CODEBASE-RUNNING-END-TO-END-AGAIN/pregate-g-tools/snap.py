"""📸 pregate-g: clone a registered mirror (APFS clonefile) into a new registered mirror: usage snap.py SRC DST"""
import json,os,subprocess,sys,time
src,dst=sys.argv[1:3]
home=os.path.join(os.environ.get("TMPDIR","/tmp"),"sm")
meta=os.path.join(home,".meta")
a=json.load(open(os.path.join(meta,src+".json")))
root=os.path.join(home,dst)
if os.path.exists(root):raise SystemExit("exists "+root)
subprocess.check_call(["cp","-c","-R",a["root"],root])
a.update(name=dst,root=root,created=time.time())
json.dump(a,open(os.path.join(meta,dst+".json"),"w"))
print("cloned",a["root"],"->",root)
