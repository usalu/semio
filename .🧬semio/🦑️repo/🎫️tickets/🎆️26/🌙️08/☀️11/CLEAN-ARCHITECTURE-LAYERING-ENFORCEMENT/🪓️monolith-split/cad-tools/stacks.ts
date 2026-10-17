const url=process.argv[2]!,count=Number(process.argv[3]??6),gap=Number(process.argv[4]??4000);
const ws=new WebSocket(url);let id=0;const pending=new Map<number,(v:any)=>void>();let onPause:((p:any)=>void)|undefined;
const send=(method:string,params:object={})=>new Promise<any>(res=>{const n=++id;pending.set(n,res);ws.send(JSON.stringify({id:n,method,params}));});
ws.onmessage=e=>{const m=JSON.parse(String(e.data));if(m.id&&pending.has(m.id)){pending.get(m.id)!(m);pending.delete(m.id);}else if(m.method==="Debugger.paused")onPause?.(m.params);};
await new Promise(r=>ws.onopen=r as any);
await send("Debugger.enable");
for(let i=0;i<count;i++){
  const paused=new Promise<any>(r=>onPause=r);
  await send("Debugger.pause");
  const p=await Promise.race([paused,new Promise(r=>setTimeout(()=>r(null),8000))]) as any;
  if(!p){console.log(`--- sample ${i}: no pause within 8s`);continue;}
  console.log(`--- sample ${i} (${new Date().toISOString()})`);
  for(const f of p.callFrames.slice(0,40))console.log(`  ${f.functionName||"<anon>"}  ${String(f.url??f.location?.scriptId).replace(/^file:\/\//,"").slice(-150)}:${f.location?.lineNumber}:${f.location?.columnNumber}`);
  await send("Debugger.resume");
  await new Promise(r=>setTimeout(r,gap));
}
ws.close();process.exit(0);
