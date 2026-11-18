import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage2");IN=B+"🕸️graph/📥️inference/";SIC=IN+"🧭️script-inputs/"
sch=IN+"🧬️schema/🔣️.json"
txt=open(R+"/"+sch,encoding="utf8").read().split("\n")
out=[]
add={"c":[('closureEntryDigestBytes',48),('closureSidecarCacheBytes',100663296),('closureSidecarFactBytes',1024),('closureSidecarWalkBytes',8)],
 "en":{'closureEntryDigestBytes':"The bytes one layer digest of an in-process closure entry is charged: the entry names the layer by its digest and holds one reference to the shared walk facts and one to the shared walk members of the layer.",
       'closureSidecarCacheBytes':"The byte budget (96 MiB) of the walk facts and walk members of the stored layers that the loaded entries of a process share by layer digest: an entry that names a layer already there holds that instance, so the sidecar of a layer exists once however many entries name it.",
       'closureSidecarFactBytes':"The bytes one shared walk fact of a stored layer is charged.",
       'closureSidecarWalkBytes':"The bytes one walk of the shared members of a stored layer is charged: a reference."},
 "de":{'closureEntryDigestBytes':"Die Bytes, die ein Schicht-Digest eines Closure-Eintrags im Prozess kostet: der Eintrag nennt die Schicht mit ihrem Digest und hält eine Referenz auf die geteilten Walk-Fakten und eine auf die geteilten Walk-Mitglieder der Schicht.",
       'closureSidecarCacheBytes':"Das Bytebudget (96 MiB) der Walk-Fakten und Walk-Mitglieder der gespeicherten Schichten, die die geladenen Einträge eines Prozesses nach Schicht-Digest teilen: ein Eintrag, der eine dort schon vorhandene Schicht nennt, hält diese Instanz, der Sidecar einer Schicht existiert also einmal, wie viele Einträge sie auch nennen.",
       'closureSidecarFactBytes':"Die Bytes, die ein geteiltes Walk-Faktum einer gespeicherten Schicht kostet.",
       'closureSidecarWalkBytes':"Die Bytes, die ein Walk der geteilten Mitglieder einer gespeicherten Schicht kostet: eine Referenz."}}
import json
for line in txt:
    out.append(line)
    if '"closureEntryValueItemBytes"' in line:
        ind=line[:len(line)-len(line.lstrip())]
        if line.strip().endswith("16,"):
            for k,v in add["c"]:out.append('%s"%s": %d,'%(ind,k,v))
        elif "The bytes one item" in line:
            for k in add["en"]:out.append('%s"%s": %s,'%(ind,k,json.dumps(add["en"][k],ensure_ascii=False)))
        else:
            for k in add["de"]:out.append('%s"%s": %s,'%(ind,k,json.dumps(add["de"][k],ensure_ascii=False)))
