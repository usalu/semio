import Parser from "web-tree-sitter";
import {execFileSync} from "node:child_process";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
await Parser.init();
const parser=new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
const ticket=resolve(import.meta.dir,".."),files=execFileSync("rg",["-l","ArtifactCommandInputs","🧰️framework","✏️s","-g","*.rs"],{encoding:"utf8"}).trim().split("\n"),changed:string[]=[];
for(const file of files){
 const before=readFileSync(file,"utf8"),tree=parser.parse(before)!,edits:{start:number,text:string}[]=[];
 const walk=(node:any)=>{
  if(node.type==="struct_expression"||node.type==="struct_pattern"){
   const type=node.childForFieldName("name")??node.childForFieldName("type")??node.namedChildren[0],body=node.childForFieldName("body")??node.namedChildren.find((child:any)=>child.type==="field_pattern_list"||child.type==="field_initializer_list"),brace=body?.children.find((child:any)=>child.text==="{")??node.children.find((child:any)=>child.text==="{");
   const fields=body??node;
   if(type?.text.split("::").pop()==="ArtifactCommandInputs"&&brace&&!/\bsnapshot_owner\b/.test(fields.text)&&!fields.namedChildren.some((child:any)=>child.type==="base_field_initializer"||child.type==="remaining_field_pattern"))edits.push({start:brace.endIndex,text:node.type==="struct_pattern"?" snapshot_owner: _,":" snapshot_owner: None,"});
  }
  for(const child of node.namedChildren)walk(child);
 };
 walk(tree.rootNode);if(!edits.length)continue;
 let after=before;for(const edit of edits.sort((a,b)=>b.start-a.start))after=after.slice(0,edit.start)+edit.text+after.slice(edit.start);
 if(parser.parse(after)!.rootNode.hasError()&&!tree.rootNode.hasError())throw Error("Introduced parse error: "+file);
 if(readFileSync(file,"utf8")!==before)throw Error("Concurrent edit: "+file);
 writeFileSync(file,after);changed.push(file);
}
const report=join(ticket,"retained-snapshot-consumer-manifest.md");
if(changed.length)writeFileSync(report,(existsSync(report)?readFileSync(report,"utf8"):"# Retained Snapshot Capability Consumers\n\nThe runtime supplies the Arc it owns alongside the borrowed immutable snapshot. Existing reducers ignore the capability explicitly; manually constructed steps have no retained owner until their producer supplies one. Production runtime uses Some(snapshot). Token-aware edits preserve unrelated changes and reject a changed file before writing.\n\n")+changed.map(file=>"- `"+file+"`").join("\n")+"\n");
process.stdout.write(JSON.stringify({changed:changed.length})+"\n");
