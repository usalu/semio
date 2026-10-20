// 📏 Counts the inventory files each profile of a rules document selects, and the extension mix of the heaviest.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readInputPrecisionV1,kindPatternsV1,inputGlobRegExpV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {staticHost} from "./static-host.ts";
const T=import.meta.dir,G=join(T,"..","🗑️generated","input-precision");
const doc=readInputPrecisionV1(JSON.parse(readFileSync(process.argv[2]??join(T,"staged2","rules.json"),"utf8")));
const inventory=readFileSync(join(G,"inventory.txt"),"utf8").split("\n").filter(p=>p&&!p.startsWith(".🧬semio/")&&!p.startsWith(".git/"));
const host=staticHost();
const select=(patterns:string[])=>{const pos=patterns.filter(p=>!p.startsWith("!")).map(p=>inputGlobRegExpV1(p.replace("{workspaceRoot}/",""))),neg=[...patterns.filter(p=>p.startsWith("!")).map(p=>p.slice(1)),...doc.exclusions.map(p=>p.slice(1))].map(p=>inputGlobRegExpV1(p.replace("{workspaceRoot}/","")));return inventory.filter(p=>pos.some(r=>r.test(p))&&!neg.some(r=>r.test(p)));};
const out:Record<string,number>={};
for(const [id,profile] of Object.entries(doc.profiles)){
 const patterns=profile.reads.flatMap(r=>"globs" in r?r.globs:"kinds" in r?kindPatternsV1(r,host):[]);
 const sel=select(patterns);out[id]=sel.length;
 if(process.argv[3]===id){const c:Record<string,number>={};for(const p of sel){const e=p.split(".").pop()!;c[e]=(c[e]??0)+1;}console.log(id,Object.entries(c).sort((a,b)=>b[1]-a[1]).slice(0,12));}
}
console.log(JSON.stringify(out));
