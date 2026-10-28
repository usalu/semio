test("a request leaves no listener on the long-lived signal of its invocation once it is released, so one farm per chunk never pins the environment of every earlier chunk",async()=>{
 const sample=async()=>({host:host(),swappedOutMegabytes:0}),listeners=(signal:AbortSignal)=>getEventListeners(signal,"abort").length;
 for(const gate of["on","off","waiting"] as const){
  const directory=fresh(),controller=new AbortController();
  for(let round=0;round<5;round++){
   const request=requestBuildFarmCapacityV1({preferredJobs:6,environment:gate==="off"?{SEMIO_HOST_CAPACITY:"off"}:{},directory,policy:fast,write:()=>{},signal:controller.signal,sample:gate==="waiting"?async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0}):sample});
   if(gate==="waiting")await new Promise(done=>setTimeout(done,50));else expect(await request.granted).toBeDefined();
   request.release();await request.granted;
  }
  expect(listeners(controller.signal)).toBe(0);
 }
},60000);
