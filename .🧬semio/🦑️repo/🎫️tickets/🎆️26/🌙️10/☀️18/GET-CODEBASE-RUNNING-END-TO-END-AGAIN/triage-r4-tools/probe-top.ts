import ts from "typescript";
import {readFileSync} from "node:fs";
const file=process.argv[2];
const source=readFileSync(file,"utf8"),parsed=ts.createSourceFile("source.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const imports:string[]=[];
for(const n of parsed.statements){
 if(ts.isImportDeclaration(n)){const c=n.importClause;const names:string[]=[];if(c?.name)names.push(c.name.text);const b=c?.namedBindings;if(b&&ts.isNamedImports(b))for(const e of b.elements)names.push(e.name.text);imports.push(names.join(",")+" <- "+(n.moduleSpecifier as any).text.slice(-60));continue;}
 if(ts.isFunctionDeclaration(n))continue;
 console.log("TOP:",n.getText(parsed).slice(0,200).replace(/\n/g," "));
}
console.log(imports.join("\n"));
