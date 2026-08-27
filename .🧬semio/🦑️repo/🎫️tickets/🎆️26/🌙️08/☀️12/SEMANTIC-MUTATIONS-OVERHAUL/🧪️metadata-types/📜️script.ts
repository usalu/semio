import { readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";

//#region 🧪️MutationLeafDescriptorContract
const workspace = process.cwd();
const commandPath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs");
const fixturePath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-leaf-descriptor/🧫️fixtures/🔣️.json");
const fixtureSchemaPath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-leaf-descriptor/🛂️schema.json");
const descriptorSchemaPath = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json");
const promotedVectorsPath = join(import.meta.dir, "../🧪️descriptor-contract-preflight/🔣️vectors.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const fixtureSchema = JSON.parse(readFileSync(fixtureSchemaPath, "utf8"));
const descriptorSchema = JSON.parse(readFileSync(descriptorSchemaPath, "utf8"));
const promotedVectors = JSON.parse(readFileSync(promotedVectorsPath, "utf8"));
const source = readFileSync(commandPath, "utf8");
const ajv = new Ajv({ allErrors: true, strict: true }).addSchema(descriptorSchema);
const validate = ajv.compile(fixtureSchema);
const validateDescriptor = ajv.getSchema("https://semio.tech/schema/mutation-descriptor/1")!;
const requiredFields = ["schema_version", "owner", "semantic_kind", "display_name", "emoji", "aggregate_variant", "payload_schema", "text_opcode", "binary_tag", "invertibility", "diff_participation", "outcome_classes", "composition", "required_language_surfaces"];
const enumTypes = ["MutationInvertibility", "MutationDiffParticipation", "MutationOutcomeClass", "MutationComposition", "MutationLanguageSurface"];
const fieldsPresent = requiredFields.every((field) => source.includes(`pub ${field}:`));
const enumsPresent = enumTypes.every((name) => source.includes(`pub enum ${name}`));
const vectorsMatch = JSON.stringify(fixture.binaryTagVectors) === JSON.stringify(promotedVectors.cases);
const fixtureSchemaValid = validate(fixture);
const ownerVectorsMatch = fixture.ownerBoundaryVectors.every((vector: { owner: string; expected: boolean }) => validateDescriptor({ ...fixture.descriptor, owner: vector.owner }) === vector.expected);
const rosterVectorsPresent = fixture.rosterVectors.length === 20 && fixture.rosterVectors.filter((vector: { expected: boolean }) => vector.expected).length === 3;
console.log(`[DEBUG] ${JSON.stringify({ fixtureSchemaValid, fieldsPresent, enumsPresent, vectorsMatch, ownerVectorsMatch, rosterVectorsPresent, fixtureFields: Object.keys(fixture.descriptor).length, vectorCount: fixture.binaryTagVectors.length, ownerVectorCount: fixture.ownerBoundaryVectors.length, rosterVectorCount: fixture.rosterVectors.length })}`);
if (!fixtureSchemaValid || !fieldsPresent || !enumsPresent || !vectorsMatch || !ownerVectorsMatch || !rosterVectorsPresent || Object.keys(fixture.descriptor).length !== 14) process.exit(1);
if (Bun.argv.includes("--cargo")) {
  const compiled = spawnSync("bun", ["nx", "run", "@semio-tech/framework-os-kernel:test", "--", "mutation_leaf_descriptor"], { cwd: workspace, encoding: "utf8", timeout: 180000 });
  process.stdout.write(compiled.stdout ?? "");
  process.stderr.write(compiled.stderr ?? "");
  if (compiled.status !== 0) process.exit(compiled.status ?? 1);
}
//#endregion 🧪️MutationLeafDescriptorContract
