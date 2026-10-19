/** 🧭️ Lists unresolved relative imports reachable from entries (static and dynamic), with the importing line. */
import {readFileSync,existsSync,statSync} from "node:fs";
import {resolve,dirname} from "node:path";
const entries=process.argv.slice(2),seen=new Set<string>(),queue=entries.map(value=>resolve(value)),missing:string[]=[];
const pattern=/(?:import|export)\b[^;]*?from\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|^\s*import\s*["']([^"']+)["']/gm;
while(queue.length){const file=queue.pop()!;if(seen.has(file))continue;seen.add(file);let text:string;try{text=readFileSync(file,"utf8");}catch{continue;}
for(const match of text.matchAll(pattern)){const specifier=match[1]??match[2]??match[3]!;if(!specifier.startsWith("."))continue;const target=resolve(dirname(file),specifier.split("?")[0]!);
if(!existsSync(target)||statSync(target).isDirectory()){missing.push(file+" -> "+specifier);continue;}if(/\.(ts|tsx|mts|js|mjs)$/.test(target))queue.push(target);}}
console.log(JSON.stringify({files:seen.size,missing},null,1));
