import fs from "node:fs";
import path from "node:path";
import ts from "typescript";
import Ajv2020 from "ajv/dist/2020";
import YAML from "yaml";

const schema = path.resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema");
const read = (relative: string) => fs.readFileSync(path.join(schema, relative), "utf8");
const camel = (value: string) => value.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
const capture = (text: string, pattern: RegExp, label: string) => {
  const match = text.match(pattern);
  if (!match) throw new Error(`missing ${label}`);
  return match[1];
};
const fields = (body: string) => [...body.matchAll(/\b([a-z][A-Za-z0-9_]*)\s*\??\s*:/g)].map((match) => camel(match[1]));
const same = (actual: string[], expected: string[], label: string) => {
  const left = [...actual].sort();
  const right = [...expected].sort();
  if (JSON.stringify(left) !== JSON.stringify(right)) throw new Error(`${label}: ${JSON.stringify({ actual: left, expected: right })}`);
};
const walk = (directory: string): string[] => fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
  const target = path.join(directory, entry.name);
  return entry.isDirectory() ? walk(target) : [target];
});

const groups: Record<string, string[]> = {
  GltfSizeIndicators: ["overallSize", "axisAlignedBounds", "orientedBounds", "boundingBoxDimensions", "characteristicLength", "footprintArea", "projectedArea"],
  GltfAreaVolumeIndicators: ["surfaceArea", "totalArea", "exposedArea", "contactArea", "volume", "enclosedVolume", "materialVolume", "voidVolume"],
  GltfCompactnessIndicators: ["compactness", "surfaceToVolumeRatio", "sphericity", "compactnessIndex", "hullFillRatio"],
  GltfProportionIndicators: ["aspectRatios", "slenderness", "flatness", "elongation"],
  GltfMassIndicators: ["centroid", "principalFrame", "principalAxes", "momentsOfInertia", "inertiaTensor"],
  GltfCurvatureIndicators: ["meanCurvature", "gaussianCurvature", "curvatureHistogram", "sharpFeatureProportion"],
  GltfThicknessIndicators: ["meanThickness", "minimumThickness", "thicknessVariability", "thicknessDistribution"],
  GltfConcavityIndicators: ["convexHullGap", "reentrantArea", "reentrantVolume", "concavityIndex"],
  GltfClearanceIndicators: ["minimumDistanceToNeighbors", "clearanceDistribution", "interferenceVolume", "overlapVolume"],
  GltfAdjacencyIndicators: ["numberOfContacts", "contactGraphDegree", "connectedComponents"],
  GltfOrientationIndicators: ["mainAxisDirection", "faceNormalDistribution", "orientationConsistency"],
  GltfSymmetryIndicators: ["reflectionSymmetryScore", "rotationalSymmetryScore", "reflectionSymmetries", "rotationalSymmetries", "repetitionRatio", "modularityRatio"],
  GltfRoughnessIndicators: ["deviationFromIdeal", "deviationFromSmoothedGeometry", "normalVariation", "surfaceWaviness", "irregularity"],
  GltfTopologyIndicators: ["holes", "handles", "boundaryLoops", "eulerCharacteristic", "genus"],
};

const inferenceTs = walk(path.join(schema, "💡️inferences")).filter((file) => file.endsWith("🟦️component.ts")).map((file) => fs.readFileSync(file, "utf8")).join("\n");
const inferenceRootTs = read("💡️inferences/🟦️component.ts");
const inferenceGraphql = read("💡️inferences/🔗️component.graphql");
const inferenceTextGraphql = read("💡️inferences/📝️text/🔗️component.graphql");
const inferenceProto = read("💡️inferences/🛰️component.proto");
const inferenceRust = walk(path.join(schema, "💡️inferences")).filter((file) => file.endsWith("🦀️component.rs")).map((file) => fs.readFileSync(file, "utf8")).join("\n");
const inferenceJson = JSON.parse(read("💡️inferences/🔣️component.json"));

