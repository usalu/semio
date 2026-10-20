/** 🔗️ Plans relative-import replacements for bare workspace specifiers that a bundler sees in the closure of the given entries; reads the live tree, writes only an edits JSON (path, base sha256, anchored replacements). */
import {readFileSync,writeFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {resolve,dirname,relative,join,sep} from "node:path";
import {createHash} from "node:crypto";
const [rootArgument,outputArgument,...entryArguments]=process.argv.slice(2),root=resolve(rootArgument!),entries=entryArguments.map(value=>resolve(root,value));
const packages=new Map<string,{directory:string;exports:Record<string,unknown>}>();
const walk=(directory:string,depth:number)=>{
  if(depth>14)return;
  let names:string[];
  try{names=readdirSync(directory);}catch{return;}
  for(const name of names){
    if(name==="node_modules"||name.startsWith(".")||name==="dist"||name==="target")continue;
    const path=join(directory,name);
    let state;
    try{state=statSync(path);}catch{continue;}
    if(state.isDirectory())walk(path,depth+1);
    else if(name==="package.json"){
      try{const json=JSON.parse(readFileSync(path,"utf8"));if(typeof json.name==="string"&&json.exports)packages.set(json.name,{directory,exports:typeof json.exports==="string"?{".":json.exports}:json.exports});}catch{}
    }
  }
};
walk(join(root,"🧰️framework"),0);
const target=(value:unknown):string|undefined=>typeof value==="string"?value:value&&typeof value==="object"?target((value as Record<string,unknown>).bun??(value as Record<string,unknown>).import??(value as Record<string,unknown>).default):undefined;
const resolveBare=(specifier:string):string|undefined=>{
  const names=[...packages.keys()].filter(name=>specifier===name||specifier.startsWith(name+"/")).sort((a,b)=>b.length-a.length);
  for(const name of names){
    const pkg=packages.get(name)!,key="."+specifier.slice(name.length);
    let value=target(pkg.exports[key]);
    if(!value)for(const [pattern,candidate] of Object.entries(pkg.exports)){
      const star=pattern.indexOf("*");
      if(star<0||!key.startsWith(pattern.slice(0,star))||!key.endsWith(pattern.slice(star+1)))continue;
      const captured=key.slice(star,key.length-(pattern.length-star-1)),raw=target(candidate);
      if(raw)value=raw.replaceAll("*",captured);
    }
    if(value)return resolve(pkg.directory,value);
  }
  return undefined;
};
const ts=new Bun.Transpiler({loader:"ts"}),tsx=new Bun.Transpiler({loader:"tsx"});
const edge=/^(?:import|export)\b[^"'`;]*?\bfrom\s*["']([^"']+)["']|^import\s*["']([^"']+)["']|(?<![\w$."'`])import\(\s*["']([^"']+)["']\s*\)/gm;
const builtin=/^(node:|bun:)|^(fs|path|os|child_process|crypto|url|util|stream|events|zlib|http|https|net|readline|worker_threads|module|assert|buffer|process|timers|tty|v8|vm|perf_hooks|async_hooks|dns|dgram|cluster|querystring|string_decoder|constants)(\/.*)?$/;
const candidates=(base:string)=>[base,`${base}.ts`,`${base}.tsx`,`${base}.js`,`${base}.mjs`,`${base}/index.ts`];
const original=new Map<string,string>(),current=new Map<string,string>(),edits=new Map<string,{old:string;replacement:string;count:number}[]>(),external=new Map<string,Set<string>>(),unresolved:string[]=[];
const escape=(value:string)=>value.replace(/[.*+?^${}()|[\]\\]/g,"\\$&");
const queue=[...entries],seen=new Set<string>();
while(queue.length){
  const file=queue.pop()!;
  if(seen.has(file)||!existsSync(file)||!statSync(file).isFile())continue;
  seen.add(file);
  if(!/\.(ts|tsx|mts|js|mjs|cjs)$/.test(file)||/\.d\.m?ts$/.test(file))continue;
  const raw=readFileSync(file,"utf8");
  original.set(file,raw);
  let text=raw;
  const bare=new Set<string>();
  let compiled=raw.replace(/^#!.*\n/,"\n");
  try{if(/\.(ts|mts)$/.test(file))compiled=ts.transformSync(compiled);else if(file.endsWith(".tsx"))compiled=tsx.transformSync(compiled);}catch{}
  const relatives:string[]=[];
  for(const match of compiled.matchAll(edge)){
    const specifier=(match[1]??match[2]??match[3])!;
    if(specifier.startsWith("."))relatives.push(specifier);
    else if(!builtin.test(specifier))bare.add(specifier);
  }
  for(const specifier of bare){
    const resolved=resolveBare(specifier);
    if(!resolved||!existsSync(resolved)){
      if(!external.has(specifier))external.set(specifier,new Set());
      external.get(specifier)!.add(relative(root,file));
      if(specifier.startsWith("@semio-tech/"))unresolved.push(`${relative(root,file)} -> ${specifier}`);
      continue;
    }
    let path=relative(dirname(file),resolved).split(sep).join("/");
    if(!path.startsWith("."))path="./"+path;
    const pattern=new RegExp(`(\\bfrom\\s*|\\bimport\\(\\s*|\\bimport\\s*)(["'])${escape(specifier)}\\2`,"g");
    const rows=new Map<string,{old:string;replacement:string;count:number}>();
    text=text.replace(pattern,(whole,head,quote)=>{const replacement=head+quote+path+quote,row=rows.get(whole)??{old:whole,replacement,count:0};row.count++;rows.set(whole,row);return replacement;});
    if(!rows.size){unresolved.push(`${relative(root,file)} -> ${specifier} (no textual anchor)`);continue;}
    if(!edits.has(file))edits.set(file,[]);
    edits.get(file)!.push(...rows.values());
    queue.push(resolved);
  }
  current.set(file,text);
  for(const specifier of relatives){
    const base=resolve(dirname(file),specifier.split("?")[0]!),hit=candidates(base).find(candidate=>existsSync(candidate)&&statSync(candidate).isFile());
    if(hit)queue.push(hit);
  }
}
const files=[...edits].map(([file,rows])=>({path:relative(root,file),sha256:createHash("sha256").update(original.get(file)!).digest("hex"),afterSha256:createHash("sha256").update(current.get(file)!).digest("hex"),replacements:rows}));
writeFileSync(resolve(outputArgument!),JSON.stringify({files:files,external:Object.fromEntries([...external].map(([key,value])=>[key,[...value]])),unresolved,scanned:seen.size},null,1));
console.log(JSON.stringify({scanned:seen.size,editedFiles:files.length,replacements:files.reduce((sum,row)=>sum+row.replacements.length,0),externalSpecifiers:[...external.keys()],unresolved}));
