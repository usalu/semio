
import ts from "typescript";import {readFileSync} from "node:fs";
const text=readFileSync(process.argv[2],"utf8"),tree=ts.createSourceFile("x.ts",text,ts.ScriptTarget.Latest,true);
const out:any={functions:[],variables:[]};
for(const st of tree.statements){if(ts.isFunctionDeclaration(st)&&st.name)out.functions.push(st.name.text);if(ts.isVariableStatement(st))for(const d of st.declarationList.declarations)out.variables.push(d.name.getText(tree));}
console.log(JSON.stringify(out));