"""📣 Applies cross-crate widening to the declared families. usage: widen_run.py [--apply]"""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from families import *
import widen
from build_crates import CR
def run(apply=False):
    deps={cid:{n.replace("semio-framework-os-flow-","") for n,_ in c.internal} for cid,c in CR.items()}
    owner={};prov=set()
    for cid,files in LIB.items():
        for f in files:
            p=FL+f
            if os.path.exists(p):owner[p]=cid;prov.add(p)
    for cid,files in TESTS.items():
        for f in files:
            p=FL+f
            if os.path.exists(p):owner[p]=cid
    rep=widen.widen(owner,dry=not apply,deps=deps,providers=prov,skip=("tests",))
    for cid,r in rep.items():
        print(cid,len(r))
        print("   ",", ".join(n for loc,n in r[:400]))
    return rep
if __name__=="__main__":
    run("--apply" in sys.argv)
