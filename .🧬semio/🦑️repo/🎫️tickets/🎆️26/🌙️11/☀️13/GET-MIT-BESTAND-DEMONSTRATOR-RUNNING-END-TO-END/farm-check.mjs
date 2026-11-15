import {mkdtempSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",W="/Users/Shared/semio",F=B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs"),pool=await import(F+"🏊️pool/🟨️.mjs"),nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const contracts=nodes.projectGeneratorContractsV1(W);
const session={version:3,workspaceRoot:W,closureStore:undefined,registry:null,contracts,commandInputs:null,lateClosures:false,projectsByRoot:[],componentRoots:[],stagingBase:mkdtempSync("/tmp/fc-"),authority:obs.inferenceAuthorityRowsV1()};
const id=obs.inferenceRecordIdentityV1();
const t=performance.now();
const farm=pool.createFarmV1({entry:F+"👷️worker/🟨️.mjs",service:"file://"+F+"👷️service/🟨️.mjs",session,workers:1,megabytes:1024,environment:process.env,accept:i=>{console.log("[DEBUG] accept",i.records===id,i.conflicts);return i.records===id;},report:(e,f)=>console.log("[DEBUG] report",e,JSON.stringify(f))});
const mode=process.env.BUSY;let stop=false;
setTimeout(()=>stop=true,20000);
const spin=ms=>{const e=performance.now()+ms;while(performance.now()<e);};
if(mode){(async()=>{while(!stop){spin(200);if(mode==="immediate")await new Promise(r=>setImmediate(r));else if(mode==="timeout")await new Promise(r=>setTimeout(r,0));else await Promise.resolve();}})();}
const usable=await Promise.race([farm.ready,new Promise(r=>setTimeout(()=>r("TIMEOUT"),90000))]);
console.log("[DEBUG] ready",usable,Math.round(performance.now()-t),"ms");
await farm.close();process.exit(0);
