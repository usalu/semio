import os,sys
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
B=LIB+"/🏗️build";N=HERE+"/new"
OUT=os.environ["R145_SCRATCH"]+"/stage"
MAP={"never-here":B+"/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🐘️never-here","doomed-handoff":B+"/🏡️resident/🧪️tests/🚫️doomed-handoff","owner-scale":B+"/👁️observation/🧪️tests/🌊️owner-scale"}
for k,d in MAP.items():
    t=absolutize(read(N+"/"+k+"/🟦️.ts"),d)
    write(OUT+"/"+k+".test.ts",t)
print("staged",list(MAP))
import patches as P
nodes=absolutize(P.patch_nodes(read(P.NODES)),os.path.dirname(P.NODES))
write(OUT+"/nodes/🟨️.mjs",nodes)
pool=absolutize(P.patch_pool(read(P.POOL)),os.path.dirname(P.POOL))
write(OUT+"/pool/🟨️.mjs",pool)
for k in ("never-here","doomed-handoff"):
    p=OUT+"/"+k+".test.ts";t=read(p)
    t=t.replace(P.NODES,OUT+"/nodes/🟨️.mjs").replace(P.POOL,OUT+"/pool/🟨️.mjs")
    write(OUT+"/staged-"+k+".test.ts",t)
write(OUT+"/staged-owner-scale.test.ts",read(OUT+"/owner-scale.test.ts"))
print("patched staged")
