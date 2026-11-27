/** 🧬️ F112: plans every schema twin through the generator's own `planTwins` (no repo run), compares with the committed file, and writes drift to JSONL (dry run) or to scratch. Usage: bun twins-plan.ts <out.jsonl> <scratchDir> [scope] */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { planTwins } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";
import { compare } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/⚖️equivalence/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const [out, scratch, scopeArg] = [process.argv[2]!, process.argv[3]!, process.argv[4] ?? root];
const plans = planTwins(scopeArg, ["typescript", "rust", "protobuf", "graphql"]);
const rows: string[] = [];
const seen = new Set<string>();
for (const plan of plans) {
  const committed = readFileSync(plan.path, "utf8");
  const state = compare(plan.language, committed, plan.text);
  if (plan.problem !== undefined) rows.push(JSON.stringify({ problem: plan.problem, path: relative(root, plan.path) }));
  if (state === "identical") continue;
  const rel = relative(root, plan.path);
  const key = rel + "|" + plan.language;
  if (seen.has(key)) continue;
  seen.add(key);
  const target = join(scratch, rel);
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, plan.text);
  rows.push(JSON.stringify({ path: rel, language: plan.language, state, before: committed.length, after: plan.text.length }));
}
writeFileSync(out, rows.join("\n") + "\n");
console.log(`plans=${plans.length} changed=${rows.length}`);
