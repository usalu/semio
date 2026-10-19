import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve,join,relative} from "node:path";
const files=process.argv.slice(2);
let bad=0,total=0;
for(const f of files){
 const abs=resolve(f),dir=dirname(abs),text=readFileSync(abs,"utf8");
 const patterns=[/new URL\(\s*(["'])([^"']+)\1\s*,\s*import\.meta\.url/g,/(?:resolve|join)\(\s*import\.meta\.dir\s*,\s*(["'])([^"']+)\1/g,/(?:import|from)\s*\(?\s*(["'])(\.[^"']+)\1/g];
 for(const re of patterns)for(const m of text.matchAll(re)){
  const spec=m[2]!.split("?")[0]!;total++;
  const target=resolve(dir,spec);
  if(!existsSync(target)){bad++;console.log("MISSING",relative(process.cwd(),abs),"->",spec);}
 }
}
console.log("checked",total,"missing",bad);
