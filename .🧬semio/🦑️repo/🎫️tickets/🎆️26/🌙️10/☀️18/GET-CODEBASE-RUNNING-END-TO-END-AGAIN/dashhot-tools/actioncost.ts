/** ⏱️ Cost of the new short-circuit inside the build action, read-only: imports the dashboard task routes of <code-root> (with batch `fresh`) and runs capture + `currentDashboardV1` against <data-root>. `bun actioncost.ts <code-root> <data-root>` */
import {join} from "node:path";
const [code,data]=process.argv.slice(2);const t0=performance.now();
const routes=await import(join(code!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts"));
const install=await import(join(code!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"));
const t1=performance.now();
const sources=await install.captureDashboardSources(join(data!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust"),data!);
const t2=performance.now();
let answer;try{answer=install.currentDashboardV1(data!,"prototype",data!,sources);}catch(error){answer=String(error);}
const t3=performance.now();
console.log(JSON.stringify({routesLoaded:typeof routes.buildAndInstallDashboard,ms:{import:Math.round(t1-t0),capture:Math.round(t2-t1),current:Math.round(t3-t2)},files:Number.parseInt(sources.digest),answer:answer===undefined?"not current (live record is v3, no compiled sources yet)":answer,rssMb:Math.round(process.memoryUsage().rss/1048576)}));
