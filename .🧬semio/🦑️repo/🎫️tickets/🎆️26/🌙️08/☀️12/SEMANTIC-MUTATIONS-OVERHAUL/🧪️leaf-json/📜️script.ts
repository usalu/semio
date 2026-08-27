import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";

//#region 🧪️LeafJsonSchemaOracle
const workspace = process.cwd();
const fixturePath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutation-leaf-json/🧫️fixtures/🔣️cases.json");
const fixtureSchemaPath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutation-leaf-json/🛂️schema/🔣️cases.json");
const descriptorSchemaPath = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const fixtureSchema = JSON.parse(readFileSync(fixtureSchemaPath, "utf8"));
const descriptorSchema = JSON.parse(readFileSync(descriptorSchemaPath, "utf8"));
const ajv = new Ajv({ allErrors: true, strict: true });
const validateFixture = ajv.compile(fixtureSchema);
const validateDescriptor = ajv.compile(descriptorSchema);
let mismatches = 0;
if (!validateFixture(fixture)) {
  console.error(`[DEBUG] fixture schema errors=${JSON.stringify(validateFixture.errors)}`);
  process.exit(1);
}
for (const vector of fixture.cases) {
  let schemaAccepted = false;
  try { schemaAccepted = validateDescriptor(JSON.parse(vector.raw)); } catch { schemaAccepted = false; }
  const matches = schemaAccepted === vector.schemaAccepted;
  console.log(`[DEBUG] ${JSON.stringify({ name: vector.name, schemaAccepted, expectedSchemaAccepted: vector.schemaAccepted, expectedParserAccepted: vector.parserAccepted, diagnostic: vector.diagnostic })}`);
  if (!matches) mismatches += 1;
}
if (mismatches) process.exit(1);
const derive = readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs"), "utf8");
const core = readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs"), "utf8");
const slice = (text: string, start: string, end: string) => text.slice(text.indexOf(start), text.indexOf(end, text.indexOf(start)) + end.length);
const parser = slice(derive, "//#region 🔣️MutationLeafJson", "//#endregion 🔣️MutationLeafJson");
const contract = slice(core, "//#region 🪪️MutationLeafDescriptor", "//#endregion 🪪️MutationLeafDescriptor");
const target = process.env.SEMIO_TEST_COMPILER_ARTIFACT_DIR ?? join(process.env.CARGO_TARGET_DIR ?? join(workspace, "target"), "debug/deps");
const deps = target.endsWith("deps") ? target : join(target, "debug/deps");
const artifact = (name: string) => {
  const rlib = readdirSync(deps).find((file) => file.startsWith(`lib${name}-`) && file.endsWith(".rlib"));
  if (!rlib) throw new Error(`missing retained ${name} rlib in ${deps}`);
  const rmeta = rlib.replace(".rlib", ".rmeta");
  if (!readdirSync(deps).includes(rmeta)) throw new Error(`missing paired retained ${name} rmeta in ${deps}`);
  return join(deps, rmeta);
};
const json = artifact("serde_json"), quote = artifact("quote"), macro = artifact("proc_macro2"), serde = artifact("serde");
const run = join(import.meta.dir, `🧫️run-${Date.now()}`), tokens = join(run, "🔣️tokens"); mkdirSync(tokens, { recursive: true });
const parserSource = `extern crate serde_json;extern crate quote;extern crate proc_macro2;use quote::quote;use std::{collections::HashSet,fs,path::PathBuf};#[derive(Debug)]struct MutationSourceAuthority{owner:String}${parser}fn main(){let f:serde_json::Value=serde_json::from_str(include_str!(${JSON.stringify(fixturePath)})).unwrap();let a=MutationSourceAuthority{owner:f["authorityOwner"].as_str().unwrap().into()};let out=PathBuf::from(std::env::args().nth(1).unwrap());let mut n=0;for v in f["cases"].as_array().unwrap(){let r=parse_mutation_leaf_descriptor(v["raw"].as_str().unwrap().as_bytes(),&a);assert_eq!(r.is_ok(),v["parserAccepted"].as_bool().unwrap(),"{}: {r:?}",v["name"]);if let Err(e)=r{assert!(e.contains(v["diagnostic"].as_str().unwrap()));}else{fs::write(out.join(format!("{n}.tokens")),emit_mutation_leaf_descriptor(&r.unwrap()).to_string()).unwrap();n+=1;}}println!("[DEBUG] parser vectors={} emitted={n}",f["cases"].as_array().unwrap().len());}`;
const parserRs = join(run, "🦀️parser.rs"), parserBin = join(run, "🦀️parser"); writeFileSync(parserRs, parserSource);
const compile = (args: string[]) => spawnSync("rustc", args, { encoding: "utf8", timeout: 30000 });
let result = compile(["--edition=2021", "--crate-name", "leaf_json_parser", parserRs, "-L", `dependency=${deps}`, "--extern", `serde_json=${json}`, "--extern", `quote=${quote}`, "--extern", `proc_macro2=${macro}`, "-o", parserBin]);
writeFileSync(join(run, "🧪️parser-compile.log"), `${result.stdout ?? ""}${result.stderr ?? ""}`); if (result.status) throw new Error("parser compile failed");
result = spawnSync(parserBin, [tokens], { encoding: "utf8", timeout: 30000 }); writeFileSync(join(run, "🧪️parser-runtime.log"), `${result.stdout ?? ""}${result.stderr ?? ""}`); if (result.status) throw new Error("parser execution failed");
let compiled = 0; for (const file of readdirSync(tokens).filter((file) => file.endsWith(".tokens"))) { const expression = readFileSync(join(tokens, file), "utf8"); const source = `extern crate serde;extern crate serde_json;mod semio_framework_os_kernel{${contract}}mod adversarial{pub enum Option<T>{Some(T),None}}use adversarial::Option::{Some,None};const VALUE:semio_framework_os_kernel::MutationLeafDescriptor=${expression};fn main(){assert!(VALUE.validate().is_ok());let v=serde_json::to_value(VALUE).unwrap();assert_eq!(v["schemaVersion"],1);assert!(v["outcomeClasses"].as_array().unwrap().len()>0);assert!(v["requiredLanguageSurfaces"].as_array().unwrap().iter().any(|x|x=="rust"));}`; const rs=join(tokens,file.replace(".tokens",".rs")), bin=join(tokens,file.replace(".tokens",""));writeFileSync(rs,source);result=compile(["--edition=2021","--crate-name","leaf_json_token",rs,"-L",`dependency=${deps}`,"--extern",`serde=${serde}`,"--extern",`serde_json=${json}`,"-o",bin]);writeFileSync(join(tokens,file.replace(".tokens",".compile.log")),`${result.stdout??""}${result.stderr??""}`);if(result.status)throw new Error(`token compile failed ${file}`);result=spawnSync(bin,{encoding:"utf8",timeout:30000});if(result.status)throw new Error(`token runtime failed ${file}`);compiled++;}
console.log(`[DEBUG] leaf-json vectors=${fixture.cases.length} emitted=${compiled} run=${run}`);
//#endregion 🧪️LeafJsonSchemaOracle
