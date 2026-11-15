import {writeFileSync,readFileSync} from "node:fs";
const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",recipes=JSON.parse(readFileSync(process.env.R5C_RECIPES!,"utf8")) as any[];
const only=process.env.R5C_ROOTS?new Set(JSON.parse(readFileSync(process.env.R5C_ROOTS,"utf8")) as string[]):undefined,want=Number(process.env.R5C_N??"40"),seen=new Set<string>(),pick:any[]=[];
for(const r of recipes){if(r.fn!==(process.env.R5C_FN??"native-target")||!r.target)continue;if(process.env.R5C_ENTRY&&!JSON.stringify(r.target).includes(process.env.R5C_ENTRY)&&!String(r.projectRoot).includes(process.env.R5C_ENTRY))continue;if(only&&!only.has(r.projectRoot))continue;const key=r.projectRoot;if(seen.has(key))continue;seen.add(key);pick.push(r);}
const step=only?1:Math.max(1,Math.floor(pick.length/want)),sample=pick.filter((_,i)=>i%step===0).slice(0,only?1000:want);
const out:any[]=[];
const files=(list:any[])=>list.filter(x=>typeof x==="string"&&x.startsWith("{workspaceRoot}/")).map(x=>x.slice(16));
for(const r of sample){
 const {inputsDigest,...target}=r.target;
 await observation.observeInferenceSnapshotV1(async()=>{
  const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false});
  const started=performance.now();
  try{
   const list=r.fn==="native-target"?await scriptInputs.nativeTargetCommandInputs(target,ws,undefined,cache,r.projectRoot):await scriptInputs.genericTargetCommandInputs(target,ws,scriptInputs.genericCommandFallbackInputs(ws),cache,r.projectRoot);
   const expanded=await scriptInputs.expandedCommandInputsV1(list,ws,cache);
   const refs=list.filter((x:any)=>x&&typeof x==="object"&&x.input);
   out.push({root:r.projectRoot,own:files(list).length,ownFiles:files(list),refs:refs.map((x:any)=>x.input),expanded:files(expanded).sort(),others:JSON.stringify(expanded.filter((x:any)=>typeof x!=="string"&&!x.input)),ownOthers:JSON.stringify(list.filter((x:any)=>typeof x!=="string")).slice(0,600),ms:Math.round(performance.now()-started)});
  }catch(error){out.push({root:r.projectRoot,error:String((error as any)?.message??error).slice(0,300)});}
 });
}
writeFileSync(process.env.R5C_OUT!,JSON.stringify(out));
console.log("sampled",out.length,"errors",out.filter(r=>r.error).length);
