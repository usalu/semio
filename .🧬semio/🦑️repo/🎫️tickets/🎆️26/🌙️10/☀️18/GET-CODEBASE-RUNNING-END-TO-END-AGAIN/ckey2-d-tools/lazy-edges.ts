/** Lists the dynamic edges (import("x"), require("x"), createRequire literals) of the static executor closure and the files each would add: bun lazy-edges.ts [classes.json classKey] */
import {existsSync,readdirSync,readFileSync,statSync}from"node:fs";
import {dirname,join,normalize,relative,resolve,sep}from"node:path";
import ts from"typescript";
const root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts";
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string)=>schemaFiles(join(dirname(path),"🧬️schema"));
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const dynamicSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.arguments.length>=1&&ts.isStringLiteralLike(node.arguments[0]!)){const callee=node.expression;if(callee.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(callee)&&["require","__require"].includes(callee.text))found.push((node.arguments[0] as ts.StringLiteral).text);}ts.forEachChild(node,visit);};visit(file);return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closure=(seeds:readonly string[],base:Set<string>=new Set()):Set<string>=>{const seen=new Set<string>(),pending=seeds.map(seed=>normalize(seed.startsWith("/")?seed:join(root,seed)));while(pending.length){const path=pending.pop()!;if(seen.has(path)||base.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}return seen;};
const base=closure([entry]);
const rows:{from:string;target:string;adds:number}[]=[];
for(const path of base){if(path.endsWith(".json"))continue;for(const specifier of dynamicSpecifiers(path)){if(!specifier.startsWith("."))continue;const target=normalize(join(dirname(path),specifier));if(!regular(target)||base.has(target))continue;rows.push({from:relative(root,path),target:relative(root,target),adds:closure([target],base).size});}}
rows.sort((a,b)=>a.adds-b.adds);
console.log("static closure",base.size,"dynamic edges to files outside it",rows.length);
for(const row of rows)console.log(String(row.adds).padStart(5),row.target.split("/").slice(-4).join("/"),"<-",row.from.split("/").slice(-3).join("/"));
