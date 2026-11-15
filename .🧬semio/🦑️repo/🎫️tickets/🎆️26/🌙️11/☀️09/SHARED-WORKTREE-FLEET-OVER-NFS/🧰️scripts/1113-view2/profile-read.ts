const [moduleRoot,mirror]=process.argv.slice(2) as [string,string];
const {readCheckpointV1}=await import(`${moduleRoot}/🔁️replica/💾️state/🟦️.ts`);
const mb=(n:number)=>(n/1048576).toFixed(0);
const show=(l:string)=>{(globalThis as any).Bun.gc(true);const m=process.memoryUsage();console.log(l.padEnd(20),"rss",mb(m.rss),"ab",mb(m.arrayBuffers));};
show("start");
const keep:Uint8Array[]=[];let total=0;
await readCheckpointV1(`${mirror}/.cache/f/state.ckp`,(name:string,length:number)=>{const a=new Uint8Array(length);keep.push(a);total+=length;return a;});
show("read parts");console.log("payload MB",mb(total));
keep.length=0;show("released");
