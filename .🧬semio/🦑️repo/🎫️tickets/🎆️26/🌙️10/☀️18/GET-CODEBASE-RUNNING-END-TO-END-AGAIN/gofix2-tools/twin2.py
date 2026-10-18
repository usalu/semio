#!/usr/bin/env python3
import sys,difflib,os
from transplant import *
for n in sys.argv[1:]:
    if n not in cl or n not in mo: print("#####",n,"no twin"); continue
    p,s,e,body=cl[n][0]; mp,_,_,mbody=mo[n][0]
    new=mapbody(mbody)
    d=list(difflib.unified_diff(body.split("\n"),new.split("\n"),"client:"+os.path.basename(os.path.dirname(p)),"module:"+os.path.basename(mp),lineterm="",n=2))
    print("#####",n,"diff lines",len(d))
    print("\n".join(l[:240] for l in d[:70]))
