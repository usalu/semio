"""R35: the memory of the graph read is bounded by its grant: the physical footprint of the parent is counted (not the resident set), the admission slows before the grant is overrun, the process settles after every published wave. Anchored edits (rel path from the worktree root, old, new, count); already applied edits count as same."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))

CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"
# landed in the first landing: add(CK,'import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦
# landed in the first two landings: add(CK,'export function useCapacityGrantV1(grant){grantOfProcess=grant;}','export function
add(CK,' let scope,chain=Promise.resolve();\n',' let scope,relief,chain=Promise.resolve();\n')
add(CK,'const run=async options=>{try{return await runOnce(options);}finally{scope=undefined;}};','const run=async options=>{try{return await runOnce(options);}finally{scope=undefined;relief?.();relief=undefined;}};')
add(CK,'  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map();\n','  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map(),gate=footprintGateV1();\n  relief=gate?.relieve(()=>{delete pass.round;partOk.clear();sweepRows();});\n')
add(CK,'if(++derivations%limits.collectEveryClosures===0)runtimeCollectGarbageV1(true);return answer;}','if(++derivations%limits.collectEveryClosures===0)releaseRuntimeMemoryV1();return answer;}')
add(CK,'while(next<list.length){signal?.throwIfAborted();await work(list[next++]);}','while(next<list.length){signal?.throwIfAborted();await gate?.admit(signal);await work(list[next++]);}')
add(CK,'   runtimeCollectGarbageV1(true);sweepRows();\n','   releaseRuntimeMemoryV1();sweepRows();\n')
add(CK,'forgetParts(true);await saveManifest();runtimeCollectGarbageV1(true);','forgetParts(true);await saveManifest();releaseRuntimeMemoryV1();')

DN=B+"🕸️graph/🧮️derived-nodes/🟨️.mjs"
add(DN,'import{canonicalJsonV1}from"../🕰️generations/🧮️fact-codec/🟨️.mjs";\n','import{canonicalJsonV1}from"../🕰️generations/🧮️fact-codec/🟨️.mjs";\nimport{footprintGateV1}from"../../🧠️memory/🦶️footprint/🟨️.mjs";\n')
add(DN,'capacityOfGrantV1(source,{rss=()=>process.memoryUsage.rss(),','capacityOfGrantV1(source,{rss=()=>footprintGateV1()?.footprintBytes()??process.memoryUsage.rss(),')

M=B+"🧠️memory/🟨️.mjs"
add(M,'import {createOwnerScopeV1} from "./🎯️owner-scope/🟨️.mjs";\n','import {createOwnerScopeV1} from "./🎯️owner-scope/🟨️.mjs";\nimport {footprintGateV1} from "./🦶️footprint/🟨️.mjs";\n')
add(M,'usage=()=>process.memoryUsage().rss}={}){','usage=()=>footprintGateV1()?.footprintBytes()??process.memoryUsage().rss}={}){')
add(M,'export function createMemoryGovernorV1({workers,headroom,attributable,measure,collect,','export function createMemoryGovernorV1({workers,headroom,attributable,measure,collect,hold,')
add(M,'    if(owners.size>0&&spacing>0){await sleep(spacing,signal);continue;}\n','    if(owners.size>0&&spacing>0){await sleep(spacing,signal);continue;}\n    if(owners.size>0&&hold?.()===true){await sleep(policy.admitMinimumMilliseconds,signal);continue;}\n')
add(M,' return createMemoryGovernorV1({...options,measure:options.measure??ports.sample,',' return createMemoryGovernorV1({...options,hold:options.hold??(()=>{const gate=footprintGateV1();return gate!==undefined&&gate.headroomBytes()<0;}),measure:options.measure??ports.sample,')

H=B+"🕸️graph/🕰️host/🟦️.ts"
add(H,'seams.published(generation);await announceGraphWaveV1({generation,info});','seams.published(generation);await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:()=>{observationV1.releaseInferenceMemoryV1();graphStringInternerV1.clear();}});')
add(H,'import {announceGraphWaveV1} from "../📈️wave-row/🟨️.mjs";\n','import {announceGraphWaveV1} from "../📈️wave-row/🟨️.mjs";\nimport {settleWaveMemoryV1} from "../../🧠️memory/🦶️footprint/🟨️.mjs";\n')
add(M[:-len("🟨️.mjs")]+"🟨️.d.mts",'attributable?:()=>number;measure?:()=>number;collect?:()=>void;ownerLimitBytes?:number;','attributable?:()=>number;measure?:()=>number;collect?:()=>void;hold?:()=>boolean;ownerLimitBytes?:number;')
# landed in the first two landings: add(CK,'export function useCapacityGrantV1(grant){grantOfProcess=grant;useFootprintGateV1(
add(CK,'report:line=>{try{process.stderr.write(line+"\\n");}catch{}}});gate?.watch();useFootprintGateV1(gate);}','report:line=>{try{process.stderr.write(line+"\\n");}catch{}trace(line);}});gate?.watch();useFootprintGateV1(gate);}')
add(CK,'import{createFootprintGateV1,footprintGateV1,grantBytesOfV1,releaseRuntimeMemoryV1,useFootprintGateV1}from"../../../🧠️memory/🦶️footprint/🟨️.mjs";\n','import{createFootprintGateV1,createFootprintTraceV1,footprintGateV1,grantBytesOfV1,releaseRuntimeMemoryV1,useFootprintGateV1}from"../../../🧠️memory/🦶️footprint/🟨️.mjs";\n')
add(CK,'export function useCapacityGrantV1(grant){grantOfProcess=grant;const gate=','export function useCapacityGrantV1(grant){grantOfProcess=grant;const trace=createFootprintTraceV1(),gate=')
add(CK,'import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";\n','import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";\nimport{profileCurrentPhaseV1}from"../../../⏱️profile/🟨️.mjs";\n')
add(CK,'report:line=>{try{process.stderr.write(line+"\\n");}catch{}trace(line);}});gate?.watch();useFootprintGateV1(gate);}','report:line=>{try{process.stderr.write(line+"\\n");}catch{}trace(line);},heartbeat:row=>trace("heartbeat phase="+(profileCurrentPhaseV1()??"-")+" footprint="+Math.round(row.footprintBytes/1048576)+" MB grant="+Math.round(row.grantBytes/1048576)+" MB rss="+Math.round(row.rssBytes/1048576)+" MB")});if(gate!==undefined)trace("gate installed pid="+process.pid+" grant="+Math.round(gate.grantBytes()/1048576)+" MB");gate?.watch();useFootprintGateV1(gate);}')
