import ts from "typescript";
import {readFileSync} from "node:fs";
const p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts";
const source=readFileSync(p,"utf8"),tree=ts.createSourceFile("b.ts",source,ts.ScriptTarget.Latest,true);
const fn=tree.statements.find((n:any)=>ts.isFunctionDeclaration(n)&&n.name?.text==="runAdmittedBuildInvocationV1");
const code=ts.transpileModule(fn!.getText(tree).replace(/^export /u,"").replace(/\bimport\(([^)]+)\)/gu,"load($1)"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
console.log(code.split("\n").slice(0,9).join("\n").slice(0,1800));
