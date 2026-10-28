import {ownershipImplementationDigestV1,ownershipMemoCurrentV1,ownershipMemoPathV1,readOwnershipMemoV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧠️memo/🟦️.ts";
import {lstatSync,readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,resolve} from "node:path";
const root=process.cwd(),cache=process.argv[2]!,file=ownershipMemoPathV1(cache,resolve(root),"🎓️teaching"),m=readOwnershipMemoV1(file);
console.log("file",file.slice(-40),"memo",!!m);
if(m){
 console.log("dirs",m.directories.length,"states",m.states.length,"manifests",m.manifests.length,"workspace",m.workspace===resolve(root));
 let bad=0;for(const [p,st] of m.directories){const e=lstatSync(join(root,...p.split("/")),{bigint:true});const live=[e.dev,e.ino,e.mode,e.mtimeNs,e.ctimeNs].map(String).join(":");if(live!==st){if(bad++<5)console.log("DIR",p,st,live);}}
 console.log("dir mismatches",bad);
 let sb=0;for(const [p,st] of m.states){let live:string;try{const e=lstatSync(join(root,...p.split("/")),{bigint:true});live=JSON.stringify([e.isSymbolicLink()?"symlink":e.isDirectory()?"directory":e.isFile()?"file":"other",String(e.dev),String(e.ino),Number(e.mode)]);}catch{live=JSON.stringify(["missing"])}if(live!==JSON.stringify(st)){if(sb++<5)console.log("STATE",p,JSON.stringify(st),live);}}
 console.log("state mismatches",sb);
 let mb=0;for(const [p,h] of m.manifests){const t=readFileSync(join(root,...p.split("/")),"utf8");if(createHash("sha256").update(t).digest("hex")!==h){if(mb++<5)console.log("MANIFEST",p);}}
 console.log("manifest mismatches",mb);
 const impl=ownershipImplementationDigestV1(new URL("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",import.meta.url),new URL("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🔣️policy.json",import.meta.url));
 console.log("current",await ownershipMemoCurrentV1(m,root,impl!,new AbortController().signal),"impl equal",impl===m.implementation);
}
