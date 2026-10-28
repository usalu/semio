// [diag1-g] preload: the graph-free entry runs without a script-process invocation (as asm-g/delta-build.mjs did)
const INVK="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🚦️invocation/🟨️.mjs";
Bun.plugin({name:"diag1-invk",setup(b){b.onLoad({filter:/invocation\/[^/]*\.mjs$/},async args=>{if(args.path!==INVK)return undefined;let t=await Bun.file(args.path).text();
 const a="return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(";
 if(!t.includes(a)){console.error("[DEBUG] invocation anchor missing");return undefined;}
 return{contents:t.replace(a,"return true?run(undefined,undefined):receiveScriptProcessInvocation("),loader:"js"};});}});
const STOR="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs";
Bun.plugin({name:"diag1-stor",setup(b){b.onLoad({filter:/storage\/[^/]*\.mjs$/},async args=>{if(args.path!==STOR)return undefined;let t=await Bun.file(args.path).text();
 const a="export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}";
 if(!t.includes(a)){console.error("[DEBUG] storage anchor missing");return undefined;}
 return{contents:t.replace(a,"export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return body({},phaseControl);}"),loader:"js"};});}});
const REUSE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs";
Bun.plugin({name:"diag1-reuse",setup(b){b.onLoad({filter:/reuse\/[^/]*\.mjs$/},async args=>{if(args.path!==REUSE)return undefined;let t=await Bun.file(args.path).text();
 const a="requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();const names=";
 if(!t.includes(a)){console.error("[DEBUG] reuse anchor missing");return undefined;}
 return{contents:t.replace(a,"if(true)return true;const names="),loader:"js"};});}});
