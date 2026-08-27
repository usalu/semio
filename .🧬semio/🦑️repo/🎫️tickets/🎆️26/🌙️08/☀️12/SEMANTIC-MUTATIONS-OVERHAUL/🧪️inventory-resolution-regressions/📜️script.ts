import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { inventoryMutationTaxonomy } from "../../../../../../../../📜️script.ts";
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️Regressions
const root = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const taxonomy = loadTaxonomy();
const filename = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
const mutationRoot = "✏️s/🔌️plugins/🧪️probe/🧬️mutations";
const leaf = `${mutationRoot}/➕️insert-page`;
const command = `✏️s/🔌️plugins/🧪️probe/🎮️commands/${filename}`;
const put = (path: string, content: string): void => {
  const absolute = join(root, path);
  mkdirSync(dirname(absolute), { recursive: true });
  writeFileSync(absolute, content);
};
put(`${mutationRoot}/${filename}`, `#[path = "➕️insert-page/${filename}"] pub mod insert_page;\npub enum ProbeMutation { InsertPage(insert_page::Mutation) }`);
put(`${leaf}/${filename}`, "pub struct Mutation;\n");
const cases = [
  { name: "unrelated-same-name-import", content: "mod unrelated { pub mod insert_page { pub struct Mutation; } }\nuse self::unrelated::insert_page::Mutation;\n", expected: false },
  { name: "nested-comment-import-decoy", content: "/* outer /* nested */\nuse crate::mutations::insert_page::Mutation;\n*/\n", expected: false },
  { name: "raw-string-import-decoy", content: "const DECOY: &str = r#\"quoted \\\"\nuse crate::mutations::insert_page::Mutation;\n\"#;\n", expected: false },
];
console.log(`[DEBUG] retained fixture root: ${root}`);
let failures = 0;
for (const vector of cases) {
  put(command, vector.content);
  const inventory = inventoryMutationTaxonomy(root);
  const record = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➕️insert-page")!;
  const actual = record.consumerEdges.some(({ sourcePath }) => sourcePath === command);
  const validMount = record.evidence.resolvedMounts.some(({ targetPath }) => targetPath === `${leaf}/${filename}`);
  console.log(`[DEBUG] ${JSON.stringify({ name: vector.name, expectedEdge: vector.expected, actualEdge: actual, expectedMount: true, actualMount: validMount })}`);
  if (actual !== vector.expected || !validMount) failures += 1;
}
const missingPath = "ticket/missing-ledger.json";
const record = inventoryMutationTaxonomy(root, { assignmentLedgerPath: missingPath }).records[0]!;
console.log(`[DEBUG] ${JSON.stringify({ name: "missing-ledger-provenance", expected: missingPath, actual: record.assignmentEvidence.ledgerPath })}`);
if (record.assignmentEvidence.ledgerPath !== missingPath) failures += 1;
console.log(`[DEBUG] failed regression groups: ${failures}`);
process.exit(failures === 0 ? 0 : 1);
//#endregion 🧪️Regressions
