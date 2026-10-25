import ts from "typescript";
import {readFileSync} from "node:fs";
const [file,...names]=process.argv.slice(2);
const text=readFileSync(file,"utf8"),tree=ts.createSourceFile("x.ts",text,ts.ScriptTarget.Latest,true);
const moduleNames=new Set<string>();
for(const st of tree.statements){
 if(ts.isImportDeclaration(st)&&st.importClause){const c=st.importClause;if(c.name)moduleNames.add(c.name.text);if(c.namedBindings){if(ts.isNamedImports(c.namedBindings))c.namedBindings.elements.forEach(e=>moduleNames.add(e.name.text));else moduleNames.add(c.namedBindings.name.text);}}
 if(ts.isFunctionDeclaration(st)&&st.name)moduleNames.add(st.name.text);
 if(ts.isVariableStatement(st))st.declarationList.declarations.forEach(d=>{if(ts.isIdentifier(d.name))moduleNames.add(d.name.text);});
 if(ts.isClassDeclaration(st)&&st.name)moduleNames.add(st.name.text);
}
for(const name of names){
 const node=tree.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text===name);
 if(!node){console.log(name,"MISSING");continue;}
 const used=new Set<string>();
 const visit=(n:ts.Node)=>{if(ts.isIdentifier(n)&&moduleNames.has(n.text)&&n.text!==name){const p=n.parent;if(!(ts.isPropertyAccessExpression(p)&&p.name===n)&&!(ts.isPropertyAssignment(p)&&p.name===n))used.add(n.text);}ts.forEachChild(n,visit);};
 visit(node);
 console.log(name+": "+[...used].sort().join(","));
}
