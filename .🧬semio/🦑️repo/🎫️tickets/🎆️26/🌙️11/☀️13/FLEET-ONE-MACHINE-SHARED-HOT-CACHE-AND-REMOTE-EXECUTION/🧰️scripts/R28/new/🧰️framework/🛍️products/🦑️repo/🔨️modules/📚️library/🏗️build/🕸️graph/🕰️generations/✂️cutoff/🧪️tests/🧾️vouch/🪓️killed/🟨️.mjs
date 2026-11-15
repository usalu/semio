import{mkdirSync,readFileSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{createClosureSettlerV1,CLOSURE_SETTLE_V1,stampOfV1}from"../../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../../🟨️.mjs";
import{createFactIndexV1}from"../../../../🟨️.mjs";

/** 🪓️ Settles `count` closures in chunks of `chunk` with the manifest saved after every chunk and kills its own process at the derivation `killAt`: only what the chunks persisted is durable. */
const[root,workspace,count,chunk,killAt]=process.argv.slice(2),rowOf=name=>({kind:"derived",path:name,fact:"old"});
const names=Array.from({length:Number(count)},(_,at)=>"q"+String(at).padStart(4,"0"));
const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),{kind:"file",path:"own/"+name,fact:"v"}]])]));
let derived=0;
const settler=createClosureSettlerV1({store:createNodeStoreV1({root}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:Number(chunk),deriveConcurrency:1,manifestSaveMilliseconds:0,manifestSaveSpendRatio:0}});
await settler.settle({pseudoRows:suspectRowsV1(index,new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))))),derive:async row=>{if(++derived===Number(killAt))process.kill(process.pid,"SIGKILL");return{fact:"old",parts:[[{kind:"file",path:"own/"+row.path,fact:readFileSync(join(workspace,"own",row.path),"utf8")}]]};},hold:async()=>true,dirty:{paths:[]},vouchStamp:name=>stampOfV1(join(workspace,name))});
