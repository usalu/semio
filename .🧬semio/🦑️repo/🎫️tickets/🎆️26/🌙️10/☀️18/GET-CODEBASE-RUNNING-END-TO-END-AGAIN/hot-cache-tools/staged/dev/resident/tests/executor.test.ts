import {describe,expect,test} from "bun:test";
import {join} from "node:path";
import {isolateRequestV1,serveExecutorV1} from "../executor/index.mjs";
import {createExecutorPoolV1} from "../delegate/executor.mjs";
import {sleep,waitFor} from "./common.ts";

const FIXTURE=join(import.meta.dir,"executor-fixture.mjs");
const pool=(extra:Record<string,string>={},size=1)=>createExecutorPoolV1({command:["bun",FIXTURE],environment:{...process.env,...extra},cwd:process.cwd(),size,readyTimeoutMilliseconds:20000});
const collect=()=>{const out:string[]=[],err:string[]=[];return{out,err,onOutput:async(stream:string,text:string)=>{(stream==="stdout"?out:err).push(text);}};};

describe("request isolation",()=>{
 const fake=()=>{const state:any={env:{KEEP:"1",CHANGE:"a"},directory:"/start",exitCode:undefined,cwd(){return this.directory;},chdir(path:string){this.directory=path;}};return state;};
 test("the patch applies inside and everything is restored after, also when the body throws",async()=>{
  const state=fake();
  const exit=await isolateRequestV1({env:{CHANGE:"b",ADD:"x",KEEP:null},cwd:"/work"},async()=>{
   expect([state.env.CHANGE,state.env.ADD,state.env.KEEP,state.directory]).toEqual(["b","x",undefined,"/work"]);state.env.LEAK="y";state.exitCode=3;
  },state);
  expect(exit).toBe(3);expect(state.env).toEqual({KEEP:"1",CHANGE:"a"});expect(state.directory).toBe("/start");expect(state.exitCode).toBeUndefined();
  await expect(isolateRequestV1({env:{ADD:"1"}},async()=>{state.env.LEAK="z";throw new Error("x");},state)).rejects.toThrow("x");
  expect(state.env).toEqual({KEEP:"1",CHANGE:"a"});
 });
 test("limits end the loop and a refused custody is reported",async()=>{
  const frames:string[]=[];
  async function* lines(){yield JSON.stringify({id:1,argv:["a"]});yield JSON.stringify({id:2,argv:["b"]});yield JSON.stringify({id:3,argv:["c"]});}
  const result=await serveExecutorV1({lines:lines(),write:t=>frames.push(t),openSession:async()=>({run:async(body:any)=>body({}),close:async()=>undefined}),run:async()=>undefined,limits:{maximumRequests:2,maximumAgeMilliseconds:1e9},state:{...fake(),on(){},console:{log(){},error(){}}}});
  expect(result).toEqual({served:2,reason:"ended"});expect(frames.filter(f=>f.includes('"done"')).length).toBe(2);
  const refused=await serveExecutorV1({lines:lines(),write:t=>frames.push(t),openSession:async()=>{throw new Error("busy");},run:async()=>undefined,limits:{maximumRequests:9,maximumAgeMilliseconds:1e9}});
  expect(refused.reason).toBe("custody");
 });
});

describe("executor processes",()=>{
 test("custody is opened once and reused by every command; the state of one command never reaches the next; streams arrive complete",async()=>{
  const p=pool();p.top();
  expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  const run=async(id:number,argv:string[],env:Record<string,string|null>={})=>{const sink=collect(),executor=p.take()!;expect(executor).toBeDefined();const result=await p.run(executor,{id,argv,cwd:process.cwd(),env},sink);return{...result,out:sink.out.join(""),err:sink.err.join("")};};
  const first=await run(1,["echo","a","b"],{FIXTURE_VAR:"v"});
  expect(first.exit).toBe(0);const a=JSON.parse(first.out);expect(a).toMatchObject({rest:["a","b"],env:"v",leaked:null,runs:1,opens:1});expect(first.err).toBe("err a b\n");
  const second=await run(2,["set"]);expect(second.exit).toBe(4);expect(second.out).toBe("set\n");
  const third=await run(3,["echo"]);const c=JSON.parse(third.out);expect(c).toMatchObject({env:null,leaked:null,runs:3,opens:1});expect(c.cwd).toBe(process.cwd());
  const big=await run(4,["big","3000000"]);expect(big.out.length).toBe(3000001);expect(big.out.endsWith("x\n")).toBe(true);
  const thrown=await run(5,["throw"]);expect(thrown.exit).toBe(1);expect(thrown.err).toBe("boom\n");
  await p.close();
 },60000);

 test("cancel interrupts the running command only; the executor serves the next one",async()=>{
  const p=pool();p.top();expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  const controller=new AbortController(),executor=p.take()!;
  const running=p.run(executor,{id:1,argv:["sleep","30000"]},{...collect(),signal:controller.signal});
  await sleep(300);const started=performance.now();controller.abort();
  const result=await running;expect(result.exit).toBe(130);expect(performance.now()-started).toBeLessThan(3000);
  expect(await waitFor(()=>p.idleCount()===1,3000)).toBe(true);
  const next=collect(),again=await p.run(p.take()!,{id:2,argv:["echo","after"]},next);expect(again.exit).toBe(0);expect(next.out.join("")).toContain("after");
  await p.close();
 },60000);

 test("an idle executor gives its custody back and exits; the next request starts a fresh one lazily",async()=>{
  const p=pool({FIXTURE_IDLE:"400"});p.top();expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  expect(await waitFor(()=>p.stats().executors===0,5000)).toBe(true);
  expect(p.take()).toBeUndefined();expect(p.stats().starts).toBe(2);
  expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  await p.close();
 },60000);

 test("a refusal of the service invocation answers that request only: the code travels, the mode stays on and the executor serves the next command",async()=>{
  const p=pool();p.top();expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  const refused=await p.run(p.take()!,{id:1,argv:["service-refuse"]},collect());
  expect(refused).toMatchObject({exit:70,refusal:"service",code:"expired",fields:{reason:"x"}});expect(p.refused).toBe(false);
  const next=collect();expect(await waitFor(()=>p.idleCount()===1,5000)).toBe(true);
  expect(await p.run(p.take()!,{id:2,argv:["echo","a"]},next)).toMatchObject({exit:0});expect(p.stats().starts).toBe(1);
  await p.close();
 },60000);

 test("an authority refusal turns the mode off for the pool; refused custody and the request limit retire executors",async()=>{
  const p=pool();p.top();expect(await waitFor(()=>p.idleCount()===1,20000)).toBe(true);
  const result=await p.run(p.take()!,{id:1,argv:["refuse"]},collect());
  expect(result).toMatchObject({exit:70,refusal:"authority"});expect(p.refused).toBe(true);expect(p.take()).toBeUndefined();
  await p.close();
  const busy=pool({FIXTURE_CUSTODY_FAIL:"1"});busy.top();await sleep(1500);expect(busy.idleCount()).toBe(0);await busy.close();
  const limited=pool({FIXTURE_MAX:"2"});limited.top();expect(await waitFor(()=>limited.idleCount()===1,20000)).toBe(true);
  await limited.run(limited.take()!,{id:1,argv:["echo"]},collect());await limited.run(limited.take()!,{id:2,argv:["echo"]},collect());
  expect(await waitFor(()=>limited.stats().executors===0,5000)).toBe(true);
  await limited.close();
 },60000);
});
