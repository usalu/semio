import {mkdirSync,mkdtempSync,writeFileSync,existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {workspaceTestAliasesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts";
import * as memo from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧠️memo/🟦️.ts";
const put=(root:string,path:string,value:unknown)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),typeof value==="string"?value:JSON.stringify(value));};
const root=mkdtempSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/dbg/r-"),cache=mkdtempSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/dbg/c-");
put(root,"package.json",{name:"root",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["unused"],owners:["*/package.json"]}}});
put(root,"own/members/a/package.json",{name:"@t/a",version:"1.0.0",exports:{".":"./i.ts"}});
put(root,"own/package.json",{name:"scope",private:true,workspaces:["members/a"],semio:{workspace:{schemaVersion:1,members:["members/**"],owners:[]}}});
const rows:string[]=[];
await workspaceTestAliasesV1(root,{owner:"own",cache,settleMilliseconds:-3600000,report:r=>rows.push(r.source+":"+r.operations)});
const file=memo.ownershipMemoPathV1(cache,resolve(root),"own");
console.log("memo file",existsSync(file));
if(existsSync(file)){const m=memo.readOwnershipMemoV1(file)!;console.log(JSON.stringify({dirs:m.directories.length,states:m.states.length,manifests:m.manifests.length,impl:m.implementation.slice(0,8)}));
 const impl=memo.ownershipImplementationDigestV1(new URL("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",import.meta.url),new URL("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🔣️policy.json",import.meta.url));
 console.log("impl",impl?.slice(0,8),"current",await memo.ownershipMemoCurrentV1(m,root,impl!,new AbortController().signal));
 console.log(JSON.stringify(m.directories.slice(0,3)),JSON.stringify(m.states.slice(0,4)));}
console.log(rows);
{
 const {lstatSync}=await import("node:fs");
 const m=memo.readOwnershipMemoV1(file)!;
 for(const [p,st] of m.directories){const e=lstatSync(join(root,p),{bigint:true});console.log("dir",p,st,[e.dev,e.ino,e.mode,e.mtimeNs,e.ctimeNs].map(String).join(":"));}
 for(const [p,st] of m.states){let live:string;try{const e=lstatSync(join(root,p),{bigint:true});live=JSON.stringify([e.isDirectory()?"directory":e.isFile()?"file":"other",String(e.dev),String(e.ino),Number(e.mode)]);}catch{live="missing"}const rec=JSON.stringify(st);if(rec!==live)console.log("STATE DIFF",p,rec,live);}
 for(const [p,h] of m.manifests){const {createHash}=await import("node:crypto");const t=readFileSync(join(root,p),"utf8");if(createHash("sha256").update(t).digest("hex")!==h)console.log("MANIFEST DIFF",p);}
}
