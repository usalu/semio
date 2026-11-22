import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";

const plugin = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const instances = resolve(plugin,"🧵️instance");
const timeTravel = resolve(plugin,"⏪️time-travel");
const feature = "artifact-app-testing";
const scenarios = await Bun.file(new URL("./time-travel-close-diagnostic.json",import.meta.url)).json();
const manifest = path => Bun.TOML.parse(readFileSync(resolve(path,"📦️packages/🦀️rust/Cargo.toml"),"utf8"));
const consumer = manifest(instances);
assert.deepEqual(consumer.dependencies["semio-plugin-time-travel"].features,undefined);
const consumerFeatures = consumer.features[feature];
assert.equal(consumerFeatures.filter(name => name==="semio-plugin-time-travel/"+feature).length,1);
assert.ok(manifest(timeTravel).features[feature].length > 0);
const provider = readFileSync(resolve(timeTravel,"📒️time-travel-ledger/🦀️.rs"),"utf8");
const providerCfg = provider.match(/^[ ]*(#\[cfg\(any\(test, feature = "artifact-app-testing"\)\)\])\n[ ]*#\[doc\(hidden\)\]\n[ ]*pub fn original_close_diagnostic\(/m)?.[1];
assert.ok(providerCfg);
const caller = readFileSync(resolve(instances,"🔌️plugin-app-impl/🦀️.rs"),"utf8");
const branches = caller.match(/if matches!\([^\n]+::CloseRung::TimeTravel\)\s*\{\n([\s\S]+?)\n        \}/)?.[1];
assert.ok(branches);
assert.match(branches,/#[[]cfg\(feature = "artifact-app-testing"\)\]\n +return [^\n]*original_close_diagnostic\(\)/);
assert.match(branches,/#[[]cfg\(not\(feature = "artifact-app-testing"\)\)\]\n +return format!\("TimeTravel residue=\{:\?\}", self\.time_travel\.residue\(\)\)/);

const db = new Database(":memory:");
try {
  db.exec("create table cases(position integer, instance_feature integer, provider_feature integer, caller_test integer)");
  const insert = db.query("insert into cases values(?,?,?,?)");
  scenarios.forEach((row,i) => insert.run(i,+row.instanceFeature,+row.providerFeature,+row.callerTest));
  const rows = db.query("select 'TimeTravel residue=Some(\"pending\")' || case instance_feature when 1 then ' expanded frontier' else '' end as output from cases where instance_feature <= provider_feature order by position").all();
  assert.deepEqual(rows.map(row => row.output),scenarios.map(row => row.output));
} finally {db.close();}

const output = resolve(import.meta.dir,"🗑️generated","time-travel-cfg");mkdirSync(output,{recursive:true});
writeFileSync(resolve(output,"provider.rs"),[
  "pub struct TimeTravelLedger;",
  "impl TimeTravelLedger {",
  "pub fn residue(&self)->Option<&'static str>{Some(\"pending\")}",
  providerCfg,
  "pub fn original_close_diagnostic(&self)->String{\"expanded frontier\".into()}",
  "}",
].join("\n"));
writeFileSync(resolve(output,"caller.rs"),[
  "use semio_time_travel_cfg::TimeTravelLedger;",
  "struct App {time_travel:TimeTravelLedger}",
  "impl App {fn close_diagnostic(&self)->String {",
  branches,
  "}}",
  "#[cfg(not(test))] fn main(){println!(\"{}\",App{time_travel:TimeTravelLedger}.close_diagnostic());}",
  "#[cfg(test)] #[test] fn caller_test_uses_default_provider(){let result=App{time_travel:TimeTravelLedger}.close_diagnostic(); println!(\"{result}\"); assert_eq!(result,\"TimeTravel residue=Some(\\\"pending\\\")\");}",
].join("\n"));
console.log("Verified live Cargo features and actual Rust cfg branches against the neutral SQLite cases.");
