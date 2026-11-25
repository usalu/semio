/** 🧾️ Read-only estimate for a unit-cache root whose entries carry v1 declarations: entries whose recorded sources differ from the tree, entries an equal-slot newer entry supersedes, and the host-store bytes only those entries reference. */
import {statSync} from "node:fs";
import {scanUnitDeclarationsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts";
import {openTreeDigestsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {casRootV1,createCasStoreV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {readFileSync} from "node:fs";import {join} from "node:path";
const [root,workspace,memo]=process.argv.slice(2) as [string,string,string];
const rows=await scanUnitDeclarationsV1(root),tree=await openTreeDigestsV1(workspace,memo);
const cas=createCasStoreV1({root:casRootV1()}),size=(s:string)=>{try{return statSync(cas.objectPath(s)).size;}catch{return 0;}};
const slotOf=new Map<string,string>();for(const r of rows){try{slotOf.set(r.key,(JSON.parse(readFileSync(join(r.directory,r.newest),"utf8")) as {slot?:string}).slot??"");}catch{}}
const newest=new Map<string,number>();for(const r of rows){const s=slotOf.get(r.key);if(s)newest.set(s,Math.max(newest.get(s)??0,r.storedAt));}
const doom=new Map<string,string>();
for(const r of rows){const d=r.declaration as {sources?:{path:string;sha256:string}[]}|undefined;
 let changed=false;for(const src of d?.sources??[]){if(await tree.digest(src.path)!==src.sha256){changed=true;break;}}
 const s=slotOf.get(r.key),sup=!!s&&(newest.get(s)??0)>r.storedAt;
 if(changed)doom.set(r.key,sup?"source-changed+superseded":"source-changed");else if(sup)doom.set(r.key,"superseded");}
await tree.flush().catch(()=>undefined);
const live=new Set<string>(),dead=new Set<string>(),why:Record<string,number>={};
for(const r of rows){const w=doom.get(r.key);if(w){why[w]=(why[w]??0)+1;r.refs.forEach(x=>dead.add(x));}else r.refs.forEach(x=>live.add(x));}
let only=0;for(const s of dead)if(!live.has(s))only+=size(s);
let held=0;for(const s of new Set(rows.flatMap(r=>r.refs)))held+=size(s);
console.log(JSON.stringify({entries:rows.length,doomed:doom.size,why,heldGiB:+(held/2**30).toFixed(2),freedGiB:+(only/2**30).toFixed(2)},null,1));
