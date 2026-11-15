#!/usr/bin/env python3
"""P12 edit set: anchored replacements on live files (each anchor must occur exactly the stated number of times) and new files copied from P12/new. Used for the scratch overlay and, unchanged, by land.py."""
import os
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=LIB+"/🕸️graph"
RES=LIB+"/🏡️resident"
MEM=LIB+"/🧠️memory"
STAGE=MEM+"/\U0001FA9C️"+"plan-stage"
EDITS=[]
def rep(path,old,new,count=1):EDITS.append((path,old,new,count))

DISC=G+"/🧭️discovery/🟦️.ts"
rep(DISC,'''import {taggedOwnersRetainedV1,TEST_TAG_V1,POLICY_TAG_V1} from "../🏷️retention/🟦️.ts";''','''import {taggedOwnersRetainedV1,TEST_TAG_V1,POLICY_TAG_V1} from "../🏷️retention/🟦️.ts";
import {createPlanStageGuardV1} from "../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import {refuseBuildV1} from "../../📖️texts/🟦️.ts";''')
rep(DISC,'''  let inferred:number|undefined;const phases:Record<string,number>={};''','''  let inferred:number|undefined;const phases:Record<string,number>={},stages=createPlanStageGuardV1({refuse:detail=>refuseBuildV1("unserved","graph.plan-too-heavy",detail)});''')
rep(DISC,'''  const captured=await withClosurePassV1(()=>observationV1.captureObservedSourceInputsV1(async()=>{
    const options={analyzeLockfile:true,''','''  const captured=await stages.run(()=>withClosurePassV1(()=>observationV1.captureObservedSourceInputsV1(async()=>{
    stages.mark("inference");
    const options={analyzeLockfile:true,''')
rep(DISC,'''    const env=await deriveEnvironmentV1(''','''    stages.mark("environment");
    const env=await deriveEnvironmentV1(''')
rep(DISC,'''    const tagged=stubs.filter(([,stub])=>stub.tag),retained=''','''    stages.mark("tests");
    const tagged=stubs.filter(([,stub])=>stub.tag),retained=''')
rep(DISC,'''    const policyOwners=Object.assign({},...[...primary,...testContributions]''','''    stages.mark("policies");
    const policyOwners=Object.assign({},...[...primary,...testContributions]''')
rep(DISC,'''    if(!retained)await discoveredForTests(()=>withholdDanglingTestProjectsV1(''','''    stages.mark("dangling");
    if(!retained)await discoveredForTests(()=>withholdDanglingTestProjectsV1(''')
rep(DISC,'''    const ownerOf=createProjectFileIndexV1(contextProjects),withheld=''','''    stages.mark("files");
    const ownerOf=createProjectFileIndexV1(contextProjects),withheld=''')
rep(DISC,'''    const testEdges:unknown=retained?[]:await discoveredForTests(''','''    stages.mark("test-edges");
    const testEdges:unknown=retained?[]:await discoveredForTests(''')
rep(DISC,'''    const implementation=streamed.size?await owner.loadCurrentImplementationV1():undefined;''','''    stages.mark("edges");
    const implementation=streamed.size?await owner.loadCurrentImplementationV1():undefined;''')
rep(DISC,'''    const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(contextProjects)''','''    stages.mark("metadata");
    const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(contextProjects)''')
rep(DISC,'''    const rows=[...unavailable.values()],testRowLists=''','''    stages.mark("result");
    const rows=[...unavailable.values()],testRowLists=''')
rep(DISC,'''  },undefined,false)).catch(error=>{rowLog.close();throw error;});''','''  },undefined,false))).catch(error=>{rowLog.close();throw error;});''')

POOL=RES+"/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
rep(POOL,'''reaper=createReaperV1(graceMilliseconds===undefined?{}:{graceMilliseconds})}){''','''reaper=createReaperV1(graceMilliseconds===undefined?{}:{graceMilliseconds}),planLimitMegabytes=hostMegabytes>0&&hostMegabytes<GRAPH_MEMORY_POLICY_V1.serveHostMegabytes&&process.env.SEMIO_GRAPH_PLAN_LIMIT_MB!=="off"?sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}):Infinity}){''')
rep(POOL,'''void observe(worker.pid).then(megabytes=>{if(megabytes>(entry.peakMegabytes??0))entry.peakMegabytes=megabytes;},()=>undefined);''','''void observe(worker.pid).then(megabytes=>{if(megabytes>(entry.peakMegabytes??0))entry.peakMegabytes=megabytes;if(megabytes>planLimitMegabytes){counters.overBudget++;finish(reject,Object.assign(new Error("Slice load grew past the memory a plan may take"),{code:"plan-memory",stage:entry.stage??"?",currentMegabytes:Math.round(megabytes),limitMegabytes:Math.round(planLimitMegabytes),sample:entry.sample}));}},()=>undefined);''')
rep(POOL,'''if(message?.type==="memory"){entry.sample=''','''if(message?.type==="stage"){entry.stage=String(message.stage);return;}
   if(message?.type==="memory"){entry.sample=''')

