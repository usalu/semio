import{expect,test}from"bun:test";
import{closureCollectionsV1}from"../🟨️.mjs";

/** 🧮️ The sidecar and the digests of the closure entries are structures the entries share: the report declares their size with every shared array charged once, never an estimate from a sample of JSON texts (263 GB for 100 MB). */
test("the shared closure structures report a declared size, not a sample",()=>{
 const rows=closureCollectionsV1() as {name:string;entries:number;bytes?:number;sample?:()=>unknown[]}[];
 for(const name of["closure:entries.digests","closure:entries.sidecar"]){
  const row=rows.find(candidate=>candidate.name===name);
  expect(row,name).toBeDefined();expect(row!.sample,name).toBeUndefined();expect(Number.isInteger(row!.bytes),name).toBe(true);
 }
 expect(rows.some(row=>row.name.startsWith("closure:walkMembers")),"no walk is a member of a layer any more").toBe(false);
});
