/** 🔎️ Schema-first scan (lane schemafirst): bun numscan.ts <list> <root> <out.jsonl>: every numeric literal >= 100 and every UPPER_CASE const with its statement context. */
import {readFileSync,writeFileSync,appendFileSync} from "node:fs";
import {createRequire} from "node:module";
const ts=createRequire("/Users/ueli/Documents/semio/package.json")("typescript") as typeof import("typescript");
const [listFile,root,out]=process.argv.slice(2) as [string,string,string];writeFileSync(out,"");
for(const file of readFileSync(listFile,"utf8").split("\n").filter(Boolean)){
 let text:string;try{text=readFileSync(`${root}/${file}`,"utf8");}catch{continue;}
 const kind=file.endsWith(".mjs")?ts.ScriptKind.JS:ts.ScriptKind.TS;
 const sf=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true,kind);
 const rows:{line:number;value:number|string;ctx:string}[]=[];
 const visit=(node:import("typescript").Node)=>{
  if(ts.isNumericLiteral(node)){const v=Number(node.text.replace(/_/gu,""));if(Math.abs(v)>=100){
   let stmt:import("typescript").Node=node;while(stmt.parent&&!ts.isSourceFile(stmt.parent)&&!ts.isBlock(stmt.parent)&&!ts.isVariableDeclaration(stmt.parent)&&!ts.isPropertyAssignment(stmt.parent))stmt=stmt.parent;
   const at=node.getStart(sf);rows.push({line:sf.getLineAndCharacterOfPosition(at).line+1,value:node.text,ctx:text.slice(Math.max(0,at-70),at+50).replace(/\s+/gu," ")});}}
  ts.forEachChild(node,visit);
 };
 visit(sf);
 appendFileSync(out,JSON.stringify({file,rows})+"\n");
}
