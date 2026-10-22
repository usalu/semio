import{test,expect,spyOn}from"bun:test";
import*as fs from"node:fs";
import{mkdtempSync,mkdirSync,readdirSync,realpathSync,rmSync,readFileSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{openRecordStore,recordKey}from"../../../../⚡️caching/🗃️records/🟦️.ts";
import{withBuildGraphWorkerWriterV1}from"../../../📥️inputs/💾️storage/🟨️.mjs";
import{readInferenceHeaderV1,analyzeSourceRecordV1,canonicalObservationRowV1,currentProjectRecordDigestV1,currentProjectRecordDirectoryV1,observationRowDigestV1,preimagedProjectValueV1,sealInferenceRecordV1,memoizedRecord,missingSourceAnalysisRecordsV1,observeDependencies,observeDetachedProjectInferenceV1,observeInferenceSnapshotV1,observeProjectInferenceV1,observedFs,observedSourceAnalysisV1,projectInferenceObservationsV1,putSourceAnalysisRecordsV1,registerSourceAnalysisV1,replayDetachedProjectInferenceV1,writeSealedBlocksV1}from"../../🟨️.mjs";

const rowKey=(row:{kind:string;path:string;fact:unknown})=>row.kind+"\0"+row.path+"\0"+JSON.stringify(row.fact);

/** 🧪️ A directory of files the computations read; they count as settled once the racy window of two seconds has passed. */
function workspace(){
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"detached-law-")),old=new Date(Date.now()-3600000);
 for(const [name,text] of [["a.txt","alpha"],["b.txt","beta"]]){const path=join(directory,name);writeFileSync(path,text);utimesSync(path,old,old);}
 mkdirSync(join(directory,"np"));
 return directory;
}

test("a project computed detached and replayed into the observing process persists the record the direct computation persists",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8"),later:observedFs.existsSync(join(directory,"missing.txt")),size:observedFs.statSync(join(directory,"b.txt")).size,memo:memoizedRecord("detached-law",["k"],()=>({computed:true}))});
  const puts:[string,Uint8Array][]=[],direct=join(directory,"np","direct.json"),farmed=join(directory,"np","farmed.json");
  let valueDirect:unknown,valueFarmed:unknown;
  await observeInferenceSnapshotV1(async()=>{
   valueDirect=await observeProjectInferenceV1({cachePath:direct,key:["law"],owner:"p",reusable:()=>true},compute);
   valueFarmed=await observeProjectInferenceV1({cachePath:farmed,key:["law"],owner:"p",reusable:()=>true},async()=>{
    const detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:{get:()=>undefined,put:(key:string,bytes:Uint8Array)=>{puts.push([key,bytes]);}}},compute);
    replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),puts);
    return detached.value;
   });
  });
  expect(valueFarmed).toEqual(valueDirect as never);
  const rowsDirect=projectInferenceObservationsV1(direct)!,rowsFarmed=projectInferenceObservationsV1(farmed)!;
  expect(rowsDirect.length).toBeGreaterThan(2);
  expect(rowsFarmed.map(rowKey)).toEqual(rowsDirect.map(rowKey));
  expect(readFileSync(farmed).equals(readFileSync(direct))).toBe(true);
  expect(puts.length).toBe(1);
  const store=openRecordStore(farmed+".records");
  try{expect(Buffer.from(store.get(puts[0]![0])!).toString("utf8")).toBe(JSON.stringify({computed:true}));}finally{store.close();}
  expect(recordKey("detached-law","k")).toBe(puts[0]![0]);
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a detached computation persists nothing and consumes no fact of its caller",async()=>{
 const directory=workspace();
 try{
  await observeInferenceSnapshotV1(async()=>{
   const detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:{get:()=>undefined,put:()=>{}}},()=>observedFs.readFileSync(join(directory,"a.txt"),"utf8"));
   expect(detached.value).toBe("alpha");
   expect(detached.rows.some(row=>row.kind==="file"&&row.path.endsWith("a.txt"))).toBe(true);
   expect(detached.started).toBeLessThanOrEqual(Date.now());
  });
  const written=Bun.spawnSync(["find",join(directory,"np"),"-type","f"]);
  expect(new TextDecoder().decode(written.stdout).trim()).toBe("");
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a value without a language-neutral JSON image is refused",async()=>{
 await observeInferenceSnapshotV1(async()=>{
  await expect(observeDetachedProjectInferenceV1({validation:new Map(),records:{get:()=>undefined,put:()=>{}}},()=>undefined)).rejects.toThrow("language-neutral JSON image");
 });
},60000);

