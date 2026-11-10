const root="/home/deck/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),obs=await import(B+"/👁️observation/🟨️.mjs");
const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:false});
const t=async(name,f)=>{const s=performance.now();let r;try{r=await obs.observeDetachedV1(f);}catch(e){console.log("[DEBUG]",name,"ERR",String(e.message).slice(0,100));return;}console.log("[DEBUG]",name,Math.round(performance.now()-s),"ms rows",r.rows.length,"layers",r.layers.length,"value",Array.isArray(r.value)?r.value.length:"");};
const entry=root+"/🧰️framework/📦️packages/🟦️typescript/📜️script.ts";
await t("walk1 cold",()=>si.relativeScriptInputs([entry],root,scripts));
await t("walk1 again (same cache)",()=>si.relativeScriptInputs([entry],root,scripts));
const other=root+"/🧰️framework/📦️packages/🦀️rust/📜️script.ts";
await t("walk other entry, same cache",()=>si.relativeScriptInputs([other],root,scripts));
await t("walk1 fresh cache",()=>si.relativeScriptInputs([entry],root,inference.createScriptInputCache({workspaceRoot:root})));
{
const {createClosureStoreV1}=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs");
const {mkdtempSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path");
const dir=mkdtempSync(join(root,".cache","entries-prof-"));
const store=createClosureStoreV1({directory:dir});
const r=await obs.observeDetachedV1(()=>si.relativeScriptInputs([entry],root,inference.createScriptInputCache({workspaceRoot:root})));
let s=performance.now();const bytes=await store.save("id",{recipe:{},value:r.value,rows:r.rows,layers:r.layers});console.log("[DEBUG] save",Math.round(performance.now()-s),"ms",bytes,"chars");
s=performance.now();const l=await store.load("id");console.log("[DEBUG] load",Math.round(performance.now()-s),"ms layers",l.layers.length);
s=performance.now();let ok=obs.observationRowsHoldV1(l.rows)&&l.layers.every(x=>obs.observationLayerHoldsV1(x));console.log("[DEBUG] hold (no round)",Math.round(performance.now()-s),"ms",ok);
s=performance.now();ok=obs.withObservationRoundV1(()=>obs.observationRowsHoldV1(l.rows)&&l.layers.every(x=>obs.observationLayerHoldsV1(x)));console.log("[DEBUG] hold (round)",Math.round(performance.now()-s),"ms",ok);
s=performance.now();ok=obs.observationRowsHoldV1(r.rows)&&r.layers.every(x=>obs.observationLayerHoldsV1(x));console.log("[DEBUG] hold original arrays (no round)",Math.round(performance.now()-s),"ms",ok);
const {rmSync}=await import("node:fs");rmSync(dir,{recursive:true,force:true});
}
