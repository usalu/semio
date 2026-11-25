const attest=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🩺️attest/🟦️.ts");
const store="/Users/Shared/semio-store",reference=process.argv[2]??"g1-server";
const report=await attest.viewDiffV1(store,reference);
const atts=await attest.readAttestationsV1(store),fleet=await attest.fleetReportV1(store,Date.now());
console.log("[DEBUG] here",report.here,"unavailable",report.unavailable);
for(const row of fleet.rows)console.log("[DEBUG] row",row.originId,row.platform,row.verdict,"code",row.code?.slice(0,16),"view",JSON.stringify(row.view));
for(const d of fleet.differing)console.log("[DEBUG] differing",JSON.stringify(d));
for(const d of report.differing){
 const top=(lines:readonly string[])=>{const m=new Map<string,number>();for(const l of lines){const p=l.slice(3).split("/").slice(0,4).join("/");const k=l.slice(0,3)+p;m.set(k,(m.get(k)??0)+1);}return [...m].sort((a,b)=>b[1]-a[1]).slice(0,8);};
 console.log("[DEBUG] diff",d.originId,"onlyThere",d.onlyThere.length,"onlyHere",d.onlyHere.length);
 console.log("  there",JSON.stringify(top(d.onlyThere)));console.log("  here",JSON.stringify(top(d.onlyHere)));
}