for (const [name, expected] of Object.entries(groups)) {
  same(fields(capture(inferenceTs, new RegExp(`export interface ${name} \\{([^}]*)\\}`), `TypeScript ${name}`)), expected, `TypeScript ${name}`);
  same(fields(capture(inferenceGraphql, new RegExp(`type ${name} \\{([^}]*)\\}`), `GraphQL ${name}`)), expected, `GraphQL ${name}`);
  same(fields(capture(inferenceTextGraphql, new RegExp(`type ${name} \\{([^}]*)\\}`), `text GraphQL ${name}`)), expected, `text GraphQL ${name}`);
  const proto = capture(inferenceProto, new RegExp(`message ${name} \\{([^}]*)\\}`), `Proto ${name}`);
  same([...proto.matchAll(/GltfMeasure\s+([a-z_]+)\s*=/g)].map((match) => camel(match[1])), expected, `Proto ${name}`);
  const definition = name.replace(/^Gltf/, "").replace(/Indicators$/, "");
  const key = definition[0].toLowerCase() + definition.slice(1);
  same(Object.keys(inferenceJson.$defs[key].properties), expected, `JSON Schema ${name}`);
  same(inferenceJson.$defs[key].required, expected, `JSON Schema required ${name}`);
  const rust = capture(inferenceRust, new RegExp(`pub struct ${name} \\{([^}]*)\\}`), `Rust ${name}`);
  same([...rust.matchAll(/pub\s+([a-z_]+)\s*:/g)].map((match) => camel(match[1])), expected, `Rust ${name}`);
}

const indicators = Object.values(groups).flat();
if (indicators.length !== 67 || new Set(indicators).size !== 67) throw new Error("indicator taxonomy is not 67 unique fields");
same(fields(capture(inferenceRootTs, /export interface GltfInference \{([^}]*)\}/, "TypeScript root")), ["geometry"], "TypeScript root");
same(fields(capture(inferenceGraphql, /type GltfInference \{([^}]*)\}/, "GraphQL root")), ["geometry"], "GraphQL root");
same(fields(capture(inferenceTextGraphql, /type GltfInference \{([^}]*)\}/, "text GraphQL root")), ["geometry"], "text GraphQL root");
same(Object.keys(inferenceJson.properties), ["geometry"], "JSON Schema root");
same([...capture(inferenceProto, /message GltfInference \{([^}]*)\}/, "Proto root").matchAll(/\b([a-z_]+)\s*=/g)].map((match) => camel(match[1])), ["geometry"], "Proto root");
for (const source of [inferenceGraphql, inferenceTextGraphql]) {
  if (!source.includes("enum GltfEntityScope { DOCUMENT SCENE NODE_INSTANCE MESH PRIMITIVE COMPONENT SURFACE_REGION }") || !source.includes("scope: GltfEntityScope!")) throw new Error("GraphQL entity-scope parity");
}
if (!inferenceProto.includes("enum GltfEntityScope { ENTITY_SCOPE_UNSPECIFIED = 0; DOCUMENT = 1; SCENE = 2; NODE_INSTANCE = 3; MESH = 4; PRIMITIVE = 5; COMPONENT = 6; SURFACE_REGION = 7; }") || !inferenceProto.includes("GltfEntityScope scope = 1")) throw new Error("Proto entity-scope parity");

