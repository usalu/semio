import json,time,os,shutil
S="/Users/Shared/semio-store/v1/origins/g1-server/"
def view():
    a=json.load(open(S+"attest.json"));return a["view"],open(S+"view.txt").read().split("\n")
d="zz-conv2"
def wait(pred,limit=150):
    t=time.time()
    while time.time()-t<limit:
        v,l=view()
        if pred(v,l):return round(time.time()-t,1)
        time.sleep(0.2)
    return None
has=lambda l:sum(1 for x in l if "zz-conv2" in x)
os.makedirs(d,exist_ok=True)
for i in range(20):open(f"{d}/f{i}.txt","w").write("x")
print("create 20 files: view shows all, not stale after s:",wait(lambda v,l:has(l)==20 and not v.get("stale")))
shutil.rmtree(d)
print("delete 20 files: view clean, not stale after s:",wait(lambda v,l:has(l)==0 and not v.get("stale")))
