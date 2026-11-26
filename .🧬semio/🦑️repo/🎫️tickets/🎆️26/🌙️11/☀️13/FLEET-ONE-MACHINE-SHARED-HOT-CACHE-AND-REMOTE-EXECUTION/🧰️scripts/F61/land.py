import os,sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        if s.count(a)!=1: sys.exit("anchor count %d: %s"%(s.count(a),a[:80]))
        s=s.replace(a,b)
    t=path+".f61.tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,path)
edit(L+"💾️entries/🟨️.mjs",[("""  async load(id){""","""  /** 🔍️ Why `load(id)` answers `undefined`, in the words of the first failing step: no entry file (with the directory), unreadable, other format, other id, malformed, digest mismatch or a lost value; `loads` when the entry loads. A diagnosis only, it never changes the store. */
  async why(id){
   const bytes=await readFile(entryPath(id)).catch(()=>undefined);if(!bytes)return"no entry file "+sha(id).slice(0,12)+" in "+entries;
   let record;try{record=JSON.parse(unpack(bytes));}catch{return"entry file unreadable";}
   if(record?.version!==3)return"entry format "+String(record?.version);
   if(record.id!==id)return"entry names another id";
   if(typeof record.valueDigest!=="string"||!Array.isArray(record.rows)||!Array.isArray(record.layers))return"entry malformed";
   if(sha(JSON.stringify(record.rows)+record.layers.join(",")+record.valueDigest)!==record.digest)return"entry digest mismatch";
   return valueSync(record.valueDigest)===undefined?"value "+record.valueDigest.slice(0,12)+" lost":"loads";
  },
  async load(id){""")])
edit(L+"🟨️.mjs",[
("""  const miss = (movedRow, absent = false) => Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS", movedRow, absent });""",
"""  const miss = (movedRow, absent = false, storeWhy = undefined) => Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS", movedRow, absent, storeWhy });"""),
("""  if (!entry) throw miss(importRefusalsV1.get(id) ?? undefined, true);""",
"""  if (!entry) throw miss(importRefusalsV1.get(id) ?? undefined, true, store ? typeof store.why === "function" ? await store.why(id).catch(() => undefined) : undefined : "no closure store bound for this workspace");"""),
("""(moved === undefined ? ", never shipped or taken here" : ", refused at the intake, moved " + moved.kind + " " + String(moved.path).slice(-120)) + ")";""",
"""(moved === undefined ? ", never shipped or taken here" : ", refused at the intake, moved " + moved.kind + " " + String(moved.path).slice(-120)) + (error.storeWhy === undefined ? "" : "; store: " + error.storeWhy) + ")";"""),
])
print("ok")
