// 🧪️ H102 check: JS rule == Python oracle (keys, readsDigest, flagged set), soundness against the toy ground truth, comparison with today's diff/consumers rule. Usage: node check.mjs <worlds.json>
import{readFileSync}from"node:fs";
import{indexEnvironmentV1,recordKeysV1,readsDigestV1,staleOwnersV1,makeRecordV1,assembleViewV1,cp}from"./environment-components.mjs";
const REAL="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧵️edge-reads/🟨️.mjs";
const{diffEnvironmentV1,edgeConsumersV1}=await import(REAL);
const worlds=JSON.parse(readFileSync(process.argv[2],"utf8"));
const tot={worlds:worlds.length,records:0,truth:0,flaggedNew:0,flaggedReal:0,flaggedAll:0,keyMismatch:0,digestMismatch:0,flaggedMismatch:0,newUnsound:0,realUnsound:0,realOnly:0,newOnly:0,viewMismatch:0};
const closed=reads=>{const out={};for(const key of["owners","paths","packages","modules","workspaces"])if(reads[key].length)out[key]=reads[key];if(reads.lock)out.lock=true;return out;};
for(const world of worlds){
 const e1=indexEnvironmentV1(world.env1),e2=indexEnvironmentV1(world.env2),records=[],rows=new Map();
 for(const rec of world.records){
  const row={name:rec.name,root:e1.rows.get(rec.name)[1],nativeRoot:e1.rows.get(rec.name)[2]},reads=closed(rec.reads);
  const keys=recordKeysV1(row,reads,e1),digest=readsDigestV1(keys,e1);
  if(JSON.stringify(keys)!==JSON.stringify(rec.keys))tot.keyMismatch++;if(digest!==rec.readsDigest)tot.digestMismatch++;
  records.push({name:rec.name,keys,readsDigest:digest});rows.set(rec.name,{root:row.root,nativeRoot:row.nativeRoot,reads});
 }
 const flagged=staleOwnersV1(records,e2);
 if(JSON.stringify(flagged)!==JSON.stringify(world.flagged))tot.flaggedMismatch++;
 const truth=new Set(world.truth);tot.records+=records.length;tot.truth+=truth.size;tot.flaggedNew+=flagged.length;tot.flaggedAll+=records.length;
 for(const name of truth)if(!flagged.includes(name))tot.newUnsound++;
 const diff=diffEnvironmentV1(world.env1,world.env2),tables=[new Map(world.env1.owners.map(r=>[r[0],[r[1],r[2]]])),new Map(world.env2.owners.map(r=>[r[0],[r[1],r[2]]]))];
 const real=edgeConsumersV1({diff,rows,tables});tot.flaggedReal+=real.size;
 for(const name of truth)if(e2.rows.has(name)&&!real.has(name)){tot.realUnsound++;(tot.realMissExamples??=[]).length<3&&tot.realMissExamples.push({name,mutations:world.mutations});}
 const set=new Set(flagged);for(const name of real)if(!set.has(name))tot.realOnly++;for(const name of set)if(!real.has(name))tot.newOnly++;
 const heads=new Map(records.map(r=>[r.name,[r]])),names=records.map(r=>r.name);
 const v2a=assembleViewV1(heads,e2,names),v2b=assembleViewV1(new Map([...heads].reverse()),e2,[...names].reverse());
 if(v2a.generation!==v2b.generation||v2a.stale.length!==flagged.length||v2a.chosen.length!==records.length-flagged.length)tot.viewMismatch++;
}
console.log(JSON.stringify(tot));
const rate=x=>(100*x/tot.records).toFixed(1)+"%";
console.log(`stale per environment change: global stamp 100%, today's consumers rule ${rate(tot.flaggedReal)}, component rule ${rate(tot.flaggedNew)}, ground truth ${rate(tot.truth)}`);
if(tot.keyMismatch||tot.digestMismatch||tot.flaggedMismatch||tot.newUnsound||tot.viewMismatch){console.error("FAIL");process.exit(1);}
