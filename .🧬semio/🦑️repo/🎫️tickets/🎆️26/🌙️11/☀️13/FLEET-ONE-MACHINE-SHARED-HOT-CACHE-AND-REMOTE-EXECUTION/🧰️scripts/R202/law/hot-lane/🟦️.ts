/** 🔥️ Contract: a take pass puts the owners that the pools of the fleet say they stored and that are not stored here in a lane of their own, first, so their looks find a record before the thousands of other looks; without a list the lanes are the two of before, and every name is in exactly one lane in its order.
 * @see ../../🟨️.mjs `takeLanesV1` */
import{expect,test}from"bun:test";
import{takeLanesV1}from"../../🟨️.mjs";
import{lookBatchesV1}from"../../../🔭️look-queue/🟨️.mjs";

const names=Array.from({length:12},(_,i)=>"n"+i);
test("published owners that are not present form the first lane",()=>{
 const present=new Set(["n0","n1","n2","n3"]),published=new Set(["n2","n5","n9","n11"]);
 const lanes=takeLanesV1(names,name=>present.has(name),name=>published.has(name));
 expect(lanes).toEqual([["n5","n9","n11"],["n0","n1","n2","n3"],["n4","n6","n7","n8","n10"]]);
 expect(lanes.flat().sort()).toEqual([...names].sort());
});
test("without a list the two lanes are kept and empty lanes are left out",()=>{
 const present=new Set(["n0","n1"]);
 expect(takeLanesV1(names,name=>present.has(name))).toEqual([["n0","n1"],names.slice(2)]);
 expect(takeLanesV1(names,()=>true,()=>true)).toEqual([names]);
 expect(takeLanesV1(names,()=>false,()=>true)).toEqual([names]);
});
test("the first look jobs hold the hot owners",()=>{
 const lanes=takeLanesV1(names,name=>name==="n0",name=>["n7","n8","n9"].includes(name));
 expect(lookBatchesV1(lanes,2)[0]).toEqual(["n7","n8"]);
});
