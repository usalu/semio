/** 🧫 Ticket rows 51 + 39 + 92: validates the 11 real retained-command fixtures against the tightened
 * `framework.ui` shared shape, as committed and again with the single-vocabulary re-casing applied
 * in memory, and prints the exact per-file edits the plugin partition still owes.
 *
 * The two vocabularies are the runtime enums in kebab case (row 92, contract §B): lanes mirror
 * `ArtifactToolPublicationLane`, disposition/admission/status mirror `InteractiveJobClassification`
 * — so `BatchOnlyPendingRewrite` and the retired shorthand `batch-only` both normalize to
 * `batch-only-pending-rewrite`, and a fixture already spelling it in full needs no edit.
 *
 *   bun wp4c-retained-command-fixtures.mjs
 */
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const Ajv = require("ajv");

const MODULE_PATH = "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json";
const LANES = { Artifact: "artifact", Config: "config", HostOnly: "host-only", hostOnly: "host-only", Draft: "draft", Presence: "presence", Transient: "transient", Child: "child", host: "host-only" };
const DISPOSITIONS = { Unclassified: "unclassified", Migrated: "migrated", BatchOnlyPendingRewrite: "batch-only-pending-rewrite", "batch-only": "batch-only-pending-rewrite", ForbiddenFromUi: "forbidden-from-ui", forbiddenFromUi: "forbidden-from-ui", Deleted: "deleted", failClosed: "fail-closed", FailClosed: "fail-closed" };
const DISPOSITION_KEYS = new Set(["disposition", "admission", "status"]);

const fixtures = execFileSync("git", ["ls-files", "✏️s"], { encoding: "utf8", maxBuffer: 1 << 28 })
  .split("\n")
  .filter((path) => /🧫️retained-command-limits\/🔣️\.json$|🛣️retained-command-routes\.json$/.test(path))
  .sort();

const module = JSON.parse(readFileSync(MODULE_PATH, "utf8"));
const ajv = new Ajv({ strict: true, allErrors: false }).addSchema(module);
const limits = ajv.getSchema(`${module.$id}#/$defs/RetainedCommandLimits`);
const routes = ajv.getSchema(`${module.$id}#/$defs/RetainedCommandRoutesDocument`);

const edits = [];
const normalize = (node, path, file) => {
  if (Array.isArray(node)) return node.map((item, index) => normalize(item, `${path}/${index}`, file));
  if (node === null || typeof node !== "object") return node;
  const out = {};
  for (const [key, value] of Object.entries(node)) {
    if (key === "lanes" || key === "emittedLanes" || key === "publicationLanes") {
      out[key] = value.map((lane, index) => {
        if (LANES[lane] === undefined) return lane;
        edits.push({ file, pointer: `${path}/${key}/${index}`, from: lane, to: LANES[lane] });
        return LANES[lane];
      });
      continue;
    }
    if (DISPOSITION_KEYS.has(key) && typeof value === "string" && DISPOSITIONS[value] !== undefined) {
      edits.push({ file, pointer: `${path}/${key}`, from: value, to: DISPOSITIONS[value] });
      out[key] = DISPOSITIONS[value];
      continue;
    }
    out[key] = normalize(value, `${path}/${key}`, file);
  }
  return out;
};

let committedPass = 0;
let normalizedPass = 0;
const failures = [];
for (const file of fixtures) {
  const data = JSON.parse(readFileSync(file, "utf8"));
  const validate = file.endsWith("🛣️retained-command-routes.json") ? routes : limits;
  const label = validate === routes ? "RetainedCommandRoutesDocument" : "RetainedCommandLimits";
  if (validate(data)) committedPass++;
  else failures.push([file, label, ajv.errorsText(validate.errors)]);
  if (validate(normalize(data, "", file))) normalizedPass++;
  else console.log(`NORMALIZED STILL FAILS ${file}\n      ${ajv.errorsText(validate.errors, { separator: "\n      " })}`);
}

console.log(`fixtures=${fixtures.length} committed-pass=${committedPass} normalized-pass=${normalizedPass}`);
if (failures.length > 0) {
  console.log(`\ncommitted failures (${failures.length}) — the plugin data still carries the retired spellings:`);
  for (const [file, label, text] of failures) console.log(`  ${file}\n    ${label}: ${text}`);
}
const byFile = new Map();
for (const edit of edits) byFile.set(edit.file, (byFile.get(edit.file) ?? []).concat(edit));
console.log(`\npending plugin re-casing: ${edits.length} values across ${byFile.size} files`);
for (const [file, list] of byFile) {
  const counts = new Map();
  for (const edit of list) counts.set(`${edit.from} → ${edit.to}`, (counts.get(`${edit.from} → ${edit.to}`) ?? 0) + 1);
  console.log(`  ${file}`);
  for (const [pair, count] of [...counts].sort()) console.log(`    ${count}× ${pair}`);
}
