/** 🚥️ Bazel test classes: the fixture is the contract, Ajv closes the table, a Python implementation (fnmatch globs) and Bazel's own binary are the oracles. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { admitBazelTestClassesV1, BAZEL_TEST_CLASSES_V1, bazelTestAttributesV1, bazelTestEnvironmentV1, bazelTestFallbackReportV1, bazelTestPlanV1, bazelTestWitnessV1, settleBazelTestV1, type BazelTestPlanV1, type BazelTestTableV1 } from "../🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import table from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const ajv = new Ajv({ strict: true, allowUnionTypes: true });
const compile = (definition: string) => ajv.compile({ $schema: schema.$schema, definitions: schema.definitions, $ref: `#/definitions/${definition}` });
const tables = { ...fixture.tables, authored: table } as unknown as Record<string, BazelTestTableV1>;

test("the schema closes the authored table and the fixture tables, and refuses every malformed one", () => {
  expect(ajv.validateSchema(schema)).toBe(true);
  const valid = compile("Table");
  expect(valid(table), JSON.stringify(valid.errors)).toBe(true);
  for (const [name, value] of Object.entries(fixture.tables)) expect(valid(value), name).toBe(true);
  for (const row of fixture.refusedTables) {
    const broken = { ...fixture.tables.rules, ...row.patch };
    expect(valid(broken), row.name).toBe(false);
    expect(() => admitBazelTestClassesV1(broken), row.name).toThrow("Closed Bazel test class table required");
  }
  const plan = compile("Plan");
  for (const row of fixture.plans) expect(plan(row.expected), row.name).toBe(true);
});

test("the authored table keeps the longest class, shards nothing and witnesses above 2000 files", () => {
  expect(BAZEL_TEST_CLASSES_V1.default).toEqual({ size: "enormous", shards: 1, flaky: false });
  expect(BAZEL_TEST_CLASSES_V1.witness.from).toBe(2000);
  expect(BAZEL_TEST_CLASSES_V1.rules.every((rule) => rule.evidence === "measured" || rule.evidence === "declared")).toBe(true);
  expect(bazelTestWitnessV1(2000)).toBe(false);
  expect(bazelTestWitnessV1(2001)).toBe(true);
  expect(() => bazelTestWitnessV1(-1)).toThrow();
  expect(() => bazelTestWitnessV1(1.5)).toThrow();
});

test("every neutral test level maps to a size whose timeout class covers the level's wall-clock budget", () => {
  const seconds = BAZEL_TEST_CLASSES_V1.sizes;
  for (const [level, size] of Object.entries(BAZEL_TEST_CLASSES_V1.levels)) expect(seconds[size].seconds * 1000, level).toBeGreaterThanOrEqual(TEST_LEVEL_BUDGET_MS[level as keyof typeof TEST_LEVEL_BUDGET_MS]);
  const order = ["short", "moderate", "long", "eternal"];
  const sizes = Object.values(BAZEL_TEST_CLASSES_V1.sizes);
  expect(sizes.map((row) => row.timeout as string)).toEqual(order);
  expect(sizes.map((row) => row.seconds)).toEqual([60, 300, 900, 3600]);
});

test("every fixture row has exactly its authored answer", () => {
  for (const row of fixture.plans) expect(bazelTestPlanV1(row.input, tables[row.table]), row.name).toEqual(row.expected as BazelTestPlanV1);
  for (const row of fixture.attributes) expect(bazelTestAttributesV1(row.plan as BazelTestPlanV1), row.name).toEqual(row.expected as never);
  for (const row of fixture.environments) expect(bazelTestEnvironmentV1(row.bazel), row.name).toEqual(row.expected as never);
  for (const row of fixture.refusedEnvironments) expect(() => bazelTestEnvironmentV1(row), JSON.stringify(row)).toThrow();
  expect(() => bazelTestPlanV1({ project: "", target: "t", files: 1 })).toThrow("Closed Bazel test input required");
});

test("an independent Python implementation agrees on every section", () => {
  const run = spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], { input: JSON.stringify({ authored: table, fixture }), encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  const answers = JSON.parse(run.stdout);
  fixture.plans.forEach((row, index) => expect(answers.plans[index], row.name).toEqual(row.expected));
  fixture.attributes.forEach((row, index) => expect(answers.attributes[index], row.name).toEqual(row.expected));
  fixture.environments.forEach((row, index) => expect(answers.environments[index], row.name).toEqual(row.expected));
  answers.refusedEnvironments.forEach((answer: unknown, index: number) => expect(answer, JSON.stringify(fixture.refusedEnvironments[index])).toBe("refused"));
});

const scratch = (): string => realpathSync(mkdtempSync(join(tmpdir(), "bazel-test-classes-")));

test("settling a finished test advertises an unhonoured shard with a warning and writes one fallback result", () => {
  const directory = scratch();
  try {
    const status = join(directory, "status"), xml = join(directory, "out", "test.xml");
    const written: string[] = [];
    const stderr = process.stderr.write.bind(process.stderr);
    process.stderr.write = ((chunk: string | Uint8Array) => { written.push(String(chunk)); return true; }) as typeof process.stderr.write;
    try { settleBazelTestV1({ TEST_SHARD_STATUS_FILE: status, TEST_SHARD_INDEX: "1", TEST_TOTAL_SHARDS: "3", XML_OUTPUT_FILE: xml }, { label: "p:test", durationMs: 1500, failure: "boom <&>" }); } finally { process.stderr.write = stderr as typeof process.stderr.write; }
    expect(written.join("")).toContain("No command of p:test honoured shard 2/3");
    expect(written.join("")).toContain("Kein Befehl von p:test hat den Shard 2/3 beachtet");
    expect(existsSync(status)).toBe(true);
    const text = readFileSync(xml, "utf8");
    expect(text).toContain('failures="1"');
    expect(text).toContain('message="boom &lt;&amp;&gt;"');
    expect(text).toBe(bazelTestFallbackReportV1({ label: "p:test", durationMs: 1500, failure: "boom <&>" }));
    writeFileSync(xml, "kept");
    settleBazelTestV1({ TEST_SHARD_STATUS_FILE: status, XML_OUTPUT_FILE: xml }, { label: "p:test", durationMs: 1 });
    expect(readFileSync(xml, "utf8")).toBe("kept");
    const python = spawnSync("python3", ["-I", "-c", "import sys,xml.etree.ElementTree as E;r=E.fromstring(sys.argv[1]);print(r.get('tests'),r.get('failures'))", bazelTestFallbackReportV1({ label: "p:test", durationMs: 5, failure: "x" })], { encoding: "utf8" });
    expect(python.stdout.trim()).toBe("1 1");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

const binary = process.env.SEMIO_TEST_BAZEL_BINARY, real = binary ? test : test.skip;
real("Bazel 8 accepts every attribute of every planned class, runs each shard once and keeps the retry attribute", () => {
  const directory = scratch(), output = join(directory, "ob");
  const startup = [`--output_base=${output}`, "--nosystem_rc", "--nohome_rc", "--max_idle_secs=15"];
  try {
    writeFileSync(join(directory, "MODULE.bazel"), 'module(name = "classes", version = "0.0.0")\n');
    writeFileSync(join(directory, "rules.bzl"), `def _test(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.write(launcher, "#!/bin/sh\\n[ -n \\"$TEST_SHARD_STATUS_FILE\\" ] && touch \\"$TEST_SHARD_STATUS_FILE\\"\\necho shard=$TEST_SHARD_INDEX/$TEST_TOTAL_SHARDS\\n", is_executable = True)
    return [DefaultInfo(executable = launcher)]
repo_test = rule(implementation = _test, test = True)
`);
    const rows = fixture.plans.map((row, index) => ({ name: `t_${index}`, plan: bazelTestPlanV1(row.input, tables[row.table]) }));
    const lines = rows.map(({ name, plan }) => `repo_test(name = "${name}", ${bazelTestAttributesV1(plan).map(([key, value]) => `${key} = ${value}`).join(", ")})\n`);
    writeFileSync(join(directory, "BUILD.bazel"), 'load(":rules.bzl", "repo_test")\n' + lines.join(""));
    const bazel = (verb: string, args: string[]) => spawnSync(binary!, [...startup, verb, "--lockfile_mode=off", "--incompatible_autoload_externally=", ...args], { cwd: directory, encoding: "utf8", env: { ...process.env, LANG: "en_US.UTF-8", LC_ALL: "en_US.UTF-8" } });
    const query = bazel("query", ["--output=build", "//:all"]);
    expect(query.status, query.stderr).toBe(0);
    for (const { name, plan } of rows) {
      const block = query.stdout.split("repo_test(").find((part) => part.includes(`name = "${name}"`))!;
      expect(block, name).toContain(`size = "${plan.size}"`);
      expect(block, name).toContain(`timeout = "${plan.timeout}"`);
      expect(block.includes("shard_count = " + plan.shards), name).toBe(plan.shards > 1);
      expect(block.includes("flaky = True"), name).toBe(plan.flaky);
    }
    const sharded = rows.filter(({ plan }) => plan.shards > 1);
    expect(sharded.length).toBeGreaterThan(0);
    const run = bazel("test", ["--test_output=all", ...sharded.map(({ name }) => `//:${name}`)]);
    expect(run.status, run.stdout + run.stderr).toBe(0);
    for (const { name, plan } of sharded) for (let shard = 0; shard < plan.shards; shard++) expect(run.stdout + run.stderr, `${name} ${shard}`).toContain(`shard=${shard}/${plan.shards}`);
  } finally {
    spawnSync(binary!, [...startup, "shutdown"], { cwd: directory });
    spawnSync("chmod", ["-R", "u+w", directory]);
    rmSync(directory, { recursive: true, force: true });
  }
}, 300000);
