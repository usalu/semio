export async function openExtension(name:string) {
 const worker = new Worker("/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js",{type:"module"});
 const actorId=name+"#request", activationGeneration=1n;
 let seq=0;
 const pending = new Map<string,{resolve:(v:any)=>void,reject:(e:Error)=>void}>();
 const send = (kind:string, extra:object) => new Promise<any>((resolve,reject)=>{
  const requestId=String(++seq);
  const timer=setTimeout(()=>{pending.delete(requestId);reject(Error(kind+" deadline"));worker.terminate();},60_000);
  pending.set(requestId,{resolve:value=>{clearTimeout(timer);resolve(value);},reject:error=>{clearTimeout(timer);reject(error);}});
  worker.postMessage({kind,requestId,actorId,activationGeneration,...extra});
 });
 let closed!: (value:boolean)=>void;
 const closure=new Promise<boolean>(resolve=>{closed=resolve;});
 worker.onmessage=({data})=>{
  if(data.kind==="original-source-close") closed(data.sourceClosed===true);
  if(data.kind!=="result")return;
  const waiting=pending.get(data.requestId);if(!waiting)return;
  pending.delete(data.requestId);if(data.ok)waiting.resolve(data.value);else waiting.reject(Error(data.error));
 };
 worker.onerror=e=>{for(const p of pending.values())p.reject(Error(e.message));};
 const grant={maximumItems:8388608,maximumCopyBytes:2147483648,maximumCapacityBytes:536870912,maximumReleaseBytes:536870912,maximumDepth:64};

 await send("activate",{moduleUrl:new URL("/🧩️extension-modules/"+name+"/🌉️bridge.js",location.href).href,originalAdmission:{operation:1n,generation:1n,grant,fuel:50000000,maximumElapsedUs:3600000000n}});
 return {
  submit:(events:unknown[],budget:object)=>send("turn",{events,budget}),
  close:async()=>{worker.postMessage({kind:"dispose",actorId,activationGeneration});let timer;try{return await Promise.race([closure,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error("close deadline")),15_000);})]);}finally{clearTimeout(timer);worker.terminate();}}
 };
}
Object.assign(globalThis,{openExtension});
