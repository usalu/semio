/** 🎚️ Tests for ledger machine. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import budgetPolicy from "../../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {formatMachinePlanV1,machinePolicyV1,parseMachineBytesV1,planMachineBudgetV1,readMachinePolicyV1,resolveMachineTotalV1,splitMachineBudgetV1,type MachinePolicyV1,type MachineUseV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};

const GIB=1024**3,clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value)),declared=machinePolicyV1();
function generator(seed:number):()=>number{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};}

function corpus(count:number):Array<{totalBytes:number;hostFreeBytes:number|null;uses:MachineUseV1[]}>{
 const random=generator(20261026),out=[];
 for(let index=0;index<count;index++){
  const heavy=random()<0.5,uses=declared.stores.map(store=>{
   const empty=random()<0.1,used=empty?0:Math.floor(random()*(heavy?30:6)*GIB*(store.loose?1:random()<0.5?1:0.1)),image=random()<0.2?null:used+Math.floor(random()*8*GIB);
   return {id:store.id,usedBytes:used,envelopeBytes:store.loose?null:image,evictableBytes:random()<0.4?null:Math.floor(used*random())};
  });
  out.push({totalBytes:Math.max(declared.stores.reduce((sum,store)=>sum+store.floorBytes,0),Math.floor((4+random()*90)*GIB)),hostFreeBytes:random()<0.2?null:random()<0.3?Math.floor(random()*declared.hostFreeFloorBytes):Math.floor((15+random()*200)*GIB),uses});
 }
 return out;
}

test("the authored policy is closed and Ajv agrees with the schema validator",()=>{
 expect(readMachinePolicyV1(policy)).toEqual(declared);
 const validate=new Ajv({strict:true,allowUnionTypes:true}).compile(schema);
 expect(validate(policy)).toBe(true);
 const probes:Array<(document:any)=>void>=[
  document=>{delete document.stores;},document=>{document.version=2;},document=>{document.extra=1;},document=>{document.stores[0].weightBasisPoints=0;},
  document=>{document.stores[0].headroomBasisPoints=9000;},document=>{document.stores[0].purpose={en:"x"};},document=>{document.environment="lower";},document=>{document.alignBytes=1;},document=>{document.stores[0].id="Bad Id";}
 ];
 for(const probe of probes){const document=clone(policy);probe(document);expect(validate(document)).toBe(false);}
 expect(validate(policy)).toBe(true);
});

test("the semantic closure refuses what the schema cannot see",()=>{
 const refused:Array<[string,(document:any)=>void]>=[
  ["duplicate store",document=>{document.stores[1].id=document.stores[0].id;}],
  ["weights below the whole",document=>{document.stores[0].weightBasisPoints-=1;}],
  ["partition and loose together",document=>{document.stores[0].loose=true;}],
  ["neither partition nor loose",document=>{delete document.stores[0].partition;}],
  ["unknown partition",document=>{document.stores[0].partition="nowhere";}],
  ["a partition in two stores",document=>{document.stores[1].partition=document.stores[0].partition;}],
  ["a partition in no store",document=>{document.stores.splice(1,1);document.stores[0].weightBasisPoints+=policy.stores[1]!.weightBasisPoints;}],
  ["floors above the default total",document=>{document.defaultTotalBytes=1<<20;}],
  ["duplicate dedupe target",document=>{document.dedupe.push(clone(document.dedupe[0]));}],
  ["dedupe target in an unknown partition",document=>{document.dedupe[0].partition="nowhere";}],
  ["dedupe directory with a parent segment",document=>{document.dedupe[0].directory="a/../b";}],
  ["two loose stores",document=>{document.stores[0]={...document.stores[0],partition:undefined,loose:true};}]
 ];
 for(const[name,mutate]of refused){const document=clone(policy);mutate(document);expect(()=>readMachinePolicyV1(document),name).toThrow("Closed machine disk budget required");}
 expect(declared.stores.filter(store=>store.partition!==undefined).map(store=>store.partition).sort()).toEqual(Object.keys(budgetPolicy.partitions).sort());
});

test("byte counts and the total resolve in a fixed order of precedence, refused in both languages",()=>{
 const rows:Array<[string|number,number|null]>=[[0,0],[1024,1024],["1024",1024],["40GiB",40*GIB],["40G",40*GIB],["40 gib",40*GIB],["1.5GiB",1.5*GIB],["512MiB",512*1024**2],["2T",2*1024**4],["64KiB",65536],["7B",7],["",null],["abc",null],["-1",null],[-1,null],[1.5,null],["40XB",null],["1e3",null]];
 for(const[text,bytes]of rows){if(bytes===null)expect(()=>parseMachineBytesV1(text),String(text)).toThrow();else expect(parseMachineBytesV1(text),String(text)).toBe(bytes);}
 const environment={[declared.environment]:"30GiB"},file=JSON.stringify({version:1,totalBytes:"20GiB"});
 expect(resolveMachineTotalV1({})).toEqual({bytes:declared.defaultTotalBytes,source:"default"});
 expect(resolveMachineTotalV1({fileText:file})).toEqual({bytes:20*GIB,source:"file"});
 expect(resolveMachineTotalV1({fileText:file,environment})).toEqual({bytes:30*GIB,source:"environment"});
 expect(resolveMachineTotalV1({fileText:file,environment,argument:"50GiB"})).toEqual({bytes:50*GIB,source:"argument"});
 expect(resolveMachineTotalV1({environment:{[declared.environment]:""}})).toEqual({bytes:declared.defaultTotalBytes,source:"default"});
 for(const bad of ["{","{}",JSON.stringify({version:1,totalBytes:-4}),JSON.stringify({version:1,totalBytes:"x"})])expect(()=>resolveMachineTotalV1({fileText:bad})).toThrow("Die Maschinen-Budget-Datei");
 expect(()=>resolveMachineTotalV1({argument:"1MiB"})).toThrow("Untergrenzen");
 expect(()=>resolveMachineTotalV1({argument:"1MiB"})).toThrow("below the floors");
});

test("the split is exact: floors, demands, weights, spare and rounding",()=>{
 const rows=[{id:"a",weight:1,floor:0,demand:100},{id:"b",weight:1,floor:0,demand:10},{id:"c",weight:2,floor:5,demand:1000}];
 expect([...splitMachineBudgetV1(120,rows,false)]).toEqual([["a",35],["b",10],["c",75]]);
 expect([...splitMachineBudgetV1(120,rows,false)].reduce((sum,[,value])=>sum+value,0)).toBe(120);
 expect([...splitMachineBudgetV1(5000,rows,true)].reduce((sum,[,value])=>sum+value,0)).toBe(5000);
 expect([...splitMachineBudgetV1(5000,rows,false)].reduce((sum,[,value])=>sum+value,0)).toBe(1110);
 expect([...splitMachineBudgetV1(7,[{id:"x",weight:1,floor:0,demand:9},{id:"y",weight:1,floor:0,demand:9},{id:"z",weight:1,floor:0,demand:9}],false)]).toEqual([["x",3],["y",2],["z",2]]);
 expect(()=>splitMachineBudgetV1(4,rows,false)).toThrow("below the floors");
});

test("the plan equals an independent exact-rational oracle on 400 seeded machines",()=>{
 const cases=corpus(400),directory=mkdtempSync(join(tmpdir(),"mb-")),file=join(directory,"cases.json");
 try{
  writeFileSync(file,JSON.stringify({stores:declared.stores,hostFreeFloorBytes:declared.hostFreeFloorBytes,alignBytes:declared.alignBytes,toleranceBytes:declared.toleranceBytes,cases}));
  const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),file],{encoding:"utf8",maxBuffer:1<<28});
  expect(run.status,run.stderr).toBe(0);
  const expected=JSON.parse(run.stdout) as Array<Record<string,unknown>>;
  expect(expected.length).toBe(cases.length);
  cases.forEach((input,index)=>{const{version:_version,...plan}=planMachineBudgetV1(input);expect(JSON.parse(JSON.stringify(plan)),"case "+index).toEqual(expected[index]);});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("laws: quotas add up, nothing under its fair share is cut, planning converges in one step, a larger budget never evicts more",()=>{
 const floors=declared.stores.reduce((sum,store)=>sum+store.floorBytes,0);
 for(const[index,input]of corpus(300).entries()){
  const plan=planMachineBudgetV1(input),label="case "+index;
  expect(plan.stores.reduce((sum,row)=>sum+row.quotaBytes,0),label).toBe(plan.effectiveTotalBytes);
  expect(plan.effectiveTotalBytes,label).toBeGreaterThanOrEqual(floors);expect(plan.effectiveTotalBytes,label).toBeLessThanOrEqual(input.totalBytes);
  for(const row of plan.stores){
   const use=input.uses.find(candidate=>candidate.id===row.id)!;
   expect(row.evictBytes+row.blockedBytes,label).toBe(row.excessBytes);expect(row.evictBytes,label).toBeLessThanOrEqual(use.evictableBytes??use.usedBytes);
   if(row.demandBytes<=row.quotaBytes)expect(row.excessBytes,label+" "+row.id+" is under its share").toBe(0);
   expect(row.quotaBytes,label).toBeGreaterThanOrEqual(declared.stores.find(store=>store.id===row.id)!.floorBytes);
  }
  if(plan.stores.reduce((sum,row)=>sum+Math.max(row.demandBytes,declared.stores.find(store=>store.id===row.id)!.floorBytes),0)<=plan.effectiveTotalBytes)expect(plan.evictBytes+plan.blockedBytes,label+" fits").toBe(0);
  const after=planMachineBudgetV1({...input,hostFreeBytes:plan.pressureBytes>0?declared.hostFreeFloorBytes:input.hostFreeBytes,uses:input.uses.map(use=>{const row=plan.stores.find(candidate=>candidate.id===use.id)!;return{...use,usedBytes:use.usedBytes-row.evictBytes,evictableBytes:use.evictableBytes===null?null:use.evictableBytes-row.evictBytes};})});
  expect(after.evictBytes,label+" converges once the host is relieved").toBe(0);
  expect(after.blockedBytes,label+" blocked never grows").toBeLessThanOrEqual(plan.blockedBytes);
  const larger=planMachineBudgetV1({...input,totalBytes:input.totalBytes+4*GIB});
  expect(larger.evictBytes+larger.blockedBytes,label+" monotone").toBeLessThanOrEqual(plan.evictBytes+plan.blockedBytes);
 }
});

test("host pressure shrinks the budget by what the host lacks, never below the floors",()=>{
 const uses=declared.stores.map(store=>({id:store.id,usedBytes:3*GIB,envelopeBytes:store.loose?null:6*GIB,evictableBytes:null})),calm=planMachineBudgetV1({totalBytes:60*GIB,hostFreeBytes:declared.hostFreeFloorBytes,uses}),tight=planMachineBudgetV1({totalBytes:60*GIB,hostFreeBytes:declared.hostFreeFloorBytes-4*GIB,uses});
 expect(calm.pressureBytes).toBe(0);expect(calm.evictBytes).toBe(0);
 expect(tight.pressureBytes).toBe(4*GIB);
 expect(tight.effectiveTotalBytes).toBeLessThan(calm.effectiveTotalBytes);
 expect(tight.evictBytes).toBeGreaterThan(0);
 const starved=planMachineBudgetV1({totalBytes:60*GIB,hostFreeBytes:0,uses:uses.map(use=>({...use,usedBytes:0}))});
 expect(starved.effectiveTotalBytes).toBe(declared.stores.reduce((sum,store)=>sum+store.floorBytes,0));
});

test("the report reads the same in English and German and names what must go and what must stay",()=>{
 const uses=declared.stores.map(store=>({id:store.id,usedBytes:store.id==="rust"?30*GIB:2*GIB,envelopeBytes:store.loose?null:store.id==="rust"?48*GIB:4*GIB,evictableBytes:store.id==="rust"?10*GIB:null})),plan=planMachineBudgetV1({totalBytes:40*GIB,hostFreeBytes:5*GIB,uses}),en=formatMachinePlanV1("en",plan,"argument"),de=formatMachinePlanV1("de",plan,"file");
 expect(en.length).toBe(de.length);expect(en.length).toBe(declared.stores.length+3);
 expect(en[0]).toContain("from --budget");expect(de[0]).toContain("aus der Maschinendatei");
 expect(en.join("\n")).toContain("kept by holders or owners");expect(de.join("\n")).toContain("von Haltern oder Verwaltern behalten");
 expect(en.join("\n")).toContain("below its free-space floor");expect(de.join("\n")).toContain("Untergrenze");
 for(const line of[...en,...de])expect(line).not.toMatch(/\{[a-z]+\}/u);
});
