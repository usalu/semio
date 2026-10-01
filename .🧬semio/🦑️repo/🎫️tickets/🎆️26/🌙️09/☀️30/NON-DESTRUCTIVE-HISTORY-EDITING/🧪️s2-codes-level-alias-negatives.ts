/** 🪪️ S2-CODES: proves the outcome-code rule reports the retired `warn` level spelling at every position that spells a
 * level — Rust hand decoders, TypeScript `refuse`/`{ level, code }`, Python tuples, Gherkin messages and committed
 * `🎯️outcome` documents — and stays silent on the canonical `warning` controls. A scratch git repository under this
 * ticket's `🗑️generated/` carries both; exits 1 unless exactly the six planted aliases are reported. */
import { copyFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { policyMutationMessageCodeBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const root = join(repo, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s2-codes/level-alias-negatives");
const vocabulary = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json";
const files: Record<string, string> = {
  "p/🧪️tests/a/🦀️.rs": 'fn level(text: &str) -> protocol::Severity {\n    match text {\n        "info" => protocol::Severity::Info,\n        "warn" => protocol::Severity::Warning,\n        "warning" => protocol::Severity::Warning,\n        _ => protocol::Severity::Fatal,\n    }\n}\n',
  "p/🧬️mutations/🟦️.ts": 'refuse("warn", "mutation.no-op", "m");\nrefuse("warning", "mutation.no-op", "m");\nconst a = { level: "warn", code: "mutation.partial" };\nconst b = { level: "warning", code: "mutation.partial" };\n',
  "p/🧪️tests/o/🐍️.py": 'return empty(), [("warn", "mutation.clamped")]\nreturn empty(), [("warning", "mutation.clamped")]\n',
  "p/🧪️tests/o/🥒️.feature": "  answers {level: warn, code: mutation.no-op}\n  answers {level: warning, code: mutation.no-op}\n  answers {level: fatal, code: mutation.apply.<detail>}\n",
  "p/🧫️fixtures/c/🎯️outcome/🔣️.json": '{"status":"no-op","messages":[{"level":"warn","code":"mutation.no-op"},{"level":"warning","code":"mutation.no-op"}]}\n',
};
const expected = ["p/🧪️tests/a/🦀️.rs:4", "p/🧬️mutations/🟦️.ts:1", "p/🧬️mutations/🟦️.ts:3", "p/🧪️tests/o/🐍️.py:1", "p/🧪️tests/o/🥒️.feature:1", "p/🧫️fixtures/c/🎯️outcome/🔣️.json:1"];
rmSync(root, { recursive: true, force: true });
for (const [path, content] of Object.entries(files)) {
  mkdirSync(dirname(join(root, path)), { recursive: true });
  writeFileSync(join(root, path), content);
}
mkdirSync(dirname(join(root, vocabulary)), { recursive: true });
copyFileSync(join(repo, vocabulary), join(root, vocabulary));
Bun.spawnSync(["git", "init", "-q"], { cwd: root });
const breaches = policyMutationMessageCodeBreaches(root);
const reported = breaches.map((breach) => /^"([^"]*)"/.exec(breach.summary)?.[1] ?? breach.summary);
for (const breach of breaches) console.log(`[s2-codes] ${breach.summary}`);
const missing = expected.filter((position) => !reported.includes(position));
const unexpected = reported.filter((position) => !expected.includes(position));
console.log(`[s2-codes] ${breaches.length} reported, missing ${JSON.stringify(missing)}, unexpected ${JSON.stringify(unexpected)}`);
rmSync(root, { recursive: true, force: true });
process.exit(missing.length === 0 && unexpected.length === 0 && breaches.length === expected.length ? 0 : 1);
