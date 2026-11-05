const fs = require("node:fs"), assert = require("node:assert/strict"), Ajv = require("ajv"), jsonc = require("jsonc-parser");
const editor = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const fixture = JSON.parse(fs.readFileSync(editor + "/🎮️commands/🧾️export-qc-report/🧪️tests/🧫️cases/🔣️.json", "utf8"));
const reportSchema = fixture.reportSchema;
const validate = new Ajv({strict:true}).compile(reportSchema);
for (const vector of fixture.cases) {
  if (vector.report === null) { assert.equal(vector.effects,0); continue; }
  const encoded = JSON.stringify(vector.report,null,2), errors=[];
  assert.deepEqual(jsonc.parse(encoded,errors,{allowTrailingComma:false,disallowComments:true}),vector.report);
  assert.deepEqual(errors,[]);
  assert(validate(vector.report),JSON.stringify(validate.errors));
  assert.equal(vector.effects,1);
}
const command = fs.readFileSync(editor + "/🎮️commands/🧾️export-qc-report/🦀️.rs","utf8");
assert(command.includes(JSON.stringify(fixture.filename)));
assert(command.includes(JSON.stringify(fixture.mimeType)));
assert(!command.includes("remodeling.qc-report.missing"),"absent report must be a successful no-op");
const manifest = fs.readFileSync(editor + "/📜️manifest/🦀️.rs","utf8");
const description = manifest.split("\n").find(line => line.includes(".action_describe(\"exportQcReport\""));
assert(description && description.includes(fixture.filename));
assert(!description.includes("remodeling-qc-report.ops"));
console.log("QC export portable JSON agrees with Ajv/jsonc-parser; current source contract agrees (native execution remains separate)");

void (async () => {
  const {pathToFileURL}=require("node:url"),path=require("node:path");
  const subset=editor.slice(0,-"✏️editor".length);
  const {decodeRemodelingSnapshot,remodelingSnapshotToJsonText}=await import(pathToFileURL(path.resolve(subset+"🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts")).href);
  const {defaultRemodelingScene}=await import(pathToFileURL(path.resolve(subset+"🧬️schema/📸️snapshot/🟦️.ts")).href);
  for(const vector of fixture.cases){
    const base=JSON.parse(remodelingSnapshotToJsonText(defaultRemodelingScene()));
    base.results.qc=vector.report;
    const actual=JSON.parse(remodelingSnapshotToJsonText(decodeRemodelingSnapshot(base)));
    assert.deepEqual(actual.results.qc,vector.documentQc);
  }
  console.log("Actual TypeScript QC snapshot encoder/decoder matches independent authored IEEE754 document cases");
})();
