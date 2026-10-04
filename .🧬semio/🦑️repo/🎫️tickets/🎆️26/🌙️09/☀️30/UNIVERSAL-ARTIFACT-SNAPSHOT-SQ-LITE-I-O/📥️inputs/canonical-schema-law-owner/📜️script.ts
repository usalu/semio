#!/usr/bin/env bun
import { readFileSync, writeFileSync, mkdirSync, existsSync, unlinkSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
let root = resolve(process.cwd());
while (!existsSync(join(root, ".git"))) {
  const parent = dirname(root);
  if (parent === root) throw Error("Repository root missing");
  root = parent;
}
const ticket = resolve(import.meta.dir, "../..");
const core = join(root, "🧰️framework/🔨️modules/🎒️pack");
const os = join(root, "🧰️framework/🛍️products/💻️os");
const changes: {path:string; before:string|null; after:string|null}[] = [];
const read = (path:string) => readFileSync(path, "utf8");
const region = (path:string, before:string, after:string) => {
  const original = read(path);
  if (original.includes(after) && !original.includes(before)) return;
  if (original.split(before).length !== 2) throw Error("Unique region missing: " + path + " " + before.slice(0, 100));
  changes.push({path, before:original, after:original.replace(before, after)});
};
const stage = (path:string, update:(text:string)=>string) => {
  const before = existsSync(path) ? read(path) : null;
  const after = update(before ?? "");
  if (after !== before) changes.push({path, before, after});
};
const phase = process.argv[2];
if (phase === "demand") {
  const storage = join(core, "🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs");
  const old = read(join(os, "🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"));
  const adapted = old.replace("owning Kernel controlled schema authority", "canonical Pack controlled schema authority").replaceAll("crate::os_pack::", "crate::record::").replace("../../../../../../../../🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json", "../../../🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json");
  stage(storage, original => {
    if (original && original !== adapted) throw Error("Different canonical storage law already exists");
    return adapted;
  });
  const record = join(core, "🌱️value/🦀️.rs");
  const mount = '\n#[cfg(test)]\n#[path="🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"]\nmod schema_storage_tests;\n';
  stage(record, text => text.includes("mod schema_storage_tests;") ? text : text + mount);
  const manifest = join(core, "📦️packages/🦀️rust/Cargo.toml");
  stage(manifest, text => {
    const test = '[[test]]\nname = "pack_schema_hash"\npath = "../../🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs"\n\n';
    if (!text.includes('name = "pack_schema_hash"')) text = text.replace("[features]\n", test + "[features]\n");
    if (!text.includes('blake3 = "1.8.2"')) text = text.replace("[dev-dependencies]\n", '[dev-dependencies]\nblake3 = "1.8.2"\n');
    return text;
  });
  const script = join(core, "📦️packages/🦀️rust/📜️script.ts");
  const classes = `/** 🔑️ Selects the canonical graph laws with their independent BLAKE3 oracle. */
class SchemaHashNativeScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("test-schema-hash-native accepts no arguments");
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-pack"],cwd:this.root,extraArgs:["--test","pack_schema_hash","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
  }
}

/** 💰️ Selects complete schema scratch accounting on both native controls. */
class SchemaStorageNativeScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("test-schema-storage-native accepts no arguments");
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-pack"],cwd:this.root,extraArgs:["--lib","schema_hash_controlled_full_allocator_requests_and_same_caller_are_admitted","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
  }
}

`;
  stage(script, text => {
    if (!text.includes("class SchemaHashNativeScript")) text = text.replace("class BuildScript extends BundleScript {", classes + "class BuildScript extends BundleScript {");
    if (!text.includes('.register("test-schema-hash-native",SchemaHashNativeScript)')) text = text.replace(".register(\"test-borrowed-preflight-native\",BorrowedPreflightNativeScript);", '.register("test-borrowed-preflight-native",BorrowedPreflightNativeScript).register("test-schema-hash-native",SchemaHashNativeScript).register("test-schema-storage-native",SchemaStorageNativeScript);');
    if (!text.includes('.register("test-schema-hash-native",SchemaHashNativeScript)')) throw Error("Core router anchor changed");
    return text;
  });
  stage(join(core, "📦️packages/🦀️rust/📋️project.json"), text => {
    const project = JSON.parse(text);
    for (const name of ["test-schema-hash-native", "test-schema-storage-native"]) {
      const target = {executor:"nx:run-commands",options:{cwd:project.root,command:"bun ./📜️script.ts " + name,forwardAllArgs:false},cache:false};
      if (project.targets[name] && JSON.stringify(project.targets[name]) !== JSON.stringify(target)) throw Error("Conflicting Core target " + name);
      project.targets[name] = target;
    }
    return JSON.stringify(project,null,2) + "\n";
  });
  stage(join(root, ".vscode/launch.json"), text => {
    const old = '"command": "bun nx run @semio-tech/framework-os-kernel:test-schema-storage-native --skip-nx-cache"';
    const replacement = '"command": "bun nx run @semio-tech/framework-pack-rs:test-schema-storage-native --skip-nx-cache"';
    if (text.includes(old)) text = text.replace(old,replacement);
    if (!text.includes(replacement)) throw Error("Storage launch anchor missing");
    const anchor = '    {\n      "name": "🧪️test🎒️pack🧬️schema💰️storage🦀️native",';
    if (!text.includes('"name": "🧪️test🎒️pack🧬️schema🔑️hash🦀️native"')) {
      if (!text.includes(anchor)) throw Error("Storage launch insertion missing");
      text = text.replace(anchor,'    {\n      "name": "🧪️test🎒️pack🧬️schema🔑️hash🦀️native",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun nx run @semio-tech/framework-pack-rs:test-schema-hash-native --skip-nx-cache",\n      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "9_gates",\n        "order": 900.0583565\n      }\n    },\n' + anchor);
    }
    return text;
  });
} else if (phase === "provider") {
  const helper = join(core, "🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs");
  stage(helper, text => {
    text = text.replace("use semio_framework_pack_record::{schema_hash, PackSchemaGraph};", "use pack::record::{schema_hash, PackSchemaGraph};").replace("../../🧫️fixtures/🔑️schema-hash/🔣️.json", "../../../🧫️fixtures/🔑️schema-hash/🔣️.json");
    for (const name of ["inner_spec","scalar_spec_controlled","inner_controlled","inner_producer","schema_hash_case_spec","schema_hash_fixture","hex_of","graph_json","independent_canonical_bytes"]) {
      text = text.replace("\nfn " + name + (name === "scalar_spec_controlled" || name === "inner_controlled" ? "<" : "("), "\npub(crate) fn " + name + (name === "scalar_spec_controlled" || name === "inner_controlled" ? "<" : "("));
    }
    return text;
  });
  stage(join(core,"📦️packages/🦀️rust/🦀️.rs"),text => text.includes("extern crate self as pack;") ? text : text.replace("pub use protocol::codec;","extern crate self as pack;\n\npub use protocol::codec;"));
  stage(join(core,"🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"),text => text.replace("../../../🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json", "../../../../🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json"));
} else if (phase === "test-fixture-joins") {
  const files=[
    join(os,"🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs"),
    join(os,"🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),
    join(os,"🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs")
  ];
  for(const file of files)stage(file,text => {
    for(const owner of ["crate::os_store::PackError","PackError"]){
      text=text.replaceAll('.map_err(|e| '+owner+'::Schema(e.to_string()))','.map_err(|error| '+owner+'::from(error.into_value_error()))');
      const prefix='return Err('+owner+'::Schema(format!(';
      text=text.split("\n").map(line => {
        if(!line.includes(prefix))return line;
        if(!line.trimEnd().endsWith(')));'))throw Error("Different envelope mismatch syntax");
        return line.replace(prefix,'return Err('+owner+'::from(pack::PackRefusal::Schema { kind: semio_framework_value::ValueRefusalKind::UnsupportedOwner, detail: format!(').replace(/\)\)\);\s*$/u,') }));');
      }).join("\n");
    }
    text=text.replace('Self::from_value(<semio_framework_value::DslValue as ArtifactPack>::decode_pack_with(bytes, options)?).map_err(|error| PackError::Schema(error.to_string()))','Self::from_value(<semio_framework_value::DslValue as ArtifactPack>::decode_pack_with(bytes, options)?).map_err(PackError::from)');
    text=text.replace('Self::__dsl_from_record(&record).map_err(|err| PackError::Schema(err.to_string()))','Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)');
    return text;
  });
} else if (phase === "retire-os-schema-laws") {
  stage(join(core,"📦️packages/🦀️rust/package.json"),text => {
    const manifest=JSON.parse(text);
    for(const command of ["test-schema-hash-native","test-schema-storage-native"])manifest.scripts[command]="bun nx run @semio-tech/framework-pack-rs:"+command;
    return JSON.stringify(manifest,null,2)+"\n";
  });
  stage(join(os,"📦️packages/🦀️rust/package.json"),text => {
    const manifest=JSON.parse(text);delete manifest.scripts["test-schema-storage-native"];
    return JSON.stringify(manifest,null,2)+"\n";
  });
  stage(join(os,"📦️packages/🦀️rust/📋️project.json"),text => {
    const project=JSON.parse(text);delete project.targets["test-schema-storage-native"];
    return JSON.stringify(project,null,2)+"\n";
  });
  stage(join(os,"📦️packages/🦀️rust/Cargo.toml"),text => text.replace('[[test]]\nname = "pack_schema_hash"\npath = "../../🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs"\n\n','').replace('# wasm32 test build of this crate (`--target wasm32-wasip2 --test pack_schema_hash`) never tries to','# wasm32 test build of this crate never tries to'));
  stage(join(os,"🔨️modules/🎒️pack/🌱️value/🦀️.rs"),text => text.replace('#[cfg(test)]\n#[path="🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"]\nmod schema_storage_tests;\n',''));
  stage(join(os,"📦️packages/🦀️rust/📜️script.ts"),text => {
    const start=text.indexOf('/** 🧮️ Measures complete schema scratch requests on both canonical controls. */');
    if(start>=0){const end=text.indexOf('/** ♻️ Proves canonical intrinsic cleanup',start);if(end<0)throw Error("Schema storage router end missing");text=text.slice(0,start)+text.slice(end);}
    return text.replace('  .register("test-schema-storage-native", SchemaStorageNativeTestScript)\n','');
  });
} else if (phase === "fixture-paths") {
  stage(join(core,"🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs"),text => text.replace('include_str!("../../../🧫️fixtures/🔑️schema-hash/🔣️.json")','include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../🌱️value/🧫️fixtures/🔑️schema-hash/🔣️.json"))'));
  stage(join(core,"🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"),text => text.replace('include_str!("../../../../🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json")','include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../🌱️value/🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json"))'));
} else if (phase === "ownership-demand") {
  stage(join(core,"🌱️value/🧪️tests/🧱️ownership/🟦️.ts"),()=>`/** 📦️ Admits the actual canonical Record module and its genuine Pack package. */
import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import Ajv from "ajv/dist/2020.js";
import TOML from "@iarna/toml";
import fixture from "../../🧫️fixtures/🧩️ownership/🔣️.json";
import schema from "../../🧬️schema/🧩️ownership/🔣️.json";
type Ownership={package:string;module:string;manifest:string;library:{name:string;path:string};mount:string;dependencies:string[];publicEntries:string[];forbiddenProviders:string[]};
const contract=fixture as unknown as Ownership,owner=resolve(import.meta.dir,"../.."),read=(path:string)=>readFileSync(resolve(owner,path),"utf8");

test("the closed Record contract names its actual package and public module",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(contract.package).toBe("semio-framework-pack");expect(contract.module).toBe("record");
 expect(new Set(contract.dependencies).intersection(new Set(contract.forbiddenProviders)).size).toBe(0);
 for(const entry of contract.publicEntries)expect(read("🦀️.rs")).toContain("pub fn "+entry+"(");
});

test("one actual Pack library mounts Record and exposes no product source authority",()=>{
 expect(contract.manifest).toBe("../📦️packages/🦀️rust/Cargo.toml");
 const path=resolve(owner,contract.manifest);expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8"),native=Bun.TOML.parse(source) as {package:{name:string};lib:{name:string;path:string};dependencies:Record<string,unknown>};
 expect(native).toEqual(TOML.parse(source) as typeof native);expect(native.package.name).toBe(contract.package);expect(native.lib).toEqual(contract.library);
 expect(Object.keys(native.dependencies).sort()).toEqual([...contract.dependencies].sort());expect(native.dependencies).not.toHaveProperty(contract.package);
 const library=readFileSync(resolve(dirname(path),contract.library.path),"utf8");expect(library).toContain('#[path = "'+contract.mount+'"]');expect(library).toContain("pub mod "+contract.module+";");
 expect(resolve(dirname(path),contract.mount)).toBe(resolve(owner,"🦀️.rs"));
 for(const path of ["🦀️.rs","🛫️encode/🦀️.rs","🏭️schema/🦀️.rs","🔎️scalar-witness/🦀️.rs"])for(const forbidden of ["crate::os_","semio_framework_os_kernel::"])expect(read(path)).not.toContain(forbidden);
});
`);
} else if (phase === "ownership-provider") {
  const contract={format:1,package:"semio-framework-pack",module:"record",manifest:"../📦️packages/🦀️rust/Cargo.toml",library:{name:"pack",path:"🦀️.rs"},mount:"../../🌱️value/🦀️.rs",dependencies:["semio-framework-async","semio-framework-diagnostic","semio-framework-dsl-record","semio-framework-hash","semio-framework-pack-error","semio-framework-pack-json","semio-framework-replication","semio-framework-value","ureq"],publicEntries:["encode_document","decode_document","encode_record_body","decode_record_body_exact","schema_hash"],forbiddenProviders:["semio-framework-os-kernel","semio-framework-plugin"]};
  stage(join(core,"🌱️value/🧫️fixtures/🧩️ownership/🔣️.json"),()=>JSON.stringify(contract,null,2)+"\n");
  const properties=Object.fromEntries(Object.entries(contract).map(([name,value])=>[name,{const:value}]));
  stage(join(core,"🌱️value/🧬️schema/🧩️ownership/🔣️.json"),()=>JSON.stringify({$schema:"https://json-schema.org/draft/2020-12/schema",type:"object",additionalProperties:false,required:Object.keys(contract),properties},null,2)+"\n");
  stage(join(core,"🌱️value/📦️packages/🦀️rust/📜️script.ts"),text => text.replace('await runCargoTestsV1({manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-pack-record"], cwd: this.root, extraArgs: rest}, readCargoTestPolicyV1(process.env));','const manifestPath=resolve(this.root,"../../../📦️packages/🦀️rust/Cargo.toml");\n    await runCargoTestsV1({manifestPath,packages:["semio-framework-pack"],cwd:resolve(this.root,"../../../📦️packages/🦀️rust"),extraArgs:["--lib","record::",...rest]},readCargoTestPolicyV1(process.env));'));
  stage(join(core,"🌱️value/📦️packages/🦀️rust/📋️project.json"),text => {
    const project=JSON.parse(text),input="{workspaceRoot}/🧰️framework/🔨️modules/🎒️pack/**/*";
    if(!project.namedInputs.default.includes(input))project.namedInputs.default.unshift(input);
    return JSON.stringify(project,null,2)+"\n";
  });
} else if (phase === "ifc-native-current-source-oracle") {
  const base=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc");
  for(const branch of ["🏅️standards/4️⃣4/🪆️subsets/✳️any","🏅️standards/🔖️2x3/🪆️subsets/🧱️base"]){
    const tests=join(base,branch,"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs");
    stage(tests,text=>{const start=text.indexOf("fn sqlite_snapshot_"+(branch.includes("4️⃣4")?"ifc4_independent_sqlite_full_value_query_and_edit":"ifc2x3_independent_sqlite_decimal_and_edm_field_edit")),end=text.indexOf("\n",start);if(start<0||end<0)throw Error("IFC owning oracle missing");let body=text.slice(start,end);for(const before of ["await import('@semio-tech/framework')","await import('@semio-tech/stdio-ifc')"]){if(body.split(before).length!==2)throw Error("IFC owner import changed");body=body.replace(before,before.includes("framework")?"await import(process.argv[1])":"await import(process.argv[2])");}const before='let mut child=Command::new("bun").args(["-e",script])';if(body.split(before).length!==2)throw Error("IFC oracle spawn changed");const after='let framework=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts").canonicalize().unwrap();let artifact=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🟦️.ts").canonicalize().unwrap();let mut child=Command::new("bun").args(["-e",script,framework.to_str().unwrap(),artifact.to_str().unwrap()])';body=body.replace(before,after);return text.slice(0,start)+body+text.slice(end);});
  }
} else if (phase === "png-native-factory-runtime-witness") {
  const snapshot=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
  region(join(snapshot,"🧪️tests/🪶️sqlite/🦀️.rs"),'    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")',`    let factories=crate::native_codecs();let factory=&factories[0];let codec=(factory.codec)();let kind=(factory.kind)();
    let declared:serde_json::Value=serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();let binding=&declared["codecs"][0]["native_factory"];
    let hash:String=codec.pack_schema_hash.iter().map(|byte|format!("{byte:02x}")).collect();
    eprintln!("[DEBUG] PNG actual native factory={} artifact={} kind={} schema={} extension={} pack_hash={} declared_binding={}",factory.id,factory.artifact,kind.id,codec.schema,codec.extension,hash,binding);
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")`);
} else if (phase === "ifc-example-pure-refusal-authority") {
  const tests=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs");
  region(tests,"Err(store::PackError::ValueRefusal(error))","Err(store::PackError::Refusal(store::PackRefusal::ValueRefusal(error)))");
} else if (phase === "png-native-law-selection") {
  const tests=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs");
  stage(tests,text=>{
    for(const [before,after] of [["typed_sqlite_file_round_trip_preserves_chunk_order_and_idat_boundaries","sqlite_snapshot_png_public_file_round_trip_preserves_chunk_order_and_idat_boundaries"],["reconstruction_refuses_invalid_canonical_bytes_atomically","sqlite_snapshot_png_reconstruction_refuses_invalid_canonical_bytes_atomically"],["sqlite_projection_and_reconstruction_obey_cancellation_and_ownership_limits","sqlite_snapshot_png_projection_and_reconstruction_obey_cancellation_and_ownership_limits"]]){
      const old="fn "+before+"()",replacement="fn "+after+"()";
      if(text.includes(replacement)&&!text.includes(old))continue;
      if(text.split(old).length!==2)throw Error("Unique PNG native law signature missing: "+before);
      text=text.replace(old,replacement);
    }
    return text;
  });
} else if (phase === "png-logical-owner-corpus-path") {
  const snapshot=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
  region(join(snapshot,"🧪️tests/🪆️owner/🦀️.rs"),'include_str!("../../../🧫️fixtures/🪶️sqlite/🪆️owner/🔣️.json")','include_str!("../../🧫️fixtures/🪶️sqlite/🪆️owner/🔣️.json")');
} else if (phase === "semio-paid-copy-cancellation-witness") {
  const base=join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot");
  stage(join(base,"🧪️tests/🪶️sqlite/🦀️.rs"),text => {
    const before='  let mut reached=false;let mut cancel=|event:SqliteSnapshotProgress|{let stop=event.phase==SqliteSnapshotPhase::DecodeNative&&event.total==schema_bytes&&event.completed>=65536&&event.completed<event.total;reached|=stop;!stop};';
    const after='  let mut reached=false;let mut validation_complete=false;let mut cancel=|event:SqliteSnapshotProgress|{let schema_stage=event.phase==SqliteSnapshotPhase::DecodeNative&&event.total==schema_bytes;if schema_stage&&event.completed==event.total{validation_complete=true;}let paid_stage=encoding==SnapshotEncoding::Text||validation_complete;let stop=schema_stage&&paid_stage&&event.completed>=65536&&event.completed<event.total;reached|=stop;!stop};';
    if(text.split(before).length!==2)throw Error("Actual Semio paid copy cancellation demand changed");
    text=text.replace(before,after);
    const settled='assert!(admitted>=schema_bytes,"allocated owner fields stay charged after cancellation");drop(input);assert!(reached);';
    if(text.split(settled).length!==2)throw Error("Actual Semio paid ownership assertion changed");
    return text.replace(settled,settled+'if encoding==SnapshotEncoding::Binary{assert!(validation_complete,"actual borrowed UTF8 validation must finish before paid copy cancellation");}');
  });
} else if (phase === "ordering-authority" || phase === "proof-and-ordering") {
  if(phase === "proof-and-ordering")stage(join(core,"🌱️value/📜️script.ts"),text => {
    const before='await import(join(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🟦️.ts"))';
    if(text.split(before).length!==2)throw Error("Actual wire proof decoder join changed");
    return text.replace(before,'await import(join(repoRoot,"🧰️framework/🛍️products/💻️os/🟦️.ts"))');
  });
  stage(join(core,"🌱️value/🧫️fixtures/🔃️ordering/🔣️.json"),()=>"{\n  \"format\": 1,\n  \"fieldMap\": {\n    \"unsorted\": [\n      {\n        \"key\": \"zeta\",\n        \"entries\": [\n          {\n            \"key\": \"é\",\n            \"value\": 9\n          },\n          {\n            \"key\": \"z\",\n            \"value\": 8\n          },\n          {\n            \"key\": \"a\\u0000\",\n            \"value\": 7\n          }\n        ]\n      },\n      {\n        \"key\": \"é\",\n        \"entries\": [\n          {\n            \"key\": \"b\",\n            \"value\": 6\n          },\n          {\n            \"key\": \"a\",\n            \"value\": 5\n          }\n        ]\n      },\n      {\n        \"key\": \"alpha\",\n        \"entries\": [\n          {\n            \"key\": \"b\",\n            \"value\": 4\n          },\n          {\n            \"key\": \"a\",\n            \"value\": 3\n          }\n        ]\n      },\n      {\n        \"key\": \"a\\u0000\",\n        \"entries\": [\n          {\n            \"key\": \"q\",\n            \"value\": 2\n          }\n        ]\n      },\n      {\n        \"key\": \"Mid\",\n        \"entries\": [\n          {\n            \"key\": \"y\",\n            \"value\": 1\n          },\n          {\n            \"key\": \"x\",\n            \"value\": 0\n          }\n        ]\n      }\n    ],\n    \"canonical\": [\n      {\n        \"key\": \"Mid\",\n        \"entries\": [\n          {\n            \"key\": \"x\",\n            \"value\": 0\n          },\n          {\n            \"key\": \"y\",\n            \"value\": 1\n          }\n        ]\n      },\n      {\n        \"key\": \"a\\u0000\",\n        \"entries\": [\n          {\n            \"key\": \"q\",\n            \"value\": 2\n          }\n        ]\n      },\n      {\n        \"key\": \"alpha\",\n        \"entries\": [\n          {\n            \"key\": \"a\",\n            \"value\": 3\n          },\n          {\n            \"key\": \"b\",\n            \"value\": 4\n          }\n        ]\n      },\n      {\n        \"key\": \"zeta\",\n        \"entries\": [\n          {\n            \"key\": \"a\\u0000\",\n            \"value\": 7\n          },\n          {\n            \"key\": \"z\",\n            \"value\": 8\n          },\n          {\n            \"key\": \"é\",\n            \"value\": 9\n          }\n        ]\n      },\n      {\n        \"key\": \"é\",\n        \"entries\": [\n          {\n            \"key\": \"a\",\n            \"value\": 5\n          },\n          {\n            \"key\": \"b\",\n            \"value\": 6\n          }\n        ]\n      }\n    ]\n  },\n  \"literalObject\": [\n    {\n      \"key\": \"z\",\n      \"value\": \"first\"\n    },\n    {\n      \"key\": \"a\\u0000\",\n      \"value\": \"middle\"\n    },\n    {\n      \"key\": \"z\",\n      \"value\": \"duplicate\"\n    },\n    {\n      \"key\": \"é\",\n      \"value\": \"last\"\n    }\n  ]\n}\n");
  stage(join(core,"🌱️value/🧬️schema/🔃️ordering/🔣️.json"),()=>"{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"type\": \"object\",\n  \"additionalProperties\": false,\n  \"required\": [\n    \"format\",\n    \"fieldMap\",\n    \"literalObject\"\n  ],\n  \"properties\": {\n    \"format\": {\n      \"const\": 1\n    },\n    \"fieldMap\": {\n      \"type\": \"object\",\n      \"additionalProperties\": false,\n      \"required\": [\n        \"unsorted\",\n        \"canonical\"\n      ],\n      \"properties\": {\n        \"unsorted\": {\n          \"$ref\": \"#/$defs/map\"\n        },\n        \"canonical\": {\n          \"$ref\": \"#/$defs/map\"\n        }\n      }\n    },\n    \"literalObject\": {\n      \"type\": \"array\",\n      \"minItems\": 1,\n      \"items\": {\n        \"$ref\": \"#/$defs/literal\"\n      }\n    }\n  },\n  \"$defs\": {\n    \"map\": {\n      \"type\": \"array\",\n      \"minItems\": 1,\n      \"items\": {\n        \"type\": \"object\",\n        \"additionalProperties\": false,\n        \"required\": [\n          \"key\",\n          \"entries\"\n        ],\n        \"properties\": {\n          \"key\": {\n            \"type\": \"string\"\n          },\n          \"entries\": {\n            \"type\": \"array\",\n            \"minItems\": 1,\n            \"items\": {\n              \"$ref\": \"#/$defs/leaf\"\n            }\n          }\n        }\n      }\n    },\n    \"leaf\": {\n      \"type\": \"object\",\n      \"additionalProperties\": false,\n      \"required\": [\n        \"key\",\n        \"value\"\n      ],\n      \"properties\": {\n        \"key\": {\n          \"type\": \"string\"\n        },\n        \"value\": {\n          \"type\": \"integer\",\n          \"minimum\": 0,\n          \"maximum\": 9007199254740991\n        }\n      }\n    },\n    \"literal\": {\n      \"type\": \"object\",\n      \"additionalProperties\": false,\n      \"required\": [\n        \"key\",\n        \"value\"\n      ],\n      \"properties\": {\n        \"key\": {\n          \"type\": \"string\"\n        },\n        \"value\": {\n          \"type\": \"string\"\n        }\n      }\n    }\n  }\n}\n");
  stage(join(core,"🌱️value/🧪️tests/🔃️ordering/🟦️.ts"),()=>"/** 🔃️ SQLite independently separates schema map byte order and intrinsic occurrence order. */\nimport{test,expect}from\"bun:test\";\nimport{Database}from\"bun:sqlite\";\nimport Ajv from\"ajv/dist/2020.js\";\nimport fixture from\"../../🧫️fixtures/🔃️ordering/🔣️.json\";\nimport schema from\"../../🧬️schema/🔃️ordering/🔣️.json\";\n\ntest(\"closed schema maps sort byte keys while intrinsic occurrences retain duplicates\",()=>{\n const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);expect(valid({...fixture,payload:\"opaque\"})).toBe(false);\n const database=new Database(\":memory:\");try{\n  database.exec(\"CREATE TABLE map_entry(parent_key BLOB NOT NULL,entry_key BLOB NOT NULL,value INTEGER NOT NULL);CREATE TABLE literal_occurrence(ordinal INTEGER PRIMARY KEY,entry_key BLOB NOT NULL,value TEXT NOT NULL)\");\n  for(const row of fixture.fieldMap.unsorted)for(const entry of row.entries)database.query(\"INSERT INTO map_entry VALUES(?,?,?)\").run(Buffer.from(row.key),Buffer.from(entry.key),entry.value);\n  fixture.literalObject.forEach((entry,ordinal)=>database.query(\"INSERT INTO literal_occurrence VALUES(?,?,?)\").run(ordinal,Buffer.from(entry.key),entry.value));\n  const file=database.serialize(),reopened=Database.deserialize(file);try{\n   const expected=fixture.fieldMap.canonical.flatMap(row=>row.entries.map(entry=>({parent_key:Buffer.from(row.key).toString(\"hex\").toUpperCase(),entry_key:Buffer.from(entry.key).toString(\"hex\").toUpperCase(),value:entry.value})));\n   expect(reopened.query(\"SELECT hex(parent_key) AS parent_key,hex(entry_key) AS entry_key,value FROM map_entry ORDER BY parent_key,entry_key\").all()).toEqual(expected);\n   expect(reopened.query(\"SELECT hex(entry_key) AS key,value FROM literal_occurrence ORDER BY ordinal\").all()).toEqual(fixture.literalObject.map(entry=>({key:Buffer.from(entry.key).toString(\"hex\").toUpperCase(),value:entry.value})));\n   expect(reopened.query(\"SELECT COUNT(*) AS n FROM literal_occurrence WHERE entry_key=?\").get(Buffer.from(\"z\"))).toEqual({n:2});\n  }finally{reopened.close();}\n }finally{database.close();}\n console.log(\"[DEBUG] independent SQLite schema map ordering and duplicate intrinsic occurrence contract\");\n});\n");
  stage(join(core,"🌱️value/🧪️tests/🚦️refusals/🟦️.ts"),text=>{
    const before='import "../🎞️intrinsic-media/🟦️.ts";';
    if(text.split(before).length!==2)throw Error("Actual refusal Source ordering import changed");
    return text.replace(before,before+'\nimport "../🔃️ordering/🟦️.ts";');
  });
  stage(join(core,"🌱️value/🧪️tests/🔬️unit/🦀️.rs"),text=>{
    const start=text.indexOf("//#region 🔖️CanonicalObjectOrder"),end=text.indexOf("//#endregion 🔖️CanonicalObjectOrder",start);
    if(start<0||end<0)throw Error("Conflicting Object law region changed");
    text=text.slice(0,start)+"//#region 🔖️SchemaMapAndLiteralOrder\n/// 🔃️ Schema maps sort key bytes while intrinsic Objects preserve exact ordered occurrences.\n#[test]\nfn schema_map_keys_are_canonical_and_intrinsic_objects_preserve_occurrences() {\n    let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"),\"/../../🌱️value/🧫️fixtures/🔃️ordering/🔣️.json\"))).expect(\"neutral ordering contract\");\n    let map=|name:&str|FieldValue::Map(fixture[\"fieldMap\"][name].as_array().unwrap().iter().map(|row|(row[\"key\"].as_str().unwrap().to_owned(),FieldValue::Map(row[\"entries\"].as_array().unwrap().iter().map(|entry|(entry[\"key\"].as_str().unwrap().to_owned(),FieldValue::UInt(entry[\"value\"].as_u64().unwrap()))).collect()))).collect());\n    let unsorted=map(\"unsorted\");let canonical=map(\"canonical\");\n    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,\"maps\",Shape::Map(Box::new(Shape::Map(Box::new(Shape::UInt)))))]);\n    let record=|value:FieldValue|{let mut record=RecordValue::default();record.fields.insert(1,value);record};\n    let plain=|value:&FieldValue|encode_record_body(&spec,&record(value.clone()),&EncodeOptions::default()).expect(\"ordinary schema map\");\n    let controlled=|value:&FieldValue|{let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut allow);encode_record_body_controlled(&spec,&record(value.clone()),&EncodeOptions::default(),&mut control).expect(\"controlled schema map\")};\n    assert_eq!(plain(&unsorted),plain(&canonical));\n    assert_eq!(controlled(&unsorted),controlled(&canonical));\n    assert_eq!(plain(&unsorted),controlled(&unsorted));\n    assert_eq!(decode_record_body_exact(&controlled(&unsorted),&spec,&DecodeOptions::default()).expect(\"decoded canonical schema map\"),record(canonical));\n    let entries:Vec<_>=fixture[\"literalObject\"].as_array().unwrap().iter().map(|entry|(entry[\"key\"].as_str().unwrap().to_owned(),DslValue::String(entry[\"value\"].as_str().unwrap().to_owned()))).collect();\n    let original=DslValue::Object(entries.clone());let mut reordered=entries;reordered.reverse();let reordered=DslValue::Object(reordered);\n    let literal_spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,\"value\",Shape::Value)]);\n    let literal_plain=|value:&DslValue|encode_record_body(&literal_spec,&record(FieldValue::Value(value.clone())),&EncodeOptions::default()).expect(\"ordinary literal Object\");\n    let literal_controlled=|value:&DslValue|{let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut allow);encode_value_record_body_controlled(1,value,&EncodeOptions::default(),&mut control).expect(\"controlled literal Object\")};\n    assert_ne!(literal_plain(&original),literal_plain(&reordered));\n    assert_ne!(literal_controlled(&original),literal_controlled(&reordered));\n    assert_eq!(literal_plain(&original),literal_controlled(&original));\n    assert_eq!(decode_record_body_exact(&literal_controlled(&original),&literal_spec,&DecodeOptions::default()).expect(\"literal occurrence roundtrip\"),record(FieldValue::Value(original)));\n    eprintln!(\"[DEBUG] schema map byte ordering and intrinsic duplicate literal occurrences match independent neutral contract\");\n}\n//#endregion 🔖️SchemaMapAndLiteralOrder\n"+text.slice(end+"//#endregion 🔖️CanonicalObjectOrder".length);
    return text.replace('    // `Map`/`DslValue::Object` are `Vec`-backed so `PartialEq` is order-sensitive; since\n    // canonical encoding always sorts entries by key bytes, these fixtures are pre-sorted\n    // ("aaa" < "zzz", "arr" < "k") so the round-trip equality check below holds exactly.\n','');
  });
} else if (phase === "wire-proof-owner") {
  const oldOwner=join(os,"🔨️modules/🎒️pack/🌱️value/📜️script.ts"),old=read(oldOwner);
  stage(join(core,"🌱️value/📜️script.ts"),text => {
    if(text)throw Error("Canonical wire proof owner already authored");
    const peerStart=old.indexOf('    const peer=join(repoRoot,'),peerEnd=old.indexOf('    const {decodePackValue,packValueToExactJson}',peerStart);
    if(peerStart<0||peerEnd<0)throw Error("Duplicate wire proof peer region changed");
    return (old.slice(0,peerStart)+old.slice(peerEnd)).replace('root=join(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value")','root=join(repoRoot,"🧰️framework/🔨️modules/🎒️pack/🌱️value")').replace('await import("../../../🟦️.ts")','await import(join(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🟦️.ts"))').replace('console.log(`wire materialization:','console.log(`[DEBUG] wire materialization:');
  });
  stage(join(core,"🌱️value/🧪️tests/🚦️refusals/🟦️.ts"),text => {
    text=text.replace('🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/📜️script.ts','🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts');
    const start=text.indexOf('    const root=resolve(import.meta.dir,"../../../../../.."),os='),end=text.indexOf('    const database=new Database',start);
    if(start<0||end<0)throw Error("Duplicate refusal peer assertions changed");
    return text.slice(0,start)+text.slice(end);
  });
  stage(join(root,"🌎️hub/📦️packages/🦀️rust/📜️script.ts"),text => {
    const before='../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/📜️script.ts';
    if(text.split(before).length!==2)throw Error("Actual Hub wire proof caller changed");
    return text.replace(before,'../../../🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts');
  });
  changes.push({path:oldOwner,before:old,after:null});
} else if (phase === "ownership-native-policy") {
  stage(join(core,"🌱️value/📦️packages/🦀️rust/📜️script.ts"),text => {
    const before='    const manifestPath=resolve(this.root,"../../../📦️packages/🦀️rust/Cargo.toml");\n    await runCargoTestsV1';
    const after='    const manifestPath=resolve(this.root,"../../../📦️packages/🦀️rust/Cargo.toml");\n    if(!process.env.SEMIO_CARGO_TEST_POLICY){\n      await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"),"native","owner-command","--manifest",manifestPath,"--cwd",resolve(this.root,"../../../📦️packages/🦀️rust"),"--",process.execPath,resolve(this.root,"📜️script.ts"),"test-native",...segments],{cwd:this.repoRoot,budgetMs:0,throwOnFailure:true});\n      return;\n    }\n    await runCargoTestsV1';
    if(text.split(before).length!==2)throw Error("Exact Record native owner policy boundary changed");
    return text.replace(before,after);
  });
} else if (phase === "actor-local-truncation-authority") {
  stage(join(root,"🧰️framework/🔨️modules/🎭️actor/🦀️.rs"),text => {
    const local="PackError::Refusal(PackRefusal::Truncated(*pos, what))";
    if(text.split(local).length!==9)throw Error("Actor local primitive truncation call sites changed");
    text=text.replaceAll(local,"PackError::Truncated(*pos, what)");
    for(const field of ["ui_patch_receipt","lifecycle_receipt"]){
      const qualified='pack::PackError::Refusal(pack::PackRefusal::Truncated(*pos, "TurnResult::'+field+'"))';
      if(text.split(qualified).length!==2)throw Error("Actor local receipt truncation call site changed");
      text=text.replace(qualified,'pack::PackError::Truncated(*pos, "TurnResult::'+field+'")');
    }
    return text;
  });
} else if (phase === "native-test-constructor-closure") {
  for(const path of [join(os,"🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),join(os,"🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs"),join(os,"🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs")])stage(path,text=>text.split('\n').map(line=>line.includes("ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!")?line.replace("envelope.binary_token())));","envelope.binary_token()))));"):line).join('\n'));
} else if (phase === "native-test-sync-refusal") {
  stage(join(os,"🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs"),text => text.split('\n').map(line=>line.includes("pack::PackRefusal::Schema { kind:")?line.replace("pack::PackRefusal::Schema { kind: semio_framework_value::ValueRefusalKind::UnsupportedOwner, detail: format!(","semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!(").replace("envelope.binary_token()) }));","envelope.binary_token())));"):line).join('\n'));
} else if (phase === "native-test-refusal-authority") {
  const base=join(os,"🔨️modules/🏪️store");
  for(const path of [join(base,"🧪️tests/🔬️unit/🦀️.rs"),join(os,"🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs")])stage(path,text => text.split('\n').map(line=>line.includes("pack::PackRefusal::Schema { kind:")?line.replace("pack::PackRefusal::Schema { kind: semio_framework_value::ValueRefusalKind::UnsupportedOwner, detail: format!(","semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!(").replace("envelope.binary_token()) }));","envelope.binary_token())));"):line).join('\n'));
  stage(join(base,"📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs"),text => text.replaceAll("PackError::ValueRefusal(error)","pack::PackRefusal::ValueRefusal(error)").replace("let PackError::TextRefusal(text) = error","let pack::PackRefusal::TextRefusal(text) = error"));
  stage(join(base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),text => {
    text=text.replace(".map_err(|error| PackError::Schema(error.to_string()))",".map_err(|error| PackError::from(error.into_value_error()))");
    for(const [message,kind]of [["fixture envelope identity","UnsupportedOwner"],["expected buffers","InvalidValue"],["expected intrinsic octets","InvalidValue"]])text=text.replace('PackError::Schema("'+message+'".into())','PackError::from(ValueError::new(ValueRefusalKind::'+kind+',"'+message+'"))');
    return text;
  });
  stage(join(os,"🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧬️intrinsic-bytes/🦀️.rs"),text => text.replace("Err(crate::store::PackError::LimitExceeded(_))","Err(crate::store::PackError::Refusal(pack::PackRefusal::LimitExceeded { kind: semio_framework_value::ValueRefusalKind::OwnershipLimit, .. }))"));
} else if (phase === "refusal-message-law") {
  for(const owner of [join(core,"🌱️value"),join(os,"🔨️modules/🎒️pack/🌱️value")])stage(join(owner,"🧪️tests/🚦️refusals/🦀️.rs"),text => text.replace("let wrapped=PackRefusal::from(refusal);","let kind=refusal.kind;let message=refusal.message.clone();let wrapped=PackRefusal::from(refusal);").replace("assert!(std::ptr::eq(source,refusal));","assert!(std::ptr::eq(source,refusal));let projected=wrapped.into_value_error();assert_eq!(projected.kind,kind);assert_eq!(projected.message,message);"));
} else if (phase === "intrinsic-occurrences" || phase === "intrinsic-occurrences-current") {
  stage(join(core,"🌱️value/🛫️encode/🦀️.rs"),text => {
    const start=text.indexOf('DslValue::Object(items)=>{output.byte(TAG_MAP,self.control)?;');
    const end=text.indexOf('\n',start);
    if(start<0||end<0)throw Error("Actual intrinsic Object emission missing");
    text=text.slice(0,start)+'DslValue::Object(items)=>{output.byte(TAG_MAP,self.control)?;output.varint(items.len()as u64,self.control)?;if !items.is_empty(){let depth=dynamic_child_depth(depth,self.options.limits.max_depth)?;push(&mut frames,DynamicFrame{items:DynamicItems::Object(items.iter()),depth},self.control)?;}},'+text.slice(end);
    return text.replace(",Sorted(std::vec::IntoIter<(usize,&'a (String,DslValue))>)","").replace(",DynamicItems::Sorted(items)=>items.next().map(|(_,(key,value))|(Some(key.as_str()),value))","");
  });
  stage(join(core,"🌱️value/🦀️.rs"),text => text.replace('            let mut sorted: Vec<&(String, DslValue)> = entries.iter().collect();\n            sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));\n            write_varint_u64(out, sorted.len() as u64);\n            for (k, val) in sorted {','            write_varint_u64(out, entries.len() as u64);\n            for (k, val) in entries {'));
} else if (phase === "os-record-test-refusals") {
  stage(join(os,"🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs"),text => text.replace("Some(PackError::Truncated(1))","Some(PackRefusal::Truncated(1))").replace("Err(PackError::LimitExceeded(_))","Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, .. })").replace("Err(PackError::LimitExceeded(_))","Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::DepthLimit, .. })").replaceAll('PackError::RetainedMalformed { what: "retained-utf8"','PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8"'));
  stage(join(os,"🔨️modules/🎒️pack/🌱️value/🧪️tests/🚦️refusals/🦀️.rs"),text => text.replace("let OutputError::Refusal(refusal)=error","let PackRefusal::ValueRefusal(refusal)=error").replace("let kind=refusal.kind;let message=refusal.message.clone();assert!(matches!(OutputError::Refusal(refusal).into_pack(),PackError::ValueRefusal(error)if error.kind==kind&&error.message==message));","let wrapped=PackRefusal::from(refusal);let PackRefusal::ValueRefusal(refusal)=&wrapped else{panic!(\"typed refusal lost at Pack boundary\")};let source=std::error::Error::source(&wrapped).unwrap().downcast_ref::<ValueError>().unwrap();assert!(std::ptr::eq(source,refusal));"));
} else if (phase === "native-joins") {
  const base = join(os,"🔨️modules/🏪️store");
  stage(join(base,"📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs"),text => text.replace("super::pack_rt::decode_document_controlled", "pack::record::decode_document_controlled").replace("&PackDecodeOptions::default()", "&pack::record::DecodeOptions::default()").replace(".map_err(super::PackError::into_value_error)", ".map_err(pack::PackRefusal::into_value_error)"));
  stage(join(base,"📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs"),text => text.replace("super::PackEncodeOptions::default()", "pack::record::EncodeOptions::default()").replace("crate::os_pack::encode_document_controlled", "pack::record::encode_document_controlled").replace(".map_err(super::PackError::into_value_error)", ".map_err(pack::PackRefusal::into_value_error)"));
  stage(join(base,"📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs"),text => text.replace("crate::os_store::PackEncodeOptions::default()", "pack::record::EncodeOptions::default()").replace("crate::os_pack::encode_document_controlled", "pack::record::encode_document_controlled").replace("crate::os_store::pack_rt::decode_document_controlled", "pack::record::decode_document_controlled").replace("crate::os_store::PackDecodeOptions::default()", "pack::record::DecodeOptions::default()").replaceAll(".map_err(crate::os_store::PackError::into_value_error)", ".map_err(pack::PackRefusal::into_value_error)"));
} else throw Error("Expected demand, provider or native-joins");
mkdirSync(join(ticket,"📥️inputs/canonical-schema-law-owner"),{recursive:true});
writeFileSync(join(ticket,"📥️inputs/canonical-schema-law-owner",phase+"-pairs.json"),JSON.stringify(changes,null,2)+"\n");
for(const change of changes) {
 const current=existsSync(change.path)?read(change.path):null;
 if(current!==change.before)throw Error("Concurrent change before mounting " + change.path);
}
for(const change of changes) {
 if(change.after===null)unlinkSync(change.path);
 else{mkdirSync(dirname(change.path),{recursive:true});writeFileSync(change.path,change.after);}
}
console.log("[DEBUG] Mounted canonical schema law " + phase + " paths=" + changes.length);
