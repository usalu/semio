import json,re,sys,os,shutil
R=os.environ.get("SEMIO_ROOT","/Users/ueli/Documents/semio")
D=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
C=D+"💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/"
files={"worker":C+"👷️closure-read-worker/🟨️.mjs","schema":C+"🧬️schema/🔣️.json","texts":D+"📖️texts/🔣️.json"}
out=sys.argv[1] if len(sys.argv)>1 else None
def sub(s,a,b):
    assert s.count(a)==1,a[:60]
    return s.replace(a,b)
w=open(files["worker"]).read()
w=sub(w,'if(++nodes>P.maximumNodes||depth>P.maximumDepth)throw Error("Bounded closure reference DATA required");','if(++nodes>P.maximumNodes||depth>P.maximumDepth)throw refuseCachingV1("invalid","closure.reference-data-bound",{nodes,depth,maximumNodes:P.maximumNodes,maximumDepth:P.maximumDepth});')
sc=open(files["schema"]).read()
sc=sub(sc,'"maximumNodes": 4194304','"maximumNodes": 16777216')
sc=sub(sc,'"kind": {"enum": ["inference", "precision", "cargo-receipt", "generation-fragment", "generation-index", "generation-frontier", "observation", "snapshot-record", "snapshot", "store-format"]}','"kind": {"enum": ["inference", "precision", "cargo-receipt", "generation-fragment", "generation-index", "generation-frontier", "observation", "snapshot-record", "snapshot", "store-format", "publication", "receipt", "json", "pack"]}') if '"kind": {"enum": ["inference", "precision"' in sc else sc
t=open(files["texts"]).read()
t=sub(t,'    "headroom.retention-ledger-invalid": {','    "closure.reference-data-bound": {\n      "en": "A stored closure payload has {nodes} nodes at depth {depth}; at most {maximumNodes} nodes and depth {maximumDepth} are read",\n      "de": "Eine gespeicherte Hüllen-Nutzlast hat {nodes} Knoten in Tiefe {depth}; gelesen werden höchstens {maximumNodes} Knoten und Tiefe {maximumDepth}"\n    },\n    "headroom.retention-ledger-invalid": {')
for k,v in(("worker",w),("schema",sc),("texts",t)):
    if k!="worker":json.loads(v)
    if out:
        p=out+"/"+k;open(p,"w").write(v)
print("ok",out)
