/** Writes a candidate scope table from traced classes: bun make-table.ts <classes.json> <out.json> <scopeId> <classKey>... [-- <scopeId> <classKey>...]
 * roots = the cheap dynamic edges of the static executor closure (<= 8 added files) plus the traced modules the walk still does not reach (iterated until covered); leaves = traced reads and physical captures not covered; classes[].traced = the sorted evidence. */
import {existsSync,readdirSync,readFileSync,statSync,writeFileSync}from"node:fs";
import {dirname,join,normalize,relative,sep}from"node:path";
import ts from"typescript";
const [report,out,...rest]=process.argv.slice(2) as string[],bound=8,root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts";
const groups:string[][]=[[]];for(const word of rest){if(word==="--")groups.push([]);else groups.at(-1)!.push(word);}
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string)=>schemaFiles(join(dirname(path),"🧬️schema"));
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const dynamicSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.arguments.length>=1&&ts.isStringLiteralLike(node.arguments[0]!)){const callee=node.expression;if(callee.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(callee)&&["require","__require"].includes(callee.text))found.push((node.arguments[0] as ts.StringLiteral).text);}ts.forEachChild(node,visit);};visit(file);return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closure=(seeds:readonly string[],base:Set<string>=new Set()):Set<string>=>{const seen=new Set<string>(),pending=seeds.map(seed=>normalize(join(root,seed)));while(pending.length){const path=pending.pop()!;if(seen.has(path)||base.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}return seen;};
const rel=(set:Set<string>)=>new Set([...set].map(path=>relative(root,path)));
type Row={processes:number;m:string[];r:string[];p:string[]};
const data=JSON.parse(readFileSync(report,"utf8")) as Record<string,Row>;
const derive=(row:{m:string[]})=>{
 const roots:string[]=[],base=rel(closure([entry]));
 for(const path of [...base].sort()){if(path.endsWith(".json"))continue;for(const specifier of dynamicSpecifiers(join(root,path))){if(!specifier.startsWith("."))continue;const target=relative(root,normalize(join(root,dirname(path),specifier)));if(base.has(target)||roots.includes(target)||!regular(join(root,target)))continue;if(closure([target],new Set([...base].map(item=>join(root,item)))).size<=bound)roots.push(target);}}
 let covered=rel(closure([entry,...roots]));
 for(const path of [...row.m].sort()){if(covered.has(path))continue;roots.push(path);covered=rel(closure([entry,...roots]));}
 return {roots,covered};
};
const scopes=groups.map(([id,...keys])=>{
 const rows=keys.map(key=>{const row=data[key];if(!row)throw Error("no such class: "+key);return {key,row};});
 const merged={m:[...new Set(rows.flatMap(item=>item.row.m))],r:[...new Set(rows.flatMap(item=>item.row.r))],p:[...new Set(rows.flatMap(item=>item.row.p))]};
 const {roots,covered}=derive(merged),leaves=[...new Set([...merged.r,...merged.p])].filter(path=>!covered.has(path)).sort();
 return {id:id!,roots:[...roots].sort(),leaves,classes:rows.map(({key,row})=>{const [command,workerKind,executionKind,semioKind,owner]=key.split("|") as [string,string,string,string,string];return {select:{command,workerKind,executionKind,semioKind:semioKind==="None"?"none":semioKind,owner},traced:{processes:row.processes,modules:[...row.m].sort(),reads:[...row.r].sort(),physical:[...row.p].sort()}};})};});
writeFileSync(out,JSON.stringify({version:1,scopes},null,2)+"\n");
console.log(scopes.map(row=>row.id+": roots "+row.roots.length+" leaves "+row.leaves.length+" classes "+row.classes.length).join("\n"));
