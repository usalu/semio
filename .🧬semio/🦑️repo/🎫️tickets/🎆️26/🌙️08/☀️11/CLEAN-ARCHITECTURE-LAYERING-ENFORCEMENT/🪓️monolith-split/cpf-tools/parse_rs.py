import sys,subprocess,os
from concurrent.futures import ThreadPoolExecutor
ROOT="/Users/ueli/Documents/semio"
paths=[l.rstrip("\n") for l in sys.stdin if l.strip()]
def chk(p):
    full=os.path.join(ROOT,p)
    if not os.path.exists(full): return (p,"MISSING")
    r=subprocess.run(["nice","-n","10","rustfmt","--edition","2024","--check","--config","skip_children=true,unstable_features=true",full],capture_output=True,text=True)
    out=r.stdout+r.stderr
    if "\nerror" in "\n"+out or out.startswith("error"):
        return (p,out[:300])
    return (p,None)
bad=0
with ThreadPoolExecutor(4) as ex:
    for p,e in ex.map(chk,paths):
        if e: bad+=1; print("FAIL",p,e.replace("\n"," | "))
print("rs checked",len(paths),"bad",bad)
