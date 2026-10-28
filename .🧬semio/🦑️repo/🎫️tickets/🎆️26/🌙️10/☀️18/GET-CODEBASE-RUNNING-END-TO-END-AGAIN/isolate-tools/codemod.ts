/** 🛠️ [isolate] proposes the private-repository conversion of test files: every checkout reference (`process.cwd()`, `process.env.SEMIO_WORKSPACE_ROOT`) whose only role is a store (argument of a store call, path joined with a live store literal, or the initializer of an identifier used only in such places) becomes `repository.root`; mixed references are listed for a manual edit, or (files named in CODEMOD_WHOLE) their initializer becomes `repository.root` as soon as one use is a store, or (files named in CODEMOD_SPLIT) only their store uses become `repository.root` while the checkout stays for code, toolchains and children. Prints a JSON plan per file: [{old,new,count}] anchored on the exact enclosing statement text, plus `manual` rows. usage: bun codemod.ts <mirror-root> <rel>... > plan.json */
import ts from "typescript";
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,...files]=process.argv.slice(2);
const law=await import((process.env.LAW_ROOT??"/private/tmp/m/is1")+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🟦️.ts"),rules=law.TEST_STORE_RULES_V1;
const store=rules.storeCalls.map((p:string)=>new RegExp(p,"u")),literals=rules.storeLiterals.map((p:string)=>new RegExp(p,"u"));
const binding=(identifier:ts.Identifier):ts.Node|undefined=>{const name=identifier.text,named=(node:ts.Node|undefined)=>node!==undefined&&(ts.isVariableDeclaration(node)||ts.isParameter(node))&&ts.isIdentifier(node.name)&&node.name.text===name?node:undefined,list=(d:ts.VariableDeclarationList|undefined)=>d?.declarations.find(x=>named(x));for(let scope:ts.Node|undefined=identifier.parent;scope;scope=scope.parent){if(ts.isFunctionLike(scope)){const p=scope.parameters.find(c=>named(c));if(p)return p;}if(ts.isCatchClause(scope)&&named(scope.variableDeclaration))return scope.variableDeclaration;if((ts.isForStatement(scope)||ts.isForOfStatement(scope)||ts.isForInStatement(scope))&&scope.initializer&&ts.isVariableDeclarationList(scope.initializer)){const f=list(scope.initializer);if(f)return f;}if(ts.isBlock(scope)||ts.isSourceFile(scope)||ts.isModuleBlock(scope)||ts.isCaseClause(scope)||ts.isDefaultClause(scope))for(const st of scope.statements)if(ts.isVariableStatement(st)){const f=list(st.declarationList);if(f)return f;}}return undefined;};
const plans:any[]=[],split=new Set((process.env.CODEMOD_SPLIT??"").split("\n").filter(Boolean)),whole=new Set((process.env.CODEMOD_WHOLE??"").split("\n").filter(Boolean));
for(const rel of files){
 const text=readFileSync(join(root!,rel),"utf8"),file=ts.createSourceFile(rel!,text,ts.ScriptTarget.Latest,true,rel!.endsWith(".tsx")?ts.ScriptKind.TSX:/\.m?js$/u.test(rel!)?ts.ScriptKind.JS:ts.ScriptKind.TS);
 const names=(e:ts.Expression):string[]=>ts.isParenthesizedExpression(e)?names(e.expression):ts.isConditionalExpression(e)?[...names(e.whenTrue),...names(e.whenFalse)]:[ts.isPropertyAccessExpression(e)?e.name.text:ts.isIdentifier(e)?e.text:""],last=(e:ts.Expression)=>names(e)[0]!;
 const isCheckout=(n:ts.Node)=>ts.isCallExpression(n)&&n.expression.getText()==="process.cwd"&&n.arguments.length===0||ts.isPropertyAccessExpression(n)&&n.name.text==="SEMIO_WORKSPACE_ROOT"&&/(^|\.)env$/u.test(n.expression.getText());
 const hasLiteral=(n:ts.Node):boolean=>((ts.isStringLiteralLike(n)||ts.isTemplateHead(n)||ts.isTemplateMiddle(n)||ts.isTemplateTail(n))&&literals.some((p:RegExp)=>p.test((n as any).text)))||ts.forEachChild(n,c=>hasLiteral(c)||undefined)===true;
 const storeRole=(n:ts.Node):boolean=>{for(let child=n,parent=n.parent;parent;child=parent,parent=parent.parent){if(ts.isCallExpression(parent)&&parent.arguments.includes(child as ts.Expression)&&names(parent.expression).some(name=>store.some((p:RegExp)=>p.test(name))))return true;if((ts.isCallExpression(parent)&&["join","resolve"].includes(last(parent.expression))||ts.isTemplateExpression(parent)||ts.isBinaryExpression(parent)&&parent.operatorToken.kind===ts.SyntaxKind.PlusToken)&&hasLiteral(parent))return true;if(ts.isBlock(parent)||ts.isSourceFile(parent)||ts.isVariableDeclaration(parent)||ts.isArrowFunction(parent)||ts.isFunctionDeclaration(parent)||ts.isExpressionStatement(parent))return false;}return false;};
 const declarations=new Map<string,ts.VariableDeclaration[]>(),uses=new Map<string,ts.Identifier[]>();
 const walk=(n:ts.Node):void=>{if(ts.isVariableDeclaration(n)&&ts.isIdentifier(n.name)){const k=n.name.text;declarations.set(k,[...(declarations.get(k)??[]),n]);}else if(ts.isIdentifier(n)&&!(ts.isPropertyAccessExpression(n.parent)&&n.parent.name===n)&&!(ts.isPropertyAssignment(n.parent)&&n.parent.name===n)&&!(ts.isVariableDeclaration(n.parent)&&n.parent.name===n)&&!ts.isImportSpecifier(n.parent)&&!ts.isParameter(n.parent))uses.set(n.text,[...(uses.get(n.text)??[]),n]);ts.forEachChild(n,walk);};walk(file);
 const replace:ts.Node[]=[],manual:string[]=[];
 const visit=(n:ts.Node):void=>{if(isCheckout(n)){
  if(storeRole(n))replace.push(n);
  else{let d:ts.Node=n;while(d.parent&&!ts.isVariableDeclaration(d.parent)&&(ts.isCallExpression(d.parent)||ts.isNonNullExpression(d.parent)||ts.isParenthesizedExpression(d.parent)||ts.isAsExpression(d.parent)))d=d.parent;const v=d.parent;
   if(v&&ts.isVariableDeclaration(v)&&ts.isIdentifier(v.name)&&v.initializer===d){const name=v.name.text,all=(uses.get(name)??[]).filter(u=>binding(u)===v);
    if(all.length>0&&all.every(u=>storeRole(u)))replace.push(n);
    else if(whole.has(rel!)&&all.some(u=>storeRole(u)))replace.push(n);
    else if(split.has(rel!)&&all.some(u=>storeRole(u)))replace.push(...all.filter(u=>storeRole(u)));
    else if(all.some(u=>storeRole(u)))manual.push((file.getLineAndCharacterOfPosition(n.getStart()).line+1)+" MIXED "+name+" store "+all.filter(u=>storeRole(u)).length+"/"+all.length+" other: "+all.filter(u=>!storeRole(u)).map(u=>{const p=u.parent;return(file.getLineAndCharacterOfPosition(u.getStart()).line+1)+":"+text.slice(Math.max(p.getStart(),u.getStart()-30),Math.min(p.getEnd(),u.getEnd()+30)).replace(/\s+/gu," ");}).join(" | "));
   }
  }}ts.forEachChild(n,visit);};
 visit(file);
 const edits=replace.map(n=>({start:n.getStart(),end:n.getEnd()})).sort((a,b)=>b.start-a.start);let out=text;for(const e of edits)out=out.slice(0,e.start)+"repository.root"+out.slice(e.end);
 plans.push({file:rel,replaced:edits.length,manual,lines:replace.map(n=>file.getLineAndCharacterOfPosition(n.getStart()).line+1),out:edits.length?out:undefined});
}
console.log(JSON.stringify(plans));
