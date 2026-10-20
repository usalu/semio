/** 🔍️ Lists every file reachable from an entry through relative imports, plus relative non-code assets, as repo-relative paths. */
import {readFileSync,existsSync,statSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const [entry]=process.argv.slice(2),root=process.cwd(),seen=new Set<string>(),assets=new Set<string>(),queue=[resolve(entry!)];
const pattern=/(?:import|export)\b[^;]*?from\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|^\s*import\s*["']([^"']+)["']/gm;
while(queue.length){const file=queue.pop()!;if(seen.has(file)||!existsSync(file))continue;seen.add(file);const text=readFileSync(file,"utf8");for(const match of text.matchAll(pattern)){const specifier=match[1]??match[2]??match[3]!;if(!specifier.startsWith("."))continue;const target=resolve(dirname(file),specifier.split("?")[0]!);if(/\.(ts|tsx|mts|js|mjs)$/.test(target))queue.push(target);else if(existsSync(target)&&statSync(target).isFile())assets.add(target);}}
for(const file of [...seen,...assets])console.log(relative(root,file));
