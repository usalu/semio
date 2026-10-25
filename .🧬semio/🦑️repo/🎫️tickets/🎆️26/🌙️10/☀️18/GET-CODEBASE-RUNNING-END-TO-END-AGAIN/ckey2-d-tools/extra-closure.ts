/** How many files a lazy root adds to the executor closure: bun extra-closure.ts <rootRel>... */
import {existsSync,readdirSync,readFileSync,statSync}from"node:fs";
import {dirname,join,normalize,relative,sep}from"node:path";
import ts from"typescript";
const root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts";
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closure=(seeds:readonly string[],base:Set<string>=new Set()):Set<string>=>{const seen=new Set<string>(),pending=seeds.map(seed=>normalize(join(root,seed)));while(pending.length){const path=pending.pop()!;if(seen.has(path)||base.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemaFiles(join(dirname(path),"🧬️schema")));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}return seen;};
const base=closure([entry]);console.log("executor static",base.size);
for(const r of process.argv.slice(2)){const add=closure([r],base);console.log(add.size,"added by",r.split("/").slice(-4).join("/"));if(add.size<60)for(const f of add)console.log("   ",relative(root,f).split("/").slice(-3).join("/"));}
