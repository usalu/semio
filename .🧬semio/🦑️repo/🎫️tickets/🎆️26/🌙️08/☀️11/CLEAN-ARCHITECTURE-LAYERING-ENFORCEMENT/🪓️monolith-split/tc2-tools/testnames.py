"""tc2: test fn name multiset of the 16 crates in the baseline snapshot vs the working tree (16 crates + companions)."""
import sys,os,json,glob,collections
sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
from ttclib import *
import ttc
from ttc import closure, test_names, snap_root
import testonly as T
mans=Manifests(Source(None)); mans.build_index(force=True)
snap=Source("snap:"+snap_root("base")); cur=Source(None)
orig=["semio-s-artifact-"+c for c in T.CRATES]
comps=[json.load(open(f))["name"] for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json"))]
b=collections.Counter(); c=collections.Counter()
per={}
for p in orig:
    man,files,code=closure(snap,mans,p); n=test_names(files,code); b+=n; per[p]=[sum(n.values()),0]
for p in orig+comps:
    man,files,code=closure(cur,mans,p); n=test_names(files,code); c+=n
    per.setdefault(p,[0,0])[1]=sum(n.values())
ob,oc=b-c,c-b
print(f"baseline {sum(b.values())} tests ({len(b)} names); current {sum(c.values())} ({len(c)} names)")
print("IDENTICAL multiset" if not ob and not oc else f"DIFFERENT only-baseline={dict(ob)} only-current={dict(oc)}")
for p,(x,y) in per.items(): print(f"  {p.replace('semio-s-artifact-','')}: baseline {x} / now {y}")
