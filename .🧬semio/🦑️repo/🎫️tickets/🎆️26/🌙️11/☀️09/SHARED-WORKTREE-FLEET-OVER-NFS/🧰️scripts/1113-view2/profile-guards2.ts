const [moduleRoot,worktree]=process.argv.slice(2) as [string,string];
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const t=performance.now();
const g=await git.gitGuardsV1(worktree,{});
console.log("guards ms",(performance.now()-t).toFixed(0),"maxRSS",Math.round(process.resourceUsage().maxRSS/1048576));
