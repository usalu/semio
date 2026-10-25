import {readFileSync,existsSync} from "node:fs";
import {join,relative,resolve} from "node:path";
import {selectPackageManagerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

const root=process.cwd(),manager=selectPackageManagerV1("javascript");
const scopes=[".","✏️s","🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"];
const sections=["dependencies","devDependencies","optionalDependencies","peerDependencies"];
for(const scope of scopes){
  const base=resolve(root,scope),manifest=JSON.parse(readFileSync(join(base,"package.json"),"utf8")),lock=manager.parseLockDocument(readFileSync(join(base,"bun.lock"),"utf8")) as any;
  const members:string[]=manifest.workspaces??[],keys=new Set(Object.keys(lock.workspaces));
  const out:string[]=[];
  const rows=[["",manifest],...members.map(m=>[m.replace(/\/$/,""),JSON.parse(readFileSync(join(base,m,"package.json"),"utf8"))] as [string,any])] as [string,any][];
  for(const [key,doc] of rows){
    const row=lock.workspaces[key];keys.delete(key);
    if(!row){out.push("missing-workspace "+key);continue;}
    if((doc.name??undefined)!==(row.name??undefined))out.push(`name ${key}: ${doc.name} vs ${row.name}`);
    for(const s of sections){const a=doc[s]??{},b=row[s]??{};for(const n of new Set([...Object.keys(a),...Object.keys(b)]))if(a[n]!==b[n])out.push(`${s} ${key}: ${n} manifest=${a[n]} lock=${b[n]}`);}
  }
  for(const k of keys)out.push("stale-workspace "+k);
  console.log(scope,"members",members.length,"lockWorkspaces",Object.keys(lock.workspaces).length,"drift",out.length);for(const l of out.slice(0,12))console.log("   ",l);
}
