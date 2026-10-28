import {withCurrentBunWorkspaceOwnershipV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
import {lstatSync} from "node:fs";
const root=process.cwd(),started=Date.now();
const observed=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"🎓️teaching"},{signal:new AbortController().signal,advance:async()=>{}},async c=>c.observed());
const limit=BigInt(started-5000)*1000000n;let dirs=0,young=0,mismatch=0;const youngest:string[]=[];
for(const row of observed.paths){if(!row.list)continue;dirs++;const e=lstatSync(row.path,{bigint:true});const t=e.mtimeNs>e.ctimeNs?e.mtimeNs:e.ctimeNs;if(t>=limit){young++;if(youngest.length<8)youngest.push(row.path.replace(root,"")+" age="+((BigInt(Date.now())*1000000n-t)/1000000n)+"ms");}}
console.log(JSON.stringify({paths:observed.paths.length,dirs,young,youngest}));
