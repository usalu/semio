
test("a record younger than the grace period is never swept, even when the store is past its maximum",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-store-young-"));
 try{
  const directory=join(root,"young"),earlier=createClosureStoreV1({directory});
  for(let index=0;index<5;index++)await earlier.save("entry-young"+index,{recipe:{fn:"fixture"},value:[index],rows:[],layers:[[{kind:"file",path:"/layer/young"+index,fact:{size:index}}]]});
  const later=createClosureStoreV1({directory,limits:{...INFERENCE_LIMITS_V1,closureStoreMaximumEntries:3,closureStoreMaximumLayers:3,closureStoreMaximumValues:3}});
  await later.save("entry-younglater",{recipe:{fn:"fixture"},value:[9],rows:[],layers:[[{kind:"file",path:"/layer/younglater",fact:{size:9}}]]});await later.settled();
  expect(readdirSync(join(directory,"e")).length,"entries").toBe(6);expect(readdirSync(join(directory,"l")).length,"layers").toBe(6);expect(readdirSync(join(directory,"v")).length,"values").toBe(6);
 }finally{rmSync(root,{recursive:true,force:true});}
});
