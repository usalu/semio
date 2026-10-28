// ts2-g: the vitest ownership walk on the real tree with the validated memo: first call walks and seals, later calls answer from the memo.
import {workspaceTestAliasesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts";
const root=process.argv[2]??process.cwd(),cache=process.argv[3],owner="🎓️teaching",rounds=Number(process.argv[4]??3);
for(let round=0;round<rounds;round++){
 let source="",operations=0;const t=performance.now(),c0=process.cpuUsage();
 const aliases=await workspaceTestAliasesV1(root,{owner,...(cache?{cache}:{}),report:row=>{source=row.source;operations=row.operations;},advance:()=>{}});
 const c=process.cpuUsage(c0);
 console.log(JSON.stringify({round,source,aliases:aliases.length,operations,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((c.user+c.system)/1000),rssMb:Math.round(process.memoryUsage.rss()/1048576)}));
}
