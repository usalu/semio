const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const root="/Users/ueli/Documents/semio",seconds=Number(process.argv[2]??120);
const fleet=await import(B+"🌐️remote-cache/🛰️fleet/🟦️.ts");
const call=fleet.remoteCacheFetchV1({...process.env,SEMIO_WORKSPACE_ROOT:root},{root,audience:"execution",roleOf:()=>"submit"});
const seen=new Map<string,{first:number;started?:number;last:number;stage:string}>();
const end=Date.now()+seconds*1000;let polls=0;
while(Date.now()<end){
 const now=Date.now();
 try{
  const load=await(await call("http://192.168.178.70:8981/v1/fleet/load")).json();polls++;
  const present=new Set<string>();
  for(const task of load.tasks){
   present.add(task.id);
   const row=seen.get(task.id)??{first:now-task.elapsedSeconds*(task.stage==="QUEUED"?1000:0),last:now,stage:task.stage};
   row.last=now;row.stage=task.stage;if(task.stage==="EXECUTING"&&row.started===undefined)row.started=now;seen.set(task.id,row);
  }
 }catch(error){console.log("poll failed",(error as Error).message);}
 await Bun.sleep(1000);
}
const waits=[...seen.values()].filter(row=>row.started!==undefined).map(row=>row.started!-row.first).sort((a,b)=>a-b);
const gone=[...seen.values()].filter(row=>row.started===undefined&&row.stage==="QUEUED");
const q=(p:number)=>waits.length?waits[Math.min(waits.length-1,Math.floor(waits.length*p))]:null;
console.log(JSON.stringify({polls,tasks:seen.size,startedAfterQueue:waits.length,p50:q(.5),p90:q(.9),max:waits.at(-1)??null,leftQueuedWithoutStarting:gone.length,queuedAgesLeft:gone.map(row=>row.last-row.first).sort((a,b)=>a-b)}));
