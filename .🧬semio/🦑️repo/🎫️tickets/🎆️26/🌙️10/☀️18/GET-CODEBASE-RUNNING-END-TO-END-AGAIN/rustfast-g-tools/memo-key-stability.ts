/** 🔬 rustfast-g: are the admission memo keys of one workspace stable across two identical `cargo build --unit-graph` runs? usage: bun memo-key-stability.ts <repo> <ug1.json> <ug2.json> <meta.json> */
import {readFileSync} from "node:fs";
const [repo,a,b,metaPath]=process.argv.slice(2);
const units=`${repo}/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts`;
const memo=`${repo}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts`;
const {readCargoCompilationGraphV1}=await import(units);
const {admissionMemoKeyV1}=await import(memo);
const meta=JSON.parse(readFileSync(metaPath,"utf8"));
const graph=(path:string)=>readCargoCompilationGraphV1(JSON.parse(readFileSync(path,"utf8")),meta,repo);
const g1=graph(a),g2=graph(b),policy={x:1},toolchain="t";
const key=(u:object)=>admissionMemoKeyV1(u,policy,toolchain);
const w1=g1.units.filter((u:any)=>u.provenance.kind==="workspace"),w2=new Map(g2.units.filter((u:any)=>u.provenance.kind==="workspace").map((u:any)=>[u.id,u]));
let same=0,differ=0,missing=0;const fields:Record<string,number>={};
for(const u of w1 as any[]){const o=w2.get(u.id) as any;if(!o){missing++;continue;}if(key(u)===key(o)){same++;continue;}differ++;for(const f of Object.keys(u))if(JSON.stringify(u[f])!==JSON.stringify(o[f]))fields[f]=(fields[f]??0)+1;}
console.log(JSON.stringify({workspaceUnits:w1.length,sameKey:same,differentKey:differ,idMissingInSecondRun:missing,differingFields:fields}));
