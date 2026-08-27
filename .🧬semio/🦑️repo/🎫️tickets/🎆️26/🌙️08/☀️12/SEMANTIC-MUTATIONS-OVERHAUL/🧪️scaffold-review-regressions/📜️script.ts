import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { newScaffoldMutationTree } from "../../../../../../../../📜️script.ts";
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️IndependentScaffoldReview
const base = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const taxonomy = loadTaxonomy();
const filename = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
const owner = "✏️s/🔌️plugins/🧪️probe/🧬️mutations";
const name = "➕️insert-page";
const vectors = [
  { name: "foreign-staging-file", aggregate: "pub enum ProbeMutation {}\n", staging: true },
  { name: "wrong-mounted-target", aggregate: `#[path = "foreign/${filename}"] pub mod insert_page;\npub enum ProbeMutation {}\n`, staging: false },
  { name: "wrong-variant-wrapper", aggregate: `#[path = "${name}/${filename}"] pub mod insert_page;\npub enum ProbeMutation { InsertPage(foreign::Mutation) }\n`, staging: false },
  { name: "private-leaf-mount", aggregate: `#[path = "${name}/${filename}"] mod insert_page;\npub enum ProbeMutation { InsertPage(insert_page::Mutation) }\n`, staging: false },
];
let failures = 0;
console.log(`[DEBUG] retained fixture base: ${base}`);
for (const vector of vectors) {
  const root = join(base, vector.name);
  const aggregate = join(root, owner, filename);
  mkdirSync(dirname(aggregate), { recursive: true });
  writeFileSync(aggregate, vector.aggregate);
  const staging = `${aggregate}.scaffold-${process.pid}`;
  if (vector.staging) writeFileSync(staging, "preexisting unrelated file\n");
  let rejected = false;
  let error = "";
  try { newScaffoldMutationTree(root, owner, name); } catch (caught) { rejected = true; error = String(caught); }
  const preserved = !vector.staging || existsSync(staging) && readFileSync(staging, "utf8") === "preexisting unrelated file\n";
  const unchanged = readFileSync(aggregate, "utf8") === vector.aggregate;
  const passed = vector.staging ? preserved : rejected && unchanged;
  console.log(`[DEBUG] ${JSON.stringify({ name: vector.name, passed, rejected, preserved, unchanged, error })}`);
  if (!passed) failures += 1;
}
console.log(`[DEBUG] failed regression groups: ${failures}`);
process.exitCode = failures === 0 ? 0 : 1;
//#endregion 🧪️IndependentScaffoldReview
