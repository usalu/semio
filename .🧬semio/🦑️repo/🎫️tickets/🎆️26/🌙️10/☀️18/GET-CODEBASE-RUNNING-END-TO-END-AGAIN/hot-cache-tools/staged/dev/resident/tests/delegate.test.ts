import {afterAll,describe,expect,test} from "bun:test";
import {mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createDelegatedExecutionPortV1,createWarmPoolV1,writeWarmPreloadV1} from "../delegate/index.mjs";
import {cleanup,sleep,tempRoot,waitFor} from "./common.ts";

const roots:string[]=[];afterAll(()=>roots.forEach(cleanup));

/** 🔗 A synthetic module closure: a chain of `count` modules, each doing some work at load. */
function fixture(count:number){
 const root=tempRoot("delegate-");roots.push(root);mkdirSync(join(root,"mods"));
 for(let i=0;i<count;i++)writeFileSync(join(root,"mods","m"+i+".mjs"),(i+1<count?`import "./m${i+1}.mjs";\n`:"")+`export const v${i}=${JSON.stringify(Array.from({length:200},(_,k)=>"x"+k+"y"+i))}.join("");\n`);
 writeFileSync(join(root,"main.mjs"),`import "./mods/m0.mjs";
if(process.argv.includes("--hang")){process.on("SIGINT",()=>process.exit(130));setInterval(()=>{},1000);}
else{console.log(JSON.stringify({argv:process.argv.slice(2),cwd:process.cwd(),env:process.env.TEST_VAR??null,removed:process.env.TEST_GONE??null}));console.error("to stderr");if(process.argv.includes("--fail"))process.exit(3);}
`);
 return root;
}
const collect=()=>{const out:{stream:string;text:string}[]=[];return{out,onOutput:async(stream:string,text:string)=>{out.push({stream,text});}};};
const poolFor=(root:string,count:number,extra:Record<string,unknown>={})=>{
 const preload=writeWarmPreloadV1(root);
 return createWarmPoolV1({command:["bun","--preload",preload,join(root,"main.mjs")],environment:{...process.env,SEMIO_WARM_MODULES:JSON.stringify([join(root,"mods","m0.mjs")]),TEST_GONE:"x"},cwd:root,size:1,idleMilliseconds:60000,...extra});
};

describe("warm pool",()=>{
 test("a taken worker runs exactly the requested command: argv, cwd, env patch, streams, exit code",async()=>{
  const root=fixture(60);const pool=poolFor(root,60);pool.top();
  expect(await waitFor(()=>pool.idleCount()===1,15000)).toBe(true);
  const worker=pool.take()!;expect(worker).toBeDefined();
  const sink=collect();
  const result=await pool.run(worker,{argv:["run","p:build","--fail"],cwd:tempRoot("delegate-cwd-"),env:{TEST_VAR:"v",TEST_GONE:null}},sink);
  expect(result.exit).toBe(3);expect(result.started).toBe(true);
  const printed=JSON.parse(sink.out.find(o=>o.stream==="stdout")!.text);
  expect(printed.argv).toEqual(["run","p:build","--fail"]);expect(printed.env).toBe("v");expect(printed.removed).toBeNull();expect(printed.cwd).toContain("delegate-cwd-");
  expect(sink.out.some(o=>o.stream==="stderr"&&o.text.includes("to stderr"))).toBe(true);
  expect(await waitFor(()=>pool.idleCount()===1,15000)).toBe(true);
  expect(pool.stats().spawned).toBe(2);
  await pool.close();
 },60000);

 test("take hands out each warm worker once; an empty pool answers undefined and refills",async()=>{
  const root=fixture(5);const pool=poolFor(root,5);pool.top();
  expect(pool.take()).toBeUndefined();
  expect(await waitFor(()=>pool.idleCount()===1,15000)).toBe(true);
  const worker=pool.take()!;expect(pool.take()).toBeUndefined();
  worker.child.kill("SIGKILL");
  await pool.close();
 },30000);

 test("cancellation interrupts the running command",async()=>{
  const root=fixture(5);const pool=poolFor(root,5);pool.top();
  expect(await waitFor(()=>pool.idleCount()===1,15000)).toBe(true);
  const controller=new AbortController(),worker=pool.take()!;
  const running=pool.run(worker,{argv:["--hang"]},{...collect(),signal:controller.signal});
  await sleep(300);controller.abort();
  expect((await running).exit).toBe(130);
  await pool.close();
 },30000);

 test("idle workers retire, a worker that never becomes ready is killed",async()=>{
  const root=fixture(3);const retiring=poolFor(root,3,{idleMilliseconds:200});retiring.top();
  expect(await waitFor(()=>retiring.idleCount()===1,15000)).toBe(true);
  expect(await waitFor(()=>retiring.idleCount()===0,3000)).toBe(true);
  await retiring.close();
  const stuck=createWarmPoolV1({command:["bun","-e","setInterval(()=>{},1000)"],environment:process.env,cwd:root,size:1,idleMilliseconds:1000,readyTimeoutMilliseconds:400});stuck.top();
  await sleep(1500);expect(stuck.idleCount()).toBe(0);await stuck.close();
 },30000);

 test("the delegated execution port falls back to a cold process and gives the same output",async()=>{
  const root=fixture(5),preload=writeWarmPreloadV1(root);const pool=poolFor(root,5);
  const port=createDelegatedExecutionPortV1({pool,coldCommand:["bun",join(root,"main.mjs")],argvFor:id=>["run",id],environment:{...process.env,TEST_GONE:"x"},cwd:root});
  const cold=collect();const first=await port.run({id:"p:build",env:{TEST_VAR:"c"},signal:new AbortController().signal,onSpawn(){},onOutput:cold.onOutput});
  expect(first.exit).toBe(0);expect(JSON.parse(cold.out[0]!.text).argv).toEqual(["run","p:build"]);
  expect(await waitFor(()=>pool.idleCount()===1,15000)).toBe(true);
  const warm=collect();const second=await port.run({id:"p:build",env:{TEST_VAR:"c"},signal:new AbortController().signal,onSpawn(){},onOutput:warm.onOutput});
  expect(second.exit).toBe(0);expect(JSON.parse(warm.out.find(o=>o.stream==="stdout")!.text)).toEqual(JSON.parse(cold.out.find(o=>o.stream==="stdout")!.text));
  expect(preload.endsWith(".mjs")).toBe(true);await pool.close();
 },60000);
});

