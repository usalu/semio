import json
S=[]
READY="(async()=>String(window.__h&&window.__h.ready()))()"
def step(name,body,**kw):
    d={"name":name,"js":"(async()=>{const h=window.__h;"+body+"})()"};d.setdefault("settleMs",300);d.update(kw);S.append(d)
def wait():
    S.append({"name":"wait for the initial document","js":"(async()=>'w')()","until":READY,"timeoutMs":420000,"settleMs":2000})
def save(name):
    json.dump(S,open(name,"w"),ensure_ascii=False,indent=1)
