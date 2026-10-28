"""[gram3-g] writes edits.json of the batch `index-chunks` (REPLACES gram2-g's batch of the same name): the chunked construction of the fact index lives in the sub-module `🕰️generations/🧱️chunk-classes` (own schema, policy and law) and `🕰️generations/🟨️.mjs` only delegates to it, so the generations module stays far below the 65,536 byte bound of the graph consumers. Each `old` must occur exactly once in the live file. usage: make_edits.py"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 10)))
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/"
MJS, DTS = G + "🟨️.mjs", G + "🟨️.d.mts"
edits = []


def edit(file, name, old, new):
    edits.append({"file": file, "name": name, "old": old, "new": new})


live = open(os.path.join(ROOT, MJS), encoding="utf8").read()
a = live.index("export function createFactIndexV1(ownerRows){")
b = live.index("/** 🩹️ Replaces the consumers in `upserts`")
create_old = live[a:b]

edit(MJS, "import",
     'import{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";',
     'import{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";\nimport{INDEX_CHUNK_POLICY_V1,addOwnerRunsV1,bulkUpsertClassesV1,classesOfChunksV1,createChunkInternerV1}from"./🧱️chunk-classes/🟨️.mjs";')
edit(MJS, "create", create_old, """export function createFactIndexV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),empty=[],interner=createChunkInternerV1();let seen=new Int32Array(1<<16),scratch=new Uint32Array(1<<12);
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name);if(scratch.length<list.length)scratch=new Uint32Array(Math.max(list.length,scratch.length*2));
  let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;scratch[count++]=id;}
  }
  if(!count){empty.push({r:[],o:[owner]});return;}
  addOwnerRunsV1(interner,scratch,count,owner);
 });
 return{version:1,owners,rows,classes:[...classesOfChunksV1(rows.length,interner),...empty]};
}

""")
edit(MJS, "patch signature",
     "export function patchFactIndexV1(index,{drop=[],upserts=new Map(),append=new Map()}){",
     "export function patchFactIndexV1(index,{drop=[],upserts=new Map(),append=new Map()},{bulkRows=INDEX_CHUNK_POLICY_V1.bulkUpsertRows}={}){")
edit(MJS, "patch bulk",
     " for(const name of[...upserts.keys()].sort(compareCodePointsV1))classes.push({r:rowIdsFor(upserts.get(name)),o:[ownerIds.get(name)]});\n",
     " let pairs=0;for(const list of upserts.values())pairs+=list.length;\n if(pairs>=bulkRows)bulkUpsertClassesV1({upserts,ownerIds,idOfRow,rowCount:rows.length,classes});\n else\n for(const name of[...upserts.keys()].sort(compareCodePointsV1))classes.push({r:rowIdsFor(upserts.get(name)),o:[ownerIds.get(name)]});\n")
edit(DTS, "patch signature",
     "append?:ReadonlyMap<string,readonly GenerationRowV1[]>}):FactIndexV1;",
     "append?:ReadonlyMap<string,readonly GenerationRowV1[]>},options?:{bulkRows?:number}):FactIndexV1;")

if __name__ == "__main__":
    for e in edits:
        text = open(os.path.join(ROOT, e["file"]), encoding="utf8").read()
        if text.count(e["old"]) != 1:
            sys.exit(f'anchor "{e["name"]}" occurs {text.count(e["old"])} times in {e["file"]}')
    json.dump({"version": 1, "edits": edits}, open(os.path.join(HERE, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(len(edits), "edits")
