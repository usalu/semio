/** 🗂️ Key-membership and precision law of the schema-index targets of `@semio-tech/dsl-derive-rs`: the JSON documents the payload parses (independent Python reader `schema-read-set.py` over the real tree) are all selected by the input-precision rule `dsl-schema-index`, the rule selects no Rust, TypeScript or other file, and its slack (documents under test and fixture collections below schema directories) stays below 12 %. The rule is also checked against minimatch (third-party).
 * Rule document: `KEYS5_ROOT` (a mirror) or the live checkout; tree: `KEYS5_TREE` or the live checkout.
 * @see https://github.com/isaacs/minimatch */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { minimatch } from "minimatch";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts";
const { deriveInputsV1, readInputPrecisionV1, selectRuleV1, globMatchesV1 } = await import(P);

const root = process.env.KEYS5_ROOT ?? "/Users/ueli/Documents/semio", tree = process.env.KEYS5_TREE ?? "/Users/ueli/Documents/semio";
const document = readInputPrecisionV1(JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json"), "utf8")));
const identity = (target: string) => ({ project: "@semio-tech/dsl-derive-rs", target });
const bare = (pattern: string) => pattern.replace(/^!?\{workspaceRoot\}\//u, "");
const bytes = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));
const stub = {} as never;

test("the rule selects the three schema-index targets and no other target of the project", () => {
  for (const target of ["generate-schema-index", "check-schema-index", "preview-schema-index"]) expect(selectRuleV1(document, identity(target))?.id, target).toBe("dsl-schema-index");
  for (const target of ["test-schema-index-source", "generate", "test", "test-exports"]) expect(selectRuleV1(document, identity(target))?.id ?? null, target).not.toBe("dsl-schema-index");
});

test("the documents the payload parses are all selected, nothing but JSON is, and the slack is small", async () => {
  const rule = selectRuleV1(document, identity("generate-schema-index"))!;
  const derived = await deriveInputsV1(document, rule, identity("generate-schema-index"), stub);
  expect(derived.drops).toEqual(expect.arrayContaining(["default", "^default", "nativeSources", "^nativeSources", "nativeTestSources", "^nativeTestSources"]));
  const positive = derived.patterns.filter((pattern: string) => !pattern.startsWith("!")), negative = derived.patterns.filter((pattern: string) => pattern.startsWith("!")).map((pattern: string) => bare(pattern.slice(1)));
  expect(positive.length).toBeGreaterThan(0);
  const listed = spawnSync("git", ["-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: tree, encoding: "utf8", maxBuffer: 256 * 1024 * 1024 }).stdout.split("\0").filter(Boolean);
  const selected = (matches: (pattern: string, file: string) => boolean) => listed.filter((file) => positive.some((pattern: string) => matches(bare(pattern), file)) && !negative.some((pattern: string) => matches(pattern, file))).sort(bytes);
  const own = selected((pattern, file) => globMatchesV1(pattern, file)), third = selected((pattern, file) => minimatch(file, pattern, { dot: true }));
  expect(third.length).toBe(own.length);
  expect(third).toEqual(own);
  const oracle = spawnSync("python3", ["-I", join(import.meta.dir, "schema-read-set.py"), tree], { encoding: "utf8", maxBuffer: 256 * 1024 * 1024 }).stdout.split("\n").filter(Boolean);
  const visible = new Set(listed), reads = oracle.filter((file) => visible.has(file));
  expect(reads.length).toBeGreaterThan(9000);
  const key = new Set(own);
  expect(reads.filter((file) => !key.has(file))).toEqual([]);
  expect(own.every((file) => file.endsWith(".json"))).toBe(true);
  expect(own.length).toBeLessThan(reads.length * 1.12);
  console.log(JSON.stringify({ read: reads.length, key: own.length, slack: own.length - reads.length }));
});
