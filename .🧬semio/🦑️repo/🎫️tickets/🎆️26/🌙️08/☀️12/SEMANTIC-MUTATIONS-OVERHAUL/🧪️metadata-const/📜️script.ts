import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧪️ActualConstMetadata
const workspace = process.cwd();
const commandPath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs");
const source = readFileSync(commandPath, "utf8");
const start = source.indexOf("//#region 🪪️MutationLeafDescriptor");
const end = source.indexOf("//#endregion 🪪️MutationLeafDescriptor", start);
if (start < 0 || end < start) throw new Error("missing actual descriptor region");
const production = source.slice(start, end + "//#endregion 🪪️MutationLeafDescriptor".length);
const root = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const externs = ["serde-73de109b1e55818a", "serde_json-0caf27179e7b9139"].flatMap((library) => ["rlib", "rmeta"].flatMap((extension) => ["--extern", `${library.slice(0, library.lastIndexOf("-"))}=${join(workspace, "target/debug/deps", `lib${library}.${extension}`)}`]));
const descriptor = `static ROOT: &str = "✏️s/🔌️plugins/🧪️probe/🧬️mutations"; static OUTCOMES: [MutationOutcomeClass; 1] = [MutationOutcomeClass::Applied]; static SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Rust]; static FIRST: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page", semantic_kind: "insert-page", display_name: "Insert Page", emoji: "➕️", aggregate_variant: "InsertPage", payload_schema: "🦀️.rs#InsertPage", text_opcode: Some("insert-page"), binary_tag: Some(1), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &SURFACES }; static SECOND: MutationLeafDescriptor = MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page", semantic_kind: "remove-page", display_name: "Remove Page", emoji: "➖️", aggregate_variant: "RemovePage", payload_schema: "🦀️.rs#RemovePage", text_opcode: Some("remove-page"), binary_tag: Some(2), ..FIRST };`;
const cases = [
  { name: "const-success", expected: true, assertion: `static ROSTER: [MutationLeafDescriptor; 2] = [FIRST, SECOND]; const _: () = match validate_mutation_leaf_descriptor(&FIRST) { Ok(()) => (), Err(_) => panic!("invalid") }; const _: () = match validate_mutation_leaf_descriptor_roster(ROOT, &ROSTER) { Ok(()) => (), Err(_) => panic!("duplicate") };` },
  { name: "const-invalid-schema", expected: false, assertion: `static INVALID: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 2, ..FIRST }; const _: () = match validate_mutation_leaf_descriptor(&INVALID) { Ok(()) => (), Err(_) => panic!("invalid schema") };` },
  { name: "const-duplicate-tag", expected: false, assertion: `static DUPLICATE: MutationLeafDescriptor = MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page", semantic_kind: "remove-page", display_name: "Remove Page", emoji: "➖️", aggregate_variant: "RemovePage", payload_schema: "🦀️.rs#RemovePage", text_opcode: Some("remove-page"), ..FIRST }; static ROSTER: [MutationLeafDescriptor; 2] = [FIRST, DUPLICATE]; const _: () = match validate_mutation_leaf_descriptor_roster(ROOT, &ROSTER) { Ok(()) => (), Err(_) => panic!("duplicate tag") };` },
];
let failures = 0;
for (const test of cases) {
  const path = join(root, `${test.name}.rs`);
  writeFileSync(path, `${production}\n${descriptor}\n${test.assertion}\nfn main() {}\n`);
  const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", test.name.replaceAll("-", "_"), path, "-L", `dependency=${join(workspace, "target/debug/deps")}`, ...externs, "-o", join(root, test.name)], { encoding: "utf8", timeout: 60_000 });
  writeFileSync(join(root, `${test.name}.log`), `${compiled.stdout}\n${compiled.stderr}`);
  const passed = (compiled.status === 0) === test.expected;
  console.log(`[DEBUG] ${JSON.stringify({ name: test.name, expected: test.expected, compilerStatus: compiled.status, passed })}`);
  if (!passed) failures += 1;
}
console.log(`[DEBUG] ${JSON.stringify({ retainedRoot: root, productionRegionSha256: createHash("sha256").update(production).digest("hex"), failures })}`);
process.exit(failures === 0 ? 0 : 1);
//#endregion 🧪️ActualConstMetadata
