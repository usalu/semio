import os,sys,hashlib
ROOT="/Users/ueli/Documents/semio"
LIB=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B=LIB+"/🏗️build"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R151"
LOAD=B+"/🕸️graph/📥️load/🟦️.ts"
GRAPH_SCHEMA=B+"/🕸️graph/🧬️schema/🔣️.json"
OBS=B+"/👁️observation/🟨️.mjs"
OBS_D=B+"/👁️observation/🟨️.d.mts"
TREE=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts"
TREE_SCHEMA=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json"
READY_SCHEMA=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json"
READY=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"
def read(p): return open(p,encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True)
    open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode("utf8")).hexdigest()
def once(t,old,new,what=""):
    n=t.count(old)
    if n!=1: raise SystemExit("anchor %s found %d times: %s"%(what,n,old[:80]))
    return t.replace(old,new)
READY_TEST=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts"
TREE_TEST=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🟦️.ts"
