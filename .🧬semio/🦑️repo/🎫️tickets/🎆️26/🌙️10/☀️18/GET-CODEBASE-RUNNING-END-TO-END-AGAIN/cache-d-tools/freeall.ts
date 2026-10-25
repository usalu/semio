/** freeall.ts <module> <test> [exclude...]: free module-level names used by all function declarations of <module> that the test text never mentions as a port key. */
import ts from "typescript";
import {readFileSync} from "node:fs";
const [file,test,...exclude]=process.argv.slice(2);
const tree=ts.createSourceFile("x.ts",readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true),testText=readFileSync(test,"utf8");
const fnNames=new Set<string>(),moduleNames=new Set<string>();
for(const st of tree.statements){
 if(ts.isImportDeclaration(st)&&st.importClause){const c=st.importClause;if(c.name)moduleNames.add(c.name.text);if(c.namedBindings){if(ts.isNamedImports(c.namedBindings))c.namedBindings.elements.forEach(e=>moduleNames.add(e.name.text));else moduleNames.add(c.namedBindings.name.text);}}
 if(ts.isFunctionDeclaration(st)&&st.name){fnNames.add(st.name.text);moduleNames.add(st.name.text);}
 if(ts.isVariableStatement(st))st.declarationList.declarations.forEach(d=>{if(ts.isIdentifier(d.name))moduleNames.add(d.name.text);});
}
const used=new Set<string>();
for(const st of tree.statements)if(ts.isFunctionDeclaration(st)&&st.name&&!exclude.includes(st.name.text)){
 const visit=(n:ts.Node)=>{if(ts.isIdentifier(n)&&moduleNames.has(n.text)&&!fnNames.has(n.text)){const p=n.parent;if(!(ts.isPropertyAccessExpression(p)&&p.name===n)&&!(ts.isPropertyAssignment(p)&&p.name===n))used.add(n.text);}ts.forEachChild(n,visit);};visit(st);
}
console.log([...used].sort().filter(name=>!new RegExp("\\b"+name+"\\b").test(testText)).join(","));
