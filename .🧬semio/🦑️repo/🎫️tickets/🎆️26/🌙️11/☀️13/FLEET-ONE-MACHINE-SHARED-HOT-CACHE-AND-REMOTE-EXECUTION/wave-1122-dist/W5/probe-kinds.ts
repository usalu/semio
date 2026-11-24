const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const obs=await import(L+"👁️observation/🟨️.mjs");
const sample=JSON.parse(await Bun.file(process.argv[2]).text()) as Record<string,any[]>;
for(const pass of [1,2,3])for(const [kind,rows] of Object.entries(sample)){
  let stale=0;const t=performance.now();
  const ok=await obs.observationsHoldWithinRoundV1(rows,64,undefined,()=>{stale++;});
  const ms=performance.now()-t;
  console.log("[DEBUG] pass",pass,kind,rows.length,"rows",ms.toFixed(0),"ms",(ms*1000/rows.length).toFixed(0),"us/row stale",stale,"ok",ok);
}
