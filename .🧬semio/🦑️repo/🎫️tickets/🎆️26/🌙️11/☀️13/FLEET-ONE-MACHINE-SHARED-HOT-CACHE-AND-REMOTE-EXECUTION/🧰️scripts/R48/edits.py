"""R48 anchored hunks (file, old, new, count); `new` is what the live file holds after the landing, `old` what it held before. The landing script verifies `new`, rolls back by replacing `new` with `old`."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SP=L+"🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
PS=L+"🧠️memory/🪜️plan-stage/🟨️.mjs"
PN=L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
SC=L+"🏡️resident/🧬️schema/🔣️.json"
TG=L+"🏡️resident/🧪️tests/🛡️plan-guard/🟦️.ts"
TL=L+"🏡️resident/🧪️tests/🪓️plan-limit/🟦️.ts"
E=[
(SP,'import {GRAPH_MEMORY_POLICY_V1} from "../../../../🧠️memory/🟨️.mjs";','import {GRAPH_MEMORY_POLICY_V1} from "../../../../🧠️memory/🟨️.mjs";\nimport {GRAPH_CUT_POLICY_V1,graphCutAfterV1,graphCutEnvironmentV1,graphCutLearnedV1,graphCutStartLevelV1,graphCutV1} from "../../../../🧠️memory/✂️cut/🟨️.mjs";',1),
(SP,'planGuard=HOST_POLICY_V1.slicePlanGuard,','planGuard=HOST_POLICY_V1.slicePlanGuard,cutPolicy=GRAPH_CUT_POLICY_V1,',1),
(SP,'waveBasisPoints=10000)=>new Promise','waveBasisPoints=10000,cut=graphCutV1(0,cutPolicy),outcome={})=>new Promise',1),
(SP,'...graphChildEnvironmentV1(),...reservation','...graphChildEnvironmentV1(),...graphCutEnvironmentV1(cut,cutPolicy),...reservation',1),
(SP,'const finish=(done,value)=>{if(settled)return;settled=true;','const finish=(done,value)=>{if(settled)return;settled=true;outcome.peakMegabytes=entry.peakMegabytes??0;',1),
(SP,'counters.overBudget++;finish(reject,Object.assign(new Error("Slice load grew past the memory a plan may take"),{code:"plan-memory",reason:','counters.overBudget++;const cutTo=graphCutAfterV1({level:cut.level,treeMegabytes:megabytes,limitMegabytes:planLimitMegabytes},cutPolicy);if(cutTo!==undefined){finish(reject,Object.assign(new Error("Slice load is cut smaller"),{code:"plan-cut",cutTo,reason:verdict.reason,currentMegabytes:Math.round(megabytes),limitMegabytes:Math.round(planLimitMegabytes)}));return;}finish(reject,Object.assign(new Error("Slice load grew past the memory a plan may take"),{code:"plan-memory",cutLevel:cut.level,reason:',1),
(SP,'else if(message?.type==="error"){counters.failed++;finish(reject,errorOfMessageV1(message));}','else if(message?.type==="error"){const error=errorOfMessageV1(message),cutTo=error.key==="build.graph.plan-too-heavy"?graphCutAfterV1({level:cut.level,treeMegabytes:Number(error.values?.megabytes),limitMegabytes:Number(error.values?.limit)},cutPolicy):undefined;if(cutTo!==undefined){counters.overBudget++;finish(reject,Object.assign(new Error("Slice plan is cut smaller"),{code:"plan-cut",cutTo,reason:"stage",currentMegabytes:Number(error.values?.megabytes)||0,limitMegabytes:Number(error.values?.limit)||0}));}else{counters.failed++;finish(reject,error);}}',1),
(SP,'let active=0,sequence=0,recent=[];','let active=0,sequence=0,recent=[],learnedLevel;',1),
(SP,'    try{for(let restarts=0;;restarts++){\n','    let level=learnedLevel??Math.min(cutPolicy.levels,graphCutStartLevelV1({limitMegabytes:planLimitMegabytes},cutPolicy));\n    try{for(let restarts=0;;restarts++){\n',1),
(SP,'     return await runWorker(request,signal,onProgress,limit,owned,booked,waveBasisPoints);','     const cut=graphCutV1(level,cutPolicy),outcome={},answer=await runWorker(request,signal,onProgress,limit,owned,booked,Math.min(waveBasisPoints,cut.waveBasisPoints),cut,outcome);\n     learnedLevel=graphCutLearnedV1({level,peakMegabytes:outcome.peakMegabytes??0,limitMegabytes:planLimitMegabytes},cutPolicy);\n     return answer;',1),
(SP,'     reservation?.release();\n     if(error?.restart!==true||signal?.aborted)throw error;','     reservation?.release();\n     if(error?.code==="plan-cut"&&!signal?.aborted){level=error.cutTo;restarts--;note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"cut",used:level,budget:cutPolicy.levels,kind:String(error.currentMegabytes),limit:error.limitMegabytes}});continue;}\n     if(error?.restart!==true||signal?.aborted)throw error;',1),
(PS,' if(planStageLimitBytesV1(environment,hostMegabytes,policy)===Infinity)return Infinity;\n return Math.max(0,Math.floor((hostMegabytes*policy.treeShare-residentMegabytes)/workerMegabytes));',' const cut=graphCutConcurrencyV1(environment);\n if(planStageLimitBytesV1(environment,hostMegabytes,policy)===Infinity)return cut;\n return Math.min(cut,Math.max(0,Math.floor((hostMegabytes*policy.treeShare-residentMegabytes)/workerMegabytes)));',1),
(PS,'import{ownerTooHeavyOfV1,','import{graphCutConcurrencyV1}from"../✂️cut/🟨️.mjs";\nimport{ownerTooHeavyOfV1,',1),
(PN,'function projectConcurrencyV1(options) {\n  const value = options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism()));','export function projectConcurrencyV1(options, environment = process.env) {\n  const value = Math.min(options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism())), graphCutConcurrencyV1(environment));',1),
(PN,'import { planFarmWorkersV1 } from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";','import { planFarmWorkersV1 } from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";\nimport { graphCutConcurrencyV1 } from "../../../🧠️memory/✂️cut/🟨️.mjs";',1),
(TG,'createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,','createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,cutPolicy:{...GRAPH_CUT_POLICY_V1,levels:0},',1),
(TL,'createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,','createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,cutPolicy:{...GRAPH_CUT_POLICY_V1,levels:0},',1),
(TG,'import {HOST_POLICY_V1','import {GRAPH_CUT_POLICY_V1} from "../../../🧠️memory/✂️cut/🟨️.mjs";\nimport {HOST_POLICY_V1',1),
(TL,'import {HOST_POLICY_V1','import {GRAPH_CUT_POLICY_V1} from "../../../🧠️memory/✂️cut/🟨️.mjs";\nimport {HOST_POLICY_V1',1),
]

SCHEMA_KEYS=[(SC,'"host-slice-owners-cut"',2)]
