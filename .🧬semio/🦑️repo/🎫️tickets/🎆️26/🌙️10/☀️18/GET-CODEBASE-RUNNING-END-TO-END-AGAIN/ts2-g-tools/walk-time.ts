// ts2-g: time one workspaceTestAliasesV1 call (the vitest config's ownership walk) and count operations.
import {workspaceTestAliasesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/📦️workspace-exports/🟦️.ts";
const root=process.argv[2]??process.cwd(),owner=process.argv[3]??"🎓️teaching";
let ops=0,last="";
const t=performance.now(),c0=process.cpuUsage();
const aliases=await workspaceTestAliasesV1(root,{owner,advance:p=>{ops=p.completedOperations;last=p.operation;}});
const c=process.cpuUsage(c0);
console.log(JSON.stringify({owner,aliases:aliases.length,ops,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((c.user+c.system)/1000),rssMb:Math.round(process.memoryUsage.rss()/1048576)}));
