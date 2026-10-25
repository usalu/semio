import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {selectPackageManagerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const manager=selectPackageManagerV1("javascript");
const scopes=[".","✏️s","🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand"];
const lockOverride=process.argv[2]?JSON.parse(process.argv[2]) as Record<string,string>:{};
for(const scope of scopes){
  const base=resolve(process.cwd(),scope),manifest=JSON.parse(readFileSync(join(base,"package.json"),"utf8"));
  const map=new Map<string,unknown>([["package.json",manifest]]);
  for(const m of manifest.workspaces??[])map.set(m.replace(/\/$/,"")+"/package.json",JSON.parse(readFileSync(join(base,m,"package.json"),"utf8")));
  const text=readFileSync(lockOverride[scope]??join(base,"bun.lock"),"utf8");
  const t=performance.now(),drift=manager.checkLock(text,map);
  console.log(scope,"members",map.size,"drift",drift.length,(performance.now()-t).toFixed(0)+"ms");
  for(const d of drift.slice(0,6))console.log("   ",JSON.stringify(d));
}
