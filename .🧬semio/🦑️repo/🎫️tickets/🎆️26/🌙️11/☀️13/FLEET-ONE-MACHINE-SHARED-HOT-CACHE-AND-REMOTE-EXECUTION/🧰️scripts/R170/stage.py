#!/usr/bin/env python3
import sys,os,importlib.util,hashlib
d=os.path.dirname(os.path.abspath(__file__));s=importlib.util.spec_from_file_location("patch",d+"/patch.py");P=importlib.util.module_from_spec(s);s.loader.exec_module(P)
S=sys.argv[1];os.makedirs(S,exist_ok=True)
for k,p in P.LIVE.items():
    t=open(p,encoding="utf8").read();n=P.edit(k,t)
    ext=os.path.splitext(p)[1]
    open(os.path.join(S,k+ext),"w",encoding="utf8").write(n)
    open(os.path.join(S,k+".base"),"w").write(hashlib.sha256(t.encode()).hexdigest())
    print(k,len(t),"->",len(n))
