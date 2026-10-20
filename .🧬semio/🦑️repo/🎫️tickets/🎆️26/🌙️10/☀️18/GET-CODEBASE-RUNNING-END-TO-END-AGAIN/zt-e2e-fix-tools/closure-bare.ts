/** 🔍️ Lists bare specifiers in the relative-import closure of an entry as the bundler sees them: TypeScript is transpiled first so type-only and unused imports are erased; edges are split into static and dynamic. */
import {readFileSync,existsSync,statSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const [entry,mode="all",follow="all"]=process.argv.slice(2),root=process.cwd(),seen=new Set<string>(),queue=[resolve(entry!)],rows:{file:string,specifier:string,kind:string}[]=[];
const ts=new Bun.Transpiler({loader:"ts"}),tsx=new Bun.Transpiler({loader:"tsx"});
const edge=/^(?:import|export)\b[^"'`;]*?\bfrom\s*["']([^"']+)["']|^import\s*["']([^"']+)["']|(?<![\w$."'`])import\(\s*["']([^"']+)["']\s*\)/gm;
const builtin=/^(node:|bun:)|^(fs|path|os|child_process|crypto|url|util|stream|events|zlib|http|https|net|readline|worker_threads|module|assert|buffer|process|timers|tty|v8|vm|perf_hooks|async_hooks|dns|dgram|cluster|querystring|string_decoder|constants)(\/.*)?$/;
const candidates=(target:string)=>[target,`${target}.ts`,`${target}.tsx`,`${target}.js`,`${target}.mjs`,`${target}/index.ts`];
while(queue.length){
  const file=queue.pop()!;
  if(seen.has(file)||!existsSync(file)||!statSync(file).isFile())continue;
  seen.add(file);
  if(!/\.(ts|tsx|mts|js|mjs|cjs)$/.test(file)||/\.d\.m?ts$/.test(file))continue;
  let text=readFileSync(file,"utf8").replace(/^#!.*\n/,"\n");
  try{if(/\.(ts|mts)$/.test(file))text=ts.transformSync(text);else if(file.endsWith(".tsx"))text=tsx.transformSync(text);}catch{console.error("transform failed",file)}
  for(const match of text.matchAll(edge)){
    const specifier=(match[1]??match[2]??match[3])!,kind=match[3]?"dynamic":"static";
    if(specifier.startsWith(".")){
      const base=resolve(dirname(file),specifier.split("?")[0]!);
      const hit=candidates(base).find(c=>existsSync(c)&&statSync(c).isFile());
      if(hit&&(follow==="all"||kind==="static"))queue.push(hit);
    }else if(!builtin.test(specifier))rows.push({file:relative(root,file),specifier,kind});
  }
}
const filtered=rows.filter(row=>mode==="all"||row.kind===mode);
console.log(JSON.stringify({files:seen.size,count:filtered.length,rows:filtered},null,1));
