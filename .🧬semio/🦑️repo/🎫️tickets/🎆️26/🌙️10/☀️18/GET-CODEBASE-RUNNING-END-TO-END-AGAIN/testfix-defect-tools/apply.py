#!/usr/bin/env python3
import hashlib,json,os,shutil,sys
R="/Users/ueli/Documents/semio/"
D=os.path.dirname(os.path.abspath(__file__))
def sha(p): return hashlib.sha256(open(p,"rb").read()).hexdigest()
def main():
    write="--write" in sys.argv
    only=[a for a in sys.argv[1:] if not a.startswith("--")]
    m=json.load(open(D+"/manifest.json",encoding="utf-8"))["entries"]
    if only: m=[x for x in m if any(o in x["target"] for o in only)]
    bad=[]
    for x in m:
        t=R+x["target"]; s=D+"/staged/"+x["staged"]; new=sha(s)
        cur=sha(t) if os.path.exists(t) else None
        if cur==new: x["state"]="applied"
        elif cur==x["before"]: x["state"]="ready"
        else: x["state"]="DRIFT"; bad.append(x)
    for x in m: print(x["state"].ljust(8),x["staged"],x["target"][-150:])
    if bad: print("REFUSED: drift on",len(bad),"targets"); sys.exit(2)
    if not write: print("dry run; pass --write"); return
    for x in m:
        if x["state"]!="ready": continue
        t=R+x["target"]; os.makedirs(os.path.dirname(t),exist_ok=True)
        tmp=t+".tfd.tmp"; shutil.copyfile(D+"/staged/"+x["staged"],tmp); os.replace(tmp,t)
    print("applied")
main()
