import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const t=performance.now();
const r=await obs.sweepSupersededObservationBlocksV1(process.argv[2],{dryRun:process.argv[3]!=="real"});
console.log(JSON.stringify(r),((performance.now()-t)/1000).toFixed(1)+"s");
