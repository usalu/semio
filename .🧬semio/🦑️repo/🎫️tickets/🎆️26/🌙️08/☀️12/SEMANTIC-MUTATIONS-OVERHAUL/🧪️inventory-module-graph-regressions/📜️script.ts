import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { inventoryMutationTaxonomy } from "../../../../../../../../📜️script.ts";
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️IndependentGraphReview
const base = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const taxonomy = loadTaxonomy();
const filename = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
const owner = "✏️s/🔌️plugins/🧪️probe";
const mutations = `${owner}/🧬️mutations`;
const leaf = `${mutations}/➕️insert-page/${filename}`;
const command = `${owner}/🎮️command/${filename}`;
const common = {
  [`${owner}/Cargo.toml`]: '[package]\nname = "module-graph-probe"\nversion = "0.1.0"\nedition = "2021"\n[lib]\npath = "📦️glue.rs"\n',
  [`${mutations}/${filename}`]: `#[path = "➕️insert-page/${filename}"] pub mod insert_page;\npub enum ProbeMutation { InsertPage(insert_page::Mutation) }\n`,
  [leaf]: "pub struct Mutation;\n",
};
const vectors = [
  {
    name: "inline-path-override", expected: true,
    files: {
      [`${owner}/📦️glue.rs`]: `#[path = "."] pub mod standards { #[path = "."] pub mod schema { #[path = "🧬️mutations/${filename}"] pub mod mutations; } }\n#[path = "🎮️command/${filename}"] pub mod command;\n`,
      [command]: "use crate::standards::schema::mutations::insert_page::Mutation;\npub fn construct() -> Mutation { Mutation }\n",
    },
  },
  {
    name: "ordinary-child-module-directory", expected: true,
    files: {
      [`${owner}/📦️glue.rs`]: `pub mod outer;\n#[path = "🎮️command/${filename}"] pub mod command;\n`,
      [`${owner}/outer.rs`]: "pub mod inner;\n",
      [`${owner}/outer/inner.rs`]: `#[path = "../🧬️mutations/${filename}"] pub mod mutations;\n`,
      [command]: "use crate::outer::inner::mutations::insert_page::Mutation;\npub fn construct() -> Mutation { Mutation }\n",
    },
  },
  {
    name: "crate-prefix-must-not-use-child-local-mount", expected: false,
    files: {
      [`${owner}/📦️glue.rs`]: `#[path = "🎮️command/${filename}"] pub mod command;\n`,
      [command]: `#[path = "../🧬️mutations/➕️insert-page/${filename}"] pub mod insert_page;\nuse crate::insert_page::Mutation;\npub fn construct() -> Mutation { Mutation }\n`,
    },
  },
  {
    name: "self-prefix-must-not-escape-inline-scope", expected: false,
    files: {
      [`${owner}/📦️glue.rs`]: `#[path = "🎮️command/${filename}"] pub mod command;\n`,
      [command]: `#[path = "../🧬️mutations/➕️insert-page/${filename}"] pub mod insert_page;\npub mod nested { use self::insert_page::Mutation; pub fn construct() -> Mutation { Mutation } }\n`,
    },
  },
];
let failures = 0;
console.log(`[DEBUG] retained fixture base: ${base}`);
for (const vector of vectors) {
  const root = join(base, vector.name);
  for (const [path, source] of Object.entries({ ...common, ...vector.files })) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), source);
  }
  const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "module_graph_probe", "--crate-type", "lib", "--emit=metadata", join(root, owner, "📦️glue.rs"), "-o", join(root, "oracle.rmeta")], { encoding: "utf8", timeout: 30_000 });
  writeFileSync(join(root, "🧪️compiler.log"), `${compiled.stdout}\n${compiled.stderr}`);
  const inventory = inventoryMutationTaxonomy(root);
  const record = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➕️insert-page")!;
  const actual = record.consumerEdges.some(({ sourcePath, targetPath, relation }) => sourcePath === command && targetPath === leaf && relation === "import");
  const compiler = compiled.status === 0;
  const passed = actual === vector.expected && compiler === vector.expected;
  console.log(`[DEBUG] ${JSON.stringify({ name: vector.name, expected: vector.expected, actual, compiler, passed })}`);
  if (!passed) failures += 1;
}
console.log(`[DEBUG] failed regression groups: ${failures}`);
process.exitCode = failures === 0 ? 0 : 1;
//#endregion 🧪️IndependentGraphReview
