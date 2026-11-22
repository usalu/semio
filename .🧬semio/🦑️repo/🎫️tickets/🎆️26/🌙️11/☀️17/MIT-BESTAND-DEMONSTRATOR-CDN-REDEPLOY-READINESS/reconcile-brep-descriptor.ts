import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { authoredReleaseDigestV1, workspace } from "./authored-release-descriptors.ts";
import { decodePackValue, encodePackValue, packByteCompare, packValueToExactJson } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
import { normalizeCatalogDescriptorEnums } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🛡️guards/🟦️.ts";
import { SHIPPED_COMPONENT_PROFILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { componentStageTargetsV1, settleComponentDescriptorV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";

const target = componentStageTargetsV1(workspace).find(owner => owner.pluginId === "flow-extension-brep");
assert.ok(target);
const sources = new Map<string, string>();
function source(path: string, facts: readonly RegExp[]): string {
  const content = readFileSync(path, "utf8");
  for (const fact of facts) assert.match(content, fact);
  sources.set(path, content);
  return content;
}
const root = join(target.ownerRoot, "..");
const os = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules");
const neural = join(os, "🧠️neural/⚙️engine");
const defeatureSource = source(join(root, "🧾️register-operators/🧮️booleans-transforms-and-solid-features/🦀️.rs"), [/"brep\.solid\.defeature"[\s\S]*?&q\("defeature", "[^"]+"\)/u]);
source(join(workspace, "🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🔖️contract/🦀️.rs"), [/\("defeature", OpQuality::ExactAnalytic\)/u]);
const defeatureSummary = defeatureSource.match(/&q\("defeature", "([^"]+)"\)/u)?.[1];
assert.ok(defeatureSummary);
source(join(root, "🦀️.rs"), [
  /extension_manifest_json\(\)/u,
  /format!\("\{summary\} \[quality:\{:\?\}\]", operation_quality\(method\)\)/u,
  /\("brep\.solid\.defeature", "defeature"\)/u,
  /FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json/u,
  /PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json/u,
]);
source(join(root, "🧾️register-operators/🦀️.rs"), [/mesh::register_mesh\(registry, session\)/u, /build_manifest_json\("brep", "Brep", env!\("CARGO_PKG_VERSION"\), &registry, vec!\["onStartup".into\(\)\]/u]);
source(join(os, "🌊️flow/🧩️extensions/🧬️schema/🦀️.rs"), [/schemas: registry.schema_catalogue\(\), operators: registry.operator_catalogue\(\)/u]);
source(join(neural, "📔️registry/🦀️.rs"), [/fn schema_catalogue[\s\S]*?items\.sort_by\(\|a, b\| a\.id\.cmp\(&b\.id\)\)/u, /fn operator_catalogue[\s\S]*?items\.sort_by\(\|a, b\| a\.id\.cmp\(&b\.id\)\)/u, /fn finalize_operator_info[\s\S]*?operators.sort\(\)/u]);
source(join(neural, "🧱️model/⚙️operator-record/🦀️.rs"), [/let \(code, abbreviation, full_name\) = derive_channel_names\(&name\)/u, /\.take\(2\)\.collect::<String>\(\)\.to_uppercase\(\)/u, /\.take\(3\)\.collect\(\)/u, /Atom::Decimal\(default\)/u, /Atom::Boolean\(default\)/u, /Atom::String\(default\.into\(\)\)/u]);
source(join(neural, "🧱️model/📚️dictionary/🦀️.rs"), [/Atom::Decimal\(d\) => DslValue::Number\(Number::Float\(\*d\)\)/u]);
source(join(workspace, "🧰️framework/🔨️modules/🎒️pack/🔤️json/🎯️float-writer/🦀️.rs"), [/write_decimal_digits\(negative, &digits, exponent, -5\.\.=15, out\)/u, /out.write_str\("\.0"\)/u]);
source(join(workspace, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧰️operator-registration/🦀️.rs"), [
  /ChannelSpec::number_default\(id, default, &\[operator_id\]\)/u, /extension: "brep"\.into\(\)/u,
  /inputs: distinct_channels\(inputs\),\s*outputs: distinct_channels\(outputs\)/u,
]);
const registration = source(join(root, "🥽️mesh/📇️registration/🦀️.rs"), [
  /"fromBrep" => "TessellatedMesh", "toBrep" => "MeshDerivedBRep", _ => "PolygonMesh"/u,
  /if !matches!\(operation, [^\n]*"importObj" \| "importStl" \| "importGlb"[^\n]*"torus" \| "uvSphere"[^\n]*requires\("mesh", &\[&id\]\)\.with_value_types\(&\["mesh"\]\)/u,
  /"combine"\|"intersect"\|"difference"\|"union"[^\n]*requires\("other", &\[&id\]\)\.with_value_types\(&\["mesh"\]\)/u,
  /"importStl" \| "importGlb"[^\n]*requires\("data", &\[&id\]\)\.with_value_types\(&\["text"\]\)/u,
  /\[\("setShading", "faces", "smooth"\), \("markSeams", "edges", "seam"\)\]/u,
  /text_default\(selection, "\[0\]", &\[&id\]\)/u, /boolean_default\(flag, true, &\[&id\]\)/u,
  /inputs.extend\(parameters.iter\(\).map\(\|\(key, value\)\| number_channel\(key, &id, \*value\)\)\)/u,
  /_ => \(vec!\[ChannelSpec::named\("M", "Mesh", "meshOut", "PolygonMesh"\)\.with_value_types\(&\["mesh"\]\)\], vec!\["mesh"\]\)/u,
  /operator_info_with_outputs\(&id, name, name, "emoji:🥽️", &format!\("\{summary\} \[quality:\{\}\]", operation_quality\(operation\)\), inputs, outputs, &\[group\]\)/u,
]);
const definitions = new Map([...registration.matchAll(/^\s*\("([A-Za-z]+)", "([^"]+)", "([^"]+)", &\[(.*)\]\),$/gmu)].map(match => [match[1]!, {name: match[2]!, group: match[3]!, params: [...match[4]!.matchAll(/\("([A-Za-z]+)", (-?\d+(?:\.\d+)?)\)/gu)].map(parameter => [parameter[1]!, Number(parameter[2]!)] as const)}]));
const summaries = new Map([...registration.matchAll(/^\s*"([A-Za-z]+)" => "([^"]+)",$/gmu)].map(match => [match[1]!, match[2]!]));
function channel(names: readonly [string, string, string, string], operators: string[], valueType: string, value: number | string | boolean | null = null) {
  const [code, abbreviation, name, fullName] = names;
  return {code, abbreviation, name, fullName, operators, valueTypes: [valueType], default: value === null ? null : {$schema: valueType, value}, label: null, cardinality: "!"};
}
function input(name: string, id: string, valueType: string, value?: number | string | boolean) {
  return channel([name.slice(0, 2).toUpperCase(), name.slice(0, 3), name, name[0]!.toUpperCase() + name.slice(1)], [id], valueType, value);
}
const changes = ["combine", "exportGlb", "exportStl", "importGlb", "importObj", "importStl", "markSeams", "recomputeNormals", "setShading", "torus", "unwrapUv", "uvSphere"];
const sourceOperators = new Map(changes.map(operation => {
  const registrationRow = definitions.get(operation);
  const summary = summaries.get(operation);
  assert.ok(registrationRow && summary);
  const {name, group, params} = registrationRow;
  const id = "brep.mesh." + operation;
  const inputs = /^(import|torus|uvSphere)/u.test(operation) ? [] : [input("mesh", id, "mesh")];
  if (operation === "combine") inputs.push(input("other", id, "mesh"));
  if (operation === "setShading") inputs.push(input("faces", id, "text", "[0]"), input("smooth", id, "boolean", true));
  if (operation === "markSeams") inputs.push(input("edges", id, "text", "[0]"), input("seam", id, "boolean", true));
  if (operation.startsWith("import")) {
    const obj = registration.match(/operation == "importObj" \{ inputs\.push\(ChannelSpec::text_default\("data", ("[^"]+"), &\[&id\]\)\); \}/u);
    assert.ok(obj);
    inputs.push(input("data", id, "text", operation === "importObj" ? JSON.parse(obj[1]!) : undefined));
  }
  for (const [parameter, value] of params) inputs.push(input(parameter, id, "number", value));
  const output = operation.startsWith("export") ? registration.match(new RegExp(`"${operation}" => \\(vec!\\[ChannelSpec::named\\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\\)\\.with_value_types\\(&\\["text"\\]\\)\\], vec!\\["text"\\]\\)`, "u")) : null;
  if (operation.startsWith("export")) assert.ok(output);
  const outputs = output ? [channel([output[1]!, output[2]!, output[3]!, output[4]!], ["brep.brep", "brep.text"], "text")] : [channel(["M", "Mesh", "meshOut", "PolygonMesh"], ["brep.mesh"], "mesh")];
  return [id, {id, extension: "brep", name, abbreviation: name, icon: "emoji:🥽️", summary: summary + " [quality:PolygonMesh]", inputs, outputs, variadicInput: null, variadicOutput: null, group: [group]}] as const;
}));
type Manifest = {contributes: {schemas: {id: string}[]; operators: {id: string; summary: string}[]}};
type Descriptor = Record<"manifest" | "contributions", {topicContributions: {payload: {manifestJson: string}}[]}>;
const texts = (descriptor: Descriptor) => [descriptor.contributions, descriptor.manifest].flatMap(group => group.topicContributions.map(topic => topic.payload));
const readPair = () => ({json: readFileSync(join(target.ownerRoot, "🔣️.json")), pack: readFileSync(join(target.ownerRoot, "🛂️.descriptor.semio"))});
const {json, pack} = readPair();
const packed = decodePackValue(pack);
assert.deepEqual(Buffer.from(encodePackValue(packed)), pack);
const descriptor = JSON.parse(json.toString("utf8")) as Descriptor;
const packDescriptor = packed as unknown as Descriptor;
const authored = texts(descriptor), old = texts(packDescriptor);
const [candidate] = authored.map(payload => payload.manifestJson);
const [previous] = old.map(payload => payload.manifestJson);
assert.ok(candidate && previous);
assert.deepEqual(authored.map(payload => payload.manifestJson), Array(4).fill(candidate));
assert.deepEqual(old.map(payload => payload.manifestJson), Array(4).fill(previous));
const sha = (value: Uint8Array) => createHash("sha256").update(value).digest("hex");
if (candidate === previous) {
  console.log(JSON.stringify({plugin: target.pluginId, semanticSha256: authoredReleaseDigestV1(target, json, pack), jsonSha256: sha(json), changed: false}));
  process.exit(0);
}
const newManifest = JSON.parse(candidate) as Manifest, oldManifest = JSON.parse(previous) as Manifest;
const orient = oldManifest.contributes.operators.find(row => row.id === "brep.mesh.orient");
const orientSummary = summaries.get("orient");
assert.ok(orient && orientSummary);
const replacements = new Map<string, Manifest["contributes"]["operators"][number]>(sourceOperators);
replacements.set(orient.id, {...orient, summary: orientSummary + " [quality:PolygonMesh]"});
const defeature = oldManifest.contributes.operators.find(row => row.id === "brep.solid.defeature");
assert.ok(defeature);
replacements.set(defeature.id, {...defeature, summary: defeatureSummary + " [quality:ExactAnalytic]"});
const byId = <T extends {id: string}>(rows: T[]) => rows.toSorted((left, right) => packByteCompare(left.id, right.id));
const project = {...oldManifest, contributes: {...oldManifest.contributes, schemas: byId(oldManifest.contributes.schemas), operators: byId([...oldManifest.contributes.operators.filter(row => !replacements.has(row.id)), ...replacements.values()])}};
assert.deepStrictEqual(project, {...newManifest, contributes: {...newManifest.contributes, schemas: byId(newManifest.contributes.schemas), operators: byId(newManifest.contributes.operators)}}, "Every embedded manifest value outside the source-derived updates must agree");
const rendered = JSON.stringify(project);
const canonical = rendered.replace(/("\$schema":"number","value":)(-?(?:0|[1-9]\d*))(?=[,}])/gu, "$1$2.0");
assert.deepStrictEqual(JSON.parse(canonical), project);
for (const topic of [...authored, ...old]) topic.manifestJson = canonical;
assert.deepStrictEqual(normalizeCatalogDescriptorEnums(descriptor), normalizeCatalogDescriptorEnums(packValueToExactJson(packed)), "Every outer Pack and JSON descriptor field must agree");
let count = 0;
const jsonBytes = Buffer.from(json.toString("utf8").replace(/("manifestJson"\s*:\s*)("(?:[^"\\]|\\.)*")/gu, (text, prefix, original) => {
  assert.equal(JSON.parse(original), candidate);
  count++;
  return prefix + JSON.stringify(canonical);
}));
assert.equal(count, 4);
assert.deepStrictEqual(JSON.parse(jsonBytes.toString("utf8")), descriptor);
const packBytes = Buffer.from(encodePackValue(packed));
const semanticSha256 = authoredReleaseDigestV1(target, jsonBytes, packBytes);
if (process.argv.includes("--write")) {
  assert.deepEqual(readPair(), {json, pack});
  for (const [path, content] of sources) assert.equal(readFileSync(path, "utf8"), content, path);
  const output = {profile: SHIPPED_COMPONENT_PROFILE, directory: target.ownerRoot};
  settleComponentDescriptorV1(workspace, target, output, {jsonBytes, packBytes}, "write");
  assert.deepEqual(settleComponentDescriptorV1(workspace, target, output, {jsonBytes, packBytes}, "verify"), []);
  assert.deepEqual(readPair(), {json: jsonBytes, pack: packBytes});
}
console.log(JSON.stringify({plugin: target.pluginId, operators: changes, summary: "orient", previousPackSha256: sha(pack), previousJsonSha256: sha(json), semanticSha256, jsonSha256: sha(jsonBytes), changed: true}));
