/** 🧮️ Static census of the cache decisions the inference plugin emits for the synthesized playground grammar and the role:artifact TypeScript packages. Reads only the corpus, the audit census and tracked manifests; loads no graph. Run from the repository root: `bun <ticket>/plugin-cache-tools/census.ts`. */
import {existsSync,readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import corpus from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🔣️.json";
const ticket=import.meta.dir+"/..",audit=JSON.parse(readFileSync(ticket+"/🗑️generated/aud-cachecov.json","utf8"));
const variants:number=audit.synthesized.variants,row=corpus.playground,profiles=row.profiles.length;
const tally=new Map<string,number>();
const add=(key:string,count:number)=>tally.set(key,(tally.get(key)??0)+count);
for(const entry of row.perProfile)add(`${entry.class}/${entry.reason}/cache=${entry.cache}`,variants*profiles);
for(const entry of row.perVariant)add(`${entry.class}/${entry.reason}/cache=${entry.cache}`,variants);
add(`${row.sites.class}/${row.sites.reason}/cache=${row.sites.cache}`,variants);
add(`${row.sessions.class}/${row.sessions.reason}/cache=${row.sessions.cache}`,variants);
const perVariant=row.perProfile.length*profiles+row.perVariant.length+2;
console.log(`playground grammar: ${variants} variants x ${perVariant} targets = ${variants*perVariant} (audit counted ${audit.synthesized.perVariantSynthesizedTargets} per variant)`);
for(const[key,count]of[...tally].sort())console.log(String(count).padStart(6),key);
const files=execFileSync("git",["ls-files","--","*project.json"],{cwd:process.cwd(),encoding:"utf8",maxBuffer:1<<26}).split("\n").filter(Boolean);
const artifact={build:0,testCached:0,testPinnedOff:0,checkCached:0,checkOff:0};
for(const file of files){if(!existsSync(file))continue;let json:any;try{json=JSON.parse(readFileSync(file,"utf8"));}catch{continue;}
 if(!json.tags?.includes("role:artifact")||!json.tags.includes("language:typescript"))continue;
 const targets=json.targets??{};
 if(targets.build)artifact.build++;
 if(targets.test){if(targets.test.cache===false)artifact.testPinnedOff++;else artifact.testCached++;}
 if(targets.check){if(targets.check.cache===false)artifact.checkOff++;else artifact.checkCached++;}
}
console.log("role:artifact typescript packages",JSON.stringify(artifact));
console.log("  build -> managed-output/outputs-are-store-links (cache=false):",artifact.build);
console.log("  test  -> deterministic/check-without-outputs (cache=true, outputs=[]):",artifact.testCached,"| explicit cache=false kept:",artifact.testPinnedOff);
