/** 🗂️ Contract: the generation file port rechecks the store once per window of operations of any kind, refuses exactly the operation that falls on a refused recheck, forwards admitted paths and options untouched, and under concurrent load never runs two rechecks at once and never more than the model's bound, as an independent Python model of the window says for the scenarios of `./🔣️.json`. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {createGenerationFilesV1,generationRecheckWindowV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=():{sequences:{rechecks:number;refused:number[]}[];concurrent:{maximumRechecks:number;peak:number}[]}=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({sequences:corpus.sequences,concurrent:corpus.concurrent}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const recording=()=>{const calls:[string,string,unknown?][]=[];const port={read:async(path:string)=>{calls.push(["read",path]);return path;},write:async(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean})=>{calls.push(["write",path,options]);},names:async(path:string)=>{calls.push(["names",path]);return[];},list:async(path:string)=>{calls.push(["list",path]);return[];},remove:async(path:string)=>{calls.push(["remove",path]);}};return{calls,port};};
const run=(files:ReturnType<typeof createGenerationFilesV1>,operation:{kind:string;path:string})=>operation.kind==="write"?files.write(operation.path,new Uint8Array([1]),{ifAbsent:true}):(files as never as Record<string,(path:string)=>Promise<unknown>>)[operation.kind]!(operation.path);

test("the corpus is closed by its schema and the default window is the authored one",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,sequences:[{...corpus.sequences[0],operations:[{kind:"copy",path:"/store/x"}]}]})).toBe(false);
 expect(generationRecheckWindowV1).toBe(corpus.defaultWindow);
});

test("every sequence refuses exactly the operations the model says, rechecks as often as it says and forwards what was admitted",async()=>{
 const model=ask();
 for(const[index,row]of corpus.sequences.entries()){
  let rechecks=0;const{calls,port}=recording(),files=createGenerationFilesV1({store:{root:"/store",recheck:async()=>{rechecks++;if(row.failRechecks.includes(rechecks))throw Error("refused "+rechecks);}},base:port,requirePath:(_root,path)=>path,window:row.window}),refused:number[]=[];
  for(const[position,operation]of row.operations.entries())try{await run(files,operation);}catch(error){expect((error as Error).message,row.name).toBe("refused "+rechecks);refused.push(position);}
  expect(rechecks,row.name).toBe(model.sequences[index]!.rechecks);expect(refused,row.name).toEqual(model.sequences[index]!.refused);
  expect(calls.map(call=>call[1]),row.name).toEqual(row.operations.filter((_,position)=>!refused.includes(position)).map(operation=>operation.path));
  expect(calls.filter(call=>call[0]==="write").every(call=>(call[2] as {ifAbsent:boolean}).ifAbsent===true),row.name).toBe(true);
 }
});

test("concurrent operations share one recheck at a time and stay within the model's bound",async()=>{
 const model=ask();
 for(const[index,row]of corpus.concurrent.entries()){
  const state={calls:0,active:0,peak:0},{port}=recording(),files=createGenerationFilesV1({store:{root:"/store",recheck:async()=>{state.calls++;state.active++;state.peak=Math.max(state.peak,state.active);await Bun.sleep(row.sleepMs);state.active--;}},base:port,requirePath:(_root,path)=>path,window:row.window});
  await Promise.all(Array.from({length:row.workers},async(_,worker)=>{for(let operation=0;operation<row.each;operation++)await files.read("/store/"+worker+"/"+operation);}));
  expect(state.peak,row.name).toBeLessThanOrEqual(model.concurrent[index]!.peak);expect(state.calls,row.name).toBeLessThanOrEqual(model.concurrent[index]!.maximumRechecks);
  if(row.window<=row.each)expect(state.calls,row.name).toBeGreaterThanOrEqual(1);else expect(state.calls,row.name).toBe(Math.floor(row.workers*row.each/row.window));
  expect(state.active,row.name).toBe(0);
 }
});

test("the port is frozen, exposes exactly the five operations and never lets a path escape the store root before the base sees it",async()=>{
 const{calls,port}=recording(),files=createGenerationFilesV1({store:{root:"/store",recheck:async()=>undefined},base:port,requirePath:(root,path)=>{if(!path.startsWith(root+"/"))throw Error("escapes "+path);return path;}});
 expect(Object.isFrozen(files)).toBe(true);expect(Object.keys(files).sort()).toEqual(["list","names","read","remove","write"]);
 for(const kind of["read","names","list","remove"] as const)await expect(run(files,{kind,path:"/elsewhere/x"})).rejects.toThrow("escapes /elsewhere/x");
 await expect(run(files,{kind:"write",path:"/store/../x"})).resolves.toBeUndefined();expect(calls.map(call=>call[1])).toEqual(["/store/../x"]);
});
