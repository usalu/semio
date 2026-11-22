import {readFileSync} from "node:fs";
const obs=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const rows=(JSON.parse(readFileSync(process.argv[2],"utf8")).observations as {kind:string;path:string}[]).filter(r=>["exists","resolve","fileExists","directoryExists","stat","typescriptText"].includes(r.kind));
const chunk=Number(process.argv[3]??200);
console.log("rows",rows.length,"chunk",chunk);
for(const mode of ["plain","round"]){
 const t=performance.now();let n=0;
 const run=async()=>{for(let i=0;i<rows.length;i+=chunk){const part=rows.slice(i,i+chunk);n++;
  mode==="plain"?await obs.observationsHold(part,64,new Set(),new Set()):await obs.observationsHoldWithinRoundV1(part,64);}};
 if(mode==="round")await obs.settleObservedComputationV1(async()=>{await run();return 1;});else await run();
 console.log(mode,n,"calls",Math.round(performance.now()-t)+"ms");
}
