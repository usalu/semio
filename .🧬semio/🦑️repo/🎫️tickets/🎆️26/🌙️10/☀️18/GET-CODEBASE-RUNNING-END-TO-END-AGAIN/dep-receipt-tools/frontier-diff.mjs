import {lstatSync,readFileSync,readdirSync} from "node:fs";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
import {readBunWorkspaceListingProjectionV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
const root=process.cwd(),dir=join(root,".cache/b/d/m/generation");
const name=readdirSync(dir).filter(n=>n.endsWith(".receipt.json")).sort().pop(),receipt=JSON.parse(readFileSync(join(dir,name),"utf8"));
const rows=readFileSync(join(dir,receipt.membership.frontier.name),"utf8").split("\n").filter(Boolean).map(l=>JSON.parse(l));
const digest=r=>createHash("sha256").update(JSON.stringify(r.map(x=>({name:x.name,kind:x.kind})))).digest("hex").slice(0,32);
let moved=0,relistOk=0,changed=[],gone=[],stateChanged=[];
for(const row of rows){
 const path=resolve(root,row[1]),f=lstatSync(path,{bigint:true,throwIfNoEntry:false});
 if(row[0]==="s"){const kind=!f?"missing":f.isSymbolicLink()?"symlink":f.isDirectory()?"directory":f.isFile()?"file":"other";const live=f?[kind,String(f.dev),String(f.ino),Number(f.mode)]:["missing"];if(JSON.stringify(live)!==JSON.stringify(row.slice(2)))stateChanged.push(row[1]);continue;}
 if(!f||!f.isDirectory()){gone.push(row[1]);continue;}
 if(row[7]===1&&String(f.dev)===row[2]&&String(f.ino)===row[3]&&Number(f.mode)===row[4]&&String(f.mtimeNs)===row[5]&&String(f.ctimeNs)===row[6])continue;
 moved++;
 if(digest(readBunWorkspaceListingProjectionV1(root,path))===row[8])relistOk++;else changed.push(row[1]);
}
console.log(JSON.stringify({receipt:name.slice(0,24),rows:rows.length,directoriesTouched:moved,relistEqual:relistOk,listingChanged:changed.slice(0,40),listingChangedCount:changed.length,gone,stateChanged:stateChanged.slice(0,20)},null,1));
