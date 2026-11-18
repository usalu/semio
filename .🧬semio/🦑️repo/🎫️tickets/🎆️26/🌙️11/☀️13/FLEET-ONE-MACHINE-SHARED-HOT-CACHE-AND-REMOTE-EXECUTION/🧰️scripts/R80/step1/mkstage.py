#!/usr/bin/env python3
import os,sys,shutil
R="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
H=os.path.dirname(os.path.abspath(__file__))
S=os.environ.get("R80_STAGE","/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R80/stage1")
shutil.rmtree(S,ignore_errors=True);os.makedirs(S)
rows=[];n=[0]
def edit(rel,subs,new=None):
    i=str(n[0]);n[0]+=1
    p=R+"/"+rel
    if new is not None:
        open(S+"/"+i+".new","w",encoding="utf8").write(new);rows.append((i,rel,"new"));return
    base=open(p,encoding="utf8").read();cur=base
    for old,rep,count in subs:
        assert cur.count(old)==count,(rel,old[:60],cur.count(old))
        cur=cur.replace(old,rep)
    open(S+"/"+i+".base","w",encoding="utf8").write(base);open(S+"/"+i+".new","w",encoding="utf8").write(cur);rows.append((i,rel,"edit"))

SR=B+"🧠️memory/📋️slice-report/"
edit(SR+"🟨️.mjs",[
('let phase={name:"start",done:0,total:0},stage,stop;',
 '''let phase={name:"start",done:0,total:0},stage,stop;
const collectionSources=new Map();

/** 🧮️ Describes one collection a part of the read holds across owners for the report: a Map, Set or array by its entries, a plain object by its keys; `sample()` answers the entries the size is estimated from. */
export function describeCollectionV1(name,collection){
 if(collection instanceof Map)return{name,entries:collection.size,sample:()=>take(collection.entries(),P.collectionSample)};
 if(collection instanceof Set)return{name,entries:collection.size,sample:()=>take(collection.values(),P.collectionSample)};
 if(Array.isArray(collection))return{name,entries:collection.length,sample:()=>collection.slice(0,P.collectionSample)};
 const keys=Object.keys(collection??{});return{name,entries:keys.length,sample:()=>keys.slice(0,P.collectionSample).map(key=>collection[key])};
}
const take=(iterator,count)=>{const out=[];for(const item of iterator){out.push(item);if(out.length>=count)break;}return out;};

/** 🔌️ Registers a function that answers the collections of one part as `{name,entries,sample()}` or `{name,entries,bytes}`; returns the function that removes it. */
export function registerSliceCollectionsV1(name,read){collectionSources.set(name,read);return()=>{if(collectionSources.get(name)===read)collectionSources.delete(name);};}

/** 📏️ The estimated bytes of `entries` entries: entries times the mean size of the sampled ones (a string by twice its length, anything else by the length of its JSON, a sample that cannot be written by 64). */
export function estimateCollectionBytesV1(entries,sample){
 if(sample.length===0)return 0;
 let sum=0;for(const item of sample){if(typeof item==="string")sum+=item.length*2;else try{sum+=(JSON.stringify(item)??"").length;}catch{sum+=64;}}
 return Math.round(entries*sum/sample.length);
}

/** 🧾️ The `top` collections by estimated bytes, bounded to `bytes` characters of JSON. */
export function collectSliceCollectionsV1(policy=P){
 const rows=[];
 for(const[,read]of collectionSources){try{for(const row of read()){try{rows.push({name:String(row.name).slice(0,80),entries:row.entries,estimatedBytes:typeof row.bytes==="number"?Math.round(row.bytes):estimateCollectionBytesV1(row.entries,row.sample?.()??[])});}catch{}}}catch{}}
 rows.sort((a,b)=>b.estimatedBytes-a.estimatedBytes);
 const kept=rows.slice(0,policy.collectionsTop);
 while(kept.length>0&&JSON.stringify(kept).length>policy.collectionsBytes)kept.pop();
 return kept;
}''',1),
('...stop===undefined?{}:{stop},sources:body};','...stop===undefined?{}:{stop},collections:collectSliceCollectionsV1(policy),sources:body};',1),
])
edit(SR+"🟨️.d.mts",[
('export function registerSliceReportSourceV1(name:string,read:()=>unknown):()=>void;',
 '''export function registerSliceReportSourceV1(name:string,read:()=>unknown):()=>void;
export type SliceCollectionV1={name:string;entries:number;sample?:()=>unknown[];bytes?:number};
export function describeCollectionV1(name:string,collection:unknown):SliceCollectionV1;
export function registerSliceCollectionsV1(name:string,read:()=>SliceCollectionV1[]):()=>void;
export function estimateCollectionBytesV1(entries:number,sample:unknown[]):number;
export function collectSliceCollectionsV1(policy?:unknown):{name:string;entries:number;estimatedBytes:number}[];''',1),
('type SliceReportPolicyV1=Readonly<{fileBytes:number;topTypes:number}>;','type SliceReportPolicyV1=Readonly<{fileBytes:number;topTypes:number;collectionsTop:number;collectionsBytes:number;collectionSample:number}>;',1),
])
edit(SR+"🧬️schema/🔣️.json",[
('"fileBytes": 65536,','"fileBytes": 98304,\n        "collectionsTop": 20,\n        "collectionsBytes": 32768,\n        "collectionSample": 16,',1),
('"sources": { "type": "object" },','"collections": { "type": "array", "items": { "type": "object", "additionalProperties": false, "required": ["name", "entries", "estimatedBytes"], "properties": { "name": { "type": "string" }, "entries": { "type": "integer", "minimum": 0 }, "estimatedBytes": { "type": "integer", "minimum": 0 } } } },\n        "sources": { "type": "object" },',1),
])
edit(SR+"🧪️tests/🟦️.ts",[
('/** 📋️ The slice memory report:','''import{collectSliceCollectionsV1,describeCollectionV1,estimateCollectionBytesV1,registerSliceCollectionsV1}from"../🟨️.mjs";
/** 📋️ The slice memory report:''',1),
],)
# append tests
p=S+"/3.new";t=open(p,encoding="utf8").read()
t+='''
test("collections are estimated by entries times the mean size of 16 sampled entries, sorted by bytes, at most 20 and bounded to 32 KiB, and a broken source does not stop the report",()=>{
 expect(estimateCollectionBytesV1(1000,["abcd","abcd"])).toBe(8000);
 expect(estimateCollectionBytesV1(10,[{a:1}])).toBe(70);
 expect(estimateCollectionBytesV1(5,[])).toBe(0);
 const big=new Map<number,string>(),small=new Map<number,string>();for(let at=0;at<3000;at++)big.set(at,"x".repeat(500));for(let at=0;at<10;at++)small.set(at,"y");
 registerSliceCollectionsV1("test-many",()=>[describeCollectionV1("big",big),describeCollectionV1("small",small),{name:"declared",entries:1,bytes:5}]);
 registerSliceCollectionsV1("test-broken",()=>{throw new Error("nope");});
 registerSliceCollectionsV1("test-flood",()=>Array.from({length:400},(_all,at)=>({name:"flood-"+at+"-".repeat(70),entries:at,bytes:at})));
 const rows=collectSliceCollectionsV1();
 expect(rows.length).toBeLessThanOrEqual(P.collectionsTop);
 expect(JSON.stringify(rows).length).toBeLessThanOrEqual(P.collectionsBytes);
 expect(rows[0]).toEqual({name:"big",entries:3000,estimatedBytes:3000*(JSON.stringify([0,"x".repeat(500)]).length)});
 for(let at=1;at<rows.length;at++)expect(rows[at-1]!.estimatedBytes).toBeGreaterThanOrEqual(rows[at]!.estimatedBytes);
 const report=buildSliceReportV1({reason:"interval"});
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},report,schema)).toEqual([]);
 expect((report.collections as {name:string}[])[0]!.name).toBe("big");
});
'''
open(p,"w",encoding="utf8").write(t)

