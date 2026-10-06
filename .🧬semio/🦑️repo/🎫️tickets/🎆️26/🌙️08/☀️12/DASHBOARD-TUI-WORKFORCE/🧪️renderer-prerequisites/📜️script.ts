import assert from "node:assert/strict";
import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {join,resolve} from "node:path";
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),generated=join(ticket,"🗑️generated"),graph="🧰️framework/🔨️modules/🕸️graph",lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",native="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint";
function edit(path:string,transform:(text:string)=>string):void{const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(after,before);const stage=join(generated,`renderer-stage-${Date.now()}`);writeFileSync(stage,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(stage,file);}
if(process.argv[2]==="test")edit(`${graph}/🧪️tests/🧩️native-prerequisites/🟦️.ts`,text=>text+readFileSync(join(import.meta.dir,"📓️tests.ts.md"),"utf8"));
if(process.argv[2]==="imports")for(const path of [`${native}/📜️script.ts`,`${native}/📦️modules/📜️script.ts`])edit(path,text=>{const lines=text.split("\n");assert.ok(lines[1].startsWith("#!/usr/bin/env bun"));[lines[0],lines[1]]=[lines[1],lines[0]];return lines.join("\n");});
if(process.argv[2]==="artifacts")for(const path of [`${graph}/🧪️tests/🧩️native-prerequisites/🟦️.ts`,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️renderer-prerequisites/📓️tests.ts.md'])edit(path,text=>text.replace('  const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"native-session-catalog-"));','  const {repoTestArtifactEnvironment}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts")).href);\n  const root=mkdtempSync(join(repoTestArtifactEnvironment(workspace,"graph-native-prerequisites").SEMIO_TEST_ARTIFACT_DIR!,"native-session-catalog-"));'));
if(process.argv[2]==="fix"){
  edit(`${lib}/🟨️.mjs`,text=>{
    const anchor='  const nativeHostView={kind:';
    assert.ok(text.includes(anchor));
    text=text.replace(anchor,'  const flowRoot="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", flowProject=projectAt(flowRoot);\n  if(!flowProject?.name || !flowProject.targets?.wasm)throw new Error(`Flow must name its authored browser producer: ${flowRoot}`);\n'+anchor);
    const flags='${operation === "smoke" ? " --smoke" : ""}`, forwardAllArgs: false';
    assert.ok(text.includes(flags));text=text.replace(flags,'${operation === "smoke" ? " --smoke" : ""}`, forwardAllArgs: true');
    const font='"semio-framework-os-infinite:fonts",';assert.equal(text.split(font).length,4);
    return text.replaceAll(font,font+' `${flowProject.name}:wasm`,');
  });
  edit(`${native}/📦️modules/📜️script.ts`,text=>{
    text='import { readGeneratedCatalogProjection } from "../../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";\n'+text;
    const before='JSON.parse(readFileSync(join(repo, registryPath, "🤖️generated/🔌️plugins.json"), "utf8"))';assert.ok(text.includes(before));
    return text.replace(before,'readGeneratedCatalogProjection(join(repo,registryPath,"dist/sessions",variant)).entries').replace('import { readFileSync } from "node:fs";\n','');
  });
  edit(`${native}/📜️script.ts`,text=>{
    text='import { readGeneratedCatalogProjection } from "../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";\n'+text;
    const before='JSON.parse(readFileSync(join(repo, registryPath, "🤖️generated/🎠️playgrounds.json"), "utf8"))';assert.ok(text.includes(before));
    return text.replace(before,'readGeneratedCatalogProjection(join(repo,registryPath,"dist/sessions",variant)).playgrounds');
  });
}
