#!/usr/bin/env python3
"""R86 stage 1: closure parts decided from row ids (closure-kinds), progress counters, cancellation. usage: mk1.py STAGE_DIR (reads N.base, writes N.new)"""
import sys,json
S=sys.argv[1]
def sub(text,old,new,count=1):
    assert text.count(old)==count,(old[:60],text.count(old))
    return text.replace(old,new)
t=open(S+"/1.base",encoding="utf8").read()
t=sub(t,'import{isAbsolute,relative as relativePath,resolve,sep}from"node:path";','import{isAbsolute,join,relative as relativePath,resolve,sep}from"node:path";')
t=sub(t,'import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";','import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";\nimport{createLayerIndexV1}from"../../📥️inference/🧭️script-inputs/🧾️layer-index/🟨️.mjs";\nimport{INFERENCE_LIMITS_V1}from"../../📥️inference/🧬️schema/🟨️.mjs";')
t=sub(t,'export function createClosureSettlerV1({store,workspace,limits=CLOSURE_SETTLE_V1,farm:farmOptions}={}){','export function createClosureSettlerV1({store,workspace,limits=CLOSURE_SETTLE_V1,farm:farmOptions,partIds}={}){')
t=sub(t,' const root=workspace?.workspaceRoot===undefined?undefined:resolve(workspace.workspaceRoot);\n',' const root=workspace?.workspaceRoot===undefined?undefined:resolve(workspace.workspaceRoot);\n const indexDirectory=partIds===false?undefined:partIds?.directory??(workspace?.cacheRoot===undefined?undefined:join(workspace.cacheRoot,"part-ids")),index=indexDirectory===undefined?undefined:createLayerIndexV1({directory:indexDirectory,limits:{...INFERENCE_LIMITS_V1,layerIndexMinimumRows:1}});\n const indexRows=(id,rows)=>{try{index?.indexSync(id,rows);}catch{}};\n')
t=sub(t,'persisted.add(id);partStore()?.put({key:{kind:"closure-part",subject:id,params:""},value:{rows},deps:[],rawReads:[],valueDigest:id,computedAtRev:"0"},"");buildMeta(id);}','persisted.add(id);partStore()?.put({key:{kind:"closure-part",subject:id,params:""},value:{rows},deps:[],rawReads:[],valueDigest:id,computedAtRev:"0"},"");buildMeta(id);indexRows(id,rows);}')
t=sub(t,'rows=rows.map(internRow);table.set(id,rows);}else rows=undefined;return rows;};','rows=rows.map(internRow);table.set(id,rows);indexRows(id,rows);}else rows=undefined;return rows;};')
t=sub(t,'const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map(),gate=footprintGateV1();\n  relief=gate?.relieve(()=>{delete pass.round;partOk.clear();sweepRows();});',
 'const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map(),idOk=new Map(),idFlight=new Map(),partStats={decided:0,byIds:0,loaded:0,probedRows:0,asked:0},gate=footprintGateV1();\n  relief=gate?.relieve(()=>{delete pass.round;partOk.clear();idOk.clear();sweepRows();});')
old_ph='''  const partHolds=async(id,abort)=>{
   if(summary!==undefined){const meta=await metaLoad(id);if(meta!==undefined&&!meta.touched)return meta.outside.length===0||await scope.holds(meta.outside,abort);}
   const rows=await loadPart(id);return rows!==undefined&&await scope.holds(rows,abort);
  };'''
