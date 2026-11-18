const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
export const PATCHES=[
 {file:G+"🟨️.mjs",edits:[
  ['stamp=P.stampVerification,workspaceRoot}){','stamp=P.stampVerification,workspaceRoot,say}){'],
  ['await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");',
   'const noisy=returned.size>P.ownerWave;if(noisy)say?.("storing",{count:returned.size});\n  let fragmentsDone=0;if(stored.length)await progress?.({phase:"fragments",done:0,total:stored.length});\n  await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,async job=>{await job();if(stored.length)await progress?.({phase:"fragments",done:++fragmentsDone,total:stored.length});},signal));mark("fragments");'],
  ['    let index=await profilePhaseV1("graph.build.patch"','    if(noisy)say?.("indexing",{count:Object.keys(owners).length});\n    let index=await profilePhaseV1("graph.build.patch"'],
  ['  const packedIndex=await profilePhaseV1("graph.build.pack"','  if(noisy)say?.("sealing",{generation:(current?.pointer.generation??-1)+1});\n  const packedIndex=await profilePhaseV1("graph.build.pack"'],
 ]},
 {file:G+"🟨️.d.mts",edits:[
  ['export interface GenerationStoreOptionsV1{workspaceRoot?:string;','export interface GenerationStoreOptionsV1{workspaceRoot?:string;say?:(key:"storing"|"indexing"|"sealing",fields:Readonly<Record<string,string|number>>)=>void;'],
 ]},
 {file:G+"🔌️load/🟨️.mjs",edits:[
  ['  exchangeFailed:"graph: the remote cache did not exchange the generation ({reason}), going on without it",','  exchangeFailed:"graph: the remote cache did not exchange the generation ({reason}), going on without it",\n  storing:"graph: all project results are in, storing {count} project fragments",\n  indexing:"graph: fragments stored, merging the facts of {count} projects into the fact index",\n  sealing:"graph: fact index merged, packing and publishing generation {generation}",'],
  ['  exchangeFailed:"Graph: der Remote-Cache hat die Generation nicht ausgetauscht ({reason}), es geht ohne sie weiter",','  exchangeFailed:"Graph: der Remote-Cache hat die Generation nicht ausgetauscht ({reason}), es geht ohne sie weiter",\n  storing:"Graph: alle Projektergebnisse liegen vor, {count} Projektfragmente werden gespeichert",\n  indexing:"Graph: Fragmente gespeichert, die Fakten von {count} Projekten werden in den Faktenindex eingefügt",\n  sealing:"Graph: Faktenindex zusammengeführt, Generation {generation} wird verpackt und veröffentlicht",'],
  ['createGenerationStoreV1({...options,answers,workspaceRoot:host.workspaceRoot})','createGenerationStoreV1({...options,answers,workspaceRoot:host.workspaceRoot,say:(key,fields)=>host.say(key,fields)})'],
 ]},
 {file:G+"🔌️load/🟨️.d.mts",edits:[
  ['|"exchangeFailed",string','|"exchangeFailed"|"storing"|"indexing"|"sealing",string'],
 ]},
 {file:G+"🧬️schema/🔣️.json",edits:[
  ['          "exchangeFailed": "graph: the remote cache did not exchange the generation ({reason}), going on without it"','          "exchangeFailed": "graph: the remote cache did not exchange the generation ({reason}), going on without it",\n          "storing": "graph: all project results are in, storing {count} project fragments",\n          "indexing": "graph: fragments stored, merging the facts of {count} projects into the fact index",\n          "sealing": "graph: fact index merged, packing and publishing generation {generation}"'],
  ['          "exchangeFailed": "Graph: der Remote-Cache hat die Generation nicht ausgetauscht ({reason}), es geht ohne sie weiter"','          "exchangeFailed": "Graph: der Remote-Cache hat die Generation nicht ausgetauscht ({reason}), es geht ohne sie weiter",\n          "storing": "Graph: alle Projektergebnisse liegen vor, {count} Projektfragmente werden gespeichert",\n          "indexing": "Graph: Fragmente gespeichert, die Fakten von {count} Projekten werden in den Faktenindex eingefügt",\n          "sealing": "Graph: Faktenindex zusammengeführt, Generation {generation} wird verpackt und veröffentlicht"'],
 ]},
];
