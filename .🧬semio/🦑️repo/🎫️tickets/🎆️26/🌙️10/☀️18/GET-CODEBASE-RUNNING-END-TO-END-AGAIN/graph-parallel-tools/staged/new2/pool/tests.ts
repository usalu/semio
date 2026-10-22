import{test,expect}from"bun:test";
import{closeGraphFarmsV1}from"../../🟨️.mjs";
import{startGraphPoolV1}from"../🟨️.mjs";

const service=new URL("../../🧪️tests/🧰️fake/🟨️.mjs",import.meta.url).href,forced={SEMIO_GRAPH_WORKERS:"2",SEMIO_HOST_CAPACITY:"off"};

test("a pool the environment switches off or the work does not justify opens to a refusal and starts no process",async()=>{
 const off=startGraphPoolV1({workspaceRoot:"/pool-law-off",projects:500,services:{law:service},environment:{SEMIO_GRAPH_WORKERS:"0"}});
 expect(off.plan.workers).toBe(0);expect(off.plan.reason).toBe("disabled");
 expect(await off.open()).toEqual({refused:"disabled"});
 const small=startGraphPoolV1({workspaceRoot:"/pool-law-small",projects:3,services:{law:service},environment:{}});
 expect(small.plan.workers).toBe(0);expect(small.plan.reason).toBe("small");expect(await small.open()).toEqual({refused:"small"});
});

test("a forced pool leases the resident pool of its workspace, serves jobs, and the next handle of the same workspace shares its workers",async()=>{
 const first=startGraphPoolV1({workspaceRoot:"/pool-law-shared",projects:500,services:{law:service},environment:forced}),opened=await first.open();
 try{
  expect(opened.refused).toBeUndefined();if(opened.refused!==undefined)return;
  expect(opened.workers).toBe(2);
  opened.farm.open("law",{identity:"law"},{accept:(_kind:string,identity:unknown)=>identity==="law"});
  const outcome=await opened.farm.infer("law",{n:7});
  expect((outcome as {reply:{value:{n:number}}}).reply.value.n).toBe(7);
  const second=startGraphPoolV1({workspaceRoot:"/pool-law-shared",projects:500,services:{law:service},environment:forced}),again=await second.open();
  try{expect(again.refused).toBeUndefined();expect(again.farm).toBe(opened.farm);}finally{again.release?.();}
  const other=startGraphPoolV1({workspaceRoot:"/pool-law-other",projects:500,services:{law:service},environment:forced}),elsewhere=await other.open();
  try{expect(elsewhere.farm).not.toBe(opened.farm);}finally{elsewhere.release?.();}
 }finally{opened.release?.();await closeGraphFarmsV1();}
},60000);

test("a prestarted pool boots every worker before any job exists",async()=>{
 const handle=startGraphPoolV1({workspaceRoot:"/pool-law-prestart",projects:500,services:{law:service},environment:forced,prestart:true}),opened=await handle.open();
 try{
  expect(opened.refused).toBeUndefined();if(opened.refused!==undefined)return;
  expect(opened.farm.stats().started).toBe(2);expect(opened.farm.stats().jobs).toBe(0);
  const idle=startGraphPoolV1({workspaceRoot:"/pool-law-lazy",projects:500,services:{law:service},environment:forced}),lazy=await idle.open();
  try{expect(lazy.refused).toBeUndefined();if(lazy.refused===undefined)expect(lazy.farm.stats().started).toBe(0);}finally{lazy.release?.();}
 }finally{opened.release?.();await closeGraphFarmsV1();}
},60000);

test("a worker count beyond the declared maximum is refused loudly",()=>{
 expect(()=>startGraphPoolV1({workspaceRoot:"/pool-law-capacity",projects:500,services:{law:service},environment:{...forced,SEMIO_GRAPH_WORKERS:"99"}})).toThrow("Graph workers of 0 to 12 required");
});
