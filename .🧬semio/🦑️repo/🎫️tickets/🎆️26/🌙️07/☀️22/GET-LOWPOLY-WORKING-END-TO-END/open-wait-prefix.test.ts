import{test,expect}from"bun:test";
import{resolve}from"node:path";import{pathToFileURL}from"node:url";import graphlib from"graphlib";import Ajv from"ajv";
import fixture from"./open-wait-prefix.json";import schema from"./open-wait-prefix.schema.json";
const owner=resolve("🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process");
const api=await import(pathToFileURL(owner+"/🟦️.ts").href);const{beginAdmissionWaitV1,admissionWaitedMillisecondsV1}=await import(pathToFileURL(owner+"/🐕️watchdog/🟦️.ts").href);
test("every observed clock position preserves genuine open admission credit",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const graph=new graphlib.Graph({directed:true});for(const[id,budget]of[["p",60],["q",60000],["a",0],["b",0],["c",0]]as const)graph.setNode(id,budget);for(const[from,to]of[["c","p"],["c","b"],["b","q"],["b","a"],["a","p"]])graph.setEdge(from!,to!);
 const minimum=Math.min(...graphlib.alg.preorder(graph,"c").map(id=>graph.node(id)as number).filter(value=>value>0));expect(minimum).toBe(fixture.shortMilliseconds);const measured=[];
 for(const delayRead of fixture.delayReads){let row:any;
 await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:fixture.owner,maximumElapsedMilliseconds:0},{fixture:"open-wait-prefix"},Date.now()),async(original:any)=>{
  const p=api.createIssuedResourceControlV1(original,"short",fixture.shortMilliseconds),q=api.createIssuedResourceControlV1(original,"long",fixture.longMilliseconds);
  await api.withRequestedIssuedResourceControlV1(p,0,async(a:any)=>api.withRequestedIssuedResourceControlV1(q,0,async(b:any)=>api.withRequestedIssuedResourceControlV1(p,0,async(c:any)=>{
   const before=admissionWaitedMillisecondsV1(),end=beginAdmissionWaitV1(),clock=Date.now;let reads=0,delayed=false,left:number|null|undefined,reason:string|undefined;
   try{Date.now=()=>{if(++reads===delayRead){const began=performance.now();while(performance.now()-began<fixture.delayMilliseconds){}delayed=true;}return clock();};left=c.remainingMilliseconds();}catch(error){reason=String(error);}finally{Date.now=clock;end();}
   row={delayRead,reads,delayed,left,reason,waited:admissionWaitedMillisecondsV1()-before,authorized:left===null||typeof left==="number"&&left>0};
  },undefined,b),undefined,a));
 });measured.push(row);
 }
 console.log("[DEBUG] real open-wait clock-position results "+JSON.stringify(measured));expect(measured.some(row=>row.delayed)).toBe(true);
 for(const row of measured){if(row.delayed){expect(row.waited).toBeGreaterThanOrEqual(fixture.delayMilliseconds);expect(row.authorized).toBe(fixture.expectedAuthorized);}if(typeof row.left==="number")expect(row.left).toBeLessThanOrEqual(minimum);}
});
