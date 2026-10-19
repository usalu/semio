import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";

const product = resolve(process.cwd(), "🧰️framework/🛍️products/📚️course");
const read = (path: string) => readFileSync(resolve(product, path), "utf8");
const { quizIssues } = await import(pathToFileURL(resolve(product, "🔨️modules/✅️validation/🟦️.ts")).href);
const quiz = JSON.parse(read("🧫️fixtures/🧺️balanced-pools/🔣️.json"));
const vectors = JSON.parse(read("🧫️fixtures/🧺️balanced-pools/🩺️validation.json"));
const schema = JSON.parse(read("🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false });
ajv.addSchema(schema);
for (const vector of vectors) {
  const document = structuredClone(quiz);
  document.tasks[0].pool.seconds = vector.seconds;
  assert.deepEqual(quizIssues(document), vector.issues);
  assert.equal(ajv.validate(schema.$id + "#/$defs/QuestionPool", document.tasks[0].pool), vector.issues.length === 0);
  assert.deepEqual((ajv.errors ?? []).map(({ instancePath, keyword }) => ({ instancePath, keyword })), vector.keyword === null ? [] : [{ instancePath: "/seconds", keyword: vector.keyword }]);
  console.log(`[DEBUG] pooled timing ${vector.seconds}s: owned TypeScript + independent AJV PASS (${vector.issues[0]?.code ?? "valid"})`);
}
const rust = read("🔨️modules/✅️validation/🦀️.rs");
assert.match(rust, /if pool\.seconds == 0\s*\{[^}]*\{base\}\/pool\/seconds[^;]*IssueCode::BelowMinimum\);\s*\}/u);
console.log("[DEBUG] Rust zero-duration source branch agrees with below-minimum; compiled Rust unit gate remains pending");
