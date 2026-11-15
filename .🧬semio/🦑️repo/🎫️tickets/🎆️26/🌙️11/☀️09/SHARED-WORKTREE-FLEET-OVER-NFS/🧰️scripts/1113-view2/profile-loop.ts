const [moduleRoot,worktree,which]=process.argv.slice(2) as [string,string,string];
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const b=await import(`${moduleRoot}/🚧️boundary/🟦️.ts`);
const tracked=(await git.trackedEntriesV1(worktree))!;
const mb=()=>Math.round(process.memoryUsage().rss/1048576);
console.log("base",mb());
const budget=b.pathBudgetV1(process.platform,false);
let t=performance.now(),n=0;
if(which==="fold"){const m=new Map<string,number>();for(const e of tracked){const k=b.foldNameV1(e.path);m.set(k,(m.get(k)??0)+1);}n=m.size;}
if(which==="refuse"){for(const e of tracked)if(b.refusePathV1(e.path)!==undefined)n++;}
if(which==="units"){for(const e of tracked)if(b.absoluteUnitsV1(budget,worktree,e.path)>budget.limit)n++;}
console.log(which,"ms",(performance.now()-t).toFixed(0),"rss",mb(),n);
