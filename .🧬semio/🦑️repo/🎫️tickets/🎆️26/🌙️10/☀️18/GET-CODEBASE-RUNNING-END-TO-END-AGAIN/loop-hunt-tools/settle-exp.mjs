import fs from "node:fs";import {join} from "node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const O="/private/tmp/loop-hunt/overlay";fs.rmSync(O,{recursive:true,force:true});fs.mkdirSync(O+"/np",{recursive:true});
const N=Number(process.argv[2]??30),EDIT_AT=process.argv[3]===undefined?-1:Number(process.argv[3]),EDIT_EVERY=Number(process.argv[4]??0);
fs.writeFileSync(O+"/F.txt","v0");for(let i=0;i<N;i++)fs.writeFileSync(O+`/p${i}.txt`,"p"+i);
let done=0,version=0,rounds=[],cursor=0;const t0=Date.now();
const edit=()=>{if(version>=Number(process.env.MAXEDITS??1))return;version++;fs.writeFileSync(O+"/F.txt","v"+version+"-"+"x".repeat(version));};
const loop=async()=>{cursor=0;let finished=0;const workers=Array.from({length:4},async()=>{while(cursor<N){const i=cursor++;
 await obs.observeProjectInferenceV1({cachePath:`${O}/np/${i}.json`,key:["x",i],owner:"p"+i,reusable:()=>true},async()=>{
  obs.observedFs.readFileSync(O+"/F.txt");obs.observedFs.readFileSync(O+`/p${i}.txt`);await new Promise(r=>setTimeout(r,40));return {i};});
 finished++;done++;
 if(EDIT_AT>=0&&(finished===EDIT_AT||(EDIT_EVERY&&finished%EDIT_EVERY===0&&finished>EDIT_AT)))edit();
}});await Promise.all(workers);return {finished};};
let roundNo=0;
try{
const r=await obs.observeInference({cachePath:O+"/emoji.json",key:["exp"]},()=>obs.settleObservedComputationV1(()=>{roundNo++;rounds.push({round:roundNo,startedAt:Date.now()-t0,doneBefore:done});return loop();},{limit:Number(process.env.LIMIT??6),onRound:async info=>{rounds.at(-1).end={at:Date.now()-t0,doneAfter:done,conflict:info.conflict,failure:info.failure&&String(info.failure.message)};}}));
console.log("RESULT",JSON.stringify(r));}catch(e){console.log("THROWN",e.message,e.code);}
console.log(JSON.stringify(rounds,null,0));
process.exit(0);
