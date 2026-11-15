import {compareCheckpointsV1,fleetReportV1,readAttestationsV1,readGapWindowsV1,spanOfV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🩺️attest/🟦️.ts";
const store="/Users/Shared/semio-store";
const now=Date.now();
const atts=await readAttestationsV1(store);
const cps=Object.entries(atts).filter(([,a])=>a.sync&&a.checkpoints!==undefined&&now-a.wallMs<60000);
for(const [o,a] of cps){const c=a.checkpoints!;console.log(o,"code",a.code.slice(0,8),"manifest",c.manifest?Math.round((now-c.manifest.wallMs)/1000)+"s":"-","git",c.git?Math.round((now-c.git.wallMs)/1000)+"s":"-","view",c.view?Math.round((now-c.view.wallMs)/1000)+"s":"-",a.view?.stale?"stale-view":"");}
const cuts=cps.flatMap(([,a])=>[a.checkpoints!.manifest,a.checkpoints!.git,a.checkpoints!.view]).filter(x=>x!==undefined).flatMap(x=>[x!.cursors,...("until" in x!&&x.until?[x.until]:[])]);
const windows=await readGapWindowsV1(store,cuts);
for(let i=0;i<cps.length;i++)for(let j=i+1;j<cps.length;j++){
 const r=compareCheckpointsV1(cps[i]![1].checkpoints!,cps[j]![1].checkpoints!,now,windows);
 console.log(cps[i]![0],cps[j]![0],"known",[...r.known].join(","),"items",r.items.length,"apart",r.items.filter(x=>!x.same).map(x=>`${x.field}${x.bucket<0?"":"#"+x.bucket}`).join(","));
}
const rep=await fleetReportV1(store);
console.log(JSON.stringify(rep.rows.map(r=>[r.originId,r.verdict,r.differs,r.basis??"now",r.checkpointSeconds??null])));
console.log(JSON.stringify(rep.differing.map(d=>[d.originId,d.field,String(d.value).slice(0,40),String(d.expected).slice(0,40)])));
