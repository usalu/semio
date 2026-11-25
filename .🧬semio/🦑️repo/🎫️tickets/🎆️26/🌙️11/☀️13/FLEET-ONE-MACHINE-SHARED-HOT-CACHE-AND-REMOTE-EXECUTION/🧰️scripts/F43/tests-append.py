import sys,os,tempfile
p=sys.argv[1]+"/🧪️tests/🚦️no-progress/🟦️.ts"
s=open(p,encoding="utf8").read()
assert "takeLoadStretchV1" not in s
s=s.replace("createProgressWatchV1,takeNoProgressTextV1,takeStrongMachineV1}from","createProgressWatchV1,takeNoProgressTextV1,takeStrongMachineV1,takeLoadStretchV1,takeStopVerdictV1,takeQuarantineReadV1,takeQuarantineWriteV1,takeNoProgressWindowV1}from",1)
s=s.replace('import{expect,test}from"bun:test";','import{expect,test}from"bun:test";\nimport{mkdtempSync,readFileSync,readdirSync,rmSync}from"node:fs";\nimport{tmpdir}from"node:os";\nimport{join}from"node:path";',1)
s+='''
test("the window stretches with the load per core up to the declared maximum and never shrinks below one",()=>{
 expect(takeLoadStretchV1(0.5,10)).toBe(1);
 expect(takeLoadStretchV1(10,10)).toBe(1);
 expect(takeLoadStretchV1(85,10)).toBe(8.5);
 expect(takeLoadStretchV1(108,10)).toBe(10.8);
 expect(takeLoadStretchV1(5000,10)).toBe(TAKE_POLICY_V1.loadStretchMaximum);
 expect(takeLoadStretchV1(Number.NaN,10)).toBe(1);
 expect(TAKE_POLICY_V1.noProgressMilliseconds*TAKE_POLICY_V1.loadStretchMaximum).toBeLessThanOrEqual(TAKE_POLICY_V1.noProgressCeilingMilliseconds);
});

test("a silent read stops only when it is stuck: no CPU use on an idle host, or at the hard ceiling",()=>{
 const window=300000,at=(silent:number,load:number,cpu?:number)=>takeStopVerdictV1({silentMilliseconds:silent,windowMilliseconds:window,load,cores:10,cpuDeltaMilliseconds:cpu});
 expect(at(299999,2,0)).toEqual({stop:false,reason:"stretched"});
 expect(at(300000,2,0)).toEqual({stop:true,reason:"stuck"});
 expect(at(300000,2,5000)).toEqual({stop:false,reason:"alive"});
 expect(at(300000,2,undefined)).toEqual({stop:true,reason:"stuck"});
 expect(at(1200000,100,0)).toEqual({stop:false,reason:"stretched"});
 expect(at(3000000,100,0)).toEqual({stop:false,reason:"starved"});
 expect(at(3000000,100,2000)).toEqual({stop:false,reason:"alive"});
 expect(at(TAKE_POLICY_V1.noProgressCeilingMilliseconds,100,9000)).toEqual({stop:true,reason:"ceiling"});
});

test("the CPU threshold of a starved host stretches so a process that gets a tenth of its share still counts as alive",async()=>{
 let at=0,used=0;
 const plain=createCpuProgressV1({sample:()=>used,now:()=>at}),stretched=createCpuProgressV1({sample:()=>used,now:()=>at,stretch:()=>10});
 at=10000;used=200;
 expect(await plain.tick()).toBe(0);expect(await stretched.tick()).toBe(0);
 at=20000;used=400;
 expect(await plain.tick()).toBe(0);expect(plain.delta()).toBe(200);
 expect(await stretched.tick()).toBe(1);
});

test("an unreadable process table leaves the own CPU of the read in the sample",async()=>{
 const failing=(_command:string,_args:string[],_options:unknown,done:(error:Error|null,out:string)=>void)=>done(new Error("ps"),"");
 expect(await takeWorkCpuV1(1,failing,{user:3000000,system:1000000})).toBe(4000);
});

test("the window without progress is the authored window times the speed factor",()=>{
 expect(takeNoProgressWindowV1()).toBe(scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,localMachineSpeedFactorV1()));
});

test("a quarantined owner is written atomically, expires after the declared time and is replaced by name",()=>{
 const folder=mkdtempSync(join(tmpdir(),"quarantine-")),path=join(folder,"deep","owner-quarantine.json");
 try{
  expect(takeQuarantineReadV1(path)).toEqual([]);
  expect(takeQuarantineWriteV1(path,["a","b"],"stuck",1000)).toBe(true);
  expect(takeQuarantineReadV1(path,2000)).toEqual(["a","b"]);
  expect(takeQuarantineWriteV1(path,["b","c"],"ceiling",3000)).toBe(true);
  expect(takeQuarantineReadV1(path,4000)).toEqual(["a","b","c"]);
  expect(JSON.parse(readFileSync(path,"utf8")).find((row:{name:string})=>row.name==="b").reason).toBe("ceiling");
  expect(takeQuarantineReadV1(path,1000+TAKE_POLICY_V1.quarantineMilliseconds)).toEqual(["b","c"]);
  expect(readdirSync(join(folder,"deep"))).toEqual(["owner-quarantine.json"]);
  expect(takeQuarantineReadV1(join(folder,"missing.json"))).toEqual([]);
 }finally{rmSync(folder,{recursive:true,force:true});}
});

test("the watch of the real read asks the verdict before it stops and quarantines what runs",()=>{
 const text=readFileSync(new URL("../../../🟨️.mjs",import.meta.url),"utf8");
 expect(text).toContain("takeStopVerdictV1(");
 expect(text).toContain("takeQuarantineWriteV1(quarantinePath");
 expect(text).toContain("takeQuarantineReadV1(");
});
'''
d=os.path.dirname(p);fd,t=tempfile.mkstemp(dir=d,prefix=".f43-");os.write(fd,s.encode());os.close(fd);os.chmod(t,0o644);os.replace(t,p)
