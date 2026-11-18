const MB=1048576;
const mem=(l:string)=>{const u=process.memoryUsage();console.log(l.padEnd(18),"rss",Math.round(u.rss/MB),"heapUsed",Math.round(u.heapUsed/MB),"external",Math.round(u.external/MB),"ab",Math.round(u.arrayBuffers/MB),"guard",Math.round((u.heapUsed+u.external)/MB));};
const keep:any[]=[];
const target=Number(process.argv[2]??700);
while(process.memoryUsage().heapUsed/MB<target){for(let i=0;i<300;i++){keep.push({id:i,name:"p"+Math.random(),rows:["a"+i,"b"+i,{x:i}],map:new Map([[i,"v"+i]])});}}
mem("built");
if(process.argv[3]==="snap"){
 const t=performance.now();
 let s:any=(globalThis as any).Bun.generateHeapSnapshot();
 mem("snapshot "+Math.round(performance.now()-t)+"ms nodes="+(s.nodes.length/4)+" edges="+(s.edges.length/4));
 const rep=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🟨️.mjs");const t2=performance.now();const sum=rep.summarizeHeapSnapshotV1(s);mem("summarized "+Math.round(performance.now()-t2)+"ms");
 s=undefined;
 mem("dropped");
 (globalThis as any).Bun.gc(true);
 mem("gc");
 await new Promise(r=>setTimeout(r,3000));
 mem("3s later");
}
console.log(keep.length);
