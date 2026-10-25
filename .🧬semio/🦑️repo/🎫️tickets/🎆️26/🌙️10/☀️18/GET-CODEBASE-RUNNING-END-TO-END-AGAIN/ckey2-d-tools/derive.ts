/** Derives the narrow scope of one traced class: bun derive.ts <classes.json> <classKey> [--entry <rel>]
 * statics = compiler-API static closure of the executor entry (the same rule as the law); roots = traced modules the static walk does not reach; leaves = traced reads and physical captures that are neither. */
import {existsSync,readdirSync,readFileSync,statSync}from"node:fs";
import {dirname,join,normalize,relative,resolve,sep}from"node:path";
import ts from"typescript";
const [report,key]=process.argv.slice(2) as [string,string],root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts";
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string)=>schemaFiles(join(dirname(path),"🧬️schema"));
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closure=(seeds:readonly string[]):Set<string>=>{const seen=new Set<string>(),pending=seeds.map(seed=>normalize(join(root,seed)));while(pending.length){const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}return seen;};
const data=JSON.parse(readFileSync(report,"utf8"))[key] as {m:string[];r:string[];p:string[];x:string[]};
const rel=(set:Set<string>)=>new Set([...set].map(path=>relative(root,path)));
let roots:string[]=[];let covered=rel(closure([entry]));
for(const path of data.m){if(covered.has(path))continue;roots.push(path);covered=rel(closure([entry,...roots]));}
const leaves=[...new Set([...data.r,...data.p])].filter(path=>!covered.has(path)&&regular(join(root,path))).sort();
const absent=[...new Set([...data.m,...data.r,...data.p])].filter(path=>!regular(join(root,path)));
console.log(JSON.stringify({class:key,static:rel(closure([entry])).size,traceModules:data.m.length,roots,leaves,covered:covered.size+leaves.length,absentFromDisk:absent,live:data.x.length},null,1));