SERVICE=RES+"/🏛️service/🌐️host/🟨️.mjs"
rep(SERVICE,'''else if(error?.code==="memory-budget"){planned.budgetRefusals++;await refuse(connection,frame.id,"memory","host-memory-budget",{current:error.currentMegabytes,limit:error.limitMegabytes,margin:error.marginMegabytes},"later");}''','''else if(error?.code==="memory-budget"){planned.budgetRefusals++;await refuse(connection,frame.id,"memory","host-memory-budget",{current:error.currentMegabytes,limit:error.limitMegabytes,margin:error.marginMegabytes},"later");}
   else if(error?.code==="plan-memory"){planned.budgetRefusals++;log("graph-plan-too-heavy",JSON.stringify({stage:error.stage,megabytes:error.currentMegabytes,limit:error.limitMegabytes,worker:error.sample}));await refuse(connection,frame.id,"unserved","host-graph-plan-too-heavy",{stage:error.stage,megabytes:error.currentMegabytes,limit:error.limitMegabytes},"later");}''')

FAIL=RES+"/🏛️service/🧯️failure/🟨️.mjs"
rep(FAIL,'''/** 🛑️ The words of a graph read that stopped projects too heavy for this machine''','''/** 🪜️ The words of a graph read whose plan (the assembly of a wave) grew past what this machine gives a plan (`build.graph.plan-too-heavy`): the refusal\\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
export function planRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.plan-too-heavy"||/graph\\.plan-too-heavy/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\\.plan-too-heavy (\\{.*\\})\\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return{stage:String(values.stage??"?"),megabytes:String(values.megabytes??0),limit:String(values.limit??0)};
 }
 return planRefusalOfV1(error.cause,depth-1);
}

/** 🛑️ The words of a graph read that stopped projects too heavy for this machine''')
rep(FAIL,''' const heavy=heavyRefusalOfV1(error);
''',''' const plan=planRefusalOfV1(error);
 if(plan!==undefined){log("graph-plan-too-heavy",JSON.stringify(plan));return{code:"unserved",key:"host-graph-plan-too-heavy",fields:plan,retry:"later"};}
 const heavy=heavyRefusalOfV1(error);
''')

SCHEMA=RES+"/🧬️schema/🔣️.json"
rep(SCHEMA,'''      "host-graph-owner-too-heavy": "The project graph could not be read on this machine yet:''','''      "host-graph-plan-too-heavy": "The project graph could not be read on this machine yet: the plan stage {stage} grew to {megabytes} MB, more than the {limit} MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.",
      "host-graph-owner-too-heavy": "The project graph could not be read on this machine yet:''')
rep(SCHEMA,'''      "host-graph-owner-too-heavy": "Der Projektgraph konnte auf diesem Rechner noch nicht gelesen werden:''','''      "host-graph-plan-too-heavy": "Der Projektgraph konnte auf diesem Rechner noch nicht gelesen werden: die Plan-Stufe {stage} ist auf {megabytes} MB gewachsen, mehr als die {limit} MB, die diese Maschine einem Plan gibt. Sie wurde gestoppt, bevor dem Rechner der Speicher ausging. An deinem Baum ist nichts falsch; führe den Befehl erneut aus, sobald ein Rechner mit freiem Speicher den Graphen bereitstellt, oder lies den Graphen auf einem größeren Rechner.",
      "host-graph-owner-too-heavy": "Der Projektgraph konnte auf diesem Rechner noch nicht gelesen werden:''')

TEXTS=LIB+"/📖️texts/🔣️.json"
rep(TEXTS,'''    "graph.owner-too-heavy": {''','''    "graph.plan-too-heavy": {
      "en": "The project graph cannot be read on this machine yet: the plan stage {stage} grew to {megabytes} MB, more than the {limit} MB this machine gives a plan. It was stopped before the host ran short of memory; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine",
      "de": "Der Projektgraph kann auf diesem Rechner noch nicht gelesen werden: die Plan-Stufe {stage} ist auf {megabytes} MB gewachsen, mehr als die {limit} MB, die diese Maschine einem Plan gibt. Sie wurde gestoppt, bevor dem Rechner der Speicher ausging; führe den Befehl erneut aus, sobald ein Rechner mit freiem Speicher den Graphen bereitstellt, oder lies den Graphen auf einem größeren Rechner"
    },
    "graph.owner-too-heavy": {''')

WORKER=RES+"/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"
rep(WORKER,'''onGenerationProgressV1((event:{phase:string;done:number;total:number;running?:unknown;decision?:unknown})=>parentPort!.postMessage({type:"progress",...event}));''','''onGenerationProgressV1((event:{phase:string;done:number;total:number;running?:unknown;decision?:unknown})=>parentPort!.postMessage({type:"progress",...event}));
const{onPlanStageV1}=await import("../../../../🧠️memory/🪜️plan-stage/🟨️.mjs");
onPlanStageV1(event=>parentPort!.postMessage(event));''')

NEWFILES=[
 (STAGE+"/🟨️.mjs","plan-stage/module.mjs"),
 (STAGE+"/🟨️.d.mts","plan-stage/module.d.mts"),
 (STAGE+"/🧬️schema/🔣️.json","plan-stage/schema/schema.json"),
 (STAGE+"/🧪️tests/🟦️.ts","plan-stage/tests/main.ts"),
 (RES+"/🧪️tests/\U0001FA93️"+"plan-limit/🟦️.ts","pool-plan-limit.ts"),
 (RES+"/🧪️tests/\U0001FA9C️"+"plan-refusal/🟦️.ts","plan-refusal.ts"),
 (G+"/🧪️tests/\U0001FA9C️"+"plan-stages/🟦️.ts","plan-stages.ts"),
]
