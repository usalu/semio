/** 🖼️ Contract: a full cache image is a typed, retried, bilingual refusal and never a stuck worker. Before a stage starts, an image that lacks its need is collected in place by the collectors below its mount (the rest of the machine is left alone); when that is not enough the worker says `image-short` (purpose, size, what is free, the largest consumers) and looks again after five minutes or when it gained 256 MB; a stage that dies with `No space left on device` inside an image is classified `image-full` from the compiler's own words, keeps the short back-off and names the image in both languages.
 * @see ../../🖼️image-guard/🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_POLICY_V1 as P,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyImageFullFieldsV1,readyImageFullLineV1,readyImageShortRefusalV1,readyRefusalV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1}from"../../🟦️.ts";
import{READY_IMAGE_POLICY_V1,readyImageHostLimitedV1,readyImageShortV1,readyImageWritersV1}from"../../🖼️image-guard/🟦️.ts";
import{DISK_WRITERS_V1,recoverNeedV1,type DiskWriterV1}from"../../../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576,MOUNT="/ws/.cache/b/r/m",TOTAL=40959;
type Volume={freeMegabytes:number};
const rowOf=(id:string)=>DISK_WRITERS_V1.find(row=>row.id===id)!;
const writer=(volume:Volume,id:string,root:string,holds:number,gives:number,log:string[]):DiskWriterV1=>({row:rowOf(id),root,measure:()=>holds*MB,collect:async()=>{log.push(id);volume.freeMegabytes+=gives;}});
const imageOf=(volume:Volume,host:number=100_000)=>({partition:"r",path:MOUNT,freeBytes:volume.freeMegabytes*MB,totalBytes:TOTAL*MB,hostFreeBytes:host*MB});

function world(volume:Volume,writers:readonly DiskWriterV1[],options:Readonly<{stopAfterSleeps?:number;external?:(round:number)=>void;results?:Readonly<Record<string,readonly Partial<ReadyRunResultV1>[]>>}>={}){
 const shared={state:emptyReadyStateV1() as ReadyLocalStateV1,marker:false,now:1_000_000},controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],calls:string[]=[];
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)}),directory=mkdtempSync(join(tmpdir(),"ready-image-"));scratch.push(directory);
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;options.external?.(sleeps.length);await new Promise<void>(done=>setTimeout(done,1));if(sleeps.length>=(options.stopAfterSleeps??40)||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  disk:name=>name==="workspace"?readyImageShortV1([imageOf(volume)]):null,
  recover:async request=>{
   const reading=()=>({partition:request.short.path,totalBytes:TOTAL*MB,freeBytes:volume.freeMegabytes*MB});
   const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*MB,writers:readyImageWritersV1(writers,request.short.path),language:"both",report:request.report,signal:request.signal});
   return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
  },
  run:async row=>{calls.push(row.name);const planned=options.results?.[row.name]?.[calls.filter(name=>name===row.name).length-1];return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[],...planned} as ReadyRunResultV1;},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
 };
 return{ports,controller,sleeps,said,updates,calls,shared};
}

test("47 MB free of 40959 MB in the Rust image: only the collectors below its mount run, in place, the stage starts when the need is free",async()=>{
 const volume={freeMegabytes:47},log:string[]=[];
 const w=world(volume,[writer(volume,"cargo-incremental",MOUNT+"/cargo",9000,5000,log),writer(volume,"cas-lru","/home/.cache/semio/cas",900,600,log),writer(volume,"image-slack","/ws/.cache/b",30_000,9000,log),writer(volume,"tool-generations","/ws/.cache/b/t/m/nx",9000,9000,log)]);
 await runReadyV1(w.ports,w.controller.signal);
 expect(log).toEqual(["cargo-incremental"]);
 expect(volume.freeMegabytes).toBe(5047);
 expect(w.calls).toContain("workspace");
 const collecting=w.updates.find(update=>update.refusal?.code==="disk-collecting")!;
 expect(collecting.refusal!.en).toContain(MOUNT);
 expect(collecting.refusal!.de).toContain(MOUNT);
});

test("an image that stays short is told as image-short in both languages with its purpose and consumers, never as unfit for six hours, and the stage starts once space returns",async()=>{
 const volume={freeMegabytes:47},log:string[]=[];
 const w=world(volume,[writer(volume,"cargo-incremental",MOUNT+"/cargo",9000,0,log)],{stopAfterSleeps:8,external:round=>{if(round===6)volume.freeMegabytes=47+READY_IMAGE_POLICY_V1.recheckGrowthMegabytes;if(round===7)volume.freeMegabytes=5000;}});
 await runReadyV1(w.ports,w.controller.signal);
 const refusals=w.updates.map(update=>update.refusal).filter(refusal=>refusal?.code==="image-short");
 expect(refusals.length).toBeGreaterThanOrEqual(2);
 for(const refusal of refusals)for(const text of[refusal!.en,refusal!.de]){expect(text).toContain(MOUNT);expect(text).toContain("40959");expect(text).toContain("4096");expect(text).not.toContain("{");}
 expect(refusals[0]!.en).toContain("Rust build");expect(refusals[0]!.de).toContain("Rust-Build");
 expect(refusals[0]!.en).toContain(rowOf("cargo-incremental").command);
 expect(w.updates.some(update=>update.refusal?.code==="disk-unfit"||update.refusal?.code==="disk-consumers")).toBe(false);
 expect(log.length).toBeGreaterThanOrEqual(2);
 expect(w.calls).toContain("workspace");
 expect(P.unfitRecheckMilliseconds).toBeGreaterThan(READY_IMAGE_POLICY_V1.recheckMilliseconds*10);
});

test("a stage that dies with No space left on device inside an image is image-full from the compiler's own words, keeps the short back-off and names the image",async()=>{
 const cargo=["error: could not parse/generate dep info at: /Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/out/x.d","Caused by:","  failed to write `/Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/fingerprint/dep-lib-x`","Caused by:","  No space left on device (os error 28)"];
 const volume={freeMegabytes:9000};
 const w=world(volume,[],{stopAfterSleeps:3,results:{workspace:[{exit:1,tail:cargo}]}});
 await runReadyV1(w.ports,w.controller.signal);
 const failure=w.shared.state.failure!;
 expect(failure.code).toBe("image-full");
 expect(failure.nextAttemptMs-failure.atMs).toBe(P.backoffInitialMilliseconds);
 expect(failure.tail.some(line=>line.includes("Rust build")&&line.includes("Rust-Build"))).toBe(true);
 const refusal=w.updates.map(update=>update.refusal).find(item=>item?.code==="image-full")!;
 expect(refusal.en).toContain("Rust build");expect(refusal.de).toContain("Rust-Build");
 expect(refusal.en).toContain(".cache/b/r/m");expect(refusal.de).toContain(".cache/b/r/m");
 for(const text of[refusal.en,refusal.de])expect(text).not.toContain("{");
 expect(P.refreshKeepingFailureCodes).toContain("image-full");
});

test("the typed texts of a full image resolve in both languages and agree with the line the failed stage keeps",()=>{
 const fields=readyImageFullFieldsV1([readyImageFullLineV1("g")]);
 expect(fields.purpose.en).toBe("project graph");expect(fields.purpose.de).toBe("Projektgraph");expect(fields.path).toBe(".cache/b/g/m");
 const full=readyRefusalV1("image-full",{stage:"workspace",...fields});
 expect(full.en).toContain("project graph");expect(full.de).toContain("Projektgraph");
 const short=readyImageShortRefusalV1("dependencies",{path:"/ws/.cache/b/d/m",resource:"space",needed:3686,available:100,partition:"d",total:36863,host:100,limitedByHost:true},[{id:"dependency-generations",root:"/ws/.cache/b/d/m/j",bytes:5*1073741824,command:"repo cache gc --area=dependencies"}]);
 expect(short.code).toBe("image-short");
 expect(short.en).toContain("dependencies");expect(short.de).toContain("Abhängigkeiten");
 expect(short.en).toContain("5120 MB");expect(short.en).toContain("the host volume has 100 MB free");expect(short.de).toContain("das Wirtsvolumen hat 100 MB frei");expect(short.en).toContain("those of the host volume");expect(short.de).toContain("Wirtsvolumens");
 const roomy=readyImageShortRefusalV1("dependencies",{path:"/ws/.cache/b/d/m",resource:"space",needed:3686,available:100,partition:"d",total:36863,host:90_000,limitedByHost:false});
 expect(roomy.en).not.toContain("those of the host volume");expect(short.de).toContain("repo cache gc --area=dependencies");
 for(const text of[short.en,short.de])expect(text).not.toContain("{");
 expect(readyImageFullLineV1("r")).toContain("No space left on device");
});

test("an image that shows the free bytes of the host is limited by the host and is told so, an image with more room than the host is not a thin one",()=>{
 const image=(free:number,host:number|null)=>({partition:"r",path:MOUNT,freeBytes:free*MB,totalBytes:TOTAL*MB,hostFreeBytes:host===null?null:host*MB});
 expect(readyImageHostLimitedV1(image(2000,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1985,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1975,2000))).toBe(false);
 expect(readyImageHostLimitedV1(image(500,2000))).toBe(false);
 expect(readyImageHostLimitedV1(image(47,19741))).toBe(false);
 expect(readyImageHostLimitedV1(image(2000,null))).toBe(false);
 expect(readyImageShortV1([image(2000,2000)])).toMatchObject({limitedByHost:true,host:2000,available:2000});
 expect(readyImageShortV1([image(200,50_000)])).toMatchObject({limitedByHost:false,host:50_000});
});