new="\n".join(out)
json.loads(new)
i=str(s.n);s.n+=1;open(s.S+"/"+i+".base","w",encoding="utf8").write("\n".join(txt));open(s.S+"/"+i+".new","w",encoding="utf8").write(new);s.rows.append((i,sch,"edit"))
s.edit(SIC+"💾️entries/🟨️.mjs",[
(' const packSidecar=',''' const sharedFacts=createBoundedCacheV1({name:"inference:closure-sidecar-facts",maxEntries:limits.closureStoreMaximumLayers,maxBytes:limits.closureSidecarCacheBytes,sizeOf:()=>limits.closureSidecarFactBytes}),sharedMembers=createBoundedCacheV1({name:"inference:closure-sidecar-members",maxEntries:limits.closureStoreMaximumLayers,maxBytes:limits.closureSidecarCacheBytes,sizeOf:list=>limits.closureSidecarWalkBytes*(list.length+8)});
 const sameList=(left,right)=>left.length===right.length&&left.every((item,at)=>item===right[at]);
 const packSidecar=''',1),
(' const unpackSidecar=({walks=[],...sidecar})=>{const table=walks.map(intern);return{...sidecar,members:sidecar.members?.map(list=>list&&list.map(at=>table[at]))};};',
 ''' const unpackSidecar=({walks=[],...sidecar},digests)=>{
  const table=walks.map(intern);
  const facts=sidecar.facts?.map((fact,at)=>{if(!fact)return fact;const text=JSON.stringify(fact),known=sharedFacts.get(digests[at]);if(known!==undefined&&known.text===text)return known.fact;sharedFacts.set(digests[at],{text,fact});return fact;});
  const members=sidecar.members?.map((list,at)=>{if(!list)return list;const mapped=list.map(index=>table[index]),known=sharedMembers.get(digests[at]);if(known!==undefined&&sameList(known,mapped))return known;sharedMembers.set(digests[at],mapped);return mapped;});
  return{...sidecar,facts,members};
 };''',1),
('return{recipe:record.recipe,value,rows:shareObservationRowsV1(record.rows),digests:record.layers,sidecar:record.sidecar?unpackSidecar(record.sidecar):null,bytes:text.length+8*record.layers.length};',
 'const digests=record.layers.map(intern);return{recipe:record.recipe,value,rows:shareObservationRowsV1(record.rows),digests,sidecar:record.sidecar?unpackSidecar(record.sidecar,digests):null,bytes:text.length+8*record.layers.length};',1),
])
s.edit(SIC+"🟨️.mjs",[
('sizeOf: entry => INFERENCE_LIMITS_V1.closureEntryBaseBytes + INFERENCE_LIMITS_V1.closureEntryRowBytes * entry.rows.length +','sizeOf: entry => INFERENCE_LIMITS_V1.closureEntryBaseBytes + INFERENCE_LIMITS_V1.closureEntryDigestBytes * (entry.digests?.length ?? 0) + INFERENCE_LIMITS_V1.closureEntryRowBytes * entry.rows.length +',1),
])
test='''import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClosureStoreV1}from"../🟨️.mjs";

/** 💾️ Loaded closure entries that name the same stored layers share the walk facts and walk members of a layer: the sidecar of a layer exists once however many entries name it (measured on the real store: 274 layers per entry, 3 766 entries held 4 GB of per-entry copies on a worker taking 180 owners). */
test("many loaded entries that share their layers hold the sidecar of a layer once, not once per entry",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-entries-share-"));
 try{
  const directory=join(root,"ce"),store=createClosureStoreV1({directory}),pool=300,layersPer=200,walks=150,entries=120;
  const layer=(key:number)=>[{kind:"file",path:"/layer/"+key,fact:{size:key}}],members=(key:number)=>Array.from({length:walks},(_all,at)=>"walk:"+key+":"+at+":"+"x".repeat(40)),fact=(key:number)=>({kind:"walk",path:"/walk/"+key,fact:{members:key}});
  const pick=(entry:number)=>Array.from({length:layersPer},(_all,at)=>(entry*7+at)%pool);
  for(let entry=0;entry<entries;entry++){const keys=pick(entry);await store.save("entry-"+entry,{recipe:{fn:"fixture"},value:[entry],rows:[],layers:keys.map(layer),sidecar:{facts:keys.map(fact),members:keys.map(members)}});}
  const reader=createClosureStoreV1({directory,readOnly:true});
  Bun.gc(true);const before=process.memoryUsage().heapUsed,held:any[]=[];
  for(let entry=0;entry<entries;entry++)held.push(await reader.load("entry-"+entry));
  Bun.gc(true);const grown=(process.memoryUsage().heapUsed-before)/1048576;
  expect(held.every(item=>item?.sidecar?.members.length===layersPer&&item.digests.length===layersPer)).toBe(true);
  expect(held[0].sidecar.members[7]).toEqual(members(7));
  expect(held[1].sidecar.facts[0]).toEqual(fact(7));
  expect(held[0].sidecar.members[7]).toBe(held[1].sidecar.members[0]);
  expect(held[0].sidecar.facts[7]).toBe(held[1].sidecar.facts[0]);
  expect(grown).toBeLessThan(10);
 }finally{rmSync(root,{recursive:true,force:true});}
});
'''
s.new(SIC+"💾️entries/🧪️tests/🟦️.ts",test)
T1=SIC+"💾️entries/🧪️tests/🟦️.ts"
s.finish([T1,SIC+"🧪️tests/closure-store-limits.test.ts",IN+"🧪️tests/🌊️layer-stream/🟦️.ts",IN+"🧪️tests/🧺️bundle-rows/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SIC+"🧹️retention/🧪️tests/🟦️.ts"],"- Step 2: loaded closure entries share the walk facts and walk members of a layer by layer digest (byte-capped), the digests are interned and the entry cache charges them; law `💾️entries/🧪️tests`: 120 entries x 200 layers x 150 walks held under 10 MB (before: per-entry copies). Cause named by the Step 1 report: `closure:entries.sidecar` carried the heap (3 766 entries, about 1.2 MB each).")
open(s.S+"/before.txt","w",encoding="utf8").write(T1+"\n")
