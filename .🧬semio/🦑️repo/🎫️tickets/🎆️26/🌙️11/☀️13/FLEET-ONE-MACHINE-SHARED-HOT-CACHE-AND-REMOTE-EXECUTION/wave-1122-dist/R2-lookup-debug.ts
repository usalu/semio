import {lookupUnitV1,digestFileV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
const [root,key,cwd,lane]=process.argv.slice(2) as [string,string,string,string];
const dir=join(root,"e",key.slice(0,2),key);
for(const name of readdirSync(dir)){
 const entry=JSON.parse(readFileSync(join(dir,name),"utf8"));
 for(const row of entry.dependencies)console.log(name.slice(0,15),"dep",row.path.slice(-60),digestFileV1(row.path,join(lane,"stamps"))?.slice(0,8),row.sha256?.slice(0,8));
}
console.log("lookup",!!lookupUnitV1(root,key,{cwd,laneRoot:lane,environment:{},stampDirectory:join(lane,"stamps")}));
