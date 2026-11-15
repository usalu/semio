import {writeFileSync,readFileSync} from "node:fs";
const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",recipes=JSON.parse(readFileSync(process.env.R5C_RECIPES!,"utf8")) as any[];
const rows:any[]=[];
const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false});
for(let at=0;at<recipes.length;at+=200){
 const slice=recipes.slice(at,at+200);
 await observation.observeInferenceSnapshotV1(async()=>{
  for(const r of slice){
   if(r.fn==="closure-of"||!r.target)continue;
   const {inputsDigest,...rest}=r.target;
   try{const e=await scriptInputs.targetScriptEntriesV1(rest,ws,r.projectRoot,r.nativeManifest);
    rows.push({fn:r.fn,entries:[...e.entries].map((x:string)=>x.startsWith(ws+"/")?x.slice(ws.length+1):x).sort(),words:[...e.words].map(([k,v]:any)=>[k.slice(ws.length+1),v?[...v].sort():null]),resources:e.resources.length,fallback:e.fallback,projectRoot:r.projectRoot});
   }catch(error){rows.push({fn:r.fn,error:String((error as any)?.message??error).slice(0,200),projectRoot:r.projectRoot});}
  }
 });
}
writeFileSync(process.env.R5C_OUT!,JSON.stringify(rows));
console.log("rows",rows.length,"errors",rows.filter(r=>r.error).length);
