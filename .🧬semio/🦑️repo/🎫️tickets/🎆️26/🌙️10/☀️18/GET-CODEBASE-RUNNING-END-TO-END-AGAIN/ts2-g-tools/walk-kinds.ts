import {workspaceTestAliasesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/📦️workspace-exports/🟦️.ts";
const kinds:Record<string,number>={};let lastT=performance.now();const time:Record<string,number>={};
await workspaceTestAliasesV1(process.cwd(),{owner:"🎓️teaching",advance:p=>{const k=p.stage+":"+p.operation;kinds[k]=(kinds[k]??0)+1;const n=performance.now();time[k]=(time[k]??0)+(n-lastT);lastT=n;}});
console.log(JSON.stringify({kinds,timeMs:Object.fromEntries(Object.entries(time).map(([k,v])=>[k,Math.round(v)]))}));
