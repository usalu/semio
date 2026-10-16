"""📣 Cross-crate widening for the workflow producers. usage: widen_wf.py [--apply]"""
import os,sys
os.environ["OSFAM"]="workflow"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import widen,wf_families as W
from manifests import FL
deps={"graph":set(),"snapshot":{"graph"},"schema":{"graph","snapshot"},"schema-laws":{"graph","snapshot","schema"}}
owner={};prov=set()
for cid,files in W.LIB.items():
    for f in files:
        p=FL+f
        if os.path.exists(p):owner[p]=cid;prov.add(p)
for cid,files in W.TESTS.items():
    for f in files:
        p=FL+f
        if os.path.exists(p):owner[p]=cid
rep=widen.widen(owner,dry="--apply" not in sys.argv,deps=deps,providers=prov,skip=("tests",))
for cid,r in rep.items():
    print(cid,len(r));print("   ",", ".join(n for loc,n in r[:400]))
