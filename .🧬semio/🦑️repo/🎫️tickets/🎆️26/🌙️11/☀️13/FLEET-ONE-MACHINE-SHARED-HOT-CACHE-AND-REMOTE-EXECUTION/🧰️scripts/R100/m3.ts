import {readdirSync,existsSync} from "node:fs";import {join} from "node:path";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference:any=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const observation:any=await import(B+"👁️observation/🟨️.mjs"),scripts:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const found:string[]=[];
const walk=(dir:string,depth:number)=>{if(depth>7||found.length>4000)return;let es:any[];try{es=readdirSync(join(R,dir),{withFileTypes:true});}catch{return;}for(const e of es){if(!e.isDirectory()||["node_modules","target",".git",".cache"].includes(e.name)||e.name.startsWith("."))continue;const p=dir+"/"+e.name;if(existsSync(join(R,p,"📜️script.ts"))&&p.endsWith("rust"))found.push(p);else walk(p,depth+1);}};
walk("🧰️framework",0);
const pick=(n:number)=>found.filter((_,i)=>i%Math.max(1,Math.floor(found.length/n))===0).slice(0,n);
console.error("rust dirs with script",found.length);
const sizes:number[]=[];
for(const root of pick(Number(process.argv[2]??16))){
 const target={options:{command:"repo ./📜️script.ts test",cwd:root}};
 try{const out=await observation.observeInferenceSnapshotV1(async()=>{
  const cache=scripts.createScriptInputCache({workspaceRoot:R,lateClosures:true}),api=scripts.closureApiV1(cache,R);
  const handle=scripts.closureHandleOfV1(await api.nativeTargetCommandInputs(target,await api.nativeCommandInputs(),root)),result=await api.resolve([[handle.key,handle.recipe]]);
  const all=[...result.rows,...result.derived.flatMap((i:any)=>i.support)];
  return[all.length,all.filter((r:any)=>["analysis","file","field"].includes(r.kind)&&String(r.path).includes("📚️library/")).length];});
  sizes.push(out[1]);console.log(root.slice(-48),out.join(" "));}catch(e:any){console.log(root.slice(-40),"ERR",String(e.message).slice(0,80));}
}
console.log("library rows: min",Math.min(...sizes),"max",Math.max(...sizes),"n",sizes.length);process.exit(0);