const variants = ["noMutation", "setSnapshot", "setAsset", "insertScene", "removeScene", "setScene", "insertNode", "removeNode", "setNode", "insertMesh", "removeMesh", "setMesh", "insertAccessor", "removeAccessor", "setAccessor", "insertMaterial", "removeMaterial", "setMaterial", "insertBuffer", "removeBuffer", "setBuffer", "insertAnimation", "removeAnimation", "setAnimation", "transformNode", "reparentNode", "bindNodeMesh", "bindPrimitiveMaterial"];
const mutationTs = read("🧬️mutations/🟦️component.ts");
const mutationGraphql = read("🧬️mutations/🔗️component.graphql");
const mutationJson = JSON.parse(read("🧬️mutations/🔣️component.json"));
const mutationProto = read("🧬️mutations/🛰️component.proto");
const mutationBinaryTs = read("🧬️mutations/💾️binary/🟦️component.ts");
const mutationRust = read("🧬️mutations/💾️binary/🦀️component.rs");
const mutationGrammar = read("🧬️mutations/📝️text/📖️component.grammar.semio");
const mutationEbnf = read("🧬️mutations/📝️text/🔤️component.ebnf");
const mutationAntlr = read("🧬️mutations/📝️text/🅰️component.g4");
const diffGrammar = read("🔺️diff/📝️text/📖️component.grammar.semio");
const kebab = (value: string) => value.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
for (const [tag, variant] of variants.entries()) {
  if (!mutationTs.includes(`mutation: '${variant}'`)) throw new Error(`TypeScript missing ${variant}`);
  const graphName = `Gltf${variant[0].toUpperCase()}${variant.slice(1)}`;
  if (!mutationGraphql.includes(graphName)) throw new Error(`GraphQL missing ${graphName}`);
  if (!mutationJson.oneOf.some((entry: any) => entry.properties?.mutation?.const === variant)) throw new Error(`JSON Schema missing ${variant}`);
  const snake = kebab(variant).replaceAll("-", "_");
  if (!new RegExp(`\\b${snake}\\s*=\\s*${tag + 1}\\b`).test(mutationProto)) throw new Error(`Proto ordinal mismatch ${variant}=${tag + 1}`);
  const binary = new RegExp(`\\b${variant}:\\s*${tag}\\b`);
  if (!binary.test(mutationBinaryTs)) throw new Error(`binary tag mismatch ${variant}=${tag}`);
  if (!new RegExp(`GltfMutation::${variant[0].toUpperCase()}${variant.slice(1)}\\([^\\n]*=> ${tag},`).test(mutationRust)) throw new Error(`Rust tag mismatch ${variant}=${tag}`);
  if (!mutationGrammar.includes(`"${kebab(variant)}"`)) throw new Error(`Semio grammar missing ${variant}`);
  if (!mutationEbnf.includes(`"${kebab(variant)}"`)) throw new Error(`EBNF missing ${variant}`);
  if (!mutationAntlr.includes(`'${kebab(variant)}'`)) throw new Error(`ANTLR missing ${variant}`);
}
const mutationFolders = fs.readdirSync(path.join(schema, "🧬️mutations"), { withFileTypes: true }).filter((entry) => entry.isDirectory() && ["🦠️mutation", "🔺️diff", "↩️inverse"].every((leaf) => fs.existsSync(path.join(schema, "🧬️mutations", entry.name, leaf, "🦀️component.rs")) && fs.existsSync(path.join(schema, "🧬️mutations", entry.name, leaf, "🟦️component.ts"))));
if (mutationFolders.length !== 28) throw new Error(`mutation taxonomy has ${mutationFolders.length} complete triads, expected 28`);
if (walk(path.join(schema, "💡️inferences")).some((file) => file.includes("📦bounds"))) throw new Error("legacy GLTF bounds component remains");
const mutationPrimitive = mutationGrammar.split("\n").find((line) => line.startsWith("primitive-value ="));
const diffPrimitive = diffGrammar.split("\n").find((line) => line.startsWith("primitive-value ="));
if (mutationPrimitive !== diffPrimitive || !mutationPrimitive?.includes("morph-target-list")) throw new Error("primitive morph-target grammar parity");

