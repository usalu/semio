import{expect,test}from"bun:test";
import{projectRowsHoldV1,registerClosureProviderV1}from"../../🟨️.mjs";

const closure={kind:"closure",path:JSON.stringify(["/w","k",["some",["/a/"]]]),fact:"true"},file={kind:"file",path:"/x",fact:{}};

/** 🔎️ A stored record found by a look is judged by its rows: the rows that are not closure rows by the caller's probe, the closure rows by the provider. */
test("the probe sees only the rows that are not closure rows and the provider verifies the closure rows",async()=>{
 const probed:string[][]=[];let verified=0;
 registerClosureProviderV1({answer:()=>"!stale",settle:async()=>undefined,verify:async(rows:readonly unknown[])=>{verified+=rows.length;return true;}} as never);
 expect(await projectRowsHoldV1([file,closure,closure],{},rest=>{probed.push(rest.map(row=>row.kind));return true;})).toBe(true);
 expect(probed).toEqual([["file"]]);expect(verified).toBe(2);
 expect(await projectRowsHoldV1([file,closure],{},()=>false)).toBe(false);expect(verified).toBe(2);
 expect(await projectRowsHoldV1([file],{},()=>true)).toBe(true);expect(verified).toBe(2);
});
