/** convert-birth.ts <module-relative test file>... : turns `receiveScriptProcessInvocation(process.env, body)` calls of real code (never of template-string child programs) into `withBuildTestInvocationV1(body)`: one shared Original birth per test process. */
import ts from "typescript";
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,relative,resolve} from "node:path";
const mod="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process";
for(const rel of process.argv.slice(2)){
 const file=resolve(mod,rel),text=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true);
 const edits:{start:number;end:number;text:string}[]=[];
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="receiveScriptProcessInvocation"&&node.arguments.length>=2&&node.arguments[0]!.getText(tree)==="process.env")
   edits.push({start:node.expression.getStart(tree),end:node.arguments[1]!.getStart(tree),text:"withBuildTestInvocationV1("});
  ts.forEachChild(node,visit);
 };
 visit(tree);
 if(!edits.length){console.log("none",rel);continue;}
 let out=text;for(const e of edits.sort((a,b)=>b.start-a.start))out=out.slice(0,e.start)+e.text+out.slice(e.end);
 const helper=relative(dirname(file),resolve(mod,"🧪️testing/📥️invocation/🟦️.ts")).split("\\").join("/");
 const rest=ts.createSourceFile(file,out,ts.ScriptTarget.Latest,true);
 let lastImport=0,hasHelper=false,stillUsed=/receiveScriptProcessInvocation/.test(out.replace(/import\s*\{[^}]*\}\s*from\s*["'][^"']*["'];?/gu,""));
 const importEdits:{start:number;end:number;text:string}[]=[];
 for(const st of rest.statements)if(ts.isImportDeclaration(st)){
  lastImport=st.getEnd();
  const names=st.importClause?.namedBindings&&ts.isNamedImports(st.importClause.namedBindings)?st.importClause.namedBindings.elements.map(e=>e.getText(rest)):[];
  if(names.some(n=>n==="withBuildTestInvocationV1"))hasHelper=true;
  if(!stillUsed&&names.includes("receiveScriptProcessInvocation")){
   const left=names.filter(n=>n!=="receiveScriptProcessInvocation");
   importEdits.push({start:st.getStart(rest),end:st.getEnd(),text:left.length?`import{${left.join(",")}}from${st.moduleSpecifier.getText(rest)};`:""});
  }
 }
 for(const e of importEdits.sort((a,b)=>b.start-a.start))out=out.slice(0,e.start)+e.text+out.slice(e.end);
 if(!hasHelper){const again=ts.createSourceFile(file,out,ts.ScriptTarget.Latest,true);let at=0;for(const st of again.statements)if(ts.isImportDeclaration(st))at=st.getEnd();out=out.slice(0,at)+`\nimport{withBuildTestInvocationV1}from"${helper.startsWith(".")?helper:"./"+helper}";`+out.slice(at);}
 writeFileSync(file,out);console.log("converted",edits.length,rel);
}
