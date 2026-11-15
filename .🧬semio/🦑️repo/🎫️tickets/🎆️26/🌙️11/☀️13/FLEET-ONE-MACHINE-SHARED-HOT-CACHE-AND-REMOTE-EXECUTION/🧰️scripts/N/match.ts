import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {fleetInstallLockV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts";
const [store,generations]=process.argv.slice(2) as [string,string];
const lockOf=(directory:string)=>fleetInstallLockV1(readFileSync(join(directory,"bun.lock"),"utf8")).sha256;
const mine=new Map(readdirSync(generations).map(name=>[lockOf(join(generations,name)),join(generations,name)]));
const folder=join(store,"v1","fleet","install");
for(const name of readdirSync(folder).filter(name=>name.startsWith("inst-")&&name.endsWith(".json"))){
 const manifest=JSON.parse(readFileSync(join(folder,name),"utf8"));
 console.log(JSON.stringify({manifest:join(folder,name),donor:mine.get(manifest.lock.sha256)??null,units:manifest.index.units,mb:Math.round(manifest.index.packedBytes/1e6)}));
}
