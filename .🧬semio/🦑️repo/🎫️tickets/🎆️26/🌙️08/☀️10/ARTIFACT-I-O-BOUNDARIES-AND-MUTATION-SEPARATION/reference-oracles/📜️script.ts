import ts from "typescript";
import {execFileSync} from "node:child_process";
import {readFileSync,writeFileSync} from "node:fs";
import {relative,dirname,resolve,join} from "node:path";
const ticket=resolve(import.meta.dir,".."),files=execFileSync("rg",["-l","-g","*.ts","(?:🚪️io/🧬️schema/🔣️.json|framework/io/schema.json)","🧰️framework","✏️s"],{encoding:"utf8",maxBuffer:8*1024*1024}).trim().split("\n"),changed:string[]=[],unresolved:string[]=[];
const contract="🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
for(const file of files){
 const source=readFileSync(file,"utf8");if(source.includes("🗿️artifact-reference/🔣️.json"))continue;
 const parsed=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS),edits:{start:number,end:number,text:string}[]=[];
 const visit=(node:ts.Node)=>{
  if(ts.isNewExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="Ajv"||ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="semioSchemaAjvV1"){
   edits.push({start:node.end,end:node.end,text:".addSchema(artifactReferenceSchema)"});return;
  }
  if(ts.isPropertyAssignment(node)&&["dependencies","references"].includes(node.name.getText(parsed))&&ts.isArrayLiteralExpression(node.initializer)){
   edits.push({start:node.initializer.end-1,end:node.initializer.end-1,text:(node.initializer.elements.length?",":"")+"artifactReferenceSchema"});
  }
  ts.forEachChild(node,visit);
 };visit(parsed);
 if(!edits.length){unresolved.push(file);continue;}
 for(const edit of edits.sort((a,b)=>b.start-a.start)){ }
 let next=source;for(const edit of edits.sort((a,b)=>b.start-a.start))next=next.slice(0,edit.start)+edit.text+next.slice(edit.end);
 const first=parsed.statements[0]?.getStart(parsed)??0,path=relative(dirname(file),contract).replaceAll("\\","/");
 next=next.slice(0,first)+"import artifactReferenceSchema from "+JSON.stringify(path.startsWith(".")?path:"./"+path)+";\n"+next.slice(first);
 writeFileSync(file,next);changed.push(file);
}
writeFileSync(join(ticket,"artifact-reference-independent-oracle-routing.md"),"# Independent Reference Schema Oracle Routing\n\nIndependent Ajv validators explicitly register the canonical semantic reference JSON schema. Existing physical IO outcome contracts stay available. Validators continue to compare exact native fixtures and use their original third-party implementations.\n\n## Updated Oracles\n\n"+changed.sort().map(p=>"- \`"+p+"\`").join("\n")+"\n\n## Other References Needing Review\n\n"+unresolved.sort().map(p=>"- \`"+p+"\`").join("\n")+"\n");
process.stdout.write(JSON.stringify({updated:changed.length,review:unresolved.length})+"\n");
