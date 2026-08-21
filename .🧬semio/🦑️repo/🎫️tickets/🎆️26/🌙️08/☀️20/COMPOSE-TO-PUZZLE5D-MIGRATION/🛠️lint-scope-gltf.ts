#!/usr/bin/env bun
/** 🧹️ Re-runs `fixtures lint`'s OWN rules (contract D1/D6) scoped to the `stdio.gltf` mutation tree
 * only, because the shared CLI truncates its error list at 40 rows repo-wide. Same rule set: the
 * hand-authored core file quartet plus both snapshot sides, `🎯️outcome.status` in
 * {applied, rejected}, a machine-readable `code` on every rejection, and
 * `🔺️diff/🚫️component.absent` (never an invented empty patch) on rejected cases. Derived encodings
 * stay warnings, exactly as the CLI treats them without `--full`. */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
const CORE = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"] as const;
const DERIVED = ["🦠️mutation/🔧️component.op.semio", "🦠️mutation/📡️component.spr.semio", "🔺️diff/🩹️component.patch.semio", "🔺️diff/📡️component.patch.spr.semio"] as const;
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"] as const;
const NON_MUTATION = new Set(["💾️binary", "📝️text"]);

const dirsIn = (path: string): string[] => (existsSync(path) ? readdirSync(path).filter((entry) => statSync(join(path, entry)).isDirectory()) : []);
const errors: string[] = [];
const warnings: string[] = [];

const leaves = dirsIn(ROOT).filter((entry) => !NON_MUTATION.has(entry) && existsSync(join(ROOT, entry, "🦠️mutation/🦀️component.rs")));
let covered = 0;
for (const leaf of leaves) {
  const cases = dirsIn(join(ROOT, leaf, "🧪️tests"));
  if (cases.length === 0) {
    errors.push(`${leaf}: no 🧪️tests cases`);
    continue;
  }
  covered += 1;
  for (const testCase of cases) {
    const dir = join(ROOT, leaf, "🧪️tests", testCase);
    const label = `${leaf}/${testCase}`;
    let rejected = false;
    const outcomeFile = join(dir, "🎯️outcome/🔣️component.json");
    if (!existsSync(outcomeFile)) errors.push(`${label}: missing 🎯️outcome/🔣️component.json`);
    else {
      try {
        const outcome = JSON.parse(readFileSync(outcomeFile, "utf8"));
        rejected = outcome.status === "rejected";
        if (!["applied", "rejected"].includes(outcome.status)) errors.push(`${label}: 🎯️outcome.status must be "applied" or "rejected", got ${JSON.stringify(outcome.status)}`);
        if (rejected && typeof outcome.code !== "string") errors.push(`${label}: rejected outcome must carry a machine-readable code`);
      } catch (error) {
        errors.push(`${label}: 🎯️outcome is not valid JSON: ${(error as Error).message}`);
      }
    }
    for (const relative of CORE) {
      if (rejected && relative.startsWith("🔺️diff/")) continue;
      if (!existsSync(join(dir, relative))) errors.push(`${label}: missing ${relative}`);
    }
    for (const relative of DERIVED) {
      if (rejected && relative.startsWith("🔺️diff/")) continue;
      if (!existsSync(join(dir, relative))) warnings.push(`${label}: missing derived ${relative}`);
    }
    if (rejected && !existsSync(join(dir, "🔺️diff/🚫️component.absent"))) errors.push(`${label}: rejected case must carry 🔺️diff/🚫️component.absent`);
    if (rejected && existsSync(join(dir, "🔺️diff/🔣️component.json"))) errors.push(`${label}: rejected case must NOT carry a serialized diff`);
    for (const side of ["⬅️before", "➡️after"]) {
      const sideDir = join(dir, "📸️snapshot", side);
      if (!existsSync(join(sideDir, "🔣️component.json"))) errors.push(`${label}: 📸️snapshot/${side} is missing 🔣️component.json`);
      for (const name of SNAPSHOT_DERIVED) if (!existsSync(join(sideDir, name))) warnings.push(`${label}: missing derived 📸️snapshot/${side}/${name}`);
    }
    // 🔣️ Beyond the CLI's own rules: every committed JSON must at least PARSE.
    for (const relative of [...CORE.filter((entry) => !(rejected && entry.startsWith("🔺️diff/"))), "📸️snapshot/⬅️before/🔣️component.json", "📸️snapshot/➡️after/🔣️component.json"]) {
      if (!relative.endsWith(".json") || !existsSync(join(dir, relative))) continue;
      try {
        JSON.parse(readFileSync(join(dir, relative), "utf8"));
      } catch (error) {
        errors.push(`${label}: ${relative} is not valid JSON: ${(error as Error).message}`);
      }
    }
  }
}

console.log(`🧬️ stdio/gltf · ${leaves.length} mutations · ${covered} covered · ${leaves.length - covered} uncovered`);
for (const finding of errors) console.log(`❌️ ${finding}`);
console.log(`${errors.length === 0 ? "✅️" : "❌️"} ${errors.length} error(s) · ⚠️ ${warnings.length} derived-encoding warning(s)`);
process.exit(errors.length === 0 ? 0 : 1);
