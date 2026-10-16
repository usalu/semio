"""↩️ Restores a split parent from the baseline snapshot and removes the folders the split created (own changes only)."""
import json,os,shutil,sys
spec=json.load(open(sys.argv[1],encoding="utf-8"));snap=sys.argv[2]
parent=spec["parent"];pdir=os.path.dirname(parent)
shutil.copyfile(snap,parent)
for c in spec["children"]:
    d=os.path.join(pdir,c["folder"])
    if os.path.isdir(d):shutil.rmtree(d)
print("restored",parent)
