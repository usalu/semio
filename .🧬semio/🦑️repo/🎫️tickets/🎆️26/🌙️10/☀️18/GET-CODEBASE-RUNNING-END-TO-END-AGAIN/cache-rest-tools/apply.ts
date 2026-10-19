// 🛠️ Applies the cache-rest plan to the shared manifests with offset edits; `--dry` prints the summary without writing.
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { apply, child, parseNode, setProperty, type Edit } from "./patch.ts";
import { buildPlan, ROOT, type TaskEdit } from "./plan.ts";

const dry = process.argv.includes("--dry");
const census = JSON.parse(readFileSync(import.meta.dir + "/../🗑️generated/cache-rest-census.json", "utf8"));
const OWNERS = new Set(["🌎️hub", "🎓️teaching", "🏢️semio-tech", "♻️mit-bestand", "👴️leutwiler", "root"]);
const rows = census.targets.filter((row: any) => OWNERS.has(row.owner));
const OUTPUTLESS = /^(test|check|verify|stdio|typecheck|lint)(-|$)|-(check|test|oracle|source|contract|lint)$/u;
const ROOT_FILE = "📋️project.json";

const rootJson = JSON.parse(readFileSync(ROOT + ROOT_FILE, "utf8"));
const rootVerify = rows.filter((row: any) => row.file === ROOT_FILE && row.cache === "false" && !row.continuous && /^bun \.\/📜️script\.ts verify(?: |$)/u.test(row.command ?? "") && !/verify canonical-architecture/u.test(row.command)).map((row: any) => row.name);
const rootTidy = rows.filter((row: any) => row.file === ROOT_FILE && row.classProposal === "deterministic-cacheable" && row.cache === "absent" && /^(test-|stdio-|mutation-outcome-law$|semantic-vocabulary$|verify-package-purity$|verify-generated-corruption$)/u.test(row.name)).map((row: any) => row.name);

const report: string[] = [];
const { files, keepOff } = buildPlan(rootVerify, rootTidy);

const EXCEPTIONS: Record<string, TaskEdit> = {
  "regenerate-example": { cache: false },
  "native-codec-projection": { cache: true },
  "mutation-leaf-taxonomy-generate": { cache: true },
};
const SKIP_TIDY = new Set(["test-discover", "test-report", "build"]);
for (const row of rows) {
  if (row.classProposal !== "deterministic-cacheable" || row.file === ROOT_FILE) continue;
  if (row.cache !== "absent" && row.outputs.kind !== "absent") continue;
  if (SKIP_TIDY.has(row.name) && row.owner === "👴️leutwiler") continue;
  const plan = (files[row.file] ??= { tasks: {} });
  if (plan.tasks[row.name]) continue;
  const edit: TaskEdit = EXCEPTIONS[row.name] ?? {};
  if (!EXCEPTIONS[row.name]) {
    if (row.cache === "absent") edit.cache = true;
    if (row.outputs.kind === "absent" && OUTPUTLESS.test(row.name)) edit.outputs = [];
  }
  if (Object.keys(edit).length) plan.tasks[row.name] = edit;
}
for (const row of rows) {
  const edit = EXCEPTIONS[row.name];
  if (edit && row.classProposal === "deterministic-cacheable" && row.file !== ROOT_FILE) (files[row.file] ??= { tasks: {} }).tasks[row.name] = edit;
}

for (const row of rows) {
  const rawTrue = row.cache === "true", rawAbsent = row.cache === "absent";
  if (((rawTrue && !row.effectiveCache) || (rawAbsent && !row.effectiveCache && !row.effectiveContinuous)) && row.classProposal !== "deterministic-cacheable") (files[row.file] ??= { tasks: {} }).tasks[row.name] = { cache: false };
}

let changedFiles = 0, changedTasks = 0;
for (const [file, plan] of Object.entries(files)) {
  const path = ROOT + file;
  const text = readFileSync(path, "utf8");
  const json = JSON.parse(text);
  const doc = parseNode(text);
  const edits: Edit[] = [];
  const targets = child(doc, "targets")?.node;
  if (!targets) { report.push(`SKIP ${file}: no targets`); continue; }
  for (const [name, edit] of Object.entries(plan.tasks)) {
    const current = json.targets?.[name];
    const node = child(targets, name)?.node;
    if (!current || !node) { report.push(`MISSING ${file} :: ${name}`); continue; }
    if (edit.cache === true && current.cache === true && edit.outputs === undefined && edit.inputs === undefined) continue;
    for (const input of edit.inputs ?? []) {
      if (typeof input !== "string" || !input.startsWith("{workspaceRoot}/")) continue;
      const rel = input.slice("{workspaceRoot}/".length), star = rel.search(/[*?{[]/u), base = star < 0 ? rel : rel.slice(0, rel.lastIndexOf("/", star));
      if (rel.startsWith("**/")) continue;
      if (!existsSync(ROOT + base)) report.push(`MISSING-PATH ${file} :: ${name} :: ${input}`);
    }
    let touched = false;
    for (const key of ["cache", "inputs", "outputs"] as const) {
      const value = edit[key];
      if (value === undefined) continue;
      if (JSON.stringify(current[key]) === JSON.stringify(value)) continue;
      if (key === "outputs" && Array.isArray(current.outputs) && current.outputs.length && value.length === 0) { report.push(`KEEP-OUTPUTS ${file} :: ${name} declares outputs`); continue; }
      edits.push(setProperty(text, node, key, value));
      touched = true;
    }
    if (touched) changedTasks++;
  }
  const named = child(doc, "namedInputs")?.node;
  if (plan.named && !named) edits.push(setProperty(text, doc, "namedInputs", plan.named));
  else for (const [name, value] of Object.entries(plan.named ?? {})) {
    if (JSON.stringify(json.namedInputs?.[name]) === JSON.stringify(value)) continue;
    edits.push(setProperty(text, named!, name, value));
  }
  if (!edits.length) continue;
  const next = apply(text, edits);
  JSON.parse(next);
  changedFiles++;
  if (!dry) {
    if (readFileSync(path, "utf8") !== text) { report.push(`RACE ${file}: changed while planning, skipped`); continue; }
    writeFileSync(path, next);
  }
  report.push(`${dry ? "WOULD " : ""}EDIT ${file}: ${edits.length} edits`);
}
console.log(report.join("\n"));
console.log(JSON.stringify({ dry, changedFiles, changedTasks, rootVerify: rootVerify.length, rootTidy: rootTidy.length, keepOff: keepOff.length }));
await Bun.write(import.meta.dir + "/../🗑️generated/cache-rest-keepoff.json", JSON.stringify(keepOff, null, 1));