describe("execution port routing",()=>{
 const sink=()=>{const out:string[]=[];return{out,onOutput:async(_s:string,t:string)=>{out.push(t);}};};
 const warmPool=(exit=0)=>({take:()=>({}),run:async(_w:any,_r:any,o:any)=>{await o.onOutput("stdout","warm\n");return{exit,started:true};}}) as any;
 test("a ready executor wins; a refusal or a lost executor without output falls through to the warm worker; a lost one with output is final",async()=>{
  const calls:string[]=[];
  const executors=(result:any,writes=false)=>({take:()=>({}),run:async(_e:any,_r:any,o:any)=>{calls.push("executor");if(writes)await o.onOutput("stdout","partial");return result;}}) as any;
  const run=(executorsPort:any)=>{const s=sink();return createDelegatedExecutionPortV1({pool:warmPool(),executors:executorsPort,coldCommand:["true"],argvFor:id=>["repo","run",id],environment:{},cwd:"/",timing:kind=>calls.push(kind)}).run({id:"a:b",env:{},signal:new AbortController().signal,onSpawn(){},onOutput:s.onOutput}).then(r=>({...r,out:s.out.join("")}));};
  expect(await run(executors({exit:0}))).toMatchObject({exit:0});expect(calls).toEqual(["executor","executor"]);
  calls.length=0;expect((await run(executors({exit:70,refusal:"authority"}))).out).toBe("warm\n");expect(calls).toEqual(["executor","warm"]);
  calls.length=0;expect((await run(executors({exit:70,lost:true}))).out).toBe("warm\n");
  calls.length=0;expect(await run(executors({exit:70,lost:true},true))).toMatchObject({exit:70,out:"partial"});expect(calls).toEqual(["executor","executor"]);
  calls.length=0;expect((await run({take:()=>undefined} as any)).out).toBe("warm\n");
 });
 test("a refusal of the service invocation is final: it reaches the caller with its code and neither the warm worker nor a cold process runs the command",async()=>{
  const calls:string[]=[];
  const executors={take:()=>({}),run:async()=>({exit:70,refusal:"service",code:"revoked",fields:{}})} as any,warm={take:()=>{calls.push("warm");return{};},run:async()=>({exit:0})} as any;
  const result=await createDelegatedExecutionPortV1({pool:warm,executors,coldCommand:["false"],argvFor:id=>["repo","run",id],environment:{},cwd:"/",timing:kind=>calls.push(kind)}).run({id:"a:b",env:{},client:{token:"t",agent:"x",named:["a"]},signal:new AbortController().signal,onSpawn(){},onOutput:async()=>{}});
  expect(result).toEqual({exit:70,refusal:{code:"revoked",fields:{}}});expect(calls).toEqual(["refused"]);
 });
});
