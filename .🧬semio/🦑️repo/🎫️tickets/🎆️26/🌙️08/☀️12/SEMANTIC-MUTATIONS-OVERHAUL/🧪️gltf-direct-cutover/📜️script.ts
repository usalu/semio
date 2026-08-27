import { existsSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const repo = process.cwd();
const subset = join(repo, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any");
const schema = join(subset, "🧬️schema");
const mutations = join(schema, "🧬️mutations");
const oraclePath = join(subset, "🧪️oracle/🔣️component.json");
const gluePath = join(repo, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs");

const semanticRenames: Record<string, string> = {
  "declare-used-extension": "add-used-extension",
  "reparent-node": "move-node-parent",
  "require-extension": "add-required-extension",
  "transform-node": "change-node-transform",
  "unrequire-extension": "remove-required-extension",
  "withdraw-used-extension": "remove-used-extension",
};

const verbRecords: Record<string, string> = {
  add: "Added",
  bind: "Bound",
  change: "Changed",
  create: "Created",
  delete: "Deleted",
  move: "Moved",
  remove: "Removed",
  reorder: "Reordered",
  unbind: "Unbound",
};

const pascal = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
const title = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join(" ");
const snake = (value: string): string => value.replaceAll("-", "_");
const escapeRust = (value: string): string => JSON.stringify(value);

const oracle = JSON.parse(readFileSync(oraclePath, "utf8"));
const catalog = oracle.mutationCatalogs.find((value: { id: string }) => value.id === "gltf-2-0-any");
if (!catalog || catalog.vectors.length !== 120) throw new Error(`expected the 120-row glTF catalog, got ${catalog?.vectors?.length ?? 0}`);
const catalogBySource = new Map<string, any>(catalog.vectors.map((value: any) => [value.sourceMutationDirectoryName, value]));

const oldSemantics = readdirSync(mutations, { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && /^[a-z]/.test(entry.name) && existsSync(join(mutations, entry.name, "🦠️mutation/🦀️component.rs")))
  .map((entry) => entry.name)
  .sort();
if (oldSemantics.length !== 120) throw new Error(`expected 120 nested glTF mutation owners, got ${oldSemantics.length}`);

type Leaf = {
  oldSemantic: string;
  semantic: string;
  emoji: string;
  directoryName: string;
  moduleName: string;
  variant: string;
  payload: string;
  mutationType: string;
  rust: string;
  typescript: string;
  graphql: string;
  protobuf: string;
  payloadSchema: any;
};

const helperRustReplacements: ReadonlyArray<readonly [string, string]> = [
  ["schema::mutations::material_animation_private", "schema::modules::mutation_support::material_animation"],
  ["schema::mutations::structure_geometry_private", "schema::modules::mutation_support::structure_geometry"],
  ["schema::mutations::top_level_collections_private", "schema::modules::mutation_support::top_level_collections"],
  ["schema::mutations::top_level_private", "schema::modules::mutation_support::top_level"],
];

function replaceIdentities(source: string, oldSemantic: string, semantic: string): string {
  return source.replaceAll(oldSemantic, semantic).replaceAll(snake(oldSemantic), snake(semantic));
}

function directTypescript(source: string, oldSemantic: string, semantic: string): string {
  let next = replaceIdentities(source, oldSemantic, semantic);
  next = next.replaceAll("../../🔒️material-animation-private/🟦️component.ts", "../../../🔨️modules/🧬️mutation-support/🎞️material-animation/🟦️component.ts");
  next = next.replaceAll("../../🔒️structure-geometry-private/🟦️component.ts", "../../../🔨️modules/🧬️mutation-support/🧱️structure-geometry/🟦️component.ts");
  next = next.replaceAll("../../🔒️top-level-collections-private/🟦️component.ts", "../../../🔨️modules/🧬️mutation-support/🗂️top-level-collections/🟦️component.ts");
  next = next.replaceAll("../../🔒️top-level-private/🟦️component.ts", "../../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts");
  next = next.replace(/from '(\.\.\/)+/g, (match) => match.replace("../", ""));
  return next;
}

function directRust(source: string, oldSemantic: string, semantic: string, payload: string, mutationType: string, variant: string): string {
  let next = replaceIdentities(source, oldSemantic, semantic);
  for (const [before, after] of helperRustReplacements) next = next.replaceAll(before, after);
  next = next.replace(/^\/\/! .*\n/, `//! 🧬️ Direct ${semantic} mutation owner: payload, validation, typed diff, inverse, and outcomes.\n`);
  const mutatingApply = next.includes("pub fn apply(snapshot: &mut GltfSnapshot");
  const forward = mutatingApply
    ? `let mut next = base.clone(); match apply(&mut next, payload) { Ok(()) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(error.code, error.path, error.detail) }`
    : `match apply(payload, base) { Ok(next) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(error.code, error.path, error.detail) }`;
  const verb = semantic.split("-")[0]!;
  const entity = semantic.slice(verb.length + 1);
  const record = `${verbRecords[verb] ?? pascal(verb)}${pascal(entity)}`;
  return `${next.trimEnd()}\n\n//#region 🧬️DirectMutation\n#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]\n#[serde(tag = "phase", content = "value", rename_all = "camelCase")]\npub enum ${mutationType} {\n    Apply(${payload}),\n    Restore(crate::artifacts::gltf::schema::diff::GltfDiff),\n}\n\nfn rejection_outcome(code: String, path: String, detail: String) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {\n    let target = path.split('/').filter(|part| !part.is_empty()).map(str::to_string).collect::<Vec<_>>();\n    if code.contains("no-observable-change") {\n        return protocol::MutationOutcome::new(Default::default()).warn("mutation.no-op", detail);\n    }\n    if code.contains("duplicate") {\n        return protocol::MutationOutcome::fatal("mutation.duplicate-id", detail, target);\n    }\n    if code.contains("out-of-range") || code.contains("missing") || code.contains("not-found") {\n        return protocol::MutationOutcome::error("mutation.target-missing", detail, target);\n    }\n    protocol::MutationOutcome::fatal("mutation.invariant", format!("{code}: {detail}"), target)\n}\n\nimpl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ${mutationType} {\n    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: ${escapeRust(verb)}, entity: ${escapeRust(entity)}, kind: ${escapeRust(semantic)}, record: ${escapeRust(record)} };\n\n    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {\n        match self {\n            Self::Apply(payload) => { ${forward} }\n            Self::Restore(diff) => match protocol::MutationDiff::apply(diff, base) {\n                Ok(_) => protocol::MutationOutcome::new(diff.clone()),\n                Err(error) => protocol::MutationOutcome::fatal("mutation.invariant", error.to_string(), error.target),\n            },\n        }\n    }\n\n    fn inverse(&self, base: &GltfSnapshot) -> Vec<super::GltfMutation> {\n        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);\n        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {\n            return Vec::new();\n        }\n        let inverse = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::inverse(outcome.diff(), base);\n        vec![super::GltfMutation::${variant}(Self::Restore(inverse))]\n    }\n\n    fn label(&self) -> String {\n        ${escapeRust(title(semantic))}.to_string()\n    }\n\n    fn target(&self) -> Vec<String> {\n        vec![${escapeRust(semantic)}.to_string()]\n    }\n}\n//#endregion 🧬️DirectMutation\n\n//#region 🧪️Tests\n#[cfg(test)]\nmod direct_leaf_tests {\n    use super::*;\n\n    #[test]\n    fn semantic_identity_matches_the_language_neutral_descriptor() {\n        assert_eq!(<${mutationType} as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, ${escapeRust(semantic)});\n    }\n}\n//#endregion 🧪️Tests\n`;
}

const leaves: Leaf[] = oldSemantics.map((oldSemantic) => {
  const vector = catalogBySource.get(oldSemantic);
  if (!vector) throw new Error(`catalog has no ${oldSemantic}`);
  const semantic = semanticRenames[oldSemantic] ?? oldSemantic;
  const emoji = vector.mutationDirectoryName.slice(0, -oldSemantic.length);
  if (!emoji) throw new Error(`catalog has no emoji prefix for ${oldSemantic}`);
  const oldDir = join(mutations, oldSemantic);
  const nested = join(oldDir, "🦠️mutation");
  const rawRust = readFileSync(join(nested, "🦀️component.rs"), "utf8");
  const payload = rawRust.match(/pub struct (Gltf[A-Za-z0-9]+Payload)\b/)?.[1];
  if (!payload) throw new Error(`cannot find payload in ${oldSemantic}`);
  const variant = pascal(semantic);
  const mutationType = `${variant}Mutation`;
  const payloadSchema = JSON.parse(readFileSync(join(nested, "🔣️component.json"), "utf8"));
  payloadSchema["x-semio-mutation"] = semantic;
  return {
    oldSemantic,
    semantic,
    emoji,
    directoryName: `${emoji}${semantic}`,
    moduleName: snake(semantic),
    variant,
    payload,
    mutationType,
    rust: directRust(rawRust, oldSemantic, semantic, payload, mutationType, variant),
    typescript: directTypescript(readFileSync(join(nested, "🟦️component.ts"), "utf8"), oldSemantic, semantic),
    graphql: replaceIdentities(readFileSync(join(nested, "🔗️component.graphql"), "utf8"), oldSemantic, semantic),
    protobuf: replaceIdentities(readFileSync(join(nested, "🛰️component.proto"), "utf8"), oldSemantic, semantic),
    payloadSchema,
  };
});

if (new Set(leaves.map((leaf) => leaf.directoryName)).size !== leaves.length) throw new Error("duplicate target mutation directories");
if (new Set(leaves.map((leaf) => leaf.variant)).size !== leaves.length) throw new Error("duplicate aggregate variants");

const supportRoot = join(subset, "🔨️modules/🧬️mutation-support");
const supportMoves: ReadonlyArray<readonly [string, string]> = [
  ["🔒️material-animation-private", "🎞️material-animation"],
  ["🔒️structure-geometry-private", "🧱️structure-geometry"],
  ["🔒️top-level-collections-private", "🗂️top-level-collections"],
  ["🔒️top-level-private", "📚️top-level"],
];
for (const [sourceName, targetName] of supportMoves) {
  const source = join(mutations, sourceName);
  const target = join(supportRoot, targetName);
  if (!existsSync(dirname(target))) Bun.spawnSync(["mkdir", "-p", dirname(target)]);
  renameSync(source, target);
}

for (const duplicate of ["🌳️reparent-node", "🔄️transform-node", "🔗️bind-node-mesh", "🔗️bind-primitive-material"]) {
  const path = join(mutations, duplicate);
  if (existsSync(path)) rmSync(path, { recursive: true });
}

for (const leaf of leaves) {
  const oldDir = join(mutations, leaf.oldSemantic);
  writeFileSync(join(oldDir, "🦀️component.rs"), leaf.rust);
  writeFileSync(join(oldDir, "🟦️component.ts"), leaf.typescript);
  writeFileSync(join(oldDir, "🔗️component.graphql"), leaf.graphql);
  writeFileSync(join(oldDir, "🛰️component.proto"), `${leaf.protobuf.trimEnd()}\n// ${leaf.semantic}\n`);
  writeFileSync(join(oldDir, "🔣️payload.schema.json"), `${JSON.stringify(leaf.payloadSchema, null, 2)}\n`);
  const owner = relative(repo, join(mutations, leaf.directoryName));
  const descriptor = {
    schemaVersion: 1,
    owner,
    semanticKind: leaf.semantic,
    displayName: title(leaf.semantic),
    emoji: leaf.emoji,
    aggregateVariant: leaf.variant,
    payloadSchema: "🔣️payload.schema.json",
    textOpcode: null,
    binaryTag: null,
    invertibility: "explicit-mutation",
    diffParticipation: "detect",
    outcomeClasses: ["applied", "warning", "error", "fatal"],
    composition: "atomic",
    requiredLanguageSurfaces: ["rust", "typescript", "graphql", "protobuf", "json-schema"],
  };
  writeFileSync(join(oldDir, "🔣️component.json"), `${JSON.stringify(descriptor, null, 2)}\n`);
  for (const nested of ["🦠️mutation", "🔺️diff", "↩️inverse"]) rmSync(join(oldDir, nested), { recursive: true });
  for (const file of readdirSync(oldDir, { recursive: true, withFileTypes: true })) {
    if (!file.isFile()) continue;
    const path = join(file.parentPath, file.name);
    const source = readFileSync(path, "utf8");
    const replaced = replaceIdentities(source, leaf.oldSemantic, leaf.semantic)
      .replaceAll(`::${snake(leaf.oldSemantic)}::mutation`, `::${snake(leaf.semantic)}`)
      .replaceAll(`/🦠️mutation/`, `/`);
    if (source !== replaced) writeFileSync(path, replaced);
  }
  renameSync(oldDir, join(mutations, leaf.directoryName));
}

const rootRust = `//! 🧬️ Transparent glTF mutation aggregate. Every concrete payload, outcome, diff, inverse, and test lives in its direct semantic leaf.\n\nuse crate::artifacts::gltf::schema::diff::GltfDiff;\nuse crate::artifacts::gltf::GltfSnapshot;\nuse serde::{Deserialize, Serialize};\n\n${leaves.map((leaf) => `pub use super::${leaf.moduleName}::${leaf.mutationType};`).join("\n")}\n\n/// 🧬️ The complete glTF 2.0 semantic mutation vocabulary.\n#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]\n#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]\n#[mutations(snapshot = GltfSnapshot, diff = GltfDiff, schema = "s.stdio.gltf")]\npub enum GltfMutation {\n${leaves.map((leaf) => `    ${leaf.variant}(${leaf.mutationType}),`).join("\n")}\n}\n\n//#region 🧪️StructuralTests\n#[cfg(test)]\nmod tests {\n    use super::*;\n    use protocol::SemanticMutation;\n\n    #[test]\n    fn aggregate_descriptor_roster_is_exactly_the_direct_leaf_roster() {\n        assert_eq!(GltfMutation::kinds().len(), ${leaves.length});\n        assert_eq!(GltfMutation::kinds().iter().map(|descriptor| descriptor.kind).collect::<std::collections::BTreeSet<_>>().len(), ${leaves.length});\n    }\n}\n//#endregion 🧪️StructuralTests\n`;
writeFileSync(join(mutations, "🦀️component.rs"), rootRust);

const rootTs = `/** 🧬 Transparent TypeScript aggregate for the complete glTF mutation vocabulary. */\n${leaves.map((leaf) => `import type { ${leaf.payload} } from './${leaf.directoryName}/🟦️component.ts';`).join("\n")}\n\nexport type GltfMutation =\n${leaves.map((leaf) => `  | { readonly mutation: '${leaf.semantic}'; readonly payload: ${leaf.payload} }`).join("\n")};\n`;
writeFileSync(join(mutations, "🟦️component.ts"), rootTs);

const rootGraphql = `# 🧬 Complete glTF direct mutation discriminator roster.\n${leaves.map((leaf) => `# ${leaf.semantic}`).join("\n")}\nenum GltfMutationKind {\n${leaves.map((leaf) => `  ${leaf.semantic.replaceAll("-", "_").toUpperCase()}`).join("\n")}\n}\nscalar GltfMutationPayload\ninput GltfMutationInput { kind: GltfMutationKind!, payload: GltfMutationPayload! }\n`;
writeFileSync(join(mutations, "🔗️component.graphql"), rootGraphql);

const protoMessages = leaves.map((leaf) => leaf.protobuf.match(/message\s+([A-Za-z0-9_]+)/)?.[1]);
if (protoMessages.some((value) => !value)) throw new Error("a direct protobuf surface has no payload message");
const rootProto = `syntax = "proto3";\npackage stdio.gltf.mutation;\n${leaves.map((leaf) => `import "${leaf.directoryName}/🛰️component.proto";`).join("\n")}\nmessage GltfMutation {\n  oneof mutation {\n${leaves.map((leaf, index) => `    ${protoMessages[index]} ${leaf.moduleName} = ${index + 1};`).join("\n")}\n  }\n}\n`;
writeFileSync(join(mutations, "🛰️component.proto"), rootProto);

const rootJson = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: "https://semio.tech/schema/stdio/gltf/2.0/mutation",
  title: "glTF Mutation",
  oneOf: leaves.map((leaf) => ({
    type: "object",
    additionalProperties: false,
    required: ["mutation", "payload"],
    properties: { mutation: { const: leaf.semantic }, payload: { $ref: `${leaf.directoryName}/🔣️payload.schema.json` } },
  })),
};
writeFileSync(join(mutations, "🔣️component.json"), `${JSON.stringify(rootJson, null, 2)}\n`);

oracle._comment = "🧬️ The glTF 2.0 Any mutation catalog is an exact 120-row mirror of the direct semantic folders, derived aggregate variants, language-neutral descriptors, payload schemas, language surfaces, and json-rust differential vectors.";
for (const vector of catalog.vectors) {
  const oldSemantic = vector.sourceMutationDirectoryName;
  const leaf = leaves.find((candidate) => candidate.oldSemantic === oldSemantic)!;
  vector.mutationId = leaf.semantic;
  vector.sourceMutationDirectoryName = leaf.directoryName;
  vector.mutationDirectoryName = leaf.directoryName;
}
writeFileSync(oraclePath, `${JSON.stringify(oracle, null, 2)}\n`);

const relativePrefix = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
const mutationBlock = `                            #[path = "."]\n                            pub mod mutations {\n                                #[path = "${relativePrefix}/🦀️component.rs"]\n                                mod component;\n                                pub use component::*;\n${leaves.map((leaf) => `                                #[path = "${relativePrefix}/${leaf.directoryName}/🦀️component.rs"]\n                                pub mod ${leaf.moduleName};`).join("\n")}\n                                #[path = "."]\n                                pub mod create_scene_private_owner {\n                                    #[path = "${relativePrefix}/${leaves.find((leaf) => leaf.semantic === "create-scene")!.directoryName}/🔒️private/🦀️component.rs"]\n                                    pub mod private;\n                                }\n                            }\n`;
let glue = readFileSync(gluePath, "utf8");
const gltfAnchor = glue.indexOf("pub mod gltf");
const mutationStart = glue.indexOf('                            #[path = "."]\n                            pub mod mutations {', gltfAnchor);
const modulesStart = glue.indexOf('                            #[path = "."]\n                            pub mod modules {', mutationStart);
if (mutationStart < 0 || modulesStart < 0) throw new Error("cannot locate glTF mutation glue block");
glue = `${glue.slice(0, mutationStart)}${mutationBlock}${glue.slice(modulesStart)}`;
glue = glue.replace(
  '                                #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🦀️component.rs"]\n                                pub mod mutation_dispatch;\n',
  `                                #[path = "."]\n                                pub mod mutation_support {\n                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🎞️material-animation/🦀️component.rs"]\n                                    pub mod material_animation;\n                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🧱️structure-geometry/🦀️component.rs"]\n                                    pub mod structure_geometry;\n                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️component.rs"]\n                                    pub mod top_level_collections;\n                                    #[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/📚️top-level/🦀️component.rs"]\n                                    pub mod top_level;\n                                }\n`,
);
writeFileSync(gluePath, glue);
