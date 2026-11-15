import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
const G="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/";
const E=(process.env.R5_OVERLAY??"/Users/Shared/semio")+G.slice("/Users/Shared/semio".length),kinds=await import(E+"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"),{createNodeStoreV1}=await import(G+"🧮️derived-nodes/💾️store/🟨️.mjs"),{derivedOwnerNameV1,suspectRowsV1}=await import(G+"🕰️generations/✂️cutoff/🟨️.mjs"),{createFactIndexV1}=await import(G+"🕰️generations/🟨️.mjs");
const N=Number(process.argv[2]??8000),K=Number(process.argv[3]??200),SHARED=50,label=process.argv[4]??"run";
const names=Array.from({length:N},(_,at)=>"q"+String(at).padStart(5,"0")),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"});
const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),{kind:"file",path:"own/"+name,fact:"v"}]] as [string,any[]])]));
const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))));
const store=mkdtempSync(join(tmpdir(),"bench-store-")),ws=mkdtempSync(join(tmpdir(),"bench-ws-"));
mkdirSync(join(ws,"own"));mkdirSync(join(ws,"shared"));
for(let at=0;at<SHARED;at++)writeFileSync(join(ws,"shared","s"+at),"s0-"+at);
for(const name of names)writeFileSync(join(ws,"own",name),"v0-"+name+"-".repeat(2000));
const text=(path:string)=>readFileSync(join(ws,path),"utf8"),sha=(value:string)=>createHash("sha256").update(value).digest("hex");
const stamp=typeof kinds.stampOfV1==="function"?(name:string)=>kinds.stampOfV1(join(ws,name)):undefined;
const pool=new Map<string,Buffer>(),blobs={put:async(bytes:Buffer)=>{const hash=createHash("sha256").update(bytes).digest("hex");pool.set(hash,Buffer.from(bytes));return{hash};},read:async(hash:string)=>pool.get(hash)};
const fast={...kinds.CLOSURE_SETTLE_V1,manifestSaveMilliseconds:0,manifestSaveSpendRatio:0};
type Out={derived:number;rows:number;ms:number;vouched?:number};
async function run(dirty:{paths:string[]}|undefined,abortAt=0):Promise<Out>{
 const settler=kinds.createClosureSettlerV1({store:createNodeStoreV1({root:store,blobs}),limits:abortAt>0?fast:kinds.CLOSURE_SETTLE_V1}),controller=new AbortController();let derived=0,rows=0;const began=performance.now();
 try{
  const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),signal:controller.signal,derive:async(row:{path:string})=>{if(++derived===abortAt)controller.abort();controller.signal.throwIfAborted();const at=Number(row.path.slice(1))%SHARED;return{fact:"old",parts:[[{kind:"file",path:"shared/s"+at,fact:sha(text("shared/s"+at))}],[{kind:"file",path:"own/"+row.path,fact:sha(text("own/"+row.path))}]]};},hold:async(list:{path:string;fact:string}[])=>{rows+=list.length;return list.every(row=>sha(text(row.path))===row.fact);},dirty,...stamp?{vouchStamp:stamp}:{}});
  await settler.markIndexed({held:settled.held});
  return{derived,rows,ms:Math.round(performance.now()-began),vouched:settled.vouched};
 }catch(error){if(!controller.signal.aborted)throw error;return{derived,rows,ms:Math.round(performance.now()-began)};}
}
const out:Record<string,unknown>={label,closures:N,editedFiles:K,vouchAvailable:stamp!==undefined};
out.cold=await run(undefined);
const edited=names.filter((_,at)=>at%Math.floor(N/K)===0).slice(0,K);for(const name of edited)writeFileSync(join(ws,"own",name),"v1-"+name);writeFileSync(join(ws,"shared/s3"),"s1-3");writeFileSync(join(ws,"shared/s7"),"s1-7");
const dirty={paths:[...edited.map(name=>"own/"+name),"shared/s3","shared/s7"]};
out.firstStartAfterEdits=await run(dirty);
out.secondStartSameTree=await run(dirty);
out.thirdStartSameTree=await run(dirty);
rmSync(store,{recursive:true,force:true});mkdirSync(store);
out.killedColdRun=await run(undefined,Math.floor(N/2));
out.restartAfterKillCleanJournal=await run({paths:[]});
console.log(JSON.stringify(out));
rmSync(store,{recursive:true,force:true});rmSync(ws,{recursive:true,force:true});