OBS=B+"👁️observation/🟨️.mjs"
edit(OBS,[
('import { createRowPoolV1, createRowTableV1, factKeyV1 } from "./🔢️row-table/🟨️.mjs";','import { createRowPoolV1, createRowTableV1, factKeyV1 } from "./🔢️row-table/🟨️.mjs";\nimport { describeCollectionV1 } from "../🧠️memory/📋️slice-report/🟨️.mjs";',1),
('const projectChecks = new WeakMap();','''const projectChecks = new WeakMap();
const trackedV1={projectChecks:new Set(),sharedChecks:new Set(),scopes:new Set()},trackedSeenV1=new WeakSet();
const trackV1=(kind,map)=>{if(map!==null&&typeof map==="object"&&!trackedSeenV1.has(map)){trackedSeenV1.add(map);trackedV1[kind].add(new WeakRef(map));}};''',1),
('checks=new Map();projectChecks.set(parent,checks);','checks=new Map();projectChecks.set(parent,checks);trackV1("projectChecks",checks);',1),
('checks=new Map();sharedChecks.set(parent,checks);','checks=new Map();sharedChecks.set(parent,checks);trackV1("sharedChecks",checks);',2),
('function retainObservationsV1(target,rows){\n if(!target)return;','function retainObservationsV1(target,rows){\n if(!target)return;trackV1("scopes",target);',1),
('export function inferenceCacheSizesV1(){','''/** 🧮️ The collections this module holds across owners, for the slice memory report: its module maps, the verdict maps of the live validation rounds and the scopes the rows of every owner are retained into, each with its entries and a sample to estimate bytes from. */
export function observationCollectionsV1(){
 const out=[],add=(name,value)=>out.push(describeCollectionV1("observation:"+name,value));
 add("inferenceStamps",inferenceStamps);add("recordStores",recordStores);add("recordStoreHandles",recordStoreHandles);add("usedInference",usedInference);add("originalInferencePublications",originalInferencePublications);add("loadObservations",loadObservations);add("projectComputations",projectComputations);add("projectImages",projectImages);add("sourceAnalyses",sourceAnalyses);add("sourceAnalysisImages",sourceAnalysisImages);add("retainedProjectImages",retainedProjectImages);add("fieldDigests",fieldDigests);add("settledReads",settledReads);add("fileRecordImages",fileRecordImages);add("inferenceRowCounts",inferenceRowCountsV1);add("verifiedBlocks",verifiedBlocks);
 const table=rowTableV1.stats();out.push({name:"observation:rowTable",entries:table.rows,bytes:table.bytes});out.push({name:"observation:rowPool",entries:rowPoolV1.size(),bytes:rowPoolV1.size()*160});
 for(const kind of Object.keys(trackedV1)){
  const alive=[];for(const ref of trackedV1[kind]){const value=ref.deref();if(value===undefined)trackedV1[kind].delete(ref);else alive.push(value);}
  if(alive.length===0)continue;const largest=alive.reduce((best,value)=>(value.size??0)>(best.size??0)?value:best,alive[0]),row=describeCollectionV1("observation:"+kind+" (largest of "+alive.length+")",largest);
  out.push({...row,entries:alive.reduce((sum,value)=>sum+(value.size??0),0)});
 }
 return out;
}
export function inferenceCacheSizesV1(){''',1),
])

SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
edit(SI,[
('const layerVerdictsV1 = new WeakMap(), layerFlightsV1 = new WeakMap(),','''const verdictMapsV1 = new Set(), trackVerdictsV1 = round => { const map = new Map(); layerVerdictsV1.set(round, map); verdictMapsV1.add(new WeakRef(map)); return map; };
/** 🧮️ The layer verdict maps of the live validation rounds, for the slice memory report. */
export function closureCollectionsV1() { let entries = 0, alive = 0, largest; for (const ref of verdictMapsV1) { const map = ref.deref(); if (map === undefined) { verdictMapsV1.delete(ref); continue; } alive++; entries += map.size; if (largest === undefined || map.size > largest.size) largest = map; } return alive === 0 ? [] : [{ name: "closure:layerVerdicts (" + alive + " rounds)", entries, bytes: entries * 96 }]; }
const layerVerdictsV1 = new WeakMap(), layerFlightsV1 = new WeakMap(),''',1),
('layerVerdictsV1.get(round) ?? layerVerdictsV1.set(round, new Map()).get(round)','layerVerdictsV1.get(round) ?? trackVerdictsV1(round)',2),
])
edit(B+"👁️observation/🟨️.d.mts",[('export function inferenceCacheSizesV1()','export function observationCollectionsV1():{name:string;entries:number;sample?:()=>unknown[];bytes?:number}[];\nexport function inferenceCacheSizesV1()',1)])
edit(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts",[('export function closureEntryCountsV1()','export function closureCollectionsV1(): { name: string; entries: number; bytes: number }[];\nexport function closureEntryCountsV1()',1)])
PN=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
edit(PN,[
('import { registerSliceReportSourceV1 } from','import { describeCollectionV1, registerSliceCollectionsV1, registerSliceReportSourceV1 } from',1),
('  const reportOff=registerSliceReportSourceV1("owners",','  const collectionsOff=registerSliceCollectionsV1("owners",()=>{const all=results.flatMap(([,row])=>Object.values(row.projects));return[{name:"owners:results.projects",entries:all.length,sample:()=>all.slice(0,16)},describeCollectionV1("owners:jobs",jobs),describeCollectionV1("owners:taken",taken),describeCollectionV1("owners:inflight",inflight),describeCollectionV1("owners:missed",missed),describeCollectionV1("owners:rowCounts",rowCounts),describeCollectionV1("owners:never",never),describeCollectionV1("owners:alone",alone),describeCollectionV1("owners:stages",stagesV1)];});\n  const reportOff=registerSliceReportSourceV1("owners",',1),
('finally{reportOff();','finally{reportOff();collectionsOff();',1),
])
DI=B+"🕸️graph/🧭️discovery/🟦️.ts"
edit(DI,[
('import * as observationV1 from "../../👁️observation/🟨️.mjs";','import * as observationV1 from "../../👁️observation/🟨️.mjs";\nimport {describeCollectionV1,registerSliceCollectionsV1} from "../../🧠️memory/📋️slice-report/🟨️.mjs";',1),
('  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}','  const partRefs={nodeRows:new WeakRef(nodeRows),derivedFacts:new WeakRef(derivedFacts)};registerSliceCollectionsV1("part",()=>{const rows=partRefs.nodeRows.deref(),facts=partRefs.derivedFacts.deref();return[...rows?[describeCollectionV1("part:nodeRows",rows)]:[],...facts?[describeCollectionV1("part:derivedFacts",facts)]:[]];});\n  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}',1),
])
WK=B+"🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"
edit(WK,[
('const report=await import("../../../../🧠️memory/📋️slice-report/🟨️.mjs"),{inferenceCacheSizesV1}=await import("../../../../👁️observation/🟨️.mjs");','const report=await import("../../../../🧠️memory/📋️slice-report/🟨️.mjs"),{inferenceCacheSizesV1,observationCollectionsV1}=await import("../../../../👁️observation/🟨️.mjs");\nconst{boundedCacheStatsV1}=await import("../../../../🧊️bounded/🟨️.mjs"),{graphStringInternerV1}=await import("../../../../🧠️memory/🟨️.mjs");\nreport.registerSliceCollectionsV1("observation",()=>observationCollectionsV1());\nreport.registerSliceCollectionsV1("bounded",()=>boundedCacheStatsV1().map((row:{name:string;entries:number;bytes:number})=>({name:"bounded:"+row.name,entries:row.entries,bytes:row.bytes})));\nreport.registerSliceCollectionsV1("interner",()=>{const row=graphStringInternerV1.stats();return[{name:"memory:graphStringInterner",entries:row.entries,bytes:row.characters*2+row.entries*48}];});',1),
('const{layerVerdictCountsV1}=await import("../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");','const{layerVerdictCountsV1,closureCollectionsV1}=await import("../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");\nreport.registerSliceCollectionsV1("closure",()=>closureCollectionsV1());',1),
])
open(S+"/index.txt","w",encoding="utf8").write("\n".join(" ".join(r) for r in rows)+"\n")
print(rows)
