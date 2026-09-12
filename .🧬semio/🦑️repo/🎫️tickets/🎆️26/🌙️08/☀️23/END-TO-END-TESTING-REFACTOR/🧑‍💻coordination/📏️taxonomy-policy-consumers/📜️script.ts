import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),source=readFileSync(join(root,"📜️script.ts"),"utf8"),file=ts.createSourceFile("📜️script.ts",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const names=["policyArtifactAppLawDelegateBreaches","policyExtractFnBody","policyMaskLiterals","policyLineOfIndex"],nodes=file.statements.filter(node=>ts.isFunctionDeclaration(node)&&node.name&&names.includes(node.name.text));
assert.equal(nodes.length,names.length);
const code=new Bun.Transpiler({loader:"ts"}).transformSync(nodes.map(node=>node.getText(file)).join("\n"));
const policyArtifactAppLawDelegateBreaches=new Function(code+"\nreturn policyArtifactAppLawDelegateBreaches;")();
for(const name of ["new_app","new_app_with_registry","meta"]){
 assert.deepEqual(policyArtifactAppLawDelegateBreaches("sample","fn "+name+"() { artifact_app_laws::"+name+"(); }"),[]);
 assert.equal(policyArtifactAppLawDelegateBreaches("sample","fn "+name+"() { invented::"+name+"(); }").length,1);
}
const {interactivityDbIoSelfTests}=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io/🟦️.ts")).href);
interactivityDbIoSelfTests();
const report=join(ticket,"📓️coordinator-testing-taxonomy-changes-2026-09-12.md");
writeFileSync(report,readFileSync(report,"utf8")+"\n## Root Policy Consumer Runtime\n\nThe actual root delegate checker, extracted with the TypeScript parser, accepted all three canonical artifact-app-law calls and rejected three nondelegating forms. The actual interactivityDbIoSelfTests export ran its existing negative and positive policy vectors successfully.\n");
console.log("[DEBUG] root delegate six vectors and actual DB I/O self-tests passed");
