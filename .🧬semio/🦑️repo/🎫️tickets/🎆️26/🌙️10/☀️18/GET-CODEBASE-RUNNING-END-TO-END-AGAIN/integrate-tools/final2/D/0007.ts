import {test,expect} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import {captureOwnedCommandV1} from "../../../../../../🎛️owned-execution/🟦️.ts";
import {createNativeImageRosterV1,createNativePhysicalGateV1,nativeRecheckWindowV1,withNativeRecheckWindowV1} from "../🟦️.ts";

const plist=(serial:number)=>'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n<plist version="1.0"><dict><key>images</key><array/><key>serial</key><integer>'+serial+'</integer></dict></plist>';
const settle=()=>new Promise<void>(done=>setTimeout(done,2));

async function replay(row:(typeof fixture.cases)[number]){
 let clock=0,released=0;const pending:Array<()=>void>=[],calls=new Map<string,Promise<{value:number;serial:number}>>(),served:Record<string,number>={};
 const roster=createNativeImageRosterV1({now:()=>clock}),capture=()=>new Promise<string>(done=>{pending.push(()=>done(plist(++released)));});
 for(const step of row.steps as Array<{op:string;id?:string;ms?:number;maxAge?:number;after?:number}>){
  if(step.op==="advance")clock+=step.ms!;
  else if(step.op==="call")calls.set(step.id!,roster.observe({capture,policy:fixture.policy,maximumAgeMilliseconds:step.maxAge??0,after:step.after??0,accept:information=>{const serial=information.serial as number;if((row.rejectSerials as number[]).includes(serial))throw Error("rejected snapshot");return serial;}}));
  else if(step.op==="release"){for(let wait=0;!pending.length&&wait<2000;wait++)await settle();expect(pending.length).toBeGreaterThan(0);pending.shift()!();await settle();}
  else if(step.op==="drain")for(const[id,promise]of calls)served[id]=(await promise).serial;
 }
 return{captures:roster.captures(),served};
}

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource=`
import json,sys
def simulate(case):
    clock=0.0;serial=0;latest=None;running=None;queued=None;captures=0;served={}
    def finish(call,number):
        nonlocal latest
        if number in case["rejectSerials"]:
            if latest is not None and latest["serial"]==number: latest=None
            request(call,True)
        else: served[call["id"]]=number
    def request(call,force=False):
        nonlocal running,queued,captures
        age=call.get("maxAge",0);after=call.get("after",0)
        if not force and age>0 and latest is not None and latest["serial"]>after and clock-latest["at"]<=age: finish(call,latest["serial"])
        elif queued is not None: queued["waiters"].append(call)
        elif running is None: running={"at":clock,"waiters":[call]};captures+=1
        else: queued={"at":None,"waiters":[call]};captures+=1
    for step in case["steps"]:
        op=step["op"]
        if op=="advance": clock+=step["ms"]
        elif op=="call": request(dict(step))
        elif op=="release":
            entry=running;serial+=1;latest={"serial":serial,"at":entry["at"]};running=queued
            if running is not None: running["at"]=clock
            queued=None
            for waiter in entry["waiters"]: finish(waiter,serial)
    return {"captures":captures,"served":served}
def gate(case):
    verified=None;fulls=0
    for at in case["runs"]:
        if case["window"]>0 and verified is not None and at-verified<=case["window"]: continue
        fulls+=1;verified=at
    return {"fulls":fulls}
fixture=json.load(open(sys.argv[1]))
print(json.dumps({"cases":[simulate(c) for c in fixture["cases"]],"gate":[gate(c) for c in fixture["gate"]]}))
`;
const oracle=python?JSON.parse(spawnSync(python,["-I","-c",oracleSource,fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"}).stdout||"null"):null;

for(const[index,row]of fixture.cases.entries())test("native roster: "+row.name,async()=>{
 const got=await replay(row);
 expect(got).toEqual({captures:row.captures,served:row.served as Record<string,number>});
 if(oracle)expect(got).toEqual(oracle.cases[index]);
});

for(const[index,row]of fixture.gate.entries())test("native physical gate: "+row.name,async()=>{
 let clock=0,cheap=0;const gate=createNativePhysicalGateV1(()=>clock);
 for(const at of row.runs){clock=at;await gate.run(row.window,()=>{cheap++;},async()=>undefined);}
 expect(gate.fulls()).toBe(row.fulls);
 expect(cheap).toBe(row.runs.length);
 if(oracle)expect({fulls:gate.fulls()}).toEqual(oracle.gate[index]);
});

test("native physical gate: concurrent runs inside the window join one full verification",async()=>{
 let clock=0,release!:()=>void;const gate=createNativePhysicalGateV1(()=>clock),held=new Promise<void>(done=>{release=done;});
 const first=gate.run(500,()=>{},()=>held),second=gate.run(500,()=>{},async()=>{throw Error("second must join");});
 release();await Promise.all([first,second]);
 expect(gate.fulls()).toBe(1);
});

test("native physical gate: a joiner whose partner failed runs its own full verification",async()=>{
 let release!:()=>void;const gate=createNativePhysicalGateV1(()=>0),held=new Promise<void>(done=>{release=done;});
 const first=gate.run(500,()=>{},async()=>{await held;throw Error("partner cancelled");}),second=gate.run(500,()=>{},async()=>undefined);
 release();
 await expect(first).rejects.toThrow("partner cancelled");
 await second;
 expect(gate.fulls()).toBe(2);
});

test("native physical gate: a failed cheap check is never skipped by the window",async()=>{
 const gate=createNativePhysicalGateV1(()=>0);await gate.run(500,()=>{},async()=>undefined);
 await expect(gate.run(500,()=>{throw Error("mount identity changed");},async()=>undefined)).rejects.toThrow("mount identity changed");
 expect(gate.fulls()).toBe(1);
});

test("native recheck window: zero outside, bounded inside, strict contexts stay outside",async()=>{
 expect(nativeRecheckWindowV1()).toBe(0);
 await withNativeRecheckWindowV1(async()=>{expect(nativeRecheckWindowV1()).toBe(500);},500);
 expect(()=>withNativeRecheckWindowV1(async()=>undefined,-1)).toThrow("Bounded native recheck window required");
 expect(nativeRecheckWindowV1()).toBe(0);
});

const run=(program:string,args:readonly string[])=>spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
test.skipIf(process.platform!=="darwin")("native roster: shared captures stay consistent for a stable image while other images attach and detach",async()=>{
 const real=fixture.real,root=realpathSync(mkdtempSync(join(tmpdir(),"roster-real-"))),children:ReturnType<typeof spawn>[]=[];
 try{
  const make=(name:string)=>{const dir=join(root,name),mount=join(dir,"m");mkdirSync(mount,{recursive:true});const image=join(dir,"t.dmg");expect(run("/usr/bin/hdiutil",["create","-size",real.imageMegabytes+"m","-fs","HFS+","-volname",name,"-type","UDIF",image]).status).toBe(0);return{dir,mount,image};};
  const stable=make("stable");
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",stable.mount,stable.image]).status).toBe(0);
  for(const churner of Array.from({length:real.churners},(_,index)=>make("churn"+index))){
   const script='while [ ! -e "$1/stop" ]; do /usr/bin/hdiutil attach -nobrowse -noautoopen -mountpoint "$2" "$3" >/dev/null 2>&1 && /usr/bin/hdiutil detach "$2" >/dev/null 2>&1; done';
   children.push(spawn("/bin/sh",["-c",script,"churn",root,churner.mount,churner.image],{stdio:"ignore"}));
  }
  const accepted=(information:Record<string,unknown>)=>{
   const listed=(information.images as Record<string,unknown>[]).filter(image=>image["image-path"]===stable.image);
   const mounts=listed.flatMap(image=>(image["system-entities"] as Record<string,unknown>[]).filter(entity=>entity["mount-point"]===stable.mount));
   if(listed.length!==1||mounts.length!==1)throw Error("Bounded unique parent roster required");
   const all=(information.images as Record<string,unknown>[]).flatMap(image=>(image["system-entities"] as Record<string,unknown>[]).map(entity=>entity["mount-point"]).filter(point=>typeof point==="string"));
   if(new Set(all).size!==all.length)throw Error("Bounded unique parent roster required");
   return information;
  };
  const roster=createNativeImageRosterV1({now:()=>performance.now()}),policy={attempts:real.attempts,backoffMilliseconds:real.backoffMilliseconds};
  const capture=()=>captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],root,{output:{maximumBytes:1048576,maximumLines:32768}});
  let next=0;
  await Promise.all(Array.from({length:real.readers},async()=>{while(next++<real.reads){const{value}=await roster.observe({capture,accept:accepted,policy,maximumAgeMilliseconds:real.windowMilliseconds});expect(Array.isArray(value.images)).toBe(true);}}));
  expect(roster.captures()).toBeLessThan(real.reads);
  const oracle=spawnSync("/usr/bin/hdiutil",["info","-plist"],{encoding:"utf8"}),converted=spawnSync("/usr/bin/plutil",["-convert","json","-o","-","-"],{input:oracle.stdout,encoding:"utf8"});
  expect(converted.status).toBe(0);
  expect((JSON.parse(converted.stdout).images as Record<string,unknown>[]).some(image=>image["image-path"]===stable.image)).toBe(true);
 }finally{
  writeFileSync(join(root,"stop"),"");
  await Promise.all(children.map(child=>new Promise<void>(done=>{if(child.exitCode!==null||child.signalCode!==null)return done();child.once("exit",()=>done());})));
  for(const dir of ["stable",...Array.from({length:fixture.real.churners},(_,index)=>"churn"+index)])run("/usr/bin/hdiutil",["detach","-force",join(root,dir,"m")]);
  rmSync(root,{recursive:true,force:true});
 }
},180000);
