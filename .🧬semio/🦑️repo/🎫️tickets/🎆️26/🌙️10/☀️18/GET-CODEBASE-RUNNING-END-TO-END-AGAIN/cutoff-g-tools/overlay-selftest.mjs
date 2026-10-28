import{setOverlay}from"./overlay.mjs";
const R="/Users/ueli/Documents/semio/",f=R+"🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
const L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const obs=await import(L+"👁️observation/🟨️.mjs");
setOverlay(new Map([[f,Buffer.from("OVERLAY")]]));
console.log("observedFs:",obs.observedFs.readFileSync(f,"utf8").slice(0,10),"calls",globalThis.__overlay.calls.read);
