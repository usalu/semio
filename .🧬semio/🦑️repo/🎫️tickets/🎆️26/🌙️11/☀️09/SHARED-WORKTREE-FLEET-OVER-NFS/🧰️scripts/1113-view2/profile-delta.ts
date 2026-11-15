const [moduleRoot,worktree]=process.argv.slice(2) as [string,string];
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const mb=(n:number)=>(n/1048576).toFixed(0);
let t=performance.now();
const full=(await git.gitViewV1(worktree))!;
console.log("full view ms",(performance.now()-t).toFixed(0),full.staged,full.modified,full.deleted,full.untracked,"maxRSS",mb(process.resourceUsage().maxRSS*1024));
const tracked=(await git.trackedPathsV1(worktree))!;
const paths=[...full.lines.slice(0,60).map((l:string)=>l.slice(3)),...Array.from({length:60},(_,i)=>tracked[(i*4519)%tracked.length]!)];
for(const n of [1,10,60,120]){
 t=performance.now();
 const delta=await git.gitViewDeltaV1(worktree,full,{paths:paths.slice(0,n),restage:false,trees:{before:null,after:null}});
 console.log("delta paths",n,"ms",(performance.now()-t).toFixed(0),"same digest",delta?.digest===full.digest,delta?.digest.slice(0,10),full.digest.slice(0,10));
}
t=performance.now();
const restage=await git.gitViewDeltaV1(worktree,full,{paths:[],restage:true,trees:{before:null,after:null}});
console.log("restage (x-diff only) ms",(performance.now()-t).toFixed(0),restage?.digest===full.digest);
console.log("maxRSS",mb(process.resourceUsage().maxRSS*1024));
