import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {dirname,join,normalize,resolve,sep} from "node:path";
import ts from "typescript";
const checkout=process.cwd(),B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/";
const entry=resolve(checkout,B+"🎬️executor/🟦️.ts");
const table=JSON.parse(readFileSync(B+"🎬️scopes/🔣️.json","utf8"));
const regular=(path:string)=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(e=>e.isDirectory()&&e.name!=="node_modules"?schemaFiles(join(folder,e.name)):e.isFile()&&e.name.endsWith(".json")?[join(folder,e.name)]:[]):[];
const specs=(path:string)=>{const f=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),out:string[]=[];for(const n of f.statements){if(ts.isImportDeclaration(n)&&!n.importClause?.isTypeOnly&&ts.isStringLiteral(n.moduleSpecifier))out.push(n.moduleSpecifier.text);if(ts.isExportDeclaration(n)&&!n.isTypeOnly&&n.moduleSpecifier&&ts.isStringLiteral(n.moduleSpecifier))out.push(n.moduleSpecifier.text);}return out;};
const lit=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
for(const scope of table.scopes){
 const seen=new Set<string>(),pending=[normalize(entry),...scope.roots.map((r:string)=>normalize(join(checkout,r)))];
 while(pending.length){const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemaFiles(join(dirname(path),"🧬️schema")));for(const s of specs(path))if(s.startsWith("."))pending.push(normalize(join(dirname(path),s)));for(const m of readFileSync(path,"utf8").matchAll(lit))pending.push(normalize(join(dirname(path),m[2]!)));}
 for(const l of scope.leaves)if(regular(normalize(join(checkout,l))))seen.add(normalize(join(checkout,l)));
 const missing=new Map<string,string[]>();
 for(const item of scope.classes)for(const kind of ["modules","reads","physical"] as const)for(const path of item.traced[kind])if(!seen.has(normalize(join(checkout,path))))missing.set(path,[...(missing.get(path)??[]),kind]);
 console.log(scope.id,"closure",seen.size,"traced entries outside the closure:",missing.size);
 for(const [p,k] of missing)console.log(" ",k.join(","),p.slice(-110),regular(join(checkout,p))?"(exists)":"(gone)");
}
