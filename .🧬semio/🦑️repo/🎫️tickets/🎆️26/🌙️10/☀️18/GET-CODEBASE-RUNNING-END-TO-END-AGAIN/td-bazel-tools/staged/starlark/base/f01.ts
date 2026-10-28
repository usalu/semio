/** 🧾️ The input witness: its runner agrees with the TypeScript reference and with Python's hashlib, and real Bazel proves what it is for: a large source set enters a test as one input, an edit reruns the test exactly when content changed, and a generated test with shards and the real test protocol runs end to end. */
import { test, expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { appendFileSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { bazelStarlarkRulesV1, renderBazelBuildV1, type BazelActionV1 } from "../../../📜️script.ts";
import { BAZEL_CARGO_RULE_V1 } from "../../../🦀️cargo/🟦️.ts";
import { admitBazelTestClassesV1, BAZEL_TEST_CLASSES_V1, bazelTestPlanV1 } from "../../🟦️.ts";
import { BAZEL_WITNESS_RUNNER, BAZEL_WITNESS_STARLARK, bazelWitnessDigestV1 } from "../🟦️.ts";

const scratch = (): string => realpathSync(mkdtempSync(join(tmpdir(), "bazel-witness-")));
const names = ["plain.txt", "with space.txt", "😀 emoji/ü ö.ts", "deep/er/est/file.bin", "empty", "Ａ-fullwidth.txt"];

const fill = (root: string): { path: string; bytes: Uint8Array }[] => names.map((name, index) => {
  const bytes = name === "empty" ? new Uint8Array() : Uint8Array.from({ length: 300 + index }, (_v, at) => (at * 7 + index) % 256);
  mkdirSync(dirname(join(root, name)), { recursive: true });
  writeFileSync(join(root, name), bytes);
  return { path: name, bytes };
});

test("the runner, the TypeScript reference and Python's hashlib compute the same digest, in the order of UTF-16 code units", () => {
  const directory = scratch();
  try {
    const rows = fill(directory);
    const runner = join(directory, "witness.mjs"), parameters = join(directory, "params"), output = join(directory, "out", "w.witness");
    writeFileSync(runner, BAZEL_WITNESS_RUNNER);
    writeFileSync(parameters, [output, ...[...names].reverse().map((name) => join(directory, name))].join("\n") + "\n");
    const run = spawnSync(process.execPath, [runner, "@" + parameters], { cwd: directory, encoding: "utf8" });
    expect(run.status, run.stderr).toBe(0);
    const actual = JSON.parse(readFileSync(output, "utf8"));
    const reference = bazelWitnessDigestV1(rows.map((row) => ({ path: join(directory, row.path), bytes: row.bytes })));
    expect(actual).toEqual(reference);
    const python = spawnSync("python3", ["-I", "-c", `import hashlib,json,sys
paths=[l for l in open(sys.argv[1],encoding="utf-8").read().split("\\n") if l][1:]
outer=hashlib.sha256()
for p in sorted(paths,key=lambda s:s.encode("utf-16-be")):
    b=open(p,"rb").read()
    outer.update((p+"\\0"+str(len(b))+"\\0"+hashlib.sha256(b).hexdigest()+"\\n").encode("utf-8"))
print(json.dumps({"version":1,"files":len(paths),"sha256":outer.hexdigest()}))`, parameters], { encoding: "utf8" });
    expect(python.status, python.stderr).toBe(0);
    expect(JSON.parse(python.stdout)).toEqual(actual);
    writeFileSync(join(directory, "plain.txt"), "changed");
    const again = spawnSync(process.execPath, [runner, "@" + parameters], { cwd: directory, encoding: "utf8" });
    expect(again.status, again.stderr).toBe(0);
    expect(JSON.parse(readFileSync(output, "utf8")).sha256).not.toBe(actual.sha256);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("a missing file fails the action loudly and an empty parameter file is refused", () => {
  const directory = scratch();
  try {
    const runner = join(directory, "witness.mjs"), parameters = join(directory, "params");
    writeFileSync(runner, BAZEL_WITNESS_RUNNER);
    writeFileSync(parameters, join(directory, "out") + "\n" + join(directory, "absent") + "\n");
    expect(spawnSync(process.execPath, [runner, "@" + parameters], { encoding: "utf8" }).status).not.toBe(0);
    writeFileSync(parameters, "");
    expect(spawnSync(process.execPath, [runner, "@" + parameters], { encoding: "utf8" }).status).not.toBe(0);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the rendered BUILD uses the witness for a witnessed test, the sources for a small one, and shares one witness per source group", () => {
  const plan = (over: Partial<ReturnType<typeof bazelTestPlanV1>>) => ({ ...bazelTestPlanV1({ project: "p", target: "test", files: 1 }), ...over });
  const action = (index: number, test: ReturnType<typeof plan>, sources: string): BazelActionV1 & { sources: string } => ({ id: `p:test${index}`, label: `t_${index.toString(16).padStart(16, "0")}`, kind: "test", restore: [], cache: { cache: true }, test, sources });
  const build = renderBazelBuildV1({ actions: [action(1, plan({ witness: true }), "d_big"), action(2, plan({ witness: true }), "d_big"), action(3, plan({}), "d_small")], groups: new Map([["d_big", { groups: [], files: ["a.ts"] }], ["d_small", { groups: [], files: ["b.ts"] }]]), bun: "/opt/bun", sourceIdentity: "a".repeat(64), extension: "" });
  expect(build.split("\n")[0]).toContain('"repo_inputs_witness"');
  expect(build.match(/^repo_inputs_witness\(/gmu)?.length).toBe(1);
  expect(build).toContain('name = "w_d_big"');
  expect(build.match(/witnesses = \[\n {8}":w_d_big",/gu)?.length).toBe(2);
  expect(build.match(/\n {4}srcs = \[\n {8}":d_small",/gu)?.length).toBe(1);
  expect(bazelStarlarkRulesV1()).toContain("repo_inputs_witness = rule(");
  expect(bazelStarlarkRulesV1()).toContain('"witnesses": attr.label_list()');
  expect(BAZEL_WITNESS_STARLARK).toContain("no-sandbox");
  const python = spawnSync("python3", ["-I", "-c", "import ast,sys;ast.parse(sys.stdin.read())"], { input: build.replace(/^load\(.*$/mu, ""), encoding: "utf8" });
  expect(python.status, python.stderr).toBe(0);
  expect(() => renderBazelBuildV1({ actions: [{ ...action(4, plan({}), "d_small"), test: undefined }], groups: new Map(), bun: "/opt/bun", sourceIdentity: "a".repeat(64), extension: "" })).toThrow("requires its test class");
});

const binary = process.env.SEMIO_TEST_BAZEL_BINARY, real = binary ? test : test.skip;
real("real Bazel: a witnessed test is cached on content, reruns on an edit, shares its witness, and shards with the real protocol end to end", () => {
  const directory = scratch(), workspace = join(directory, "ws"), checkout = join(directory, "checkout"), output = join(directory, "ob"), disk = join(directory, "disk"), log = join(directory, "runs.log");
  mkdirSync(workspace);
  mkdirSync(checkout);
  mkdirSync(disk);
  writeFileSync(log, "");
  const modules = join(import.meta.dir, "..", "..");
  const stub = `import { appendFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { prepareTestCommandV1 } from ${JSON.stringify(join(modules, "..", "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🏃️process", "🧪️testing", "🚦️protocol", "🟦️.ts"))};
import { bazelTestEnvironmentV1, settleBazelTestV1 } from ${JSON.stringify(join(modules, "🟦️.ts"))};
const target = process.env.TEST_TARGET ?? "?", started = performance.now();
appendFileSync(${JSON.stringify(log)}, target + " " + (process.env.TEST_SHARD_INDEX ?? "-") + "\\n");
const environment = { ...process.env, ...bazelTestEnvironmentV1(process.env) };
let failed = false;
if (target === "//:t_0000000000000003") {
  const prepared = prepareTestCommandV1(process.execPath, ["test", ...${JSON.stringify(["alpha", "beta", "gamma", "delta"].map((name) => join(checkout, `${name}.test.ts`)))}], environment, process.env.TEST_TMPDIR!);
  const run = spawnSync(process.execPath, prepared.args as string[], { env: { ...environment, ...prepared.environment }, encoding: "utf8" });
  prepared.finish({ code: run.status, signal: run.signal, durationMs: performance.now() - started });
  failed = run.status !== 0;
}
settleBazelTestV1(process.env, { label: target, durationMs: performance.now() - started });
process.exit(failed ? 1 : 0);
`;
  try {
    const big = Array.from({ length: 2600 }, (_v, index) => `big/${String(index % 40).padStart(2, "0")}/${index}.txt`), small = ["small/a.txt", "small/b.txt"], tests = ["alpha", "beta", "gamma", "delta"].map((name) => `${name}.test.ts`);
    for (const path of [...big, ...small]) { mkdirSync(dirname(join(checkout, path)), { recursive: true }); writeFileSync(join(checkout, path), "content " + path); }
    big.push("😀/emoji file.txt");
    mkdirSync(join(checkout, "😀"));
    writeFileSync(join(checkout, "😀/emoji file.txt"), "emoji");
    for (const name of tests) writeFileSync(join(checkout, name), `import { test, expect } from "bun:test";\ntest("${name} one", () => expect(1).toBe(1));\ntest("${name} two", () => expect(1).toBe(1));\n`);
    symlinkSync(checkout, join(workspace, "s"));
    writeFileSync(join(workspace, "MODULE.bazel"), 'module(name = "semio_tasks", version = "0.0.0")\n');
    writeFileSync(join(workspace, "tasks.bzl"), bazelStarlarkRulesV1() + "\n" + BAZEL_CARGO_RULE_V1);
    writeFileSync(join(workspace, "witness.mjs"), BAZEL_WITNESS_RUNNER);
    writeFileSync(join(workspace, "implementation.json"), "{}");
    writeFileSync(join(workspace, "stub-test.ts"), stub);
    writeFileSync(join(workspace, "📜️script.ts"), `import { writeFileSync, chmodSync } from "node:fs";\nif (process.argv[4] === "birth-launcher") { writeFileSync(process.argv[6], '#!/bin/sh\\nexec ' + JSON.stringify(process.execPath) + ' ' + JSON.stringify(${JSON.stringify(join(workspace, "stub-test.ts"))}) + '\\n'); chmodSync(process.argv[6], 0o755); } else process.exit(9);\n`);
    const plan = (files: number, shards: number) => bazelTestPlanV1({ project: "p", target: "test", files }, { version: 1, sizes: { small: { timeout: "short", seconds: 60 }, medium: { timeout: "moderate", seconds: 300 }, large: { timeout: "long", seconds: 900 }, enormous: { timeout: "eternal", seconds: 3600 } }, levels: { fundamental: "small", quick: "medium", long: "large", exhaustive: "enormous" }, witness: { from: 2000 }, measure: { headroom: 3, minimumRuns: 1, shardAboveSeconds: 300 }, default: { size: "enormous", shards: 1, flaky: false }, messages: BAZEL_TEST_CLASSES_V1.messages, rules: shards > 1 ? [{ id: "shards", evidence: "declared", reason: "four fixture test files", targets: ["test"], shards }] : [] });
    const action = (index: number, test: ReturnType<typeof plan>, sources: string): BazelActionV1 & { sources: string } => ({ id: `p:test${index}`, label: `t_${index.toString(16).padStart(16, "0")}`, kind: "test", restore: [], cache: { cache: true }, test, sources });
    const label = (index: number) => `//:t_${index.toString(16).padStart(16, "0")}`;
    writeFileSync(join(workspace, "BUILD.bazel"), renderBazelBuildV1({ actions: [action(1, plan(2603, 1), "d_big"), action(2, plan(2, 1), "d_small"), action(3, plan(4, 4), "d_tests"), action(4, plan(2603, 1), "d_big")], groups: new Map([["d_big", { groups: [], files: big }], ["d_small", { groups: [], files: small }], ["d_tests", { groups: [], files: tests }]]), bun: process.execPath, sourceIdentity: "a".repeat(64), extension: "" }));
    for (const index of [1, 2, 3, 4]) { writeFileSync(join(workspace, `t_${index.toString(16).padStart(16, "0")}.json`), JSON.stringify({ task: { id: `p:test${index}`, project: "p", target: `test-${index}` } })); writeFileSync(join(workspace, `t_${index.toString(16).padStart(16, "0")}.input`), "x"); }
    const bazel = (verb: string, args: string[]) => spawnSync(binary!, [`--output_base=${output}`, "--nosystem_rc", "--nohome_rc", "--max_idle_secs=30", verb, "--lockfile_mode=off", "--incompatible_autoload_externally=", "--incompatible_strict_action_env", `--disk_cache=${disk}`, ...args], { cwd: workspace, encoding: "utf8", env: { ...process.env, LANG: "en_US.UTF-8", LC_ALL: "en_US.UTF-8" } });
    const runs = () => readFileSync(log, "utf8").trim().split("\n").filter(Boolean);
    const flags = ["--build_tests_only", "--test_output=errors", "--cache_test_results=auto"];
    const first = bazel("test", [...flags, `--build_event_json_file=${join(directory, "bep-1.json")}`, label(1), label(2), label(3), label(4)]);
    expect(first.status, first.stdout + first.stderr).toBe(0);
    expect(runs().filter((line) => line.startsWith(label(3))).sort()).toEqual([0, 1, 2, 3].map((shard) => `${label(3)} ${shard}`));
    expect(runs().length).toBe(7);
    const info = bazel("info", ["bazel-testlogs"]).stdout.trim().split("\n").at(-1)!;
    const names = (xml: string) => [...xml.matchAll(/<testcase\b[^>]*?\sname="([^"]*)"/gu)].map((match) => match[1]!).sort();
    const seen = [1, 2, 3, 4].map((shard) => names(readFileSync(join(info, `t_0000000000000003/shard_${shard}_of_4/test.xml`), "utf8")));
    expect(seen.flat().sort()).toEqual(tests.flatMap((name) => [`${name} one`, `${name} two`]).sort());
    expect(seen.every((row) => row.length === 2)).toBe(true);
    expect(readFileSync(join(info, "t_0000000000000001/test.xml"), "utf8")).toContain("<testsuite");
    const second = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(second.status).toBe(0);
    expect(/\(cached\) PASSED/u.test(second.stdout + second.stderr)).toBe(true);
    expect(runs().length).toBe(7);
    const witnesses = bazel("cquery", ["--output=label", 'kind("repo_inputs_witness", //:all)']).stdout.trim().split("\n").map((row) => row.split(" ")[0]);
    expect(witnesses).toEqual(["//:w_d_big"]);
    writeFileSync(join(checkout, "small/a.txt"), "edited small");
    const smallRun = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(smallRun.status).toBe(0);
    expect(runs().length).toBe(8);
    expect(runs().at(-1)).toBe(`${label(2)} -`);
    writeFileSync(join(checkout, "big/05/5.txt"), "edited big");
    const edited = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(edited.status).toBe(0);
    expect(runs().length).toBe(10);
    expect(new Set(runs().slice(8))).toEqual(new Set([`${label(1)} -`, `${label(4)} -`]));
    const touched = join(checkout, "big/06/6.txt");
    writeFileSync(touched, readFileSync(touched));
    const same = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(same.status).toBe(0);
    expect(runs().length).toBe(10);
    writeFileSync(join(checkout, "😀/emoji file.txt"), "emoji edited");
    const emoji = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(emoji.status, emoji.stdout + emoji.stderr).toBe(0);
    expect(runs().length).toBe(12);
    bazel("clean", []);
    const restored = bazel("test", [...flags, label(1), label(2), label(3), label(4)]);
    expect(restored.status, restored.stdout + restored.stderr).toBe(0);
    expect(runs().length).toBe(12);
    const forced = bazel("test", [...flags.filter((flag) => !flag.startsWith("--cache_test")), "--nocache_test_results", `--build_event_json_file=${join(directory, "bep-2.json")}`, label(1), label(2), label(3), label(4)]);
    expect(forced.status, forced.stdout + forced.stderr).toBe(0);
    const tablePath = join(directory, "classes.json");
    writeFileSync(tablePath, JSON.stringify(BAZEL_TEST_CLASSES_V1));
    const measured = spawnSync("python3", ["-I", join(import.meta.dir, "..", "..", "📐️measure", "🐍️.py"), "--table", tablePath, "--workspace", workspace, "--minimum-runs", "2", "--shardable", "p:test3", join(directory, "bep-1.json"), join(directory, "bep-2.json")], { encoding: "utf8" });
    expect(measured.status, measured.stderr).toBe(0);
    const report = JSON.parse(measured.stdout) as { rows: { label: string; runs: number; p95Seconds: number; size: string; id: string }[]; proposal: { id: string; targets: string[]; projects: string[]; size: string; evidence: string }[] };
    expect(report.rows.map((row) => row.id).sort()).toEqual(["p:test1", "p:test2", "p:test3", "p:test4"]);
    expect(report.rows.every((row) => row.runs === 2 && row.size === "small" && row.p95Seconds > 0)).toBe(true);
    expect(report.proposal.map((rule) => rule.targets[0]).sort()).toEqual(["test-1", "test-2", "test-3", "test-4"]);
    const admitted = admitBazelTestClassesV1({ ...BAZEL_TEST_CLASSES_V1, rules: report.proposal });
    expect(bazelTestPlanV1({ project: "p", target: "test-3", files: 1 }, admitted).size).toBe("small");
    expect(bazelTestPlanV1({ project: "other", target: "test-3", files: 1 }, admitted).size).toBe("enormous");
  } finally {
    spawnSync(binary!, [`--output_base=${output}`, "--nosystem_rc", "--nohome_rc", "shutdown"], { cwd: workspace });
    spawnSync("chmod", ["-R", "u+w", directory]);
    rmSync(directory, { recursive: true, force: true });
  }
}, 600000);
