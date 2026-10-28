/** ✅️ [isolate] scans the files of one batch (or a list file) in <root> with the mirror's law scanner and prints every remaining finding. usage: bun check.ts <root> <batch|list-file> */
import {readFileSync,existsSync} from "node:fs";
const [root,arg]=process.argv.slice(2),here=import.meta.dir,list=existsSync(arg!)?arg!:here+"/batches/"+arg+".txt";
const law=await import("/private/tmp/m/is1/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🟦️.ts");
let total=0;for(const rel of readFileSync(list,"utf8").split("\n").filter(Boolean)){const path=root+"/"+rel;if(!existsSync(path))continue;for(const finding of law.scanTestStoreReachesV1(readFileSync(path,"utf8"),undefined,rel)){total++;console.log(rel.split("/").slice(-4,-1).join("/")+":"+finding.line+" "+finding.rule+" "+finding.text.slice(0,150));}}
console.log("findings",total);
