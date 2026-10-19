import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {existsSync,readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
const uiRoot="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
const bad=[resolve(uiRoot,"🎯️targets/⚛️react/🟦️.tsx"),resolve(uiRoot,"🧱️elements/🔌️Ports/🟦️.tsx")];
const seen=new Map();
const visit=(path,parent)=>{
  if(seen.has(path))return;seen.set(path,parent);
  if(bad.includes(path)){let c=path,o=[];while(c){o.push(c.replace(uiRoot,""));c=seen.get(c);}console.log(o.reverse().join(" -> "));return;}
  const source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);const specs=[];
  const dyn=n=>{if(ts.isCallExpression(n)&&n.expression.kind===ts.SyntaxKind.ImportKeyword&&n.arguments[0]&&ts.isStringLiteral(n.arguments[0]))specs.push(n.arguments[0].text);ts.forEachChild(n,dyn);};dyn(source);
  for(const st of source.statements){
    if(!ts.isImportDeclaration(st)&&!ts.isExportDeclaration(st))continue;if(!st.moduleSpecifier||!ts.isStringLiteral(st.moduleSpecifier))continue;
    if(ts.isImportDeclaration(st)&&st.importClause?.isTypeOnly)continue;
    const b=ts.isImportDeclaration(st)?st.importClause?.namedBindings:st.exportClause;
    if(b&&(ts.isNamedImports(b)||ts.isNamedExports(b))&&b.elements.every(x=>x.isTypeOnly))continue;
    if(ts.isExportDeclaration(st)&&st.isTypeOnly)continue;specs.push(st.moduleSpecifier.text);}
  for(const s of specs){if(/^(?:@semio-tech\/ui-react$|three(?:\/|$)|@react-three\/|@xyflow\/|pdfjs-dist(?:\/|$))/.test(s)){console.log("BADSPEC",s,"in",path.replace(uiRoot,""));}
    if(!s.startsWith("."))continue;const t=resolve(dirname(path),s);const r=[t,t+".ts",t+".tsx"].find(existsSync);if(!r){console.log("UNRESOLVED",s,"in",path.replace(uiRoot,""));continue;}visit(r,path);}
};
visit(resolve(uiRoot,"🎯️targets/⚛️react/👋️introduction/🟦️.tsx"),null);console.log("seen",seen.size);
