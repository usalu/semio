/** 🔍️ Lists bare workspace specifiers reachable from an entry through relative static and dynamic imports. */
import {readFileSync,existsSync} from "node:fs";
import {resolve,dirname} from "node:path";
const [entry,...prefixes]=process.argv.slice(2),seen=new Set<string>(),bare=new Map<string,Set<string>>(),queue=[resolve(entry!)];
const pattern=/(?:import|export)\b[^;]*?from\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|^\s*import\s*["']([^"']+)["']/gm;
while(queue.length){const file=queue.pop()!;if(seen.has(file)||!existsSync(file))continue;seen.add(file);const text=readFileSync(file,"utf8");for(const match of text.matchAll(pattern)){const specifier=match[1]??match[2]??match[3]!;if(specifier.startsWith(".")){const target=resolve(dirname(file),specifier.split("?")[0]!);if(/\.(ts|tsx|mts|js|mjs)$/.test(target))queue.push(target);}else if(prefixes.some(prefix=>specifier.startsWith(prefix))){if(!bare.has(specifier))bare.set(specifier,new Set());bare.get(specifier)!.add(file);}}}
console.log(JSON.stringify({files:seen.size,bare:Object.fromEntries([...bare].map(([key,value])=>[key,[...value]]))},null,1));