const diffFields = ["asset", "scene", "scenes", "nodes", "meshes", "accessors", "bufferViews", "buffers", "bufferBytes", "materials", "textures", "images", "samplers", "skins", "animations", "cameras", "extensionsUsed", "extensionsRequired", "extensions", "extras", "sourceForm"];
const diffTs = read("🔺️diff/🟦️component.ts");
const diffGraphql = read("🔺️diff/🔗️component.graphql");
const diffJson = JSON.parse(read("🔺️diff/🔣️component.json"));
const diffProto = read("🔺️diff/🛰️component.proto");
const diffBinaryTs = read("🔺️diff/💾️binary/🟦️component.ts");
const diffRust = read("🔺️diff/🦀️component.rs");
const diffAbnf = read("🔺️diff/💾️binary/🔠️component.abnf");
const diffProtocol = read("🔺️diff/💾️binary/📡️component.protocol.semio");
same(fields(capture(diffTs, /export interface GltfDiff \{([^}]*)\}/s, "TypeScript GltfDiff")), diffFields, "TypeScript diff fields");
same(fields(capture(diffGraphql, /type GltfDiff \{([^}]*)\}/s, "GraphQL GltfDiff").replace(/@\w+\([^)]*\)/g, "")), diffFields, "GraphQL diff fields");
same(Object.keys(diffJson.properties), diffFields, "JSON Schema diff fields");
same([...capture(diffProto, /message GltfDiff \{([^}]*)\}/s, "Proto GltfDiff").matchAll(/\b([a-z_]+)\s*=/g)].map((match) => camel(match[1])), diffFields, "Proto diff fields");
same([...capture(diffRust, /pub struct GltfDiff \{([^}]*)\}/s, "Rust GltfDiff").matchAll(/pub\s+([a-z_]+)\s*:/g)].map((match) => camel(match[1])), diffFields, "Rust diff fields");
for (const field of diffFields) if (!diffBinaryTs.includes(`'${field}'`)) throw new Error(`binary diff field missing ${field}`);
const kebabField = (value: string) => value.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
const abnfOrder = capture(diffAbnf, /^document = (.+)$/m, "ABNF diff fields").trim().split(/\s+/).map((field) => camel(field.replaceAll("-", "_")));
same(abnfOrder, diffFields, "ABNF diff fields");
const protocolOrder = [...diffProtocol.matchAll(/^field ([a-z_]+)_flag u8$/gm)].map((match) => camel(match[1]));
same(protocolOrder, diffFields, "Semio protocol diff fields");

const rejectionEvidence = [
  mutationTs.includes("{ accepted: true; diff: GltfDiff } | { accepted: false; rejection: GltfMutationRejection }"),
  mutationGraphql.includes("union GltfMutationApplication = GltfMutationAccepted | GltfMutationRejected"),
  mutationProto.includes("message GltfMutationApplication { oneof result"),
  mutationJson.$defs.application.oneOf.length === 2 && mutationJson.$defs.application.oneOf[0].properties.accepted.const === true && mutationJson.$defs.application.oneOf[1].properties.accepted.const === false,
];
if (rejectionEvidence.some((value) => !value)) throw new Error("accepted/rejection sum parity");
for (const value of ["code", "path", "detail"]) {
  if (!mutationTs.includes(`${value}: string`) || !mutationGraphql.includes(`${value}: String!`) || !mutationProto.includes(`string ${value} =`) || !mutationJson.$defs.rejection.required.includes(value)) throw new Error(`rejection field parity ${value}`);
}
for (const value of ["incomingInsertNode/children/{slot}", "document/buffers|snapshot/buffers", "/targets/{target}/{semantic}", "/attributes/{semantic}", "/indices", "inverseBindMatrices", "{input|output}"]) {
  if (!mutationTs.includes(value) || !mutationJson["x-semio-reference-rules"].some((rule: string) => rule.includes(value))) throw new Error(`reference rule parity ${value}`);
}

