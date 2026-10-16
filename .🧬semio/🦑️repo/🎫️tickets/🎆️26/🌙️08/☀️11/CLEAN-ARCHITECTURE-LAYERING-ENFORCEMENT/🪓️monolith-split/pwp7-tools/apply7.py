import sys, shutil, hashlib, json, os, time
sys.path.insert(0,'.')
from finish7 import *
SNAP=GEN+'/pwp7-snap'
def sha(b): return hashlib.sha256(b).hexdigest()
def apply(writes, deletes, tag):
    pre={}
    for rel in list(writes)+list(deletes):
        p=PLUG+rel
        pre[rel]=sha(open(p,'rb').read()) if os.path.exists(p) else None
    # snapshot (first pre-state wins)
    for rel,h in pre.items():
        if h is None: continue
        sp=SNAP+'/'+rel
        if not os.path.exists(sp):
            os.makedirs(os.path.dirname(sp),exist_ok=True)
            shutil.copy2(PLUG+rel,sp)
    created=[];modified=[];removed=[]
    for rel,txt in writes.items():
        p=PLUG+rel
        cur=sha(open(p,'rb').read()) if os.path.exists(p) else None
        assert cur==pre[rel],('peer edit during apply',rel)
        os.makedirs(os.path.dirname(p),exist_ok=True)
        old=open(p,encoding='utf-8').read() if cur else None
        if old==txt: continue
        open(p,'w',encoding='utf-8').write(txt)
        (modified if cur else created).append(rel)
    for rel in deletes:
        p=PLUG+rel
        if os.path.exists(p): os.remove(p); removed.append(rel)
    st={'tag':tag,'created':created,'modified':modified,'removed':removed,'time':time.time()}
    tj=GEN+'/pwp7-touched.json'
    allt=json.load(open(tj)) if os.path.exists(tj) else []
    allt.append(st); json.dump(allt,open(tj,'w'),ensure_ascii=False,indent=1)
    return st
if __name__=='__main__':
    W,writes,deletes=finish()
    st=apply(writes,deletes,sys.argv[1] if len(sys.argv)>1 else 'batch')
    print({k:len(v) for k,v in st.items() if isinstance(v,list)})
