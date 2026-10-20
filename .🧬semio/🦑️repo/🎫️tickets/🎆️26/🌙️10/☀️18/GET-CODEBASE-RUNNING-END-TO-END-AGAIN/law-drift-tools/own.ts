import ts from "typescript";
import {readFileSync} from "node:fs";
import {join,dirname,relative,resolve} from "node:path";
const root=process.argv[2]!;
const fixture=JSON.parse(readFileSync(join(root,"🧪️tests/🧫️fixtures/🏗️source-services/🔣️.json"),"utf8"));
const paths=fixture.owners.map((o:any)=>o.path);
for(const owner of fixture.owners){const path=join(root,owner.path),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
const imports=source.statements.filter(ts.isImportDeclaration).map((n:any)=>n.moduleSpecifier.text);
const owned=imports.filter((s:string)=>s.startsWith(".")).map((s:string)=>relative(root,resolve(dirname(path),s))).filter((d:string)=>paths.includes(d));
const a=[...new Set(owned)].sort().join(","),b=[...owner.dependencies].sort().join(",");
console.log(a===b?"OK  ":"DIFF",owner.path,a===b?"":"\n   actual: "+a+"\n   fixture:"+b);}
