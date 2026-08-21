#!/usr/bin/env bun
/** 🔎️ Scoped re-run of `fixtures lint`'s OWN rules over just the two DIN norm trees.
 *
 * The repo-wide CLI truncates its error list at 40 rows, so a tree can be clean-but-invisible or
 * broken-but-hidden behind another lane's 385 findings. The rule bodies below are transcribed
 * verbatim from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` (`declaredMutations`,
 * `lintArtifact`, `lintCase`, `lintReference`) — same file sets, same error/warn split, same
 * variant-vs-leaf coverage check — so a clean run here means the CLI would print nothing for these
 * two trees either.
 */
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const TREES = [
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
];

const NON_MUTATION_DIRS = new Set(["💾️binary", "📝️text"]);
const CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"] as const;
const DERIVED_CASE_FILES = ["🦠️mutation/🔧️component.op.semio", "🦠️mutation/📡️component.spr.semio", "🔺️diff/🩹️component.patch.semio", "🔺️diff/📡️component.patch.spr.semio"] as const;
const SNAPSHOT_CORE = "🔣️component.json";
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"] as const;
const SNAPSHOT_REF = "🔗️component.ref.json";

type Finding = { readonly level: "error" | "warn"; readonly where: string; readonly what: string };
const dirsIn = (path: string): string[] => (existsSync(path) ? readdirSync(path).filter((entry) => statSync(join(path, entry)).isDirectory()) : []);

function declaredMutations(mutationsRoot: string) {
  const aggregate = existsSync(join(mutationsRoot, "🦀️component.rs")) ? readFileSync(join(mutationsRoot, "🦀️component.rs"), "utf8") : "";
  const enumBody = aggregate.match(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/);
  const variants = enumBody ? [...enumBody[1].matchAll(/^\s+([A-Z][A-Za-z0-9]*)\(/gm)].map((m) => m[1]) : [];
  const leaves = dirsIn(mutationsRoot)
    .filter((entry) => !NON_MUTATION_DIRS.has(entry))
    .filter((entry) => existsSync(join(mutationsRoot, entry, "🦠️mutation/🦀️component.rs")))
    .map((entry) => {
      const source = readFileSync(join(mutationsRoot, entry, "🦠️mutation/🦀️component.rs"), "utf8");
      return { dir: entry, struct: source.match(/^pub struct ([A-Za-z0-9]+)/m)?.[1] ?? null, path: join(mutationsRoot, entry) };
    });
  return { variants, leaves };
}

function lintReference(refFile: string, label: string): Finding[] {
  try {
    const ref = JSON.parse(readFileSync(refFile, "utf8"));
    for (const key of ["artifact", "standard", "subset", "example", "asset"]) {
      if (typeof ref[key] !== "string") return [{ level: "error", where: label, what: `reference is missing string field "${key}"` }];
    }
    if (Object.values(ref).some((value) => typeof value === "string" && (value.includes("..") || value.includes("/")))) {
      return [{ level: "error", where: label, what: "reference fields must not contain path traversal or separators" }];
    }
    return [];
  } catch (error) {
    return [{ level: "error", where: label, what: `reference is not valid JSON: ${(error as Error).message}` }];
  }
}

function lintCase(caseDir: string, label: string, full: boolean): Finding[] {
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
    findings.push({ level: full ? "error" : "warn", where: label, what: `missing derived ${relative}` });
  }
  if (rejected && !existsSync(join(caseDir, "🔺️diff/🚫️component.absent"))) findings.push({ level: "error", where: label, what: "rejected case must carry 🔺️diff/🚫️component.absent" });
  for (const side of ["⬅️before", "➡️after"]) {
    const sideDir = join(caseDir, "📸️snapshot", side);
    if (!existsSync(sideDir)) {
      findings.push({ level: "error", where: label, what: `missing 📸️snapshot/${side}` });
      continue;
    }
    if (existsSync(join(sideDir, SNAPSHOT_REF))) {
      if (existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} has both a reference and inline encodings` });
      findings.push(...lintReference(join(sideDir, SNAPSHOT_REF), `${label}/📸️snapshot/${side}`));
      continue;
    }
    if (!existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} is missing ${SNAPSHOT_CORE}` });
    for (const name of SNAPSHOT_DERIVED) {
      if (existsSync(join(sideDir, name))) continue;
      findings.push({ level: full ? "error" : "warn", where: label, what: `missing derived 📸️snapshot/${side}/${name}` });
    }
  }
  return findings;
}

let errors = 0;
let warnings = 0;
for (const tree of TREES) {
  const root = join(REPO, tree);
  const { variants, leaves } = declaredMutations(root);
  const byStruct = new Map(leaves.filter((leaf) => leaf.struct).map((leaf) => [leaf.struct as string, leaf]));
  const findings: Finding[] = [];
  for (const variant of variants) if (!byStruct.has(variant)) findings.push({ level: "error", where: `${tree}:${variant}`, what: "enum variant has no mutation directory" });
  let covered = 0;
  let cases = 0;
  for (const leaf of leaves) {
    const leafCases = dirsIn(join(leaf.path, "🧪️tests"));
    if (leafCases.length === 0) {
      findings.push({ level: "error", where: `${tree}/${leaf.dir}`, what: "no 🧪️tests cases" });
      continue;
    }
    covered += 1;
    cases += leafCases.length;
    for (const testCase of leafCases) findings.push(...lintCase(join(leaf.path, "🧪️tests", testCase), `${tree}/${leaf.dir}/${testCase}`, false));
  }
  const treeErrors = findings.filter((finding) => finding.level === "error");
  const treeWarnings = findings.filter((finding) => finding.level === "warn");
  errors += treeErrors.length;
  warnings += treeWarnings.length;
  console.log(`🧬️ ${tree}`);
  console.log(`   ${variants.length} enum variants · ${leaves.length} leaves · ${covered} covered · ${leaves.length - covered} uncovered · ${cases} cases · ${treeErrors.length} error(s) · ${treeWarnings.length} derived-encoding warning(s)`);
  for (const finding of treeErrors) console.log(`   ❌️ ${finding.where}: ${finding.what}`);
}
console.log(errors === 0 ? `✅️ scoped: 0 error(s), ${warnings} expected derived-encoding warning(s)` : `❌️ scoped: ${errors} error(s)`);
process.exit(errors === 0 ? 0 : 1);
