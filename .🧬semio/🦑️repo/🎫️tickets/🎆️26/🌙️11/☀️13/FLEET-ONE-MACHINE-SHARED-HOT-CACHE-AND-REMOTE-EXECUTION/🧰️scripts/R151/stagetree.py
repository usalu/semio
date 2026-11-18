import sys,os,re
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
import patches as P
def absolutize(text,origin_dir):
    def r(m):
        s=m.group(2)
        return m.group(1)+os.path.normpath(os.path.join(origin_dir,s))+m.group(3) if s.startswith("../") or s.startswith("./") else m.group(0)
    return re.sub(r'''((?:from\s*|import\s*\(\s*|import\s*)["'])(\.{1,2}/[^"']+)(["'])''',r,text)
OUT=SC+"/stagetree"
D=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready"
files={TREE:P.patch_tree,READY:P.patch_ready,TREE_SCHEMA:P.patch_tree_schema,READY_SCHEMA:P.patch_ready_schema}
staged={}
for path,fn in files.items():
    out=OUT+"/"+path[len(B)+1:].replace("/","_")
    new=fn(read(path))
    if path.endswith(".json"): staged[path]=(out,new)
    else: staged[path]=(out,absolutize(new,os.path.dirname(path)))
READY_OUT=OUT+"/staged-ready/🟦️.ts"
def remap(t):
    for path,(out,_) in staged.items(): t=t.replace(path,READY_OUT if path==READY else out)
    return t
for path,(out,new) in staged.items():
    write(READY_OUT if path==READY else out,remap(new))
tt=read(D+"/🧪️tests/🕰️stale-stage/🟦️.ts")
write(OUT+"/stale-stage.test.ts",remap(absolutize(tt,D+"/🧪️tests/🕰️stale-stage")))
write(OUT+"/staged-ready/🏃️wrapper/🟦️.ts",read(D+"/🏃️wrapper/🟦️.ts"))
mt=absolutize(P.patch_ready_test(read(READY_TEST)),D+"/🧪️tests")
write(OUT+"/main.test.ts",remap(mt))
tt2=absolutize(P.patch_tree_test(read(P.TREE_TEST)),os.path.dirname(P.TREE_TEST))
write(OUT+"/tree.test.ts",remap(tt2))
print("ok",OUT)
