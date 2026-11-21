import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope, withIssuedResourceOperationV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withCargoWorkspaceSelectionV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

const artifacts=resolve(import.meta.dir,"🗑️generated");
mkdirSync(artifacts,{recursive:true});
const root=mkdtempSync(join(artifacts,"c-")), manifest=join(root,"Cargo.toml");
const before='[workspace]\n[workspace.metadata]\ngeneration="before"\n';
const after='[workspace]\n[workspace.metadata]\ngeneration="newer bytes"\n';
const envelope=()=>createScriptProcessEnvelope({version:1,owner:"cargo-manifest-churn-repro",maximumElapsedMilliseconds:30000},{artifacts:root},Date.now());
let calls=0;
try {
 writeFileSync(manifest,before);
 await withScriptProcessEnvelope(envelope(),async original=>{
  const control=createIssuedResourceControlV1(original,"cargo:workspace-selection",0);
  const current=await withCargoWorkspaceSelectionV1(root,"Cargo.toml",control,async view=>{
   if(++calls===1)writeFileSync(manifest,after);
   await view.recheck();
   return view.workspaceManifestText();
  });
  assert.equal(current,after);
  assert.equal(calls,2);
  assert.equal(await withIssuedResourceOperationV1(control,async()=>42),42);
 });
 console.log("[DEBUG] Rechecked the current manifest in a live original scope.");
 writeFileSync(manifest,before);
 let refused=0;
 await assert.rejects(withScriptProcessEnvelope(envelope(),async original=>{
  const control=createIssuedResourceControlV1(original,"cargo:workspace-selection",0);
  await assert.rejects(withCargoWorkspaceSelectionV1(root,"Cargo.toml",control,async view=>{
   refused++;
   writeFileSync(manifest,after+"#".repeat(262144));
   await view.recheck();
  }),{key:"process.source.read-bytes-exceeded"});
  assert.throws(()=>withIssuedResourceOperationV1(control,async()=>42),/admission closed/);
 }),{key:"process.source.read-bytes-exceeded"});
 assert.equal(refused,1);
 console.log("[DEBUG] Tracked byte-limit refusal closed its original scope without retry.");
} finally {
 console.log("[DEBUG] Workspace selection entered",calls,"time(s).");
 rmSync(root,{recursive:true});
}
