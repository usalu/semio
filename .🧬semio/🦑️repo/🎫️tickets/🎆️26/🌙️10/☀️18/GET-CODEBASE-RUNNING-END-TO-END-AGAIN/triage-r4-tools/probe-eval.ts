import ts from "typescript";
import {readFileSync} from "node:fs";
const [file,...names]=process.argv.slice(2);
const source=readFileSync(file,"utf8"),parsed=ts.createSourceFile("source.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const picked=parsed.statements.filter((node:any)=>!ts.isImportDeclaration(node)&&names.some(name=>ts.isFunctionDeclaration(node)&&node.name?.text===name||ts.isVariableStatement(node)&&node.declarationList.declarations.some((value:any)=>value.name.text===name)));
for(const n of picked){const t=n.getText(parsed);console.log(t.slice(0,80).replace(/\n/g," "),"| import.meta:",t.includes("import.meta"));}