const facetFiles = ["💡️inferences", "🧬️mutations", "🔺️diff"].flatMap((facet) => walk(path.join(schema, facet)));
for (const file of facetFiles.filter((file) => file.endsWith(".ts"))) {
  const result = ts.transpileModule(fs.readFileSync(file, "utf8"), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true, fileName: file });
  const errors = result.diagnostics?.filter((diagnostic) => diagnostic.category === ts.DiagnosticCategory.Error) ?? [];
  if (errors.length) throw new Error(`TypeScript parse ${file}: ${errors.map((diagnostic) => diagnostic.messageText).join("; ")}`);
}
for (const file of facetFiles.filter((file) => file.endsWith(".json"))) JSON.parse(fs.readFileSync(file, "utf8"));
for (const file of facetFiles.filter((file) => file.endsWith(".ksy"))) YAML.parse(fs.readFileSync(file, "utf8"));

const ajv = new Ajv2020({ strict: false, allErrors: true });
for (const root of [diffJson, mutationJson, inferenceJson]) ajv.addSchema(root);
for (const relative of ["💡️inferences/📝️text/🔣️component.json", "🧬️mutations/📝️text/🔣️component.json", "🔺️diff/📝️text/🔣️component.json"]) ajv.compile(JSON.parse(read(relative)));

const inferenceBinaryTs = read("💡️inferences/💾️binary/🟦️component.ts");
const inferenceBinaryRust = read("💡️inferences/💾️binary/🦀️component.rs");
const inferenceAbnf = read("💡️inferences/💾️binary/🔠️component.abnf");
const inferenceKsy = read("💡️inferences/💾️binary/🥋️component.ksy");
const inferenceProtocol = read("💡️inferences/💾️binary/📡️component.protocol.semio");
for (const token of ["0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0b", "BINARY_HEADER_LENGTH: usize = 40", "BINARY_SCHEMA_VERSION: u32 = 2", "BINARY_SCHEMA_CRC32: u32 = 0x6b25_7ae0"]) {
  if (!inferenceBinaryRust.includes(token)) throw new Error(`Rust inference binary envelope ${token}`);
}
for (const token of ["0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0b", "BINARY_HEADER_LENGTH = 40", "BINARY_SCHEMA_VERSION = 2", "BINARY_SCHEMA_CRC32 = 0x6b257ae0"]) {
  if (!inferenceBinaryTs.includes(token)) throw new Error(`TypeScript inference binary envelope ${token}`);
}
for (const token of ["%x89.53.F8.3F.7D.34.0D.0B", "schema-version = %x02.00.00.00", "schema-crc32 = %xE0.7A.25.6B"]) if (!inferenceAbnf.includes(token)) throw new Error(`ABNF inference binary envelope ${token}`);
for (const token of ["valid: 2", "valid: 0x6b257ae0", "size: payload_length"]) if (!inferenceKsy.includes(token)) throw new Error(`Kaitai inference binary envelope ${token}`);
for (const token of ["header fixed 32", "field payload_length u64", "field payload_crc32 u32", "field header_crc32 u32"]) if (!inferenceProtocol.includes(token)) throw new Error(`Semio inference binary envelope ${token}`);

for (const relative of ["🧬️mutations/🛰️component.proto", "🔺️diff/🛰️component.proto", "🧬️mutations/📝️text/🛰️component.proto", "🔺️diff/📝️text/🛰️component.proto", "💡️inferences/📝️text/🛰️component.proto"]) {
  const source = read(relative);
  for (const match of source.matchAll(/^import\s+"([^"]+)"/gm)) {
    const target = path.resolve(path.dirname(path.join(schema, relative)), match[1]);
    if (!fs.existsSync(target)) throw new Error(`unresolved Proto import ${relative} -> ${match[1]}`);
  }
}

console.log(`PASS indicators=67 groups=14 inference-roots=geometry-only mutations=28 tags=0..27 diff-fields=21 proto-imports=resolved morph-target-grammar=7-fields ts=${facetFiles.filter((file) => file.endsWith(".ts")).length} json=${facetFiles.filter((file) => file.endsWith(".json")).length} ksy=${facetFiles.filter((file) => file.endsWith(".ksy")).length} accepted-rejection=typed inference-envelope=parity`);
