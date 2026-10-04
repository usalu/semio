#!/usr/bin/env bun
/** 💰️ C3 budget probe: writes the quiz sources as they were before work package C3 — the working tree without C3's
 * edits — as a map from absolute path to source text, for `c3_budget.vite.ts`, which builds the site from them, so the
 * entry script's growth by C3 can be told from everybody else's. For every file whose difference from the index is C3's
 * alone the index holds that text (read with `git show :<path>`, nothing is written to the repository); the i18n module
 * also carries another ticket's unstaged lines, so there only C3's keys are taken out of the working tree's text.
 * Usage from the repository root: `bun <TK>/c3_budget.ts`, then build with `c3_budget.vite.ts` (see its header). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../..");
const react = "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
const fromIndex = [`${react}/🟦️.tsx`, ...["🐾️pets", "🎛️preferences", "🪟️chrome", "📖️quiz-page", "🗂️classification", "↕️sorting", "🃏️matching", "▶️run", "🏁️results"].map((module) => `${react}/🔨️modules/${module}/🟦️.tsx`)];
const i18n = `${react}/🔨️modules/🌐️i18n/🟦️.ts`;
const keys = /^\s+pets(?:Play|Mischief|Resting|PlayWith|Hello|Trick|Pet|Toss|HelloSaid|TrickSaid|PetSaid|TossSaid): phrase\(.*\),\n/gmu;
const marks = /petProp|PetTopic|usePetPlay|petsPlay|petsMischief|data-pet-topic|QUIZ_PET_CONTROLS|QUIZ_PET_PROPS|PET_DEED|PetsPlayground|topic=\{quiz\.id\}/u;
const variant: Record<string, string> = {};
for (const path of fromIndex) {
  const shown = Bun.spawnSync(["git", "-c", "core.quotepath=false", "show", `:${path}`], { cwd: repoRoot });
  if (shown.exitCode !== 0) throw new Error(`git show :${path}: ${shown.stderr.toString()}`);
  const text = shown.stdout.toString();
  if (marks.test(text)) throw new Error(`the index already holds C3's edits of ${path}: the variant would not be the tree before C3`);
  variant[resolve(repoRoot, path).replaceAll("\\", "/")] = text;
}
const current = readFileSync(resolve(repoRoot, i18n), "utf8");
const removed = current.match(keys)?.length ?? 0;
if (removed !== 24) throw new Error(`expected the 24 keys of C3 in the i18n module, found ${removed}`);
variant[resolve(repoRoot, i18n).replaceAll("\\", "/")] = current.replace(keys, "");
const out = resolve(here, "🗑️generated/c3");
mkdirSync(out, { recursive: true });
writeFileSync(resolve(out, "pre-variant.json"), JSON.stringify(variant));
process.stdout.write(`${Object.keys(variant).length} files, ${removed} keys taken out of the i18n module\n`);
