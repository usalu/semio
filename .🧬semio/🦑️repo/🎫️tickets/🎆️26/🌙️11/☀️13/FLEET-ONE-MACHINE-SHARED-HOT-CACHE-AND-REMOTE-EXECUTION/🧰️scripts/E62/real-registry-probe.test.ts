import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
const reads=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-reads/🟨️.mjs");
test("real registry: cost and key law",()=>{
 const registry=JSON.parse(readFileSync("/Users/ueli/Documents/semio/📋️build-owners.json","utf8"));
 let t=performance.now();const digest=reads.ownerRegistryDigestV1(registry);const digestMs=performance.now()-t;
 t=performance.now();for(let i=0;i<20000;i++)reads.answerOwnerRegistryQuestionV1(registry,["within","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"+i]);const withinMs=performance.now()-t;
 const edited=structuredClone(registry);edited.owners[100].packages=[...edited.owners[100].packages,"extra"];
 const rows=registry.owners.map((o:any)=>reads.ownerRowDigestV1(o)),rows2=edited.owners.map((o:any)=>reads.ownerRowDigestV1(o));
 const moved=rows.map((d:string,i:number)=>d!==rows2[i]?registry.owners[i].name:null).filter(Boolean);
 console.log(JSON.stringify({owners:registry.owners.length,digestMs:Math.round(digestMs),within20kMs:Math.round(withinMs),moved,digestMoved:reads.ownerRegistryDigestV1(edited)!==digest}));
 expect(moved.length).toBe(1);
});
