import ts from "typescript";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import assert from "node:assert/strict";
import { Euler, Quaternion, Vector3 } from "three";

const root = process.cwd();
if (process.argv[2] === "draw-image-source") {
 const artifact=resolve(root,"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing"),base=resolve(artifact,"🏅️standards/🔖️1/🪆️subsets/✳️any");
 const source=readFileSync(resolve(artifact,"🦀️.rs"),"utf8"),asset=source.slice(source.indexOf("pub struct DrawingImageAsset"),source.indexOf("pub struct DrawingLayerBase"));
 assert(!/pub mime:|pub data:/.test(asset)&&asset.includes("pub samples:"),"Drawing image semantic asset still retains encoded source");
 for(const file of ["🧬️schema/🦀️.rs","🧬️schema/🎬️scene/🔍️trace/🦀️.rs","🧬️schema/🎬️scene/📷️raster/🦀️.rs","🧬️schema/🎬️scene/👁️view/🦀️.rs"]){const text=readFileSync(resolve(base,file),"utf8");assert(!/decode_png|ImageDecodeJob|base64|data:.*mime/.test(text),`Draw semantic owner still contains physical image admission: ${file}`);}
 for(const file of [resolve(artifact,"🦀️.rs"),...['🧬️schema/🦀️.rs','🧬️schema/🎬️scene/🔍️trace/🦀️.rs','🧬️schema/🎬️scene/📷️raster/🦀️.rs','🧬️schema/🎬️scene/📋️prepare/🦀️.rs','🧬️schema/🎬️scene/👁️view/🦀️.rs','🚪️io/🖼️image/🦀️.rs','🚪️io/🖼️image/🧪️tests/🦀️.rs','🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs','🚪️io/🦀️.rs'].map(file=>resolve(base,file))]){const parsed=Bun.spawnSync(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",file]);assert.equal(parsed.exitCode,0,new TextDecoder().decode(parsed.stderr));}
 const {testDrawingImageAdmission}=await import(resolve(base,"🚪️io/🖼️image/🧪️tests/🟦️.ts"));await testDrawingImageAdmission();
 const strict=Bun.spawnSync([process.execPath,resolve(root,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",...['🚪️io/🖼️image/🟦️.ts','🧬️schema/🎬️scene/📋️prepare/🟦️.ts','🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🟦️.ts'].map(file=>resolve(base,file))]);assert.equal(strict.exitCode,0,new TextDecoder().decode(strict.stdout)+new TextDecoder().decode(strict.stderr));console.log("[DEBUG] Draw image/scene/SVG production strict TypeScript graph accepted");process.exit(0);
}

if (process.argv[2] === "config-ui-source") {
  const base=resolve(root,"🧰️framework/🛍️products/💻️os/🎚️config");
  const source=readFileSync(resolve(base,"🧬️schema/🦀️.rs"),"utf8");
  assert(!/pub type JsonValue|json_value_bridge/.test(source),"Config UI domain still exports an external JSON value bridge");
  for(const name of ["UiDriver","UiTheme","UserNamedLayout","UiPreferences"]){
    const record=source.match(new RegExp("#\\[derive\\([^\\]]+\\)\\]\\s*(?:#\\[[^\\]]+\\]\\s*)*pub struct "+name+"\\s*\\{[^}]+\\}"))?.[0];
    assert(record,`missing config domain record ${name}`);
    assert(!/serde|schemars|with\s*=/.test(record),`${name} owns physical codec metadata`);
    assert(/ToValue, FromValue/.test(record),`${name} must own first-party admission`);
  }
  assert((source.match(/pub (?:config|layout): DslValue/g)||[]).length===3,"All customization payloads must own semantic values directly");
  for(const path of [resolve(base,"🧬️schema/🦀️.rs"),resolve(base,"🚪️io/📝️text/🎨️ui-preferences/🦀️.rs"),resolve(base,"🧬️schema/🎨️ui-preferences/🧪️tests/🌱️owned-customization/🦀️.rs"),resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")]){
    const result=Bun.spawnSync(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",path]);assert.equal(result.exitCode,0,new TextDecoder().decode(result.stderr));
  }
  const fixture=JSON.parse(readFileSync(resolve(base,"🧬️schema/🎨️ui-preferences/🧫️fixtures/🌱️owned-customization/🔣️.json"),"utf8"));
  const {default:Ajv}=await import("ajv");
  const schema=JSON.parse(readFileSync(resolve(base,"🧬️schema/🎨️ui-preferences/🔣️.json"),"utf8"));
  const ajv=new Ajv({strict:false}).addSchema(schema);
  for(const sample of fixture.samples)for(const [name,key,id] of [["UiDriver","driverId",fixture.driverId],["UiTheme","themeId",fixture.themeId]])assert(ajv.getSchema(`${schema.$id}#/$defs/${name}`)!({[key]:id,label:fixture.label,config:sample}));
  console.log(`[DEBUG] Config UI owns first-party semantic payloads; independent AJV customization samples=${fixture.samples.length}`);process.exit(0);
}

if (process.argv[2] === "puzzle5d-source") {
  const base=resolve(root,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any");
  const source=readFileSync(resolve(base,"✏️editor/🦀️.rs"),"utf8");
  const runtime=source.slice(0,source.indexOf("#[cfg(test)]\n"));
  assert(!/serde_json::|puzzle5d_document_delta_operations/.test(runtime),"Puzzle5d host still depends on the external JSON bridge");
  assert(source.includes("fn puzzle5d_snapshot_from_document")&&source.includes("fn puzzle5d_operations_between_snapshots"),"Puzzle5d native admission and granular typed intent derivation are required");
  const admission=source.slice(source.indexOf("pub fn puzzle5d_snapshot_from_document"),source.indexOf("pub fn puzzle5d_document_from_snapshot"));
  assert(!/unwrap_or_else\(\|_\|/.test(admission),"Puzzle5d host admission silently substitutes failed decoding");
  assert(source.includes("puzzle5d_snapshot_mutations(before, after)"),"Puzzle5d host must derive canonical sparse typed intent directly");
  assert(source.includes("fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Result<Vec<Puzzle5dMutation>"),"Puzzle5d malformed document admission must preserve typed refusal");
  const files: string[] = [];
  const visit = (directory: string): void => {
    for (const item of readdirSync(directory, {withFileTypes:true})) {
      const path = resolve(directory,item.name);
      if (item.isDirectory() && !item.name.includes("tests")) visit(path);
      else if (item.isFile() && item.name.endsWith(".rs")) files.push(path);
    }
  };
  visit(resolve(base,"✏️editor")); visit(resolve(base,"👁️viewer"));
  for (const path of files) {
    const native = readFileSync(path,"utf8").replace(/\/\/[^\n]*/g, "");
    assert(!/serde_json::/.test(native),`Puzzle5d host/viewer exports external JSON at ${path}`);
    const syntax = Bun.spawnSync(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",path]);
    assert(syntax.exitCode === 0, new TextDecoder().decode(syntax.stderr));
  }
  console.log(`[DEBUG] Puzzle5d host first-party projection, typed admission, granular intent and Rust syntax owners=${files.length}`);process.exit(0);
}

if (process.argv[2] === "pdf-source") {
  const base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base");
  const source=readFileSync(resolve(base,"🧬️schema/📸️snapshot/🦀️.rs"),"utf8");
  const font=source.slice(source.indexOf("pub enum PdfFontProgram"),source.indexOf("impl PdfFontProgram"));
  const image=source.slice(source.indexOf("pub struct PdfImage {"),source.indexOf("impl PdfImage"));
  assert(!/Vec<u8>|length[123]:|PdfImageCodec|pub data:|row_bytes|pub fn jpeg/.test(font+image),"PDF semantic image/font body still contains native codec representation");
  assert(source.includes("pub enum PdfImageBody")&&font.includes("ArtifactRef"),"PDF logical samples and typed foreign artifacts must be explicit");
  console.log("[DEBUG] PDF image/font body ownership admits logical samples or typed foreign artifacts only");process.exit(0);
}

if (process.argv[2] === "flow-source") {
  const base = resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained");
  const {flowSelectedCopySelfTests}=await import(resolve(base,"📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts"));
  const {flowTypedRetirementSelfTests}=await import(resolve(base,"🧪️tests/🔬️flow-typed-retirement/🟦️.ts"));
  console.log(`[DEBUG] Flow neutral selected-copy stable oracle and ownership mutants=${flowSelectedCopySelfTests()}; typed frontier law=${flowTypedRetirementSelfTests()}`);
  process.exit(0);
}

const cad = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any";
const lowpoly = "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any";
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const semantic = read(`${cad}/🧬️schema/💡️inferences/🟦️.ts`);
if (process.argv[2] !== "geometry-only") assert(!/chevrotain|JSON\.parse|function parseConstruct|defaultConstructRunner/.test(semantic), "Construct semantic owner still admits physical text");
if (process.argv[2] !== "geometry-only") assert(/runConstructAst\(ast: ConstructAst/.test(semantic), "Construct execution must consume the typed AST");
const io = read(`${lowpoly}/🚪️io/🦀️.rs`);
assert(!/fn (?:euler_degrees_to_quaternion|rotate|apply_transform|triangle_normal)\(/.test(io), "Lowpoly IO still defines geometry algebra");
const { applyTransform, eulerDegreesToQuaternion, triangleNormal } = await import(resolve(root, `${lowpoly}/🧬️schema/💡️inferences/🟦️.ts`));
const { binary32 } = await import(resolve(root, "🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts"));
const fixtures = JSON.parse(read(`${lowpoly}/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🧫️fixtures/🔄️transform.json`));
for (const row of fixtures.cases) {
  const radians = row.transform.rotation.map((angle: number) => angle * Math.PI / 180);
  const quaternion = new Quaternion().setFromEuler(new Euler(...radians as [number, number, number], "XYZ"));
  const scale = row.local.map((v: number, i: number) => Math.fround(Math.fround(v) * Math.fround(row.transform.scale[i])));
  const world = new Vector3(...scale as [number, number, number]).applyQuaternion(quaternion).add(new Vector3(...row.transform.position as [number, number, number]));
  const owned = { position: row.transform.position.map(binary32), rotation: row.transform.rotation.map(binary32), scale: row.transform.scale.map(binary32) };
  const ownQuaternion = eulerDegreesToQuaternion(owned.rotation), ownWorld = applyTransform(owned, row.local);
  ownQuaternion.forEach((v: number, i: number) => assert(Math.abs(v - quaternion.toArray()[i]!) < 1e-12));
  ownWorld.forEach((v: number, i: number) => assert(Math.abs(v - world.toArray()[i]!) < 1e-10));
  quaternion.toArray().forEach((v: number, i: number) => assert(Math.abs(v - row.quaternion[i]) < 1e-12));
  world.toArray().forEach((v: number, i: number) => assert(Math.abs(v - row.world[i]) < 1e-10));
}
assert.deepEqual(triangleNormal([0, 0, 0], [1, 0, 0], [0, 1, 0]), [0, 0, 1]);
assert.deepEqual(triangleNormal([0, 0, 0], [0, 0, 0], [0, 0, 0]), [0, 0, 0]);
console.log(`[DEBUG] Construct physical ownership and Lowpoly geometry independent Three oracle cases=${fixtures.cases.length}`);

const leaves = [`${cad}/🧬️schema/💡️inferences/🟦️.ts`, `${cad}/🚪️io/📝️text/💡️inferences/🟦️.ts`, `${lowpoly}/🧬️schema/💡️inferences/🟦️.ts`].map(path => resolve(root, path));
const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, strict: true, esModuleInterop: true, resolveJsonModule: true, allowImportingTsExtensions: true, skipLibCheck: true, noEmit: true, baseUrl: root, paths: { "@semio-tech/framework-3d-js": ["🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/🟦️.ts"] } };
const program = ts.createProgram(leaves, options);
const diagnostics = ts.getPreEmitDiagnostics(program);
const ownDiagnostics = diagnostics.filter(row => row.category === ts.DiagnosticCategory.Error && (!row.file || leaves.includes(resolve(row.file.fileName))));
if (ownDiagnostics.length) throw new Error(ts.formatDiagnosticsWithColorAndContext(ownDiagnostics, { getCanonicalFileName: path => path, getCurrentDirectory: () => root, getNewLine: () => "\n" }));
console.log(`[DEBUG] Construct/Lowpoly selected semantic and codec owners strict TypeScript leaves=${leaves.length}; dependency diagnostics=${diagnostics.length}; full dependency graph is not certified`);

const gltf = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations";
const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
const domains = taxonomy.mutationDomainOwners[gltf] as Record<string, Record<string, string>>;
const identities = Object.entries(domains).flatMap(([domain, operations]) => Object.entries(operations).map(([operation, semantic]) => ({ owner: `${gltf}/${domain}/${operation}`, semantic })));
const physical = JSON.parse(read(`${gltf}/🔣️.json`)).oneOf.map((row: { properties: { mutation: { const: string } } }) => row.properties.mutation.const).sort();
const camel = (value: string) => value.replace(/-([a-z])/g, (_, letter: string) => letter.toUpperCase());
assert.deepEqual(identities.map(row => camel(row.semantic)).sort(), physical, "glTF registered domains must exactly match the current mutation schema");
const native = read(`${gltf}/🦀️.rs`).split("pub enum GltfMutation {")[1]!.split("\n}")[0]!;
const nativeIdentities = [...native.matchAll(/^    ([A-Z]\w*)\(/gm)].map(row => row[1]![0]!.toLowerCase() + row[1]!.slice(1)).sort();
assert.deepEqual(nativeIdentities, physical, "glTF native variants must exactly match schema variants");
const authority = JSON.parse(read("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json"));
assert.deepEqual(authority.mutationDomainOwners[gltf], domains, "glTF compiler authority must match the authored vocabulary");
for (const row of identities) for (const name of [authority.sourceFilename, authority.descriptorFilename]) assert(existsSync(resolve(root, row.owner, name)), `${row.owner}/${name} must be a real operation owner`);
console.log(`[DEBUG] glTF exact registered/schema/native/physical operation census=${identities.length}`);
