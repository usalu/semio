import sys,os
sys.path.insert(0,os.path.dirname(__file__))
from locksplit import *
E="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
head,pk=parse(E+"/Cargo.lock")
ed=resolve(pk)
epw=[i for i,p in enumerate(pk) if p["name"]=="semio-s-plugin-energy-epw"][0]
incoming=set(j for e in ed for j in e)
roots=[i for i in range(len(pk)) if i not in incoming]
print("roots",len(roots),"epw root",epw in roots)
sub=reach(ed,[epw]); print("epw closure",len(sub))
keepE=reach(ed,[r for r in roots if r!=epw]); print("energy keep",len(keepE),"of",len(pk))
print("dropped from energy",sorted(pk[i]["name"] for i in range(len(pk)) if i not in keepE))
if len(sys.argv)>1 and sys.argv[1]=="write":
    write(E+"/🧩️extensions/🌦️epw/Cargo.lock",head,pk,sub)
    write(E+"/Cargo.lock",head,pk,keepE)
