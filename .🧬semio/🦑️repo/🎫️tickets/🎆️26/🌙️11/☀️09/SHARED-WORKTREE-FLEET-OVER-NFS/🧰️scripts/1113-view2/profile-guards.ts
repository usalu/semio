const [moduleRoot,worktree]=process.argv.slice(2) as [string,string];
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const mb=(n:number)=>(n/1048576).toFixed(0);
const show=(l:string)=>{(globalThis as any).Bun.gc(true);const m=process.memoryUsage();console.log(l.padEnd(24),"rss",mb(m.rss),"heapUsed",mb(m.heapUsed),"maxRSS",mb(process.resourceUsage().maxRSS*1024));};
show("start");
let t=performance.now();const u0=process.cpuUsage();
const guards=await git.gitGuardsV1(worktree,{});
const u=process.cpuUsage(u0);
console.log("guards ms",(performance.now()-t).toFixed(0),"cpu s",((u.user+u.system)/1e6).toFixed(1),guards?.trackedIgnored,guards?.caseMismatch,guards?.modeMismatch);show("guards");
