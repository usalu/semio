import ts from "typescript";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {dirname,join,normalize,relative} from "node:path";
const root="/Users/ueli/Documents/semio",ticket=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",target=root+"/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
const lists=[ticket+"/tst4-tools/envelope-gated.txt",...process.argv.slice(2)];
const files=lists.flatMap(list=>readFileSync(list,"utf8").split("\n").filter(Boolean));
const seen=new Set<string>(),queue=files.map(file=>join(root,file)),out:unknown[]=[],skipped:string[]=[];
const importRe=/(?:from|import)\s*["'](\.{1,2}\/[^"']+)["']/gu;
while(queue.length){
 const path=queue.pop()!;if(seen.has(path)||!existsSync(path))continue;seen.add(path);
 const text=readFileSync(path,"utf8");
 for(const match of text.matchAll(importRe)){const next=normalize(join(dirname(path),match[1]!));if((next.includes("/🧪️tests/")||next.includes("/🧪️testing/"))&&/\.(ts|mjs)$/u.test(next)&&!next.includes("/🔨️modules/🏃️process/"))queue.push(next);}
 if(!text.includes("receiveScriptProcessInvocation("))continue;
 if(/process\.env\.SEMIO_[A-Z_]*CHILD\b/u.test(text)){skipped.push(path.replace(root+"/","")+" child-mode");continue;}
 const file=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true,path.endsWith(".mjs")?ts.ScriptKind.JS:ts.ScriptKind.TS);
 const calls:ts.CallExpression[]=[];let importDecl:ts.ImportDeclaration|undefined,namespaceImport=false;
 const visit=(node:ts.Node)=>{
  if(ts.isCallExpression(node)){const name=ts.isIdentifier(node.expression)?node.expression.text:ts.isPropertyAccessExpression(node.expression)?node.expression.name.text:"";if(name==="receiveScriptProcessInvocation")calls.push(node);}
  if(ts.isImportDeclaration(node)&&ts.isStringLiteral(node.moduleSpecifier)&&node.moduleSpecifier.text.endsWith("routing/🟦️.ts")){const clause=node.importClause;if(clause?.namedBindings&&ts.isNamedImports(clause.namedBindings)&&clause.namedBindings.elements.some(e=>e.name.text==="receiveScriptProcessInvocation"))importDecl=node;if(clause?.namedBindings&&ts.isNamespaceImport(clause.namedBindings))namespaceImport=true;}
  ts.forEachChild(node,visit);
 };
 visit(file);
 const direct=calls.filter(call=>call.arguments.length===2&&call.arguments[0]!.getText(file)==="process.env");
 if(!direct.length)continue;
 if(direct.length!==calls.length){skipped.push(path.replace(root+"/","")+" other-call-forms");continue;}
 const edits:{start:number;end:number;old:string;new:string}[]=[];
 for(const call of direct){const start=call.getStart(file),end=call.arguments[1]!.getStart(file);edits.push({start,end,old:text.slice(start,end),new:"withBuildTestInvocationV1("});}
 const rel=relative(dirname(path),target).replaceAll("\\","/"),spec="import {withBuildTestInvocationV1} from \""+(rel.startsWith(".")?rel:"./"+rel)+"\";";
 const hasTarget=text.includes("withBuildTestInvocationV1");
 const remaining=(ts.forEachChild as unknown,0);
 if(importDecl){
  const decl=importDecl as ts.ImportDeclaration,bindings=decl.importClause!.namedBindings as ts.NamedImports,names=bindings.elements.filter(e=>e.name.text!=="receiveScriptProcessInvocation").map(e=>e.getText(file));
  const start=decl.getStart(file),end=decl.getEnd(),old=text.slice(start,end),keep=names.length?"import {"+names.join(",")+"} from "+decl.moduleSpecifier.getText(file)+";":"";
  edits.push({start,end,old,new:[keep,hasTarget?"":spec].filter(Boolean).join("\n")});
 }else if(!hasTarget){
  const firstImport=file.statements.find(ts.isImportDeclaration);
  if(!firstImport){skipped.push(path.replace(root+"/","")+" no-import-anchor");continue;}
  const start=firstImport.getStart(file),end=firstImport.getEnd();edits.push({start,end,old:text.slice(start,end),new:text.slice(start,end)+"\n"+spec});
 }
 const point=(offset:number)=>[...text.slice(0,offset)].length;
 out.push({file:path.replace(root+"/",""),edits:edits.map(edit=>({...edit,start:point(edit.start),end:point(edit.end)}))});
}
writeFileSync(ticket+"/env-pass2-tools/staged/patches/R.json",JSON.stringify(out,null,1));
console.log(out.length,"files patched; skipped",skipped);
