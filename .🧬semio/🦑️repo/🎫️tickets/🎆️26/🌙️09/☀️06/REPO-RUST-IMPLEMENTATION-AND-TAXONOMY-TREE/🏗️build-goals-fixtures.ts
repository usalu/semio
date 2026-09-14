#!/usr/bin/env bun
/**
 * 🏗️ Records `🎯️goals/🧫️fixtures/🎯️goal-documents.json` from this repository's own goals tree.
 *
 * The fixture is real data, not an answer key: every `json` member is the exact text of a
 * committed `🎯️goal.json`, so a codec that re-encodes it must reproduce those bytes. The sample
 * is chosen to cover a root goal with a milestone, a first generation goal with an issue, a deep
 * goal, and a goal with no management link at all.
 */
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const goalsDir = join(repoRoot, ".🧬semio", "🦑️repo", "🎯️goals");
const fixture = join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🎯️goals", "🧫️fixtures", "🎯️goal-documents.json");

const found: { id: string; json: string }[] = [];
const walk = (dir: string, id: string): void => {
  for (const entry of readdirSync(dir).sort()) {
    const path = join(dir, entry);
    if (!statSync(path).isDirectory()) continue;
    const child = id === "" ? entry : `${id}/${entry}`;
    try {
      found.push({ id: child, json: readFileSync(join(path, "🎯️goal.json"), "utf8") });
    } catch {
      /* a directory without a document is a grouping directory */
    }
    walk(path, child);
  }
};
walk(goalsDir, "");

const depth = (id: string): number => id.split("/").length - 1;
const has = (entry: { json: string }, member: string): boolean => (JSON.parse(entry.json) as Record<string, unknown>)[member] !== undefined;
const pick = (predicate: (entry: { id: string; json: string }) => boolean): { id: string; json: string } | undefined => found.find(predicate);

const chosen = [
  pick((entry) => depth(entry.id) === 0 && has(entry, "github")),
  pick((entry) => depth(entry.id) === 1 && has(entry, "github")),
  pick((entry) => depth(entry.id) >= 3 && has(entry, "github")),
  pick((entry) => !has(entry, "github")),
].filter((entry): entry is { id: string; json: string } => entry !== undefined);

writeFileSync(
  fixture,
  `${JSON.stringify(
    {
      schema: "semio.repo.goals.goal-documents/1",
      $comment: `📄️ Real 🎯️goal.json documents copied verbatim out of .🧬semio/🦑️repo/🎯️goals by 🏗️build-goals-fixtures.ts. Recorded from ${found.length} goals in the tree.`,
      documents: chosen,
    },
    null,
    2,
  )}\n`,
);
console.log(`[fixtures] ${chosen.length} of ${found.length} goal documents recorded`);
