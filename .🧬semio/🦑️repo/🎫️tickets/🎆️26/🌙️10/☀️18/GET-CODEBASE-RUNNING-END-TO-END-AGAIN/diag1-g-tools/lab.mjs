// [diag1-g] preload (FIRST, before ../cutoff-g-tools/overlay-plugin.mjs): serves every repository module with (a) the overlay fs shim, (b) graph-free anchors (no script invocation, no storage custody, no inventory control), (c) optionally the git INDEX version (pre-repair state) or a scratch patch file instead of the live file. Nothing is written to the tree.
// DIAG_INDEX=rel1,rel2   DIAG_PATCHMAP=rel1=<file>,rel2=<file> (absolute patch files)   DIAG_SKIP_INDEX=1 serves the live files
import{execFileSync}from"node:child_process";import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",here=import.meta.dir,fsShim=here+"/../cutoff-g-tools/overlay-fs-shim.mjs",fspShim=here+"/../cutoff-g-tools/overlay-fsp-shim.mjs",E=process.env;
const idx=new Set((E.DIAG_INDEX??"").split(",").filter(Boolean).map(r=>R+r)),patch=new Map((E.DIAG_PATCHMAP??"").split(",").filter(Boolean).map(s=>{const i=s.lastIndexOf("=");return[R+s.slice(0,i),s.slice(i+1)];}));
const shim=text=>/["'](?:node:)?fs(?:\/promises)?["']/.test(text)?text.replace(/(["'])(?:node:)?fs\/promises\1/g,(_m,q)=>q+fspShim+q).replace(/(["'])(?:node:)?fs\1/g,(_m,q)=>q+fsShim+q):text;
const L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const anchors=new Map([
 [L+"🕸️graph/📥️inference/🚦️invocation/🟨️.mjs",["return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(","return true?run(undefined,undefined):receiveScriptProcessInvocation("]],
 [L+"📥️inputs/💾️storage/🟨️.mjs",["export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}","export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return body({},phaseControl);}"]],
 [L+"👁️observation/📥️reuse/🟨️.mjs",["requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();const names=","if(true)return true;const names="]],
]);
Bun.plugin({name:"diag1-lab",setup(b){b.onLoad({filter:/^\/Users\/ueli\/Documents\/semio\/(?!\.cache\/)(?!.*\/node_modules\/)(?!.*cutoff-g-tools)(?!.*diag1-g-tools).*\.(mjs|ts)$/},async args=>{
 let text,served="live";
 if(patch.has(args.path)){text=readFileSync(patch.get(args.path),"utf8");served="patch";}
 else if(idx.has(args.path)){text=execFileSync("git",["show",":"+args.path.slice(R.length)],{cwd:R,maxBuffer:1<<28}).toString("utf8");served="index";}
 else text=readFileSync(args.path,"utf8");
 const anchor=anchors.get(args.path);if(anchor){if(!text.includes(anchor[0]))console.error("[DEBUG] anchor missing "+args.path.slice(-50));else text=text.replace(anchor[0],anchor[1]);}
 if(served!=="live")console.error("[DEBUG] served "+served+": "+args.path.slice(-60));
 return{contents:shim(text),loader:args.path.endsWith(".ts")?"ts":"js"};});}});
