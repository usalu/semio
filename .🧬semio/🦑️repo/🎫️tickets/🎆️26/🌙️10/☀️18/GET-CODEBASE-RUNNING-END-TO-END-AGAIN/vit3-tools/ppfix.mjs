import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,writeFileSync} from "node:fs";
for(const p of process.argv.slice(2)){
 const text=readFileSync(p,"utf8");const sf=ts.createSourceFile(p,text,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
 const edits=[];
 const visit=n=>{
  if(ts.isConstructorDeclaration(n)&&n.body){
   const props=n.parameters.filter(q=>ts.getModifiers(q)?.some(m=>[ts.SyntaxKind.PublicKeyword,ts.SyntaxKind.PrivateKeyword,ts.SyntaxKind.ProtectedKeyword,ts.SyntaxKind.ReadonlyKeyword].includes(m.kind)));
   if(props.length){
    const fields=[],assigns=[];
    for(const q of props){
     const mods=ts.getModifiers(q).map(m=>m.getText(sf)).join(" ");const name=q.name.getText(sf);
     if(!q.type){console.error("UNTYPED",p,name);continue;}
     fields.push(mods+" "+name+(q.questionToken?"?":"")+":"+q.type.getText(sf)+";");assigns.push("this."+name+"="+name+";");
     edits.push([ts.getModifiers(q)[0].getStart(sf),q.name.getStart(sf),""]);
    }
    const first=n.body.statements[0];
    const isSuper=first&&ts.isExpressionStatement(first)&&ts.isCallExpression(first.expression)&&first.expression.expression.kind===ts.SyntaxKind.SuperKeyword;
    if(isSuper)edits.push([first.getEnd(),first.getEnd(),assigns.join("")]);else edits.push([n.body.getStart(sf)+1,n.body.getStart(sf)+1,assigns.join("")]);
    const at=n.getStart(sf,true);edits.push([at,at,fields.join("")+"\n"+" ".repeat(sf.getLineAndCharacterOfPosition(n.getStart(sf)).character)]);
   }
  }
  ts.forEachChild(n,visit);
 };
 visit(sf);
 edits.sort((a,b)=>b[0]-a[0]||b[1]-a[1]);
 let out=text;for(const [s,e,r] of edits)out=out.slice(0,s)+r+out.slice(e);
 writeFileSync(p,out);console.log("fixed",p,edits.length);
}
