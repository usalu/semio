import {readFile} from "node:fs/promises";
const M="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";
const {openReplicaV1}=await import(`${M}/🔁️replica/🟦️.ts`);
const S=process.argv[2]!;
const machine=JSON.parse(await readFile(`${S}/mir/.cache/f/machine.json`,"utf8"));
const replica=await openReplicaV1({worktree:`${S}/mir`,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
const {manifest,files,book}=replica;
const out:string[]=[];
for(const id of manifest.ids()){const p=book.name(id);if(!p.startsWith(".git/objects/pack/")||!p.endsWith(".idx"))continue;
 const base=p.slice(0,-4);const pk=book.find(base+".pack");
 const pkOp=pk===undefined?"absent":manifest.operationAt(pk);const pkLive=pk===undefined?false:files.live(pk);
 const bad=manifest.operationAt(id)==="put"&&(pk===undefined||pkOp!=="put"||!pkLive);
 if(bad)out.push(`BAD idx ${p.slice(-50)} idxOp=${manifest.operationAt(id)} origin=${book.origins[manifest.origin[id]!]} hlc=${manifest.entryAt(id).hlc.slice(0,13)} pack=${pkOp} live=${pkLive}${pk!==undefined?" packorigin="+book.origins[manifest.origin[pk]!]+" hlc="+manifest.entryAt(pk).hlc.slice(0,13):""}`);}
console.log(out.join("\n")||"none bad");
process.exit(0);
