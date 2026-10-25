import {readFileSync} from "node:fs";
import ts from "typescript";
const path=process.argv[2]!;
const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
for(const n of file.statements){if(ts.isImportDeclaration(n))console.log("import",n.importClause?.isTypeOnly?"type":"value",(n.moduleSpecifier as ts.StringLiteral).text.slice(-60));}
