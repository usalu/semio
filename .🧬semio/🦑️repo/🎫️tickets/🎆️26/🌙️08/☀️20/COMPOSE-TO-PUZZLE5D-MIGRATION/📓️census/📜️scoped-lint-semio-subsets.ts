/** 🔍️ Same rules as `📜️script.ts fixtures lint`, scoped to the nine 🧿️semio multi-mutation subsets,
 * with NO 40-row truncation. Logic transcribed verbatim from that file's lintArtifact/lintCase. */
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SUBSETS = ["✳️drawing", "✳️mesh", "✳️kit", "✳️brep", "✳️image", "✳️graph", "✳️object", "✳️table", "✳️text"];
const NON_MUTATION_DIRS = new Set(["💾️binary", "📝️text"]);
const CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"];
const DERIVED_CASE_FILES = ["🦠️mutation/🔧️component.op.semio", "🦠️mutation/📡️component.spr.semio", "🔺️diff/🩹️component.patch.semio", "🔺️diff/📡️component.patch.spr.semio"];
const SNAPSHOT_CORE = "🔣️component.json";
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"];
const SNAPSHOT_REF = "🔗️component.ref.json";
const dirsIn = (p: string): string[] => (existsSync(p) ? readdirSync(p).filter((e) => statSync(join(p, e)).isDirectory()) : []);
type Finding = { level: "error" | "warn"; where: string; what: string };

function lintCase(caseDir: string, label: string): Finding[] {
  const findings: Finding[] = [];
  const outcomeFile = join(caseDir, "🎯️outcome/🔣️component.json");
  let rejected = false;
  if (existsSync(outcomeFile)) {
    try {
      const outcome = JSON.parse(readFileSync(outcomeFile, "utf8"));
      rejected = outcome.status === "rejected";
      if (!["applied", "rejected"].includes(outcome.status)) findings.push({ level: "error", where: label, what: `🎯️outcome.status must be "applied" or "rejected", got ${JSON.stringify(outcome.status)}` });
      if (rejected && typeof outcome.code !== "string") findings.push({ level: "error", where: label, what: "rejected outcome must carry a machine-readable code" });
    } catch (error) {
      findings.push({ level: "error", where: label, what: `🎯️outcome is not valid JSON: ${(error as Error).message}` });
    }
  }
  for (const relative of CORE_CASE_FILES) {
    if (rejected && relative.startsWith("🔺️diff/")) continue;
    if (!existsSync(join(caseDir, relative))) findings.push({ level: "error", where: label, what: `missing ${relative}` });
  }
  for (const relative of DERIVED_CASE_FILES) {
    if (rejected && relative.startsWith("🔺️diff/")) continue;
    if (existsSync(join(caseDir, relative))) continue;
    findings.push({ level: "warn", where: label, what: `missing derived ${relative}` });
  }
  if (rejected && !existsSync(join(caseDir, "🔺️diff/🚫️component.absent"))) findings.push({ level: "error", where: label, what: "rejected case must carry 🔺️diff/🚫️component.absent" });
  for (const side of ["⬅️before", "➡️after"]) {
    const sideDir = join(caseDir, "📸️snapshot", side);
    if (!existsSync(sideDir)) { findings.push({ level: "error", where: label, what: `missing 📸️snapshot/${side}` }); continue; }
    if (existsSync(join(sideDir, SNAPSHOT_REF))) {
      if (existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} has both a reference and inline encodings` });
      continue;
    }
    if (!existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} is missing ${SNAPSHOT_CORE}` });
    for (const name of SNAPSHOT_DERIVED) if (!existsSync(join(sideDir, name))) findings.push({ level: "warn", where: label, what: `missing derived 📸️snapshot/${side}/${name}` });
  }
  return findings;
}

let totalErrors = 0, totalWarn = 0;
for (const subset of SUBSETS) {
  const root = join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets", subset, "🧬️schema/🧬️mutations");
  const aggregate = readFileSync(join(root, "🦀️component.rs"), "utf8");
  const enumBody = aggregate.match(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/);
  const variants = enumBody ? [...enumBody[1].matchAll(/^\s+([A-Z][A-Za-z0-9]*)\(/gm)].map((m) => m[1]) : [];
  const leaves = dirsIn(root).filter((e) => !NON_MUTATION_DIRS.has(e)).filter((e) => existsSync(join(root, e, "🦠️mutation/🦀️component.rs")))
    .map((e) => ({ dir: e, struct: (readFileSync(join(root, e, "🦠️mutation/🦀️component.rs"), "utf8").match(/^pub struct ([A-Za-z0-9]+)/m) ?? [])[1] ?? null, path: join(root, e) }));
  const byStruct = new Map(leaves.filter((l) => l.struct).map((l) => [l.struct as string, l]));
  const findings: Finding[] = [];
  for (const variant of variants) if (!byStruct.has(variant)) findings.push({ level: "error", where: `${subset}:${variant}`, what: "enum variant has no mutation directory" });
  let covered = 0;
  for (const leaf of leaves) {
    const cases = dirsIn(join(leaf.path, "🧪️tests"));
    if (cases.length === 0) { findings.push({ level: "error", where: `${subset}/${leaf.dir}`, what: "no 🧪️tests cases" }); continue; }
    covered++;
    for (const c of cases) findings.push(...lintCase(join(leaf.path, "🧪️tests", c), `${subset}/${leaf.dir}/${c}`));
  }
  const errors = findings.filter((f) => f.level === "error");
  const warns = findings.filter((f) => f.level === "warn");
  totalErrors += errors.length; totalWarn += warns.length;
  console.log(`${subset}: ${covered}/${leaves.length} leaves covered · ${errors.length} error(s) · ${warns.length} derived-encoding warning(s)`);
  for (const e of errors) console.log(`   ❌️ ${e.where}: ${e.what}`);
}
console.log(`TOTAL: ${totalErrors} error(s), ${totalWarn} derived-encoding warning(s) across the nine trees`);