test("parser records prepared by another process are found by the analysis that follows, which then parses nothing",async()=>{
 const directory=workspace();
 try{
  let calls=0;
  const provider=registerSourceAnalysisV1({id:"detached-law",revision:"a".repeat(64),analyze:(_path:string,bytes:Buffer)=>{calls++;return {length:bytes.length};}});
  const files=[join(directory,"a.txt"),join(directory,"b.txt")];
  await observeDependencies({receiptPath:join(directory,"np","law.receipt.json"),recordDirectory:join(directory,"mr")},async()=>{
   expect(missingSourceAnalysisRecordsV1(provider,[...files,join(directory,"absent.txt")])).toEqual(files);
   const prepared=files.map(file=>analyzeSourceRecordV1(provider,file));
   expect(calls).toBe(2);
   expect(putSourceAnalysisRecordsV1(prepared.map(row=>[row.key,row.bytes]))).toBe(2);
   expect(missingSourceAnalysisRecordsV1(provider,files)).toEqual([]);
   expect(observedSourceAnalysisV1(provider,files[0]!)).toEqual({length:5});
   expect(observedSourceAnalysisV1(provider,files[1]!)).toEqual({length:4});
   expect(calls).toBe(2);
  });
  expect(putSourceAnalysisRecordsV1([["0".repeat(64),new Uint8Array(1)]])).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a fact is named by the digest of its exact content, whichever process read it",()=>{
 const row={kind:"file" as const,path:"/world/a.ts",fact:{size:3,mtimeMs:1,ctimeMs:2,ino:4,sha256:"a".repeat(64)}};
 const same=canonicalObservationRowV1(structuredClone(row)),again=canonicalObservationRowV1(structuredClone(row));
 expect(again).toBe(same);
 expect(observationRowDigestV1(same)).toBe(observationRowDigestV1(structuredClone(row) as never));
 expect(observationRowDigestV1(same)).toMatch(/^[0-9a-f]{32}$/u);
 expect(observationRowDigestV1({...row,fact:{...row.fact,sha256:"b".repeat(64)}} as never)).not.toBe(observationRowDigestV1(same));
 expect(observationRowDigestV1({...row,path:"/world/b.ts"} as never)).not.toBe(observationRowDigestV1(same));
 expect(observationRowDigestV1({...row,kind:"exists",fact:true} as never)).not.toBe(observationRowDigestV1(same));
},60000);

/** 🗂️ Hashes every regular file below a directory by its relative name, leaving out the parser record stores of the projects. */
function tree(root:string,prefix=""):Record<string,string>{
 const out:Record<string,string>={};
 for(const name of readdirSync(root).sort()){
  const path=join(root,name);
  if(statSync(path).isDirectory())Object.assign(out,tree(path,prefix+name+"/"));
  else if(!name.endsWith(".json.records")&&!name.includes(".tmp"))out[prefix+name]=Bun.hash(readFileSync(path)).toString(16);
 }
 return out;
}

test("a record sealed in the worker is written byte for byte as the record the observing process builds itself, its blocks (written by the worker) included, and falls back when a block it was told it has is missing",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8"),b:observedFs.existsSync(join(directory,"b.txt")),size:observedFs.statSync(join(directory,"b.txt")).size});
  const shim={get:()=>undefined,put:()=>{}};
  const sealedRun=async(cachePath:string,known:Set<string>)=>observeProjectInferenceV1({cachePath,key:["seal"],owner:"p",reusable:()=>true},async()=>{
   const digest=currentProjectRecordDigestV1()!,detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:shim},compute);
   const sealed=sealInferenceRecordV1({digest,image:detached.image,rows:detached.rows,known,emitted:new Set()}),created=writeSealedBlocksV1(currentProjectRecordDirectoryV1()!,sealed),{blocks:_blocks,...rest}=sealed;
   replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),[],created===undefined?undefined:rest);
   return preimagedProjectValueV1(detached.image);
  });
  for(const name of ["direct","sealed","missing"])mkdirSync(join(directory,name));
  await observeInferenceSnapshotV1(async()=>{
   const direct=await observeProjectInferenceV1({cachePath:join(directory,"direct","p.json"),key:["seal"],owner:"p",reusable:()=>true},compute);
   expect(await sealedRun(join(directory,"sealed","p.json"),new Set())).toEqual(direct as never);
   const sealedTree=tree(join(directory,"sealed")),directTree=tree(join(directory,"direct"));
   expect(Object.keys(directTree).length).toBeGreaterThan(1);
   expect(sealedTree).toEqual(directTree);
   const blocks=Object.keys(directTree).flatMap(name=>/([a-f0-9]{64})\.br$/u.exec(name)?.[1]??[]);
   expect(blocks.length).toBeGreaterThan(0);
   expect(await sealedRun(join(directory,"sealed","q.json"),new Set(blocks))).toEqual(direct as never);
   expect(readFileSync(join(directory,"sealed","q.json")).equals(readFileSync(join(directory,"direct","p.json")))).toBe(true);
   expect(await sealedRun(join(directory,"missing","p.json"),new Set(blocks))).toEqual(direct as never);
   expect(tree(join(directory,"missing"))).toEqual(directTree);
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a sealed record is used only for the digest and the facts it was built for, and a block leaf that changed is read and validated again",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8"),size:observedFs.statSync(join(directory,"b.txt")).size}),shim={get:()=>undefined,put:()=>{}};
  for(const name of ["direct","wrong","short"])mkdirSync(join(directory,name));
  await observeInferenceSnapshotV1(async()=>{
   const direct=await observeProjectInferenceV1({cachePath:join(directory,"direct","p.json"),key:["seal"],owner:"p",reusable:()=>true},compute);
   const directTree=tree(join(directory,"direct"));
   const forged=async(name:string,change:(sealed:ReturnType<typeof sealInferenceRecordV1>)=>object)=>observeProjectInferenceV1({cachePath:join(directory,name,"p.json"),key:["seal"],owner:"p",reusable:()=>true},async()=>{
    const digest=currentProjectRecordDigestV1()!,detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:shim},compute),original=sealInferenceRecordV1({digest,image:detached.image,rows:detached.rows,known:new Set(),emitted:new Set()});
    writeSealedBlocksV1(currentProjectRecordDirectoryV1()!,original);
    replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),[],change(original) as never);
    return preimagedProjectValueV1(detached.image);
   });
   expect(await forged("wrong",sealed=>({...sealed,digest:"0".repeat(64),packed:Buffer.from("not a record")}))).toEqual(direct as never);
   expect(tree(join(directory,"wrong"))).toEqual(directTree);
   expect(await forged("short",sealed=>({...sealed,rowCount:sealed.rowCount+1,packed:Buffer.from("not a record")}))).toEqual(direct as never);
   expect(tree(join(directory,"short"))).toEqual(directTree);
   const blocks=Object.keys(directTree).filter(name=>name!=="p.json");
   expect(blocks.length).toBeGreaterThan(0);
   expect(readInferenceHeaderV1(join(directory,"direct","p.json")).observations.length).toBeGreaterThan(1);
   const leaf=join(directory,"direct",blocks[0]!);
   const old=new Date(Date.now()-3600000);
   writeFileSync(leaf,Buffer.from("changed"));utimesSync(leaf,old,old);
   expect(()=>readInferenceHeaderV1(join(directory,"direct","p.json"))).toThrow();
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("the blocks of a sealed record are written once, content addressed, only when they are missing, and a declined grant makes the seal unusable",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8"),size:observedFs.statSync(join(directory,"b.txt")).size}),shim={get:()=>undefined,put:()=>{}};
  for(const name of ["worker","small","scratch"])mkdirSync(join(directory,name));
  await observeInferenceSnapshotV1(async()=>{
   let sealed:ReturnType<typeof sealInferenceRecordV1>|undefined;
   await observeProjectInferenceV1({cachePath:join(directory,"scratch","p.json"),key:["seal"],owner:"p",reusable:()=>true},async()=>{
    const digest=currentProjectRecordDigestV1()!,detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:shim},compute);
    sealed=sealInferenceRecordV1({digest,image:detached.image,rows:detached.rows,known:new Set(),emitted:new Set()});
    expect(currentProjectRecordDirectoryV1()).toBe(join(directory,"scratch"));
    replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),[]);
    return preimagedProjectValueV1(detached.image);
   });
   expect(sealed!.blocks.length).toBeGreaterThan(0);
   const roots={inference:join(directory,"worker")};
   let report:any;
   const created=withBuildGraphWorkerWriterV1({roots,grantBytes:1<<20},writer=>{const made=writeSealedBlocksV1(join(directory,"worker"),sealed!);report=writer.report();return made;});
   expect(created).toEqual(sealed!.blocks.map(block=>block.digest));
   expect(report.leaves.map((leaf:{name:string})=>leaf.name).sort()).toEqual(sealed!.blocks.map(block=>"ob/"+block.digest+".br").sort());
   for(const block of sealed!.blocks)expect(readFileSync(join(directory,"worker","ob",block.digest+".br")).equals(Buffer.from(block.packed))).toBe(true);
   const again=withBuildGraphWorkerWriterV1({roots,grantBytes:1<<20},writer=>{const made=writeSealedBlocksV1(join(directory,"worker"),sealed!);report=writer.report();return made;});
   expect(again).toEqual([]);expect(report.leaves).toEqual([]);
   const declined=withBuildGraphWorkerWriterV1({roots:{inference:join(directory,"small")},grantBytes:1},writer=>{const made=writeSealedBlocksV1(join(directory,"small"),sealed!);report=writer.report();return made;});
   expect(declined).toBeUndefined();expect(report.declined.length).toBe(1);expect(Bun.spawnSync(["find",join(directory,"small"),"-type","f"]).stdout.toString().trim()).toBe("");
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("inside a writer scope a block that was verified is not probed again, outside one every record probes the blocks it names",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8"),size:observedFs.statSync(join(directory,"b.txt")).size}),shim={get:()=>undefined,put:()=>{}},inference=join(directory,"graph");
  mkdirSync(join(inference,"np"),{recursive:true});
  const name=(n:number)=>join(inference,"np",n.toString(16).padStart(24,"0")+".json");
  const sealedRun=(path:string,known:Set<string>)=>observeProjectInferenceV1({cachePath:path,key:["seal"],owner:"p",reusable:()=>true},async()=>{
   const digest=currentProjectRecordDigestV1()!,detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:shim},compute),sealed=sealInferenceRecordV1({digest,image:detached.image,rows:detached.rows,known,emitted:new Set()}),{blocks:_blocks,...rest}=sealed;
   replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),[],rest);
   return preimagedProjectValueV1(detached.image);
  });
  const realLstat=fs.lstatSync,probes=()=>{const seen:string[]=[],lstat=spyOn(fs,"lstatSync").mockImplementation(((...args:any[])=>{seen.push(String(args[0]));return(realLstat as any)(...args);})as any);return{seen,restore:()=>lstat.mockRestore()};};
  await observeInferenceSnapshotV1(async()=>{
   await observeProjectInferenceV1({cachePath:name(1),key:["seal"],owner:"p",reusable:()=>true},compute);
   const blocks=readdirSync(join(inference,"np","ob")).map(file=>file.replace(/\.br$/u,""));
   expect(blocks.length).toBeGreaterThan(0);
   await withBuildGraphWorkerWriterV1({roots:{inference},grantBytes:1<<24},async()=>{
    await sealedRun(name(2),new Set(blocks));
    const spy=probes();
    try{await sealedRun(name(3),new Set(blocks));}finally{spy.restore();}
    expect(spy.seen.filter(path=>path.includes("/ob/"))).toEqual([]);
    expect(readFileSync(name(3)).byteLength).toBeGreaterThan(0);
   });
   await withBuildGraphWorkerWriterV1({roots:{inference},grantBytes:1<<24},async()=>{
    const spy=probes();
    try{await sealedRun(name(4),new Set(blocks));}finally{spy.restore();}
    expect(spy.seen.filter(path=>path.includes("/ob/")).length).toBeGreaterThan(0);
   });
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a block another writer finished between the check and the rename is kept as it is, its temporary file goes, and nothing is reported as created by this worker",async()=>{
 const directory=workspace();
 try{
  await new Promise(done=>setTimeout(done,2200));
  const compute=()=>({a:observedFs.readFileSync(join(directory,"a.txt"),"utf8")}),shim={get:()=>undefined,put:()=>{}};
  for(const name of ["scratch","shared"])mkdirSync(join(directory,name));
  await observeInferenceSnapshotV1(async()=>{
   let sealed:ReturnType<typeof sealInferenceRecordV1>|undefined;
   await observeProjectInferenceV1({cachePath:join(directory,"scratch","p.json"),key:["seal"],owner:"p",reusable:()=>true},async()=>{
    const digest=currentProjectRecordDigestV1()!,detached=await observeDetachedProjectInferenceV1({validation:new Map(),records:shim},compute);
    sealed=sealInferenceRecordV1({digest,image:detached.image,rows:detached.rows,known:new Set(),emitted:new Set()});
    replayDetachedProjectInferenceV1(detached.rows.map(canonicalObservationRowV1),[]);
    return preimagedProjectValueV1(detached.image);
   });
   const roots={inference:join(directory,"shared")},first=sealed!.blocks[0]!,leaf=join(directory,"shared","ob",first.digest+".br");
   withBuildGraphWorkerWriterV1({roots,grantBytes:1<<20},()=>writeSealedBlocksV1(join(directory,"shared"),{blocks:[first]}));
   const before=readFileSync(leaf),stamp=statSync(leaf).ino;
   const exists=spyOn(fs,"existsSync").mockImplementationOnce((()=>false)as never);
   let report:any;
   try{withBuildGraphWorkerWriterV1({roots,grantBytes:1<<20},writer=>{const made=writeSealedBlocksV1(join(directory,"shared"),{blocks:[first]});report=writer.report();return made;});}finally{exists.mockRestore();}
   expect(report.leaves).toEqual([]);expect(report.declined).toEqual([]);
   expect(readFileSync(leaf).equals(before)).toBe(true);expect(statSync(leaf).ino).toBe(stamp);
   expect(readdirSync(join(directory,"shared","ob")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);
