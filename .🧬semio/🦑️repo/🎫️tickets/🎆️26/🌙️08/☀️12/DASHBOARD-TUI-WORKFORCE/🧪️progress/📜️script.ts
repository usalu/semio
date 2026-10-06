import assert from "node:assert/strict";
import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {join,resolve} from "node:path";
const root=process.cwd(),generated=resolve(import.meta.dir,"../🗑️generated"),native="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration",mode=process.argv[2];
function edit(path:string,transform:(value:string)=>string):void{const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(after,before);const staged=join(generated,`progress-${Date.now()}.ts`);writeFileSync(staged,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(staged,file);}
if(mode==="test")edit(`${native}/🧪️tests/📋️owner-command-policy/🟦️.ts`,text=>text+readFileSync(join(import.meta.dir,"📓️test.ts.md"),"utf8"));
if(mode==="artifact-test"){
  edit(`${native}/🧪️tests/📋️owner-command-policy/🟦️.ts`,text=>text+readFileSync(join(import.meta.dir,"📓️artifact-test.ts.md"),"utf8"));
  edit(`${native}/🧫️fixtures/📣️progress/🔣️.json`,text=>{const corpus=JSON.parse(text);corpus.artifactEnvironment={route:"native-owner-command-policy",defaultRelativeRoot:".🧬semio/🦑️repo/⚡️cache/🧪️tests"};return JSON.stringify(corpus,null,2)+"\n";});
  edit(`${native}/🧬️schema/📣️progress/🔣️.json`,text=>{const schema=JSON.parse(text);schema.required.push("artifactEnvironment");schema.properties.artifactEnvironment={type:"object",additionalProperties:false,required:["route","defaultRelativeRoot"],properties:{route:{const:"native-owner-command-policy"},defaultRelativeRoot:{const:".🧬semio/🦑️repo/⚡️cache/🧪️tests"}}};return JSON.stringify(schema,null,2)+"\n";});
}
if(mode==="artifact-fix")edit('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts',text=>text.replace('      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");\n',''));
if(mode==="artifact-correct")edit('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts',text=>{
  const guard='      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");\n';
  const prior='      if (!revision && !imports && segments.length !== 1) throw Error("Expected test nx-project-inference [revision|imports]");\n';assert.ok(text.includes(prior+'      if (revision)'));
  text=text.replace(prior,prior+guard);
  const branch='      if (segments.length !== 1) throw Error("Expected test native-owner-command-policy");\n      resolveTestLevel([], "quick");\n';assert.ok(text.includes(branch+guard));return text.replace(branch+guard,branch);
});
if(mode==="budget")edit('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts',text=>{const before='      if (segments.length !== 1) throw Error("Expected test native-owner-command-policy");';assert.ok(text.includes(before));return text.replace(before,before+'\n      resolveTestLevel([], "quick");');});
if(mode==="fixture")for(const path of [`${native}/🧪️tests/📋️owner-command-policy/🟦️.ts`,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️progress/📓️test.ts.md'])edit(path,text=>{
  const before='  const output=mkdtempSync(join(artifacts,"native-progress-")),manifest=join(output,"Cargo.toml");\n  writeFileSync(manifest,"[workspace]\\nmembers = []\\n");';
  const after='  const output=mkdtempSync(join(artifacts,"native-progress-"));\n  const workspace=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️owner-command-policy/🔣️.json"),"utf8")).cases.find((row:{classification:string})=>row.classification==="workspace");\n  const manifest=join(root,workspace.manifest),cwd=join(root,workspace.cwd);';
  assert.ok(text.includes(before));return text.replace(before,after).replace('"--cwd",relative(root,output),','"--cwd",relative(root,cwd),');
});
if(mode==="cwd")for(const path of [`${native}/🧪️tests/📋️owner-command-policy/🟦️.ts`,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️progress/📓️test.ts.md'])edit(path,text=>text.replace('"--cwd",relative(root,cwd),','"--cwd",relative(root,cwd)||".",'));
if(mode==="fix"){
  edit(`${native}/🟦️.ts`,text=>{const before='onProgress:line=>process.stderr.write(`${line}\\n`)';assert.ok(text.includes(before));return text.replace(before,'onProgress:env.SEMIO_NATIVE_OWNER_PROGRESS==="delegated"?()=>{}:line=>process.stderr.write(`${line}\\n`)');});
  edit('🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📋️project.json',text=>{const project=JSON.parse(text);project.targets.run.options.env={...project.targets.run.options.env,SEMIO_NATIVE_OWNER_PROGRESS:"delegated"};return JSON.stringify(project,null,2)+"\n";});
}
