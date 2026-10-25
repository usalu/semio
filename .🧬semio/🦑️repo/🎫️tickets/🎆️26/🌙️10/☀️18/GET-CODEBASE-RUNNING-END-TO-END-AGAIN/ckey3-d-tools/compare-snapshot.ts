/** Compares a really captured scope snapshot with its derivation: bun compare-snapshot.ts <implementation.<id>.json> <table.json> <scopeId>
 * the snapshot's file list must equal the compiler-API closure of the entry, the row's roots and leaves (live capture == derivation), and every traced path of every class of the row must be in it. */
import {existsSync,readdirSync,readFileSync,statSync}from"node:fs";
import {dirname,join,normalize,relative,sep}from"node:path";
import ts from"typescript";
const [snapshotPath,tablePath,id]=process.argv.slice(2) as string[],root=process.cwd();
const snapshot=JSON.parse(readFileSync(snapshotPath,"utf8")) as {entry:string;files:{path:string}[]},row=(JSON.parse(readFileSync(tablePath,"utf8")) as {scopes:{id:string;roots:string[];leaves:string[];classes:{traced:{modules:string[];reads:string[];physical:string[]}}[]}[]}).scopes.find(item=>item.id===id)!;
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string)=>schemaFiles(join(dirname(path),"🧬️schema"));
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const seen=new Set<string>(),pending=[snapshot.entry,...row.roots].map(path=>normalize(join(root,path)));
while(pending.length){const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}
for(const leaf of row.leaves)if(regular(join(root,leaf)))seen.add(normalize(join(root,leaf)));
const derived=new Set([...seen].map(path=>relative(root,path))),captured=new Set(snapshot.files.map(file=>file.path));
const onlyDerived=[...derived].filter(path=>!captured.has(path)),onlyCaptured=[...captured].filter(path=>!derived.has(path));
const missing=row.classes.flatMap(item=>[...item.traced.modules,...item.traced.reads,...item.traced.physical]).filter(path=>!captured.has(path));
console.log(JSON.stringify({captured:captured.size,derived:derived.size,onlyDerived,onlyCaptured,tracedMissingFromSnapshot:[...new Set(missing)]},null,1));
process.exit(onlyDerived.length||onlyCaptured.length||missing.length?1:0);