new_ph='''  const beats=new Map(),beat=async(phase,done,total,force=false)=>{const now=Date.now();if(!force&&now-(beats.get(phase)??0)<CUTOFF_POLICY_V1.partProgressMilliseconds)return;beats.set(phase,now);await progress?.({phase,done,total:Math.max(done,total)});};
  const decideIds=async(entries,abort)=>{
   abort?.throwIfAborted();
   partStats.probedRows+=entries.length;
   if(await hold(entries.map(entry=>entry.row),abort)){for(const entry of entries)idOk.set(entry.id,true);return;}
   if(entries.length===1){idOk.set(entries[0].id,false);return;}
   const middle=entries.length>>1;await decideIds(entries.slice(0,middle),abort);await decideIds(entries.slice(middle),abort);
  };
  const claimIds=(entries,abort)=>{const run=decideIds(entries,abort).finally(()=>{for(const entry of entries)idFlight.delete(entry.id);});for(const entry of entries)idFlight.set(entry.id,run);return run;};
  const holdByIds=async(id,abort)=>{
   const ids=index?.idsSync(id);if(ids===undefined)return undefined;
   const unknown=[],queued=new Set();
   for(let at=0;at<ids.length;at++){const rowId=ids[at];if(!idOk.has(rowId)&&!queued.has(rowId)){queued.add(rowId);unknown.push(rowId);}}
   for(let from=0;from<unknown.length;from+=CUTOFF_POLICY_V1.partRowBatchRows){
    abort?.throwIfAborted();
    const entries=[],joined=new Set();
    for(const rowId of unknown.slice(from,from+CUTOFF_POLICY_V1.partRowBatchRows)){
     if(idOk.has(rowId))continue;
     const flying=idFlight.get(rowId);if(flying!==undefined){joined.add(flying);continue;}
     const row=index.rowSync(rowId);if(row===undefined)return undefined;
     entries.push({id:rowId,row});
    }
    if(entries.length)joined.add(claimIds(entries,abort));
    await Promise.all(joined);
    if(unknown.slice(from,from+CUTOFF_POLICY_V1.partRowBatchRows).some(rowId=>idOk.get(rowId)===false))return false;
   }
   for(let at=0;at<ids.length;at++)if(idOk.get(ids[at])!==true)return false;
   return true;
  };
  const partHolds=async(id,abort)=>{
   if(summary!==undefined){const meta=await metaLoad(id);if(meta!==undefined&&!meta.touched)return meta.outside.length===0||await scope.holds(meta.outside,abort);}
   const listed=await holdByIds(id,abort);if(listed!==undefined){partStats.byIds++;return listed;}
   partStats.loaded++;
   const rows=await loadPart(id);return rows!==undefined&&await scope.holds(rows,abort);
  };'''
t=sub(t,old_ph,new_ph)
old_hp='holdParts:async(ids,abort)=>{for(const id of ids){let ok=partOk.get(id);if(ok===undefined){ok=await partHolds(id,abort);partOk.set(id,ok);}if(!ok)return false;}return true;}'
new_hp='holdParts:async(ids,abort)=>{for(const id of ids){abort?.throwIfAborted();let ok=partOk.get(id);if(ok===undefined){partStats.asked++;ok=await partHolds(id,abort);partOk.set(id,ok);partStats.decided++;await beat("flush parts",partStats.decided,partStats.asked);}if(!ok)return false;}return true;}'
t=sub(t,old_hp,new_hp)
old_sup='''   const supportAt=Date.now();
   for(const name of chunk){
    const row=pseudoRows.get(name),node=nodes.get(name);'''
new_sup='''   const supportAt=Date.now();
   let built=0;beats.set("flush support",Date.now());
   for(const name of chunk){
    signal?.throwIfAborted();if(built>0)await beat("flush support",built,chunk.length);built++;
    const row=pseudoRows.get(name),node=nodes.get(name);'''
t=sub(t,old_sup,new_sup)
t=sub(t,'return{held,kept,changed,answers,failed,vouched,slow:slow.length,farmed:farm?.stats().answered??0,reused:Math.max(0,names.length-failed-fresh.size)};','return{held,kept,changed,answers,failed,vouched,slow:slow.length,farmed:farm?.stats().answered??0,reused:Math.max(0,names.length-failed-fresh.size),parts:{...partStats,rowsDecided:idOk.size}};')
t=sub(t,'  forgetParts(true);await clock.time("save",saveManifest);','  if(partStats.asked>0)await beat("flush parts",partStats.decided,partStats.asked,true);\n  forgetParts(true);await clock.time("save",saveManifest);')
open(S+"/1.new","w",encoding="utf8").write(t)
# schema stage
s=open(S+"/2.base",encoding="utf8").read();d=json.loads(s);c=d["definitions"]["Policy"]["const"]
c["partRowBatchRows"]=4096;c["partProgressMilliseconds"]=5000
d["description"]+=" EN: A closure part is decided from the dictionary ids of its rows: `partRowBatchRows` rows are probed per batch, every distinct row once per settle, and the settle reports `flush parts N of M` at least every `partProgressMilliseconds`. DE: Ein Closure-Teil wird aus den Dictionary-Ids seiner Zeilen entschieden: je Stapel werden `partRowBatchRows` Zeilen geprueft, jede verschiedene Zeile einmal je Abgleich, und der Abgleich meldet `flush parts N of M` mindestens alle `partProgressMilliseconds`."
open(S+"/2.new","w",encoding="utf8").write(json.dumps(d,indent=1,ensure_ascii=False)+("\n" if s.endswith("\n") else ""))
print("staged")
