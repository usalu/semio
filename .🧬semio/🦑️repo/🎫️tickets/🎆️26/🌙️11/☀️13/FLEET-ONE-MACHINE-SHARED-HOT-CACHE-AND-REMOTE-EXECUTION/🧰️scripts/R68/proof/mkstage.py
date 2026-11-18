import os,sys,hashlib,shutil
R="/Users/ueli/Documents/semio";W="/private/tmp/r68/ws"
OUT=sys.argv[1];S=OUT+"/stage"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";GS=B+"🪪️graph-state/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
r67=[l for l in open("/private/tmp/r68/r67files.txt",encoding="utf8").read().splitlines() if l]
mine=[PN+"🟨️.mjs",PN+"🌐️owner-pool/🟦️.ts",GS+"🙋️wanted/📖️read/🟦️.ts",GS+"📦️project-records/🟦️.ts",
 PN+"📥️take/🟨️.mjs",PN+"📥️take/🟨️.d.mts",PN+"📥️take/🧬️schema/🔣️.json",PN+"📥️take/🧪️tests/🟦️.ts",PN+"📥️take/🧪️tests/🐍️.py",PN+"📥️take/🧪️tests/🧫️fixtures/🔣️.json",
 GS+"📦️project-records/🌱️fresh/🟦️.ts",GS+"📦️project-records/🌱️fresh/🧬️schema/🔣️.json",GS+"📦️project-records/🌱️fresh/🧪️tests/🟦️.ts",GS+"📦️project-records/🌱️fresh/🧪️tests/🐍️.py",GS+"📦️project-records/🌱️fresh/🧪️tests/🔣️.json",
 B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",PN+"🧪️tests/📥️take/🟦️.ts",PN+"🧪️tests/📥️take/🐍️.py",PN+"🧪️tests/📥️take/🧫️fixtures/🔣️.json"]
files=list(dict.fromkeys(r67+mine))
if os.path.exists(S):shutil.rmtree(S)
os.makedirs(S)
rows=[];same=0
for rel in files:
    new=W+"/"+rel;live=R+"/"+rel
    assert os.path.exists(new),rel
    i=hashlib.sha1(rel.encode("utf8")).hexdigest()[:6]
    nb=open(new,"rb").read()
    if os.path.exists(live):
        lb=open(live,"rb").read()
        if lb==nb:same+=1;continue
        open(S+"/"+i+".base","wb").write(lb);kind="mod"
    else:kind="new"
    open(S+"/"+i+".new","wb").write(nb);rows.append((i,rel,kind))
open(S+"/index.txt","w",encoding="utf8").write("\n".join(" ".join(r) for r in rows)+"\n")
print(len(rows),"files staged;",sum(1 for r in rows if r[2]=="mod"),"mod",sum(1 for r in rows if r[2]=="new"),"new;",same,"identical to live")
