/** 🔎 Lists the observations of the print preparation witness that no longer hold and times a full replay of the holding rest (read-only). `bun witfail.ts <root>` */
import {readFileSync} from "node:fs";
const root=process.argv[2]!,L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const w=await import(L+"/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts"),prep=await import(L+"/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts");
const paths=w.cargoPreparationRecordPathsV1(prep.repositoryCargoPreparationStorageV1(root),root,"Cargo.toml",["semio-framework-print-tests"]);
const a0=performance.now(),witness=JSON.parse(readFileSync(paths.witness,"utf8"));console.log(`[DEBUG] parse ${(performance.now()-a0).toFixed(0)} ms`);
const rows=witness.observations as any[];const bad:any[]=[];
for(const row of rows)if(!w.cargoPhysicalObservationsHoldV1(root,[row]))bad.push(row);
console.log("[DEBUG] failing",bad.length,"of",rows.length);for(const b of bad.slice(0,10))console.log("[DEBUG]",b.operation,b.path.replace(root+"/",""));
const good=rows.filter(r=>!bad.includes(r));for(let i=0;i<2;i++){const a=performance.now(),ok=w.cargoPhysicalObservationsHoldV1(root,good);console.log(`[DEBUG] replay of ${good.length}: ${ok} ${(performance.now()-a).toFixed(0)} ms`);}
