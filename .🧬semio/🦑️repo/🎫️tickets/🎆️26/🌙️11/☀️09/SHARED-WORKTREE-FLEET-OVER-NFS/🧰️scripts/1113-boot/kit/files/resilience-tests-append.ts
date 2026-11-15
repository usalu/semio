
test("a sealed snapshot is published to the store with the per-file hashes of the language-agnostic fixture, replaced when damaged, listed, pruned and never half written",async()=>{
 for(const item of fixture.store){
  const root=temp("store-code"),store=temp("store");
  for(const file of item.files)put(root,file.path,file.text);
  const moduleRoot=join(root,"m"),digest=await digestOf(moduleRoot),sealed=await sealCodeV1({root,moduleRoot,digest,digestOf});
  expect(sealed.created,item.name).toBe(true);
  expect((JSON.parse(readFileSync(join(codeDirectoryV1(root),digest,"sealed.json"),"utf8")) as {files:unknown}).files,item.name).toEqual(item.expected);
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:true,reason:null});
  const target=join(storeCodeDirectoryV1(store),digest);
  expect((JSON.parse(readFileSync(join(target,"sealed.json"),"utf8")) as {files:unknown}).files,item.name).toEqual(item.expected);
  for(const file of item.files)expect(readFileSync(join(target,"files",...file.path.split("/")),"utf8")).toBe(file.text);
  expect(await verifyCodeSnapshotV1(target)).toBeNull();
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:false,reason:"present"});
  expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digest]);
  put(store,`v1/bootstrap/code/${digest}/files/${item.expected[0]!.path}`,"damaged");
  expect(await verifyCodeSnapshotV1(target)).toContain("changed");
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:true,reason:null});
  expect(await verifyCodeSnapshotV1(target)).toBeNull();
  put(root,"m/a/🟦️.ts","export const changed=1;\n");
  expect((await publishCodeSnapshotV1({root,store,digest:"0".repeat(64),originId:"g1-m01"})).reason).toContain("local snapshot");
  mkdirSync(join(storeCodeDirectoryV1(store),".tmp-g1-m09-dead"),{recursive:true});
  expect(await listStoreCodeV1(store)).toHaveLength(1);
 }
});

test("old store snapshots are pruned down to the newest ones, but never one that a live origin attests, and old unfinished directories go",async()=>{
 const store=temp("store-prune"),base=storeCodeDirectoryV1(store),digests=["1","2","3","4"].map(n=>n.repeat(64));
 digests.forEach((digest,index)=>{
  put(store,`v1/bootstrap/code/${digest}/sealed.json`,JSON.stringify({version:1,digest,createdMs:1000+index,moduleRoot:"m",files:[]}));
 });
 mkdirSync(join(base,".tmp-old"),{recursive:true});utimesSync(join(base,".tmp-old"),1000,1000);mkdirSync(join(base,".tmp-new"),{recursive:true});
 expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digests[3]!,digests[2]!,digests[1]!,digests[0]!]);
 const removed=await pruneStoreCodeV1(store,2,new Set([digests[0]!]));
 expect(removed).toEqual([digests[1]!]);
 expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digests[3]!,digests[2]!,digests[0]!]);
 expect(existsSync(join(base,".tmp-old"))).toBe(false);expect(existsSync(join(base,".tmp-new"))).toBe(true);
});

test("the boot stage copy is installed through a signature that is written last, and only changes when the stage changes",async()=>{
 const root=temp("stage"),script=join(temp("script"),"script.ts");
 writeFileSync(script,"export const a=1;\n");
 expect(await installBootStageV1(root,script)).toEqual({installed:true});
 expect(await installBootStageV1(root,script)).toEqual({installed:false});
 const signature=JSON.parse(readFileSync(join(root,".cache","f","boot","stage.json"),"utf8")) as {sha256:string;bytes:number};
 requireResilienceShapeV1("BootStage",signature,"Closed boot stage required");
 expect(signature.sha256).toBe(createHash("sha256").update("export const a=1;\n").digest("hex"));
 writeFileSync(script,"export const a=2;\n");
 expect(await installBootStageV1(root,script)).toEqual({installed:true});
 expect(readFileSync(join(root,".cache","f","boot","stage.ts"),"utf8")).toBe("export const a=2;\n");
});

test("the beat of a live daemon is read while the file is fresh and the process exists, and the fallback guard is read only when it is closed",async()=>{
 const root=temp("beat"),now=Date.now(),beat={version:1,pid:process.pid,parent:1,startedMs:now-5000,wallMs:now,phase:"run",base:"/x",source:"snapshot",lagMs:42_000};
 put(root,".cache/f/boot/beat.json",JSON.stringify(beat));
 requireResilienceShapeV1("Beat",beat,"Closed beat required");
 expect(await readBeatV1(root,now)).toEqual(beat);
 expect(await readBeatV1(root,now+30_000)).toBeUndefined();
 put(root,".cache/f/boot/beat.json",JSON.stringify({...beat,pid:2_000_000_000}));
 expect(await readBeatV1(root,now)).toBeUndefined();
 put(root,".cache/f/boot/beat.json",JSON.stringify({...beat,phase:"later"}));
 expect(await readBeatV1(root,now)).toBeUndefined();
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:JSON.stringify({guard:"code-fallback",source:"store",reason:"x"})})).toEqual({guard:"code-fallback",source:"store",reason:"x"});
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:JSON.stringify({guard:"other",source:"store",reason:"x"})})).toBeUndefined();
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:"{"})).toBeUndefined();
 expect(readFallbackV1({})).toBeUndefined();
});

test("a cooperative yield hands the event loop back within its budget while a long loop runs, and costs almost nothing when the work is short",async()=>{
 let worst=0,last=performance.now(),ticks=0;
 const timer=setInterval(()=>{const now=performance.now();worst=Math.max(worst,now-last);last=now;ticks++;},2);
 const turn=yieldingV1(5),end=performance.now()+300;
 let sum=0;
 while(performance.now()<end){for(let index=0;index<2000;index++)sum+=Math.sqrt(index);await turn();}
 clearInterval(timer);
 expect(sum).toBeGreaterThan(0);expect(ticks).toBeGreaterThan(20);expect(worst).toBeLessThan(120);
 const quick=yieldingV1(1000);let paused=0;
 for(let index=0;index<100_000;index++)if(quick()!==undefined)paused++;
 expect(paused).toBe(0);
 let blocked=0,started=performance.now();
 const heavy=yieldingV1(5),blocker=setInterval(()=>{blocked++;},1);
 while(performance.now()-started<100){for(let index=0;index<5000;index++)Math.sqrt(index);await heavy();}
 clearInterval(blocker);
 expect(blocked).toBeGreaterThan(5);
});
