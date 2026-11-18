/** 🪪️ Contract: every resident service records why it lives (its route, the pid of its owner and the idle seconds after which it ends); a service that a repo command started implicitly ends after its short idle time, a service kept by a launchd job or run in the foreground keeps the long one.
 * @see ../../🏛️service/🌐️host/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {requestHostStatusV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","oi-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));

const make=(name:string,route:string,state?:string)=>{
 const directory=join(base,name);mkdirSync(directory,{recursive:true});
 const environment={XDG_RUNTIME_DIR:directory,TMPDIR:directory,LOCALAPPDATA:directory,SEMIO_SERVICE_ROUTE:route,...state?{SEMIO_MACHINE_STATE_DIRECTORY:state}:{}} as Record<string,string>,implementation=createHash("sha256").update(name).digest("hex"),runtime=hostEndpointsV1(implementation,environment);
 const planner=createGraphPlannerPortV1({loadSlice:async()=>({graph:{version:1,projects:{},dependencies:[]}})});
 const service=createHostServiceV1({runtime,implementation,environment,ports:{planner,runInFlight:async()=>false},policy:{residentCheckMilliseconds:50,idleShutdownMilliseconds:60000,implicitIdleShutdownMilliseconds:300}} as never);
 return{service,runtime,environment,implementation};
};

test("a service started implicitly by a command ends after the short idle time and says why it lives",async()=>{
 const h=make("implicit","direct");await h.service.start();
 const exits:string[]=[];void h.service.closed.then(reason=>exits.push(String(reason)));
 const descriptor=JSON.parse(readFileSync(h.runtime.descriptor,"utf8"));
 expect(descriptor.owner).toMatchObject({route:"direct",idleSeconds:1});expect(descriptor.owner.pid).toBeGreaterThan(0);
 const status:any=await requestHostStatusV1({implementation:h.implementation,environment:h.environment} as never);
 expect(status.owner.route).toBe("direct");expect(status.identity.tag).toBe(h.runtime.tag);
 for(let at=0;at<100&&exits.length===0;at++)await sleep(50);
 expect(exits).toEqual(["idle"]);
});

test("a service kept by a launchd job or run in the foreground keeps the long idle time",async()=>{
 for(const route of["launchd","foreground"]){
  const h=make(route,route);await h.service.start();
  const exits:string[]=[];void h.service.closed.then(reason=>exits.push(String(reason)));
  await sleep(1200);
  expect(exits).toEqual([]);
  expect(JSON.parse(readFileSync(h.runtime.descriptor,"utf8")).owner).toMatchObject({route,idleSeconds:60});
  await h.service.stop("test");
 }
});

test("two machine state roots on one checkout run two services side by side and each answers with its own identity",async()=>{
 const one=make("sa","foreground",join(base,"state-one")),two=make("sa","foreground",join(base,"state-two"));
 expect(one.runtime.endpoint).not.toBe(two.runtime.endpoint);
 await one.service.start();await two.service.start();
 try{
  const first:any=await requestHostStatusV1({implementation:one.implementation,environment:one.environment} as never),second:any=await requestHostStatusV1({implementation:two.implementation,environment:two.environment} as never);
  expect(first.identity.state).toBe(join(base,"state-one"));expect(second.identity.state).toBe(join(base,"state-two"));
  expect(first.identity.root).toBe(second.identity.root);expect(first.identity.tag).not.toBe(second.identity.tag);
  expect(first.instance).not.toBe(second.instance);
 }finally{await one.service.stop("test");await two.service.stop("test");}
});
