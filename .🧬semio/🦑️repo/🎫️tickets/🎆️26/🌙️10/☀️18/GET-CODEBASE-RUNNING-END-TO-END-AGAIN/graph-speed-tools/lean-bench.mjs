// usage: bun lean-bench.mjs OVERLAY_ROOT   Times the lean reader + the production validators over every closure leaf of the real store (read only), then the stamp-memo pass.
import {readdirSync} from "node:fs";
const W="/Users/ueli/Documents/semio",overlay=process.argv[2],L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const lean=await import(`${overlay}/${L}/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs`),data=await import(`${overlay}/${L}/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs`);
const store=W+"/.cache/b/g/m";let reads=0,bytes=0;const t0=performance.now();
for(const [dir,validate] of [["build-input-closures",data.captureSourceInputInventoryV1],["build-lock-closures",data.captureCargoLockInventoryV1]]){
 const directory=`${store}/${dir}`;
 for(const name of readdirSync(directory)){if(!/^[a-f0-9]{64}\.json$/.test(name))continue;const key=name.slice(0,-5),file=lean.readInventoryRecordLeanV1(directory,key,134217728);validate(file.value,W);lean.rememberInventoryRecordV1(W,directory,key,file.stamp);reads++;bytes+=file.byteLength;}
}
const t1=performance.now();let holds=0;
for(const [dir] of [["build-input-closures"],["build-lock-closures"]]){const directory=`${store}/${dir}`;for(const name of readdirSync(directory)){if(!/^[a-f0-9]{64}\.json$/.test(name))continue;if(lean.inventoryRecordStillHoldsV1(W,directory,name.slice(0,-5)))holds++;}}
const t2=performance.now();
console.log(JSON.stringify({leaves:reads,megabytes:Math.round(bytes/1e6),readHashValidateMs:Math.round(t1-t0),perLeafMs:+((t1-t0)/reads).toFixed(2),stillHoldPassMs:Math.round(t2-t1),stillHold:holds,loadAverage:require("node:os").loadavg()[0].toFixed(1)}));
