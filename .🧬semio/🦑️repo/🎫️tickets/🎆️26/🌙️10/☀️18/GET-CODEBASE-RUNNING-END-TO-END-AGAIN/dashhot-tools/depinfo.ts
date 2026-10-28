/** 🧮 Read-only: applies the new compiled-source rule of <code-root> to the REAL dep-info of the installed dashboard binary on <data-root>, with the build start = the dep-info file time; prints count, first changed source and cost. `bun depinfo.ts <code-root> <data-root> <semio.d>` */
import {join,relative,resolve} from "node:path";
import {readFileSync,statSync} from "node:fs";
const [code,data,depinfo]=process.argv.slice(2);
const install=await import(join(code!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"));
const {cargoDepInfoSourcesV1}=await import(join(code!,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts"));
const sources=[...new Set(cargoDepInfoSourcesV1(readFileSync(depinfo!,"utf8")).map((source:string)=>relative(data!,resolve(data!,source)).replaceAll("\\","/")))].sort() as string[];
const builtAtMs=statSync(depinfo!).mtimeMs,t0=performance.now();const changed=install.changedDashboardSourceV1(data!,{builtAtMs,sources});const t1=performance.now();
console.log(JSON.stringify({sources:sources.length,outsideRoots:sources.filter(path=>path.includes("📚️library/")||path.includes("🏃️process/")).length,builtAt:new Date(builtAtMs).toISOString(),changed:changed??null,ms:+(t1-t0).toFixed(1)}));
