import json,time,os,sys,shutil
A="/Users/Shared/semio-store/v1/origins/g1-server/attest.json"
def v():
    a=json.load(open(A));return a["view"]
d="zz-conv"
def waitfor(pred,limit=120):
    t=time.time()
    while time.time()-t<limit:
        x=v()
        if pred(x):return time.time()-t,x
        time.sleep(0.25)
    return None,v()
base=v();print("base",base)
os.makedirs(d,exist_ok=True)
for i in range(20):open(f"{d}/f{i}.txt","w").write("x")
u0=base["untracked"]
print("create ->",waitfor(lambda x:x["untracked"]==u0+20 and not x.get("stale")))
shutil.rmtree(d)
print("delete ->",waitfor(lambda x:x["untracked"]==u0 and not x.get("stale")))
