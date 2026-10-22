// usage (inside the overlay): bun bench.ts   prints latency and throughput of the service on this machine (indicative only; the host is shared with a fleet)
import {createHash} from "node:crypto";
import {startClusterV1} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts";
const sha=(data:Uint8Array)=>createHash("sha256").update(data).digest("hex"),NS="build.bazel.remote.execution.v2";
const quantile=(rows:number[],q:number)=>[...rows].sort((a,b)=>a-b)[Math.min(rows.length-1,Math.floor(rows.length*q))]!;
for(const sandbox of["isolated-root","auto"] as const){
 const cluster=await startClusterV1({workers:0});
 cluster.startWorker({name:"b1",concurrency:4,sandbox});cluster.startWorker({name:"b2",concurrency:4,sandbox});
 while(cluster.server.scheduler.status().workers.length<2)await Bun.sleep(20);
 const count=120,latencies:number[]=[],started=performance.now();
 let next=0;
 await Promise.all(Array.from({length:8},async()=>{for(;;){const index=next++;if(index>=count)return;const action=await cluster.action({arguments:["/bin/sh","-c","echo $0 > out.txt","x"+index],files:{"in.txt":"data"+index},outputPaths:["out.txt"],salt:"bench"+index}),begin=performance.now();await cluster.execute(action);latencies.push(performance.now()-begin);}}));
 const seconds=(performance.now()-started)/1000;
 console.log(`${sandbox}: ${count} distinct actions on 2 workers x 4 slots, 8 clients: ${(count/seconds).toFixed(1)} actions/s, latency p50 ${quantile(latencies,0.5).toFixed(0)} ms p95 ${quantile(latencies,0.95).toFixed(0)} ms`);
 const cached=await cluster.action({arguments:["/bin/echo","cached"],salt:"cached"});await cluster.execute(cached);
 const hits:number[]=[];for(let index=0;index<50;index++){const begin=performance.now();await cluster.execute(cached);hits.push(performance.now()-begin);}
 console.log(`${sandbox}: action cache hit through Execute: p50 ${quantile(hits,0.5).toFixed(1)} ms`);
 if(sandbox==="isolated-root"){
  const digests=Array.from({length:5000},(_,index)=>({hash:sha(new TextEncoder().encode("missing"+index)),size_bytes:7}));
  let begin=performance.now();await cluster.client.unary(`${NS}.ContentAddressableStorage/FindMissingBlobs`,{instance_name:"main",blob_digests:digests});console.log(`FindMissingBlobs of 5000 absent digests: ${(performance.now()-begin).toFixed(0)} ms`);
  const blob=new Uint8Array(100*1048576).map((_,index)=>(index*131+7)&255),digest={hash:sha(blob),size_bytes:blob.length};
  begin=performance.now();await cluster.client.upload("google.bytestream.ByteStream/Write",function*(){for(let offset=0;offset<blob.length;offset+=65536)yield {resource_name:offset===0?`main/uploads/b/blobs/${digest.hash}/${blob.length}`:"",write_offset:offset,data:blob.subarray(offset,offset+65536),finish_write:offset+65536>=blob.length};}());
  console.log(`ByteStream write 100 MiB (64 KiB frames): ${((performance.now()-begin)/1000).toFixed(2)} s`);
  begin=performance.now();let total=0;for await(const piece of cluster.client.stream("google.bytestream.ByteStream/Read",{resource_name:`main/blobs/${digest.hash}/${blob.length}`}))total+=piece.data.length;console.log(`ByteStream read 100 MiB: ${((performance.now()-begin)/1000).toFixed(2)} s (${total} bytes)`);
 }
 await cluster.stop();
}
