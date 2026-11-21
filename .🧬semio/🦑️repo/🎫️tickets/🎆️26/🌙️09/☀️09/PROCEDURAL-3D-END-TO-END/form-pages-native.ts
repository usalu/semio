import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();
const semantic = join(root, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📝️form");
const plugin = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🦀️.rs");
const budgetFile = join(root, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📏️budget/🧬️schema/🔣️.json");
const imageFile = join(root, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🧫️fixtures/form-image.json");
const [fixture, images, pages, carriers, contract] = await Promise.all([
  Bun.file(join(semantic, "pages.json")).json(),
  Bun.file(imageFile).json(),
  readFile(join(semantic, "pages.rs"), "utf8"),
  readFile(plugin, "utf8"),
  Bun.file(budgetFile).json(),
]);
const limits = Object.values(contract).find(value => value?.documentNodes !== undefined);
if (!limits) throw Error("missing UI capacities");
const begin = carriers.indexOf("pub fn paged_text_carrier_nodes");
const finish = carriers.indexOf("\n}", begin);
if (begin < 0 || finish < 0) throw Error("missing carrier count");
const row = fixture.cases.find(row => row.name === "retained image sources");
const checks = fixture.cases.map(entry => {
  const fields = entry.fields.flatMap(field => Array.from({ length: field.count }, () => `form_pages::FieldCost { nodes: ${field.nodes}, section: ${field.section === undefined ? "None" : `Some(${field.section})`} }`));
  const actual = `form_pages::form_page([${fields.join(",")}], ${entry.budget}, ${entry.requested})`;
  const expected = entry.expect
    ? `Ok(form_pages::FormPage { index: ${entry.expect.index}, total: ${entry.expect.total}, fields: ${entry.expect.first}..${entry.expect.last}, nodes: ${entry.expect.nodes} })`
    : `Err(${entry.rejected})`;
  return `assert_eq!(${actual}, ${expected}, ${JSON.stringify(entry.name)});`;
});
const generated = join(import.meta.dirname, "🗑️generated/form-pages");
await mkdir(generated, { recursive: true });
const source = join(generated, "proof.rs");
const binary = join(generated, process.platform === "win32" ? "proof.exe" : "proof");
await writeFile(source, [
  "mod form_pages {",
  pages.split("#[cfg(test)]", 1)[0],
  "}",
  `const UI_TEXT_MAX_BYTES: usize = ${limits.textMaxBytes};`,
  `const UI_FIXED_LIST_ITEMS: usize = ${limits.fixedListItems};`,
  `const UI_BUILT_CHILDREN_MAX: usize = ${limits.documentNodes};`,
  carriers.slice(begin, finish + 2),
  "#[test] fn matches_the_language_neutral_page_laws() {",
  ...checks,
  "}",
  "#[test] fn the_long_image_cost_matches_form_admission() {",
  `assert_eq!(2 + paged_text_carrier_nodes(${JSON.stringify(images.en.steps[0].blocks[0].src)}), ${row.fields[0].nodes});`,
  `assert_eq!(UI_BUILT_CHILDREN_MAX - 2 - ${limits.treeFixedHeadroom}, ${row.budget});`,
  "}",
].join("\n"));
for (const argv of [
  ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024", "--test", source, "-o", binary],
  [binary],
]) {
  const code = await Bun.spawn(argv, { stdout: "inherit", stderr: "inherit" }).exited;
  if (code !== 0) process.exit(code);
}
