import {readFileSync} from "node:fs";
import ts from "typescript";
const rows:{file:string;spec:string;target:string}[]=JSON.parse(readFileSync(process.argv[2]!,"utf8"));
const sig=new Map<string,Set<string>>();
for(const r of rows){
 const text=readFileSync(r.file,"utf8");
 const sf=ts.createSourceFile(r.file,text,ts.ScriptTarget.Latest,true);
 const names:string[]=[];
 sf.forEachChild(n=>{
  if((ts.isImportDeclaration(n)||ts.isExportDeclaration(n))&&n.moduleSpecifier&&(n.moduleSpecifier as ts.StringLiteral).text===r.spec){
   if(ts.isImportDeclaration(n)){const c=n.importClause;if(!c)names.push("(side-effect)");else{if(c.name)names.push("default:"+c.name.text);const b=c.namedBindings;if(b){if(ts.isNamespaceImport(b))names.push("*:"+b.name.text);else b.elements.forEach(e=>names.push((e.propertyName?.text??e.name.text)));}}}
   else{const c=n.exportClause;if(c&&ts.isNamedExports(c))c.elements.forEach(e=>names.push("export:"+(e.propertyName?.text??e.name.text)));else names.push("export *");}
  }
 });
 const key=r.target.replace(/^.*📚️library\//,"L/")+"  =>  "+(names.sort().join(", ")||"(dynamic/other)");
 if(!sig.has(key))sig.set(key,new Set());sig.get(key)!.add(r.file);
}
for(const [k,v] of [...sig].sort((a,b)=>b[1].size-a[1].size))console.log(v.size,k);
