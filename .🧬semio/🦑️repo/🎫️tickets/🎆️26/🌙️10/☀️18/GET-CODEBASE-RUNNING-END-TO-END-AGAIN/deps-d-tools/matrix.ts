import {readFileSync} from "node:fs";
import {selectPackageManagerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const manager=selectPackageManagerV1("javascript");
const rows=JSON.parse(readFileSync(process.argv[2]!,"utf8")) as any[];
for(const row of rows){
  const map=new Map<string,unknown>(Object.entries(row.manifests).map(([k,v])=>[k==="."?"package.json":k+"/package.json",v]));
  const drift=manager.checkLock(row.lock,map);
  const frozen=drift.some(d=>d.frozen);
  const verdict=row.frozenReal;
  const flag=frozen&&!verdict?"FALSE-REFUSAL":(!frozen&&verdict)?"missed(tier2)":(drift.length===0&&verdict)?"MISSED-ALL":"ok";
  console.log(row.case.padEnd(52),"bun",verdict,"law drift",drift.length,"frozen",frozen,flag);
  if(process.argv[3])for(const d of drift)console.log("      ",JSON.stringify(d));
}
