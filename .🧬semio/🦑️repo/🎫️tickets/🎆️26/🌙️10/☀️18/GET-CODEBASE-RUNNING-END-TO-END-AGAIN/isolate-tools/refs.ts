/** 🔬️ [isolate] Prints every checkout reference of the given test files with its enclosing call (callee + argument index), one line each: `file:line callee#arg text`. usage: bun refs.ts <root> <rel>... */
import ts from "typescript";
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,...files]=process.argv.slice(2),live=/\.cache\/b(?:\/|$)|⚡️cache|\.🧬semio\/🦑️repo/u;
for(const rel of files){
 const text=readFileSync(join(root!,rel),"utf8"),file=ts.createSourceFile("x.ts",text,ts.ScriptTarget.Latest,true),short=rel.split("/").slice(-4,-1).join("/");
 const context=(node:ts.Node)=>{let child=node,parent=node.parent;while(parent){if(ts.isCallExpression(parent)||ts.isNewExpression(parent)){const index=(parent.arguments??[]).indexOf(child as ts.Expression);if(index>=0||parent.expression!==child)return parent.expression.getText().slice(-40)+"#"+index;}if(ts.isVariableDeclaration(parent))return "="+parent.name.getText();if(ts.isPropertyAssignment(parent))return ":"+parent.name.getText();child=parent;parent=parent.parent;}return "?";};
 const visit=(node:ts.Node)=>{
  let hit:string|undefined;
  if(ts.isCallExpression(node)&&node.expression.getText()==="process.cwd")hit="cwd";
  else if(ts.isPropertyAccessExpression(node)&&node.name.text==="SEMIO_WORKSPACE_ROOT")hit="ENV";
  else if((ts.isStringLiteral(node)||ts.isNoSubstitutionTemplateLiteral(node)||ts.isTemplateHead(node)||ts.isTemplateMiddle(node)||ts.isTemplateTail(node))&&live.test(node.text))hit=JSON.stringify(node.text).slice(0,60);
  if(hit)console.log(short+":"+(file.getLineAndCharacterOfPosition(node.getStart()).line+1)+" "+context(node)+" "+hit);
  ts.forEachChild(node,visit);
 };
 visit(file);
}
