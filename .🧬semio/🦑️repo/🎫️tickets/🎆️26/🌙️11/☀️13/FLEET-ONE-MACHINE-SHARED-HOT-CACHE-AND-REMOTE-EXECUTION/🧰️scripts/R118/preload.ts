import{readFileSync}from"node:fs";
const STAGE=process.env.R118_STAGE!,LIVE=process.env.R118_LIVE!,ROOT=process.cwd();
const observation=LIVE+"/🏗️build/👁️observation/🟨️.mjs",assembly=LIVE+"/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs";
const inputs=LIVE+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",staged=new Map([[inputs,STAGE+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"],[observation,STAGE+"/🏗️build/👁️observation/🟨️.mjs"],[assembly,STAGE+"/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"]]);
Bun.plugin({name:"r118",setup(build:any){
 build.onResolve({filter:/📂️source-names\/🟨️\.mjs$/u},()=>({path:STAGE+"/🏗️build/👁️observation/📂️source-names/🟨️.mjs"}));
 build.onLoad({filter:/🟨️\.mjs$/u},(args:any)=>{return{contents:readFileSync(staged.get(args.path)??args.path,"utf8"),loader:"js"};});

 build.onLoad({filter:/🧭️script-inputs\/🧪️tests\/🔣️\.json$/u},(args:any)=>({contents:"export default "+readFileSync(STAGE+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🔣️.json","utf8"),loader:"js"}));
 build.onLoad({filter:/📂️source-names\/🧪️tests\/🟦️\.ts$/u},(args:any)=>({contents:readFileSync(args.path,"utf8").replace('from"ignore"','from"'+ROOT+'/node_modules/ignore/index.js"').replace('from"../../🟨️.mjs"','from"'+observation+'"').replace('from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"','from"'+inputs+'"'),loader:"ts"}));
}});
