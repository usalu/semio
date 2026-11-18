import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
const here=dirname(fileURLToPath(import.meta.url));
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=B+"🕸️graph/🕰️generations/";
export const PATCHES=[
 {step:4,file:G+"🟨️.mjs",edits:[
  ["import schema from\"./🧬️schema/🔣️.json\"with{type:\"json\"};","import schema from\"./🧬️schema/🔣️.json\"with{type:\"json\"};\nimport{BUILD_JOURNAL_V1,buildJournalKeyV1,createBuildJournalV1}from\"./🔖️journal/🟨️.mjs\";\nimport{readGenerationPointerV1,readGenerationManifestV1,readGenerationFragmentV1}from\"./📖️documents/🟨️.mjs\";\nimport{createNodeFilePortV1,createFileLeasePortV1,defaultSleep}from\"./🗃️file-ports/🟨️.mjs\";\nexport{readGenerationPointerV1,readGenerationManifestV1,readGenerationFragmentV1,createNodeFilePortV1,createFileLeasePortV1};"],
  ["workspaceRoot,say}){","workspaceRoot,say,stepped}){"],
  ["progress,builder,requestedAt,published,planned,unchanged}){","progress,builder,requestedAt,published,wave,planned,unchanged}){"],
  ["progress,published,hint,only:scope","progress,published,wave,hint,only:scope"],
  ["  for(let attempt=0;attempt<8;attempt++){\n   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());\n   if(result.published||!result.built)return{...result,served:\"built\",...result.needsBuild&&carried?{planned:carried}:{}};\n","  let resumedComputed;\n  for(let attempt=0;attempt<8;attempt++){\n   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());\n   if(result.resumed){resumedComputed=(resumedComputed??0)+(result.computed??0);continue;}\n   if(result.published||!result.built)return{...result,...resumedComputed===undefined?{}:{published:true,computed:(result.computed??0)+resumedComputed},served:\"built\",...result.needsBuild&&carried?{planned:carried}:{}};\n"],
  ["await sweep(P.frontiers,frontiers,\".json.br\");","await sweep(P.frontiers,frontiers,\".json.br\");await sweep(BUILD_JOURNAL_V1.directory,new Set(),\"\");"],
  {between:["/** 📌️ Admits the closed pointer to the newest generation","function readFactIndexV1(value){"],to:""},
  {between:["/** 🗂️ The filesystem port on real files","async function mapLimit"],to:""},
  {from:"/** 🔒️ A reference exclusive lease over lock files",toEnd:true,to:""},
 {between:[" /** 🏗️ Builds and publishes the next generation"," /** 🧷️ Publishes the same owners under a refreshed"],to:readFileSync(join(here,"build-new.mjs"),"utf8")},
 ]},
 {step:4,file:G+"🟨️.d.mts",edits:[
  ["say?:(key:\"storing\"|\"indexing\"|\"sealing\",fields:Readonly<Record<string,string|number>>)=>void;","say?:(key:\"storing\"|\"indexing\"|\"sealing\"|\"resumed\"|\"resumedWaves\",fields:Readonly<Record<string,string|number>>)=>void;stepped?:(step:\"storing\"|\"indexing\"|\"sealing\")=>void|Promise<void>;"],
  ["published?:(generation:number,info?:{done:number;total:number})=>unknown;","published?:(generation:number,info?:{done:number;total:number})=>unknown;wave?:(info:{generation:number;done:number;total:number})=>unknown;"],
 ]},
 {step:4,file:G+"🔌️load/🟨️.mjs",edits:[
  ["  indexing:\"graph: fragments stored, merging the facts of {count} projects into the fact index\",","  indexing:\"graph: fragments stored, merging the facts of {count} projects into the fact index\",\n  resumed:\"graph: generation {generation} ({owners} projects) was built by a command that was cut off and is published now; the tree is checked against it next\",\n  resumedWaves:\"graph: {done} of {total} projects were derived by a command that was cut off, deriving the rest\","],
  ["  indexing:\"Graph: Fragmente gespeichert, die Fakten von {count} Projekten werden in den Faktenindex eingefügt\",","  indexing:\"Graph: Fragmente gespeichert, die Fakten von {count} Projekten werden in den Faktenindex eingefügt\",\n  resumed:\"Graph: Generation {generation} ({owners} Projekte) wurde von einem abgebrochenen Befehl aufgebaut und wird jetzt veröffentlicht; danach wird der Arbeitsbaum dagegen geprüft\",\n  resumedWaves:\"Graph: {done} von {total} Projekten wurden von einem abgebrochenen Befehl abgeleitet, der Rest wird abgeleitet\","],
  ["published:builder&&host.published?(generation,info)=>host.published(generation,info):undefined,","published:builder&&host.published?(generation,info)=>host.published(generation,info):undefined,wave:builder&&host.waveDone?info=>host.waveDone(info):undefined,"],
 ]},
 {step:4,file:G+"🔌️load/🟨️.d.mts",edits:[
  ["parseUnavailable?:unknown[];rows:GenerationRowV1[];env:{digest:string;rows:GenerationRowV1[];","parseUnavailable?:unknown[];rows:GenerationRowSourceV1;env:{digest:string;rows:GenerationRowV1[];"],
  ["|\"exchangeFailed\"|\"storing\"|\"indexing\"|\"sealing\",string","|\"exchangeFailed\"|\"storing\"|\"indexing\"|\"sealing\"|\"resumed\"|\"resumedWaves\",string"],
  [" published?(generation:number,info?:{done:number;total:number}):void|Promise<void>;"," published?(generation:number,info?:{done:number;total:number}):void|Promise<void>;\n /** 🌊️ Told after every wave that was derived but not published (a read publishes one generation, at its end), so the progress row and the memory of the inference are settled between the waves; optional. */\n waveDone?(info:{generation:number;done:number;total:number}):void|Promise<void>;"],
 ]},
 {step:4,file:G+"🧬️schema/🔣️.json",edits:[
  ["          \"sealing\": \"graph: fact index merged, packing and publishing generation {generation}\"","          \"sealing\": \"graph: fact index merged, packing and publishing generation {generation}\",\n          \"resumed\": \"graph: generation {generation} ({owners} projects) was built by a command that was cut off and is published now; the tree is checked against it next\",\n          \"resumedWaves\": \"graph: {done} of {total} projects were derived by a command that was cut off, deriving the rest\""],
  ["          \"sealing\": \"Graph: Faktenindex zusammengeführt, Generation {generation} wird verpackt und veröffentlicht\"","          \"sealing\": \"Graph: Faktenindex zusammengeführt, Generation {generation} wird verpackt und veröffentlicht\",\n          \"resumed\": \"Graph: Generation {generation} ({owners} Projekte) wurde von einem abgebrochenen Befehl aufgebaut und wird jetzt veröffentlicht; danach wird der Arbeitsbaum dagegen geprüft\",\n          \"resumedWaves\": \"Graph: {done} von {total} Projekten wurden von einem abgebrochenen Befehl abgeleitet, der Rest wird abgeleitet\""],
 ]},
 {step:4,file:B+"🕸️graph/🕰️host/🟦️.ts",edits:[
    ["    published:async(generation:number,info?:{done:number;total:number})=>{seams.published(generation);","    waveDone:async(info:{generation:number;done:number;total:number}):Promise<void>=>{const generation=info.generation;await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:()=>{observationV1.releaseInferenceMemoryV1();graphStringInternerV1.clear();}});if(process.env.SEMIO_GRAPH_BACKGROUND===\"1\")carriedStagesV1.yield=await(await import(\"../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts\")).yieldBackgroundGraphV1({generation,signal:invocation.control.signal,language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language as \"en\"|\"de\"});},\n    published:async(generation:number,info?:{done:number;total:number})=>{seams.published(generation);"],
 ]},
 {step:4,file:G+"🧪️tests/🏃️world/🟦️.ts",edits:[
  ["published?:(generation:number)=>unknown})=>ReturnType<GenerationStoreV1[\"refresh\"]>;","published?:(generation:number)=>unknown;wave?:(info:{generation:number;done:number;total:number})=>unknown})=>ReturnType<GenerationStoreV1[\"refresh\"]>;"],
  ["ownerDelayMs?:number}={}):World{","ownerDelayMs?:number;stepped?:(step:string)=>unknown}={}):World{"],
  ["...(options.now?{now:options.now}:{})});","...(options.now?{now:options.now}:{}),...(options.stepped?{stepped:options.stepped as never}:{})});"],
  ["...(opts.published?{published:opts.published}:{}),","...(opts.published?{published:opts.published}:{}),...(opts.wave?{wave:opts.wave as never}:{}),"],
 ]},
 {step:4,file:G+"🧪️tests/🌊️waves/🟦️.ts",edits:[
  ["an uninterrupted refresh publishes one generation per wave and equals the oracle","an uninterrupted refresh publishes one generation however many waves it derives and equals the oracle"],
  ["expect((await world.store.read())!.pointer.generation-before).toBe(3);","expect((await world.store.read())!.pointer.generation-before).toBe(1);"],
  ["the build tells the host the generation of every wave it published, in order, and a host that throws does not fail the build","the build tells the host the generation it published once, the progress of every wave before it separately, and a host that throws does not fail the build"],
  ["const result=await world.refresh({published:generation=>{told.push(generation);if(told.length===2)throw new Error(\"host failed\");}});","const result=await world.refresh({published:generation=>{told.push(generation);throw new Error(\"host failed\");},wave:()=>{throw new Error(\"host failed\");}});"],
  ["expect(told).toEqual([before+1,before+2,before+3]);\n  expect((await world.store.read())!.pointer.generation).toBe(before+3);","expect(told).toEqual([before+1]);\n  expect((await world.store.read())!.pointer.generation).toBe(before+1);"],
  ["  const kept=await world.store.read();\n  expect(kept!.pointer.generation-before).toBe(1);\n  expect(kept!.manifest!.built.dirty.length).toBe(GENERATION_POLICY_V1.ownerWave);\n  const finished=new Set(kept!.manifest!.built.dirty);\n","  expect((await world.store.read())!.pointer.generation).toBe(before);\n  const finished=new Set(world.log.flatMap(entry=>entry.full).slice(0,GENERATION_POLICY_V1.ownerWave));\n"],
  ["   await edit(dir,COUNT);\n   await expect(world.refresh({signal:controller.signal})).rejects.toThrow();\n   const kept=await world.store.read();\n   expect(kept!.pointer.generation-before).toBe(1);\n   const finished=new Set(kept!.manifest!.built.dirty),log:Parameters<typeof createCompute>[1]=[];","   await edit(dir,COUNT);\n   world.log.length=0;\n   await expect(world.refresh({signal:controller.signal})).rejects.toThrow();\n   expect((await world.store.read())!.pointer.generation).toBe(before);\n   const finished=new Set(world.log.flatMap(entry=>entry.full).slice(0,GENERATION_POLICY_V1.ownerWave)),log:Parameters<typeof createCompute>[1]=[];"],
 ]},
 {step:4,file:G+"🧪️tests/📈️adaptive-waves/🟦️.ts",edits:[
  ["expect((await world.store.read())!.pointer.generation-before).toBe(3);","expect((await world.store.read())!.pointer.generation-before).toBe(1);"],
  ["  const told:{done:number;total:number;phases?:Record<string,number>}[]=[];\n  await world.refresh({published:(_generation:number,info:never)=>{told.push(info);}});\n  expect(told.length).toBe(3);\n  for(const[at,info]of told.entries()){\n   expect(Object.keys(info.phases!).sort()).toEqual([...(at===0?[\"plan.facts\",\"plan.index\",\"plan.settle\"]:[]),\"compute\",\"compute.projects\",\"compute.setup\",\"compute.tail\",\"fragments\",\"pack\",\"patch\",\"plan\",\"prime\",\"publish\"].sort());","  const told:{done:number;total:number;phases?:Record<string,number>}[]=[];\n  await world.refresh({published:(_generation:number,info:never)=>{told.push(info);},wave:info=>{told.push(info as never);}});\n  expect(told.length).toBe(3);\n  for(const[at,info]of told.entries()){\n   expect(Object.keys(info.phases!).sort()).toEqual([...(at===0?[\"plan.facts\",\"plan.index\",\"plan.settle\"]:[]),\"compute\",\"compute.projects\",\"compute.setup\",\"compute.tail\",\"fragments\",\"plan\",\"prime\",...(at===2?[\"pack\",\"patch\",\"publish\"]:[])].sort());"],
  ["every published wave carries the seconds of its stages, the stages of the part included","every wave carries the seconds of its stages, the stages of the part included, and the last one those of the publication"],
 ]},
 {step:4,file:G+"🧪️tests/📈️wave-progress/🟦️.ts",edits:[
  ["  const result=await world.refresh({published:(generation:number,info:unknown)=>{told.push([generation,info]);}});","  const result=await world.refresh({published:(generation:number,info:unknown)=>{told.push([generation,info]);},wave:(info:{generation:number})=>{told.push([info.generation,info]);}});"],
  ["[[before+1,{done:wave,total:COUNT}],[before+2,{done:wave*2,total:COUNT}],[before+3,{done:COUNT,total:COUNT}]]","[[before+1,{done:wave,total:COUNT}],[before+1,{done:wave*2,total:COUNT}],[before+1,{done:COUNT,total:COUNT}]]"],
 ]},
 {step:4,file:G+"🧪️tests/⏱️stages/🟦️.ts",edits:[
  ["  await world.refresh({published:(_generation:number,info:Info)=>{told[1]!.push(info);}});\n  expect(told[1]!.length).toBe(3);","  await world.refresh({published:(_generation:number,info:Info)=>{told[1]!.push(info);},wave:(info:unknown)=>{told[1]!.push(info as Info);}});\n  expect(told[1]!.length).toBe(3);"],
  ["  for(const wave of [first!,second!,third!])for(const name of [\"prime\",\"plan\",\"compute\",\"fragments\",\"patch\",\"pack\",\"publish\"])expect(Number.isSafeInteger(wave[name]),name).toBe(true);","  for(const wave of [first!,second!])for(const name of [\"prime\",\"plan\",\"compute\",\"fragments\"])expect(Number.isSafeInteger(wave[name]),name).toBe(true);\n  for(const name of [\"prime\",\"plan\",\"compute\",\"fragments\",\"patch\",\"pack\",\"publish\"])expect(Number.isSafeInteger(third![name]),name).toBe(true);"],
 ]},
 {step:4,file:B+"🕸️graph/🧭️discovery/🟦️.ts",edits:[
  ["path:join(rowLogDirectoryV1(root),\"rl-\"+process.pid+\"-\"+Date.now().toString(36)+\".rows\")","path:join(rowLogDirectoryV1(root),\"rl-\"+process.pid+\"-\"+Date.now().toString(36)+\"-\"+(rowLogSerialV1++)+\".rows\")"],
  ["/** 🛢️ Where the rows of one graph slice are spilled:","/** 🔢️ The number of row logs this process opened, so two waves of one read never share a file even within one millisecond. */\nlet rowLogSerialV1=0;\n\n/** 🛢️ Where the rows of one graph slice are spilled:"],
 ]},
 {step:4,file:G+"🔌️load/🧪️tests/🔁️carried-release/🟦️.ts",edits:[
  ["const host={...mock.host,release:async(options?:{more?:boolean})=>{releases.push(options);},published:async(_generation:number,info:Info)=>{infos.push(info);carriedStagesV1.yield=7;}};","const host={...mock.host,release:async(options?:{more?:boolean})=>{releases.push(options);},waveDone:async(info:Info)=>{infos.push(info);carriedStagesV1.yield=7;},published:async(_generation:number,info:Info)=>{infos.push(info);carriedStagesV1.yield=7;}};"],
 ]},
 {step:4,file:G+"🧪️tests/🟦️.ts",edits:[
  ["expect(world.log.length-waves).toBe(rounds);expect(next.current.manifest!.generation).toBe(rounds);","expect(world.log.length-waves).toBe(rounds);expect(next.current.manifest!.generation).toBe(1);"],
 ]},
];
export const NEWS=[
 {step:4,from:"new/documents.mjs",to:G+"📖️documents/🟨️.mjs"},
 {step:4,from:"new/documents.d.mts",to:G+"📖️documents/🟨️.d.mts"},
 {step:4,from:"new/file-ports.mjs",to:G+"🗃️file-ports/🟨️.mjs"},
 {step:4,from:"new/file-ports.d.mts",to:G+"🗃️file-ports/🟨️.d.mts"},
 {step:4,from:"new/journal.mjs",to:G+"🔖️journal/🟨️.mjs"},
 {step:4,from:"new/journal.d.mts",to:G+"🔖️journal/🟨️.d.mts"},
 {step:4,from:"new/journal.schema.json",to:G+"🔖️journal/🧬️schema/🔣️.json"},
 {step:4,from:"new/journal.test.ts",to:G+"🧪️tests/🔖️journal/🟦️.ts"},
];
export const NEWS_ROOT=here;
export function applyEdits(text,edits,file){
 for(const edit of edits){
  if(Array.isArray(edit)){const [from,to]=edit,count=text.split(from).length-1;if(count!==1)throw Error("anchor count "+count+" in "+file+": "+from.slice(0,70));text=text.replace(from,()=>to);}
  else if(edit.toEnd){const i=text.indexOf(edit.from);if(i<0||text.indexOf(edit.from,i+1)>=0)throw Error("end anchor in "+file);text=text.slice(0,i)+edit.to;}
  else{const [a,b]=edit.between,i=text.indexOf(a),j=text.indexOf(b,i);if(i<0||j<0||text.indexOf(a,i+1)>=0)throw Error("between anchors in "+file+": "+a.slice(0,60));text=text.slice(0,i)+edit.to+text.slice(j);}
 }
 return text;
}
