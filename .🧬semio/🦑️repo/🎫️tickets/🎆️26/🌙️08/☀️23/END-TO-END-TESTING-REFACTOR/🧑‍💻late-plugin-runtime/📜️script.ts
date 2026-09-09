import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
const binary = process.argv[2];
const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
if (!binary || !root) throw new Error("Explicit binary and repository paths are required");
const ticket = dirname(import.meta.dir);
const report = await readFile(join(ticket, "📓️test-layout-late-rust-mutation-fixtures-2026-09-09.md"), "utf8");
const arrays = [...report.matchAll(/```json\n([\s\S]*?)\n```/g)].map(match => JSON.parse(match[1]!));
const paths = arrays.find(value => Array.isArray(value) && value.every(item => typeof item === "string")) as string[];
const depfile = await readFile(binary.replace(/\.exe$/, "") + ".d", "utf8");
const dependencies = new Set(depfile.split("\n")[0]!.slice(depfile.indexOf(": ") + 2).split(/(?<!\\)\s+/).filter(Boolean).map(path => resolve(root, path.replace(/\\ /g, " "))));
const built = (await stat(binary)).mtimeMs;
let checked = 0;
for (const path of paths.filter(path => path.endsWith("/🦀️.rs"))) {
  const source = join(root, path);
  const metadata = await stat(source).catch(() => undefined);
  if (!metadata) continue;
  assert.ok(dependencies.has(source), `Compiler did not include ${path}`);
  assert.ok(metadata.mtimeMs <= built, `Source is newer than binary: ${path}`);
  checked++;
}
const names = [
  "component::builder::dependency_fixture::mutations::add_value::tests::direct_leaf_contract",
  "component::contributed_mutation_wire::mutations::add_value::tests::direct_leaf_descriptor_and_inverse_law",
  "component::declaration_fixture_mutations::std1_any::set_value::tests::actual_leaf_descriptor_and_provenance",
  "component::declaration_fixture_mutations::std1_any::set_value::tests::assignment_inverse_and_structural_diff",
  "component::declaration_fixture_mutations::std1_any::set_value::tests::source_json_codecs_and_i32_boundaries",
  "component::declaration_fixture_mutations::std1_strict::set_value::tests::actual_leaf_descriptor_and_provenance",
  "component::declaration_fixture_mutations::std1_strict::set_value::tests::assignment_inverse_and_structural_diff",
  "component::declaration_fixture_mutations::std1_strict::set_value::tests::source_json_codecs_and_i32_boundaries",
  "component::declaration_fixture_mutations::std2_any::set_value::tests::actual_leaf_descriptor_and_provenance",
  "component::declaration_fixture_mutations::std2_any::set_value::tests::assignment_inverse_and_structural_diff",
  "component::declaration_fixture_mutations::std2_any::set_value::tests::source_json_codecs_and_i32_boundaries",
  "component::test_app_mutation_fixture::config::mutations::change_test_config_selection::tests::nullable_selection_serde_text_and_binary_round_trip",
  "component::test_app_mutation_fixture::config::mutations::change_test_config_selection::tests::structural_config_diff_serde_preserves_identity_clear_and_set",
  "component::test_app_mutation_fixture::document::mutations::set_count::tests::descriptor_has_set_count_identity",
  "component::test_app_mutation_fixture::document::mutations::set_label::tests::descriptor_has_set_label_identity",
  "component::test_app_mutation_fixture::document::mutations::tests::direct_leaves_preserve_generic_document_codecs_and_laws"
];
for (const name of names) {
  const child = Bun.spawn([binary, name, "--exact", "--nocapture"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [code, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
  assert.equal(code, 0, stdout + stderr);
  assert.match(stdout, /1 passed; 0 failed/);
  console.log(`[DEBUG] ${name} PASS`);
}
console.log(`[DEBUG] ${names.length} exact plugin laws passed; ${checked} current Rust inputs verified against compiler dependency information`);
