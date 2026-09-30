/** 🧪️ W3-CODES: proves the outcome-code rule flags every position it owns — a scratch git repository (under this ticket's
 * `🗑️generated/`) carries one violation per position plus the vocabulary document; exits 1 unless each is reported. */
import { mkdirSync, rmSync, writeFileSync, copyFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { policyMutationMessageCodeBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const root = join(repo, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/w3-codes/gate-negatives");
const vocabulary = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json";
const files: Record<string, string> = {
  "p/🧬️mutations/a/🔺️diff/🦀️.rs": 'fn diff() -> MutationOutcome<D> {\n    MutationOutcome::error("mutation.missing", "m", [""]);\n    MutationOutcome::error("mutation.invariant", "m", [""]);\n    MutationOutcome::empty().warn("wfc3d.seed.unchanged", "m");\n    Err("mutation.asset-missing")\n}\n',
  "p/🔺️diff/🦀️.rs": 'fn apply() { MutationApplyError::new("invalid-add-index", "m"); MutationApplyError { code: "mutation.apply.Bad".into() }; }\n',
  "p/🧬️mutations/🟦️.ts": 'refuse("error", "mutation.referenced", "m"); const x = { level: "fatal", code: "mutation.target-missing" };\n',
  "p/🧪️tests/o/🐍️.py": 'return empty(), [("error", "wfc3d.slot.missing")]\nraise Fatal("mutation.colour-in-use")\n',
  "p/🧪️tests/o/🥒️.feature": "  | case | mutation.content-gap |\n  reads 🦠️mutation.json and mutation.apply.<detail>\n",
  "p/🧫️fixtures/c/🎯️outcome/🔣️.json": '{"status":"rejected","messages":[{"level":"warn","code":"mutation.duplicate-id"},{"level":"error","code":"forms.missing-response"}]}\n',
};
const expected = ["mutation.missing", "mutation.invariant", "wfc3d.seed.unchanged", "mutation.asset-missing", "invalid-add-index", "mutation.apply.Bad", "mutation.referenced", "mutation.target-missing", "wfc3d.slot.missing", "mutation.colour-in-use", "mutation.content-gap", "mutation.duplicate-id", "forms.missing-response"];
rmSync(root, { recursive: true, force: true });
for (const [path, content] of Object.entries({ ...files, [vocabulary]: "" })) {
  mkdirSync(dirname(join(root, path)), { recursive: true });
  if (path === vocabulary) copyFileSync(join(repo, vocabulary), join(root, path));
  else writeFileSync(join(root, path), content);
}
Bun.spawnSync(["git", "init", "-q"], { cwd: root });
const breaches = policyMutationMessageCodeBreaches(root);
const reported = new Set(breaches.map((breach) => /outcome code "([^"]*)"/.exec(breach.summary)?.[1]));
const missing = expected.filter((code) => !reported.has(code));
for (const breach of breaches) console.log(`[w3-codes] ${breach.summary}`);
console.log(`[w3-codes] ${breaches.length} reported, missing ${JSON.stringify(missing)}, unexpected ${JSON.stringify([...reported].filter((code) => !expected.includes(code!)))}`);
rmSync(root, { recursive: true, force: true });
process.exit(missing.length === 0 && reported.size === expected.length ? 0 : 1);
