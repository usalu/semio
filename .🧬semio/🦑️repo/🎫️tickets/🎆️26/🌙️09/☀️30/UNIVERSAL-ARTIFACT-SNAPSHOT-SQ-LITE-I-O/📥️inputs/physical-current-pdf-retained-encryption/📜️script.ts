import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {join,dirname} from "node:path";
import assert from "node:assert/strict";
let root=import.meta.dir;
while(!existsSync(join(root,"AGENTS.md"))){const parent=dirname(root);assert.notEqual(parent,root);root=parent;}
const pairsPath=join(import.meta.dir,"held-pairs.json");
if(process.argv[2]==="mount"){
 const pairs=JSON.parse(readFileSync(pairsPath,"utf8"));
 for(const p of pairs)assert.equal(readFileSync(join(root,p.path),"utf8"),p.before,"Concurrent PDF profile provider "+p.path);
 for(const p of pairs)writeFileSync(join(root,p.path),p.after);
 console.log("[DEBUG] PDF retained typed encryption profile provider mounted paths="+pairs.length);
}else{
 assert.equal(process.argv[2],"hold");
 const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets";
 const profiles=[["🗄️a","check_pdf_a_conformance","PDF/A"],["🖨️x","check_x_conformance","PDF/X"],["📐️e","check_e_conformance","PDF/E"]];
 const pairs=profiles.map(([subset,fn,label])=>{
  const path=join(base,subset,"🧬️schema/🦀️.rs"),before=readFileSync(join(root,path),"utf8");
  const start="pub fn "+fn+"(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {\n        let objects = &snapshot.objects;\n        let mut out = Vec::new();";
  assert.equal(before.split(start).length,2,path);
  const after=before.replace(start,start+'\n        if snapshot.encryption.is_some() {\n            out.push(hard(CODE_ENCRYPT, "'+label+' forbids retained document encryption".into()));\n        }');
  return {path,before,after};
 });
 writeFileSync(pairsPath,JSON.stringify(pairs,null,2)+"\n");
 console.log("[DEBUG] PDF retained typed encryption profile provider held paths="+pairs.length+" production_mutations=0");
}

