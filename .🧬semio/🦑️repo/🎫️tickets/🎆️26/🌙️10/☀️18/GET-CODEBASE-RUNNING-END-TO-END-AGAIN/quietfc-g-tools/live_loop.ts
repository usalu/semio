// 🧪️ quietfc-g: one real door run (real policy, real gate, temp directory) that marks plan/execution, to see which classes a finished run teaches.
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const root=process.argv[2]!;
const {withBuildCapacityV1}=await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`);
const {upgradeBuildCapacityV1}=await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`);
const {markBuildCapacityPhaseV1}=await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🚏️marks/🟦️.ts`);
const directory=mkdtempSync(join(tmpdir(),"quietfc-"));
const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_BUILD_TOOL:"bazel"};
const args=["run","@teaching/architecture-course:build"];
const sleep=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));
await withBuildCapacityV1(args,environment,async granted=>{
 console.log("granted",granted.handle?.reservedMegabytes,granted.handle?.jobs);
 Object.assign(process.env,environment);
 await markBuildCapacityPhaseV1("plan",environment);await sleep(4500);
 if(process.argv[4]==="work")await upgradeBuildCapacityV1(undefined,environment);
 await markBuildCapacityPhaseV1("execution",environment);await sleep(Number(process.argv[3]??20000));
},{strip:a=>a});
const store=JSON.parse(readFileSync(join(directory,"estimates.json"),"utf8")).classes;
console.log(Object.entries(store).map(([k,v]:any)=>k+" "+JSON.stringify(v.samples.map((s:any)=>s.ratioBasisPoints))).join("\n"));
rmSync(directory,{recursive:true,force:true});
