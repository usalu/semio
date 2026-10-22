import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{createCargoResolutionContextV1,flushCargoReceiptsV1,readCargoResolutionPolicyV1}from"../../🟦️.ts";
import{encodeCargoReceiptV1}from"../../💾️receipt/🟨️.mjs";
import policyDocument from"../../🔣️.json"with{type:"json"};

const inputs=readFileSync(fileURLToPath(new URL("../../../🟨️.mjs",import.meta.url)),"utf8");

test("a receipt is the same bytes whichever order its parts were derived in",()=>{
 const build={id:"a".repeat(64),root:"",reads:[{path:"Cargo.toml",sha:"b".repeat(64)}]},test={id:"c".repeat(64),root:"",reads:[]},lock={path:"Cargo.lock",sha:"d".repeat(64),basis:"e".repeat(64),digest:"f".repeat(64)};
 const first=encodeCargoReceiptV1({version:1,crate:"crates/a",b:build,t:test,lock}as never),second=encodeCargoReceiptV1({lock,t:test,crate:"crates/a",b:build,version:1}as never);
 expect(Buffer.from(first).equals(Buffer.from(second))).toBe(true);
 expect(Buffer.from(encodeCargoReceiptV1({version:1,crate:"crates/a",b:build}as never)).equals(Buffer.from(first))).toBe(false);
});

test("a crate that turns dirty while the receipts are being written is written by the next flush instead of being forgotten",async()=>{
 const written:string[]=[],context=createCargoResolutionContextV1({read:async()=>null,digest:async()=>null},readCargoResolutionPolicyV1(policyDocument),{read:()=>undefined,write:async(leaf:{crate:string})=>{if(leaf.crate==="a"){context.dirty.add("b");context.leaves.set("b",Promise.resolve({version:1,crate:"b"}as never));}written.push(leaf.crate);}}as never);
 context.dirty.add("a");context.leaves.set("a",Promise.resolve({version:1,crate:"a"}as never));
 expect(await flushCargoReceiptsV1(context)).toBe(1);
 expect(written).toEqual(["a"]);expect([...context.dirty]).toEqual(["b"]);
 expect(await flushCargoReceiptsV1(context)).toBe(1);
 expect(written).toEqual(["a","b"]);expect(context.dirty.size).toBe(0);
});

test("a reader that cannot write keeps its changes in memory and a flush never loses the order of the crates",async()=>{
 const reader=createCargoResolutionContextV1({read:async()=>null,digest:async()=>null},readCargoResolutionPolicyV1(policyDocument),{read:()=>undefined}as never);
 reader.dirty.add("z");reader.leaves.set("z",Promise.resolve({version:1,crate:"z"}as never));
 expect(await flushCargoReceiptsV1(reader)).toBe(0);expect(reader.dirty.size).toBe(0);expect(await reader.leaves.get("z")).toEqual({version:1,crate:"z"});
 const written:string[]=[],writer=createCargoResolutionContextV1({read:async()=>null,digest:async()=>null},readCargoResolutionPolicyV1(policyDocument),{read:()=>undefined,write:async(leaf:{crate:string})=>{written.push(leaf.crate);}}as never);
 for(const crate of["m","c","x","a"]){writer.dirty.add(crate);writer.leaves.set(crate,Promise.resolve({version:1,crate}as never));}
 await flushCargoReceiptsV1(writer);
 expect(written).toEqual(["a","c","m","x"]);
});

test("the receipts of a graph are refreshed after its projects, once per crate, in order, through one resolution run, and a failing crate still writes what it derived",()=>{
 const refresh=inputs.slice(inputs.indexOf("export async function refreshCargoReceiptsV1"),inputs.indexOf("export async function warmCargoResolutionNowV1"));
 expect(refresh).toContain("const key = {};");expect(refresh).toContain("[...manifests].sort()");expect(refresh).toContain("await withBuildGraphOptionalBatchV1(");
 expect(refresh.match(/warmCargoResolutionNowV1\(/gu)?.length).toBe(1);
 const wrapper=inputs.slice(inputs.indexOf("async function warmCargoResolutionV1"),inputs.indexOf("export async function refreshCargoReceiptsV1"));
 expect(wrapper).toContain("if (deferBuildGraphReceiptV1(manifest)) return;");
 const now=inputs.slice(inputs.indexOf("export async function warmCargoResolutionNowV1"));
 expect(now).toContain("try { await cargoResolutionPartsV1(run, manifest, { tolerant: true }); } finally { await flushCargoReceiptsV1(run); }");
 expect(inputs.match(/warmCargoResolutionV1\(/gu)?.length).toBe(2);
});
