import {readFileSync,writeFileSync} from "node:fs";
const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",cg=JSON.parse(readFileSync(process.env.R5C_IN!,"utf8")) as any[];
const big=cg.filter(x=>x.ownFiles&&x.ownFiles.length>1000);
const count=new Map<string,number>();for(const x of big)for(const f of new Set<string>(x.ownFiles))count.set(f,(count.get(f)??0)+1);
const H=new Set([...count].filter(([,n])=>n>=big.length*0.9).map(([f])=>f));
console.log("big owners",big.length,"shared files",H.size);
const candidates=(process.env.R5C_CANDIDATES??"").split(",").filter(Boolean);
const out:any={shared:H.size,candidates:{}};
await observation.observeInferenceSnapshotV1(async()=>{
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false});
 for(const c of candidates){
  try{
   const walked=await scriptInputs.relativeScriptInputs([ws+"/"+c],ws,cache),pruned=scriptInputs.withoutSchedulingV1(walked).filter((x:any)=>typeof x==="string").map((x:string)=>x.slice(16));
   const inH=pruned.filter((f:string)=>H.has(f)).length;
   out.candidates[c]={walk:pruned.length,inShared:inH,coverage:Math.round(1000*inH/H.size)/10};
   console.log(c.slice(-80),"walk",pruned.length,"inShared",inH,"coverage%",out.candidates[c].coverage);
  }catch(error){console.log(c,"ERR",String((error as any)?.message).slice(0,100));}
 }
});
// also: files of H grouped by 5-segment prefix
const g=new Map<string,number>();for(const f of H){const k=f.split("/").slice(0,9).join("/");g.set(k,(g.get(k)??0)+1);}
console.log([...g].sort((a,b)=>b[1]-a[1]).slice(0,12).map(([k,n])=>n+" "+k.slice(-70)).join("\n"));
