/** 🧟️ Which re-checked row of a sealed graph-memo record is stale now (read-only): `bun memostale.ts <root> <memo-dir> <record.json>`; prints the identities that moved and every row the engine's own `observationsHold` reports stale (one pass per row group of 4,096, so several stale rows are found), with the fact it recorded and what the disk says now. */
import {readFileSync,statSync,existsSync} from "node:fs";
import {join,isAbsolute} from "node:path";
const [root,directory,recordFile]=process.argv.slice(2) as [string,string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),observation=await import(join(L,"👁️observation/🟨️.mjs"));
const record=JSON.parse(readFileSync(recordFile,"utf8")),recheck=JSON.parse(readFileSync(join(directory,record.relevance.id+".chk"),"utf8"));
for(const identity of recheck.identities)if(!memo.memoIdentityHoldsV1(root,identity)){const p=isAbsolute(identity.path)?identity.path:join(root,identity.path);console.log(JSON.stringify({identity:identity.path,recorded:identity,now:existsSync(p)?(({size,mtimeMs,ctimeMs,ino})=>({size,mtimeMs,ctimeMs,ino}))(statSync(p)):"absent"}));}
const kinds:Record<string,number>={};for(const row of recheck.rows)kinds[row.kind]=(kinds[row.kind]??0)+1;console.log(JSON.stringify({rows:recheck.rows.length,kinds}));
let stale=0;
for(let at=0;at<recheck.rows.length;at+=4096){const group=recheck.rows.slice(at,at+4096);for(let round=0;round<8;round++){let hit:any;const holds=await observation.observationsHold(group,64,new Set(),true,undefined,(row:any)=>{hit=row;});if(holds||!hit)break;stale++;console.log(JSON.stringify({stale:{kind:hit.kind,path:String(hit.path).slice(0,300),fact:JSON.stringify(hit.fact)?.slice(0,200)}}));const index=group.indexOf(hit);if(index<0)break;group.splice(index,1);}}
console.log(JSON.stringify({staleRows:stale}));
process.exit(0);
