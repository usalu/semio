// 🔬️ Probe: how much of the build library does the graph inference closure import? Usage: node impl-closure-probe.mjs <entry.mjs> [<dir-to-test-membership> ...]
import{readFileSync,existsSync,statSync}from"node:fs";import{dirname,resolve}from"node:path";
const re=/(?:import|export)\s*(?:[^"';]*?from\s*)?["'](\.[^"']+)["']|import\(\s*["'](\.[^"']+)["']\s*\)/gu;
const seen=new Set(),todo=[resolve(process.argv[2])];
while(todo.length){const file=todo.pop();if(seen.has(file)||!existsSync(file)||!statSync(file).isFile())continue;seen.add(file);
 if(!/\.(mjs|js)$/u.test(file))continue;
 for(const m of readFileSync(file,"utf8").matchAll(re)){const spec=m[1]??m[2];todo.push(resolve(dirname(file),spec));}}
const list=[...seen];console.log(JSON.stringify({entry:process.argv[2],files:list.length}));
for(const dir of process.argv.slice(3)){const abs=resolve(dir);console.log(JSON.stringify({dir,inClosure:list.filter(f=>f.startsWith(abs+"/")).length}));}
