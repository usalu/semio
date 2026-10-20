/** 🔗️ Plans relative-import replacements for every runtime workspace specifier in the closures of the given entries: each specifier becomes the relative path to the authored export target of its package, so no package internal is touched; reads a tree, writes only the plan JSON. Usage: bun rel-plan.ts <root> <entries.json> <out.json> */
import {readFileSync,writeFileSync} from "node:fs";
import {isBuiltin} from "node:module";
import {posix} from "node:path";
import {createHash} from "node:crypto";
import {diskZeroTouchTreeV1,scanModuleSpecifiersV1,workspaceSpecifierTargetV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
const [rootArgument,entriesArgument,outArgument]=process.argv.slice(2),tree=diskZeroTouchTreeV1(rootArgument!),entries=JSON.parse(readFileSync(entriesArgument!,"utf8")) as Record<string,string[]>;
const CODE=/\.(?:[cm]?[jt]s|tsx)$/u,EXT=[".ts",".tsx",".mts",".js",".mjs"],sha=(text:string)=>createHash("sha256").update(text,"utf8").digest("hex");
const relativeTarget=(from:string,specifier:string)=>{const base=posix.normalize(posix.join(posix.dirname(from),specifier.split("?")[0]!));for(const candidate of [base,...EXT.map(extension=>base+extension)])if(CODE.test(candidate)&&tree.read(candidate)!==undefined)return candidate;return undefined;};
const escape=(value:string)=>value.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
const seen=new Set<string>(),pending=Object.values(entries).flat(),pairs=new Map<string,Map<string,string>>(),problems:string[]=[];
while(pending.length){
  const file=pending.pop()!;
  if(seen.has(file))continue;
  seen.add(file);
  const text=tree.read(file);
  if(text===undefined||!CODE.test(file))continue;
  for(const {specifier} of scanModuleSpecifiersV1(file,text)){
    if(specifier.startsWith(".")){const target=relativeTarget(file,specifier);if(target)pending.push(target);}
    else if(specifier.startsWith("node:")||specifier.startsWith("bun:")||specifier==="bun"||isBuiltin(specifier))continue;
    else if(Object.keys(tree.packages).some(name=>specifier===name||specifier.startsWith(name+"/"))){
      const target=workspaceSpecifierTargetV1(tree.packages,specifier);
      if(target===undefined||tree.read(target)===undefined){problems.push(`${file} -> ${specifier}: no export target`);continue;}
      if(!pairs.has(file))pairs.set(file,new Map());
      pairs.get(file)!.set(specifier,target);
      pending.push(target);
    }
  }
}
const files:{path:string;sha256:string;afterSha256:string;replacements:{old:string;new:string;count:number}[]}[]=[];
for(const [file,specifiers] of [...pairs].sort(([left],[right])=>left<right?-1:1)){
  const original=tree.read(file)!;let text=original;const rows=new Map<string,{old:string;new:string;count:number}>();
  for(const [specifier,target] of specifiers){
    let path=posix.relative(posix.dirname(file),target);
    if(!path.startsWith("."))path="./"+path;
    const pattern=new RegExp(`(\\bfrom\\s*|\\bimport\\s*\\(\\s*|\\bimport\\s*|\\brequire\\s*\\(\\s*)(["'])${escape(specifier)}\\2`,"gu");
    let found=0;
    text=text.replace(pattern,(whole,head:string,quote:string)=>{found++;const replacement=head+quote+path+quote,row=rows.get(whole)??{old:whole,new:replacement,count:0};row.count++;rows.set(whole,row);return replacement;});
    if(!found)problems.push(`${file} -> ${specifier}: no textual anchor`);
  }
  files.push({path:file,sha256:sha(original),afterSha256:sha(text),replacements:[...rows.values()]});
}
writeFileSync(outArgument!,JSON.stringify({version:1,scanned:seen.size,problems,files},null,1));
console.log(JSON.stringify({scanned:seen.size,editedFiles:files.length,replacements:files.reduce((sum,row)=>sum+row.replacements.reduce((inner,item)=>inner+item.count,0),0),problems}));
