import {test} from "bun:test";
import {readFileSync} from "node:fs";
test("x",async()=>{
 const m=await import(process.env.OUT!);
 const root="/Users/ueli/Documents/semio";
 const {readModuleTreeSource}=await import("@semio-tech/framework-compiler-ts/syntax/module-tree");
 const splitRoots=["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs"];
 const read=(path:string)=>({path,source:splitRoots.includes(path)?readModuleTreeSource(root+"/"+path):readFileSync(root+"/"+path,"utf8")});
 const bodies=Object.fromEntries(Object.entries(m.memberStoreProducerSourceDeclarationV1.roles).map(([r,ref]:any)=>[r,read(ref.path)]));
 const ok=m.toolJobMemberStoreProducerExact({...bodies,extensions:[]} as any);
 console.log("[RESULT]",ok,"line",(globalThis as any).__f,"\nMISS",JSON.stringify([...new Set((globalThis as any).__miss)],null,1));
});
