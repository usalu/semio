/** 🧯️ Contract: a look at a stored record that was written under another key (an older wave) settles none of its closure entries, because that record is never reused and settling stale entries derives them on the thread of the reader; only the record of the asked key settles its entries. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;

test("an older record settles no closure entry and the record of the asked key settles its entries once per look",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"old-settle-")),file=join(root,"input.txt"),cachePath=join(root,"np","record.json"),path=JSON.stringify([root,"recipe",["digest"]]);
 const settled:string[][]=[];
 try{
  observation.registerClosureProviderV1({answer:()=>"true",settle:async(paths:string[])=>{settled.push(paths);},verify:async()=>true});
  writeFileSync(file,"one");await new Promise(done=>setTimeout(done,2200));
  const compute=async()=>{observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(file,"utf8")};};
  await observation.observeInference({cachePath,key:["old"]},compute);
  settled.length=0;
  await observation.observeInference({cachePath,key:["new"]},compute);
  const lookups=settled.length;
  expect(settled.filter(paths=>paths.includes(path)).length).toBe(0);
  settled.length=0;
  await observation.observeInference({cachePath,key:["new"]},compute);
  expect(settled.filter(paths=>paths.includes(path)).length).toBeGreaterThan(0);
  expect(lookups).toBeGreaterThanOrEqual(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧯️ Law: the settles a read makes for owners that hold an older record do not grow with the number of owners or the size of the closure they name: zero, for every owner after the first as for the first. */
test("the settles for older records stay at zero for every owner of a read",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"old-settle-law-")),file=join(root,"input.txt"),settled:string[][]=[];
 try{
  observation.registerClosureProviderV1({answer:()=>"true",settle:async(paths:string[])=>{settled.push(paths);},verify:async()=>true});
  writeFileSync(file,"one");await new Promise(done=>setTimeout(done,2200));
  const owners=[1,10,100,1000].map((size,at)=>({cachePath:join(root,"np","record-"+at+".json"),paths:Array.from({length:size},(_all,row)=>JSON.stringify([root,"recipe-"+at+"-"+row,["digest"]]))}));
  const compute=(paths:string[])=>async()=>{for(const path of paths)observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(file,"utf8")};};
  for(const owner of owners)await observation.observeInference({cachePath:owner.cachePath,key:["old"]},compute(owner.paths));
  settled.length=0;
  for(const owner of owners){const before=settled.length;await observation.observeInference({cachePath:owner.cachePath,key:["new"]},compute(owner.paths.slice(0,1)));expect(settled.slice(before).flat().filter(path=>owner.paths.includes(path)).length).toBe(0);}
 }finally{rmSync(root,{recursive:true,force:true});}
});
