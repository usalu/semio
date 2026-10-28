/** 🧪️ Tests for inputs read-oracle. */
import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync, mkdtempSync, readdirSync, realpathSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {backendOrderV1, readOracleRequestV1, readOracleTextV1, runReadOracleV1, windowsPlanV1, type ReadOracleBackendV1} from "../🟦️.ts";
import {readOracleArgumentsV1} from "../🚪️cli/🟦️.ts";
import {landlockGrantsV1, linuxAvailableV1, straceReadsV1, straceStringV1} from "../🐧️linux/🟦️.ts";
import {buildMirrorV1, missingInMirrorV1, rebaseV1, type MirrorFileSystemV1} from "../🪞️mirror/🟦️.ts";
import {captureV1} from "../🏃️run/🟦️.ts";
import {namedFilesV1, planGrantsV1, planV1, splitPlanV1, type ReadPlanV1} from "../🪢️policy/🟦️.ts";
import {awaitLogFlushV1, seatbeltAvailableV1, seatbeltDenialsV1, seatbeltProfileV1, seatbeltRefusalV1} from "../🍎️seatbelt/🟦️.ts";
import document from "../🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type: "json"};

const tree = corpus.tree as Record<string, string>, files = Object.keys(tree).sort();
const directoriesOf = (paths: readonly string[]) => new Set(paths.flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
const listerOf = (paths: readonly string[]) => {
  const directories = directoriesOf(paths);
  return (directory: string) => {
    if (directory !== "" && !directories.has(directory)) return undefined;
    const prefix = directory ? `${directory}/` : "";
    return [...new Set(paths.filter((path) => path.startsWith(prefix)).map((path) => path.slice(prefix.length).split("/")[0]!))];
  };
};
const directories = directoriesOf(files), lister = listerOf(files);
const ROOT = "/w/ws", here = import.meta.dirname, oracle = join(here, "🐍️oracle/🐍️.py");

type Definition = {allowed: string[]; ambient: string[]; names: string[]};
const planOf = (row: Definition & {tree?: string[]; refusals?: boolean}) => planV1(ROOT, row, listerOf(row.tree ?? files), {refusals: row.refusals ?? true});
const linuxFixture = (row: (typeof corpus.linuxGrants)[number]) => ({list: (absolute: string) => (row.tree as Record<string, string[]>)[absolute], isDirectory: (absolute: string) => row.directories.includes(absolute), real: (absolute: string) => absolute});

let seed = 20261026;
const random = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
const pick = <T>(values: readonly T[]): T => values[Math.floor(random() * values.length)]!;
const WORDS = ["🧪", "📦", "a", "b c", 'q"x', "p.json", "back\\slash"];

/** The random workspaces of the laws: a real directory per case, so the Python evaluator and the macOS kernel see the same files as the cover. */
const base = realpathSync(mkdtempSync(join(tmpdir(), "ro-law-")));
const laws = Array.from({length: 40}, (_, index) => {
  const root = join(base, `case-${index}`), made: string[] = [];
  const grow = (directory: string, depth: number) => {
    const width = 2 + Math.floor(random() * 5);
    for (let at = 0; at < width; at++) {
      const name = `${pick(WORDS)}${at}`, path = directory ? `${directory}/${name}` : name;
      if (depth < 3 && random() < 0.45) grow(path, depth + 1);
      else { const file = name.startsWith("p.json") ? `${directory ? `${directory}/` : ""}p.json` : path; if (!made.includes(file)) { mkdirSync(join(root, dirname(file)), {recursive: true}); writeFileSync(join(root, file), file); made.push(file); } }
    }
  };
  mkdirSync(root, {recursive: true});
  grow("", 0);
  const share = [0.05, 0.5, 0.9, 1][index % 4]!, top = [...new Set(made.map((path) => path.split("/")[0]!))];
  const allowed = made.filter(() => random() < share).sort(), ambient = index % 5 === 0 ? [pick(top)] : [], names = index % 3 === 0 ? ["p.json"] : [];
  return {root, files: made.slice().sort(), allowed: [...allowed, "gone/none.txt"], ambient, names};
});
const expectedOf = (row: {files: string[]; allowed: string[]; ambient: string[]; names: string[]}) => {
  const trees = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache", ...row.ambient];
  return row.files.filter((path) => row.allowed.includes(path) || trees.some((t) => path === t || path.startsWith(`${t}/`)) || row.names.includes(path.slice(path.lastIndexOf("/") + 1))).sort();
};
const realLister = (root: string) => (directory: string) => { try { return readdirSync(directory ? join(root, directory) : root); } catch { return undefined; } };
const lawPlans = laws.map((row) => planV1(row.root, row, realLister(row.root), {refusals: true, alternatives: true}));
const capacityOf = (plan: ReadPlanV1) => Math.max(3, Math.ceil(plan.rules.length / 3));
const lawRuns = lawPlans.map((plan) => splitPlanV1(plan, capacityOf(plan)));

const tiny = Array.from({length: 60}, (_, index) => {
  let paths: string[] = [];
  for (let attempt = 0; attempt < 50; attempt++) {
    paths = [];
    const top = 1 + Math.floor(random() * 3);
    for (let at = 0; at < top; at++) {
      const name = `n${at}`;
      if (random() < 0.5) paths.push(name);
      else for (let leaf = 0; leaf < 1 + Math.floor(random() * 2); leaf++) paths.push(`${name}/${random() < 0.2 ? `d${leaf}/` : ""}f${leaf}`);
    }
    if (new Set([...directoriesOf(paths), ...paths]).size + 1 <= 8) break;
  }
  return {files: paths.sort(), allowed: paths.filter(() => random() < 0.5), ambient: [] as string[], names: random() < 0.2 ? ["f0"] : [], refusals: index % 4 !== 0};
});

const probe = {
  root: ROOT,
  policy: Object.fromEntries(corpus.policy.map((row) => [row.id, {profile: seatbeltProfileV1(planOf(row)), rules: planOf(row).rules}])),
  linuxGrants: Object.fromEntries(corpus.linuxGrants.map((row) => [row.id, landlockGrantsV1({...planV1(row.root, row, (directory) => (row.tree as Record<string, string[]>)[directory ? join(row.root, directory) : row.root], {refusals: false}), ambient: row.ambient}, linuxFixture(row))])),
  random: laws.map((row, index) => ({root: row.root, files: row.files, allowed: row.allowed, ambient: row.ambient, names: row.names, profile: seatbeltProfileV1(lawPlans[index]!), runProfiles: (lawRuns[index] ?? []).map(seatbeltProfileV1)})),
  brute: tiny.map((row) => ({files: row.files, allowed: row.allowed, ambient: row.ambient, names: row.names, refusals: row.refusals})),
};
let memo: ReturnType<typeof JSON.parse> | undefined;
const python = () => {
  if (memo) return memo;
  const scratch = mkdtempSync(join(tmpdir(), "ro-oracle-")), probePath = join(scratch, "probe.json");
  try {
    writeFileSync(probePath, JSON.stringify(probe));
    const done = spawnSync(testPythonV1() ?? "python3", ["-I", oracle, join(here, "🔣️.json"), probePath], {encoding: "utf8"});
    expect(done.status, done.stderr).toBe(0);
    return (memo = JSON.parse(done.stdout));
  } finally {
    rmSync(scratch, {recursive: true, force: true});
  }
};

/** 🔭️ The corpus, the document of texts and a request satisfy their strict schemas; a request outside the schema is refused. */
test("the corpus, the document and the request satisfy their strict schemas", () => {
  const ajv = new Ajv({strict: true, allErrors: true});
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, document), JSON.stringify(ajv.errors)).toBe(true);
  const request = {version: 1, language: "en", root: "/w/ws", cwd: "", allowed: ["a.txt"], ambient: [], names: [], command: ["true"], environment: {}, rounds: 1, timeoutMs: 1000, backend: "auto", compare: "output"};
  expect(readOracleRequestV1(request)).toEqual(request as never);
  for (const bad of [{...request, language: "fr"}, {...request, rounds: 0}, {...request, command: []}, {...request, backend: "bwrap"}, {...request, extra: 1}, {...request, allowed: ["../x"]}]) expect(() => readOracleRequestV1(bad), JSON.stringify(bad)).toThrow(/refused/u);
  expect(new Set(corpus.runs.map((row) => row.id)).size).toBe(corpus.runs.length);
});

test("the cover grants big trees and refuses their exceptions, and the plan grants what the policy says", () => {
  const evaluated = python().policy as {id: string; direct: string[]; seatbelt: string[]; rules: string[]}[];
  corpus.policy.forEach((row, index) => {
    const plan = planOf(row), expected = row.readable.slice().sort(), tree = row.tree.slice().sort();
    expect(plan.rules, row.id).toEqual(row.rules as never);
    expect(tree.filter((path) => planGrantsV1(plan, path)), row.id).toEqual(expected);
    expect(evaluated[index]!.direct, `${row.id} python direct`).toEqual(expected);
    expect(evaluated[index]!.seatbelt, `${row.id} python reads the rendered profile`).toEqual(expected);
    expect(evaluated[index]!.rules, `${row.id} python rules`).toEqual(expected);
    if (!row.refusals) expect(plan.rules.every((rule) => rule.effect === "allow"), `${row.id} allows only`).toBe(true);
  });
  expect(namedFilesV1(["package.json"], ["node_modules"], lister, (path) => directories.has(path))).toEqual(["pkg/package.json"]);
});

/** 🧮️ Law of the cover: for random workspaces and random declared sets, the rendered profile reads EXACTLY the declared files (plus the ambient trees and names), as the TypeScript grant function, an independent Python evaluator of the rendered text (rule order, last match) and the macOS kernel all say; the cover is never larger than one rule per declared file, a plan that may only allow never refuses, and its size equals the smallest rule count an exhaustive search finds. */
test("the rendered profile reads exactly the declared files, as TypeScript, the Python evaluator of the text and the kernel say", () => {
  const evaluated = python().random as {direct: string[]; seatbelt: string[]; runs: string[][]}[];
  laws.forEach((row, index) => {
    const plan = lawPlans[index]!, expected = expectedOf(row), declared = row.allowed.filter((path) => row.files.includes(path)).length;
    expect(row.files.filter((path) => planGrantsV1(plan, path)), `case ${index} typescript`).toEqual(expected);
    expect(evaluated[index]!.direct, `case ${index} python direct`).toEqual(expected);
    expect(evaluated[index]!.seatbelt, `case ${index} python reads the profile`).toEqual(expected);
    expect(plan.rules.length, `case ${index} size`).toBeLessThanOrEqual(Math.max(declared, 0));
    const allowOnly = planV1(row.root, row, realLister(row.root), {refusals: false});
    expect(allowOnly.rules.every((rule) => rule.effect === "allow"), `case ${index} allow only`).toBe(true);
    expect(row.files.filter((path) => planGrantsV1(allowOnly, path)), `case ${index} allow only reads`).toEqual(expected);
    expect(allowOnly.rules.length, `case ${index} allow only is not cheaper`).toBeGreaterThanOrEqual(plan.rules.length);
  });
  expect(laws.some((row, index) => lawPlans[index]!.rules.some((rule) => rule.effect === "deny")), "some case needs refusals").toBe(true);
  expect(laws.some((row, index) => lawPlans[index]!.rules.some((rule) => rule.effect === "allow" && rule.kind === "subpath")), "some case grants a tree").toBe(true);
});

test("the cover is as small as an exhaustive search over all labellings finds on tiny trees", () => {
  const smallest = python().brute as number[];
  tiny.forEach((row, index) => {
    const plan = planV1(ROOT, row, listerOf(row.files), {refusals: row.refusals}), expected = row.files.filter((path) => row.allowed.includes(path) || row.names.includes(path.slice(path.lastIndexOf("/") + 1)));
    expect(plan.rules.length, `tiny ${index} ${JSON.stringify(row)}`).toBe(smallest[index]!);
    expect(row.files.filter((path) => planGrantsV1(plan, path)).sort(), `tiny ${index} reads`).toEqual(expected.sort());
  });
  expect(tiny.some((row, index) => planV1(ROOT, row, listerOf(row.files), {refusals: row.refusals}).rules.some((rule) => rule.effect === "deny"))).toBe(true);
});

/** ✂️ Law of the split: every run of a split plan holds at most `capacity` rules, reads every declared file, and the runs together refuse every undeclared file (each refusal is in a run; a file is refused in the run that holds the refusal that decides it), as TypeScript and the Python evaluator of each rendered run say. */
test("a split plan reads every declared file in every run and refuses every undeclared file in at least one", () => {
  const evaluated = python().random as {runs: string[][]}[];
  let split = 0;
  laws.forEach((row, index) => {
    const runs = lawRuns[index], expected = expectedOf(row), capacity = capacityOf(lawPlans[index]!);
    if (!runs) return;
    split += runs.length > 1 ? 1 : 0;
    runs.forEach((run, at) => {
      expect(run.rules.length, `case ${index} run ${at} size`).toBeLessThanOrEqual(capacity);
      const reads = row.files.filter((path) => planGrantsV1(run, path));
      expect(reads.filter((path) => expected.includes(path)), `case ${index} run ${at} reads the declared files`).toEqual(expected);
      expect(evaluated[index]!.runs[at], `case ${index} run ${at} python`).toEqual(reads);
    });
    for (const path of row.files.filter((file) => !expected.includes(file))) expect(runs.some((run) => !planGrantsV1(run, path)), `case ${index} ${path} is refused somewhere`).toBe(true);
  });
  expect(split, "some case needs several runs").toBeGreaterThan(2);
  const rules = planOf({allowed: ["many/m1.txt", "many/m3.txt", "many/m4.txt", "many/m6.txt", "many/m7.txt", "many/m8.txt"], ambient: [], names: [], tree: Object.keys(tree).concat(Array.from({length: 8}, (_, at) => `many/m${at + 1}.txt`))}).rules;
  expect(rules.map((rule) => `${rule.effect} ${rule.path}`)).toEqual(["allow many", "deny many/m2.txt", "deny many/m5.txt"]);
  const plan = {...planOf({allowed: [], ambient: [], names: []}), rules};
  expect(splitPlanV1(plan, 3)!.map((run) => run.rules.length)).toEqual([3]);
  expect(splitPlanV1(plan, 2)!.map((run) => run.rules.map((rule) => rule.path))).toEqual([["many", "many/m2.txt"], ["many", "many/m5.txt"]]);
  expect(splitPlanV1(plan, 1)).toBeUndefined();
});

test("a cover of thirty thousand files is computed from the declared directories only, fast, and reads exactly the declared files", () => {
  const paths: string[] = [], children = new Map<string, Set<string>>();
  for (let top = 0; top < 30; top++) for (let mid = 0; mid < 20; mid++) for (let leaf = 0; leaf < 50; leaf++) paths.push(`t${top}/m${mid}/f${leaf}.ts`);
  for (const path of paths) { const parts = path.split("/"); for (let at = 0; at < parts.length; at++) { const directory = parts.slice(0, at).join("/"); children.set(directory, (children.get(directory) ?? new Set()).add(parts[at]!)); } }
  const kept = paths.filter(() => random() < 0.7);
  let lists = 0;
  const list = (directory: string) => { lists++; const names = children.get(directory); return names && [...names]; };
  const started = performance.now(), plan = planV1(ROOT, {allowed: kept, ambient: [], names: []}, list, {refusals: true, alternatives: true});
  expect(performance.now() - started).toBeLessThan(20000);
  expect(plan.rules.length).toBeLessThan(kept.length);
  expect(lists).toBeLessThanOrEqual(6 * (1 + 30 + 600));
  const keptSet = new Set(kept);
  for (const path of paths.filter((_, at) => at % 20 === 0)) expect(planGrantsV1(plan, path), path).toBe(keptSet.has(path));
});

test("the kernel enforces the rendered profile: declared files read, every other file is refused, in every run of a split plan", () => {
  if (process.platform !== "darwin") { expect(seatbeltAvailableV1().available).toBe(false); return; }
  for (const index of [3, 7, 11, 25, 34]) {
    const row = laws[index]!, expected = expectedOf(row), runs = lawRuns[index] ?? [lawPlans[index]!];
    for (const [at, run] of runs.entries()) {
      const profile = join(base, `kernel-${index}-${at}.sb`);
      writeFileSync(profile, seatbeltProfileV1(run));
      for (const path of row.files) {
        const read = spawnSync("sandbox-exec", ["-f", profile, "/bin/cat", join(row.root, path)], {encoding: "utf8"});
        expect(read.status === 0, `case ${index} run ${at} ${path}`).toBe(planGrantsV1(run, path));
        if (runs.length === 1) expect(read.status === 0, `case ${index} ${path} is declared`).toBe(expected.includes(path));
        else if (expected.includes(path)) expect(read.status, `case ${index} run ${at} reads declared ${path}`).toBe(0);
      }
    }
  }
}, 300000);

test("the Seatbelt denial parser, the strace parser and the string decoder agree with the corpus and the Python parser", () => {
  const evaluated = python();
  corpus.seatbeltLog.forEach((row, index) => {
    expect(seatbeltDenialsV1(row.log, row.root), row.id).toEqual(row.denials);
    expect(evaluated.seatbeltLog[index], `${row.id} python`).toEqual(row.denials);
  });
  corpus.strace.forEach((row, index) => {
    expect(straceReadsV1(row.log, row.root, row.cwd), row.id).toEqual({opened: row.opened, denied: row.denied} as never);
    expect(evaluated.strace[index], `${row.id} python`).toEqual({opened: row.opened, denied: row.denied});
  });
  corpus.straceStrings.forEach((row, index) => {
    expect(straceStringV1(row.literal), row.literal).toBe(row.text);
    expect(evaluated.straceStrings[index], `${row.literal} python`).toBe(row.text);
  });
});

test("Landlock grants cover everything outside the root and only the plan inside it, as the Python oracle evaluates them", () => {
  const evaluated = python().linuxGrants as {id: string; readable: string[]; files: string[]}[];
  corpus.linuxGrants.forEach((row, index) => {
    expect(probe.linuxGrants[row.id], row.id).toEqual(row.grants);
    expect(evaluated[index]!.readable, `${row.id} python`).toEqual(row.readable.slice().sort());
  });
});

test("the sparse mirror lays out exactly the declared files and the named files, links the ambient trees and rebases paths", () => {
  const made = new Map<string, string>(), links = new Map<string, string>(), plan = planOf({allowed: ["dir/c.txt"], ambient: [], names: ["package.json"]});
  const fs: MirrorFileSystemV1 = {copy: (from, to) => void made.set(to, from), link: (target, at) => void links.set(at, target), exists: (path) => path.startsWith(`${ROOT}/`) && (files.includes(path.slice(ROOT.length + 1)) || directories.has(path.slice(ROOT.length + 1))), make: () => undefined, list: (absolute) => lister(absolute === ROOT ? "" : absolute.slice(ROOT.length + 1)), isDirectory: (absolute) => directories.has(absolute.slice(ROOT.length + 1))};
  expect(buildMirrorV1(plan, ["dir/c.txt"], "/m", fs)).toBe(2);
  expect([...made.keys()].sort()).toEqual(["/m/dir/c.txt", "/m/pkg/package.json"]);
  expect([...links]).toEqual([["/m/node_modules", `${ROOT}/node_modules`]]);
  expect(rebaseV1(`cat ${ROOT}/a.txt ${ROOT}`, ROOT, "/m")).toBe("cat /m/a.txt /m");
  const output = "denied:/m/b.txt:ENOENT\nok:/m/a.txt\nError: ENOENT: no such file or directory, open '/m/dir/c.txt'\nunrelated /m/zzz";
  expect(missingInMirrorV1(output, "/m", ROOT, (path) => path === `${ROOT}/b.txt` || path === `${ROOT}/dir/c.txt`)).toEqual(["b.txt", "dir/c.txt"]);
});

test("a declared file inside an ambient tree is reachable through the link, a nested ambient tree is not linked twice, a declared directory is skipped", () => {
  const made = new Map<string, string>(), links = new Map<string, string>(), present = new Set(["/w/ws/node_modules", "/w/ws/node_modules/m/x.js", "/w/ws/a.txt", "/w/ws/dir", "/w/ws/target", "/w/ws/target/o.rlib", "/w/ws/cache", "/w/ws/cache/sub"]);
  const plan = planV1(ROOT, {allowed: ["a.txt", "node_modules/m/x.js", "dir", "target/o.rlib"], ambient: ["cache", "cache/sub"], names: []}, () => []);
  const fs: MirrorFileSystemV1 = {copy: (from, to) => void made.set(to, from), link: (target, at) => void links.set(at, target), exists: (path) => present.has(path) || (path.startsWith("/m/") && (made.has(path) || links.has(path))), make: () => undefined, list: () => [], isDirectory: (path) => path === "/w/ws/dir"};
  expect(buildMirrorV1(plan, ["a.txt", "node_modules/m/x.js", "dir", "target/o.rlib"], "/m", fs)).toBe(1);
  expect([...made.keys()]).toEqual(["/m/a.txt"]);
  expect([...links.keys()].sort()).toEqual(["/m/cache", "/m/node_modules", "/m/target"]);
});

test("the kernel refusing to compile a profile is told apart from a failing command", () => {
  expect(seatbeltRefusalV1(65, "sandbox-exec: data object length 75339 exceeds maximum (65535)\n")).toBe(true);
  expect(seatbeltRefusalV1(65, "\nsandbox-exec: profile.sb:3: syntax error\n")).toBe(true);
  expect(seatbeltRefusalV1(128, "Assertion failed: (diff <= INSTR_JUMP_NE_MAX_LENGTH), function push_jne_instr, file serialize.c, line 240.\n")).toBe(true);
  expect(seatbeltRefusalV1(128, "tests ran\nAssertion failed: (diff <= INSTR_JUMP_NE_MAX_LENGTH), file serialize.c")).toBe(false);
  expect(seatbeltRefusalV1(0, "sandbox-exec: data object length 1 exceeds maximum")).toBe(false);
  expect(seatbeltRefusalV1(1, "error: EPERM reading /w/ws/b.txt\nsandbox-exec: was mentioned later")).toBe(false);
  expect(seatbeltRefusalV1(65, "bun test v1.4.2\nsandbox-exec: data object length 9 exceeds maximum")).toBe(false);
});

test("every platform has an ordered list of backends ending in the mirror, and the Windows plan has both languages", () => {
  expect(backendOrderV1("darwin")).toEqual(["seatbelt", "mirror"]);
  expect(backendOrderV1("linux")).toEqual(["landlock", "mirror"]);
  expect(backendOrderV1("win32")).toEqual(["mirror"]);
  expect(backendOrderV1("freebsd")).toEqual(["mirror"]);
  for (const language of ["en", "de"] as const) {
    const plan = windowsPlanV1(language);
    expect(plan.length).toBe(document.windows.options.length + 1);
    expect(plan[1]).toContain("sparse-mirror");
    expect(readOracleTextV1(language, "round", {round: 2, exit: 1, denied: 3, seconds: 1.5, backend: "mirror"})).toContain("3");
  }
  expect(document.windows.options.filter((option) => option.userland && option.strength !== "partial-trace").map((option) => option.id)).toEqual(["sparse-mirror"]);
});

test("the command line parses into a request and refuses a missing language", () => {
  const list = join(mkdtempSync(join(tmpdir(), "ro-cli-")), "allow.txt");
  try {
    writeFileSync(list, "a.txt\nb.txt\n");
    const parsed = readOracleArgumentsV1(["--language", "de", "--root", "/w/ws", "--allow-list", list, "--allow-tree", "tools", "--allow-name", "package.json", "--env", "K=V=1", "--rounds", "3", "--timeout", "30", "--out", "/o.json", "--", "bun", "x.ts"]);
    expect(parsed.out).toBe("/o.json");
    expect(parsed.request).toEqual({version: 1, language: "de", root: "/w/ws", cwd: "", allowed: ["a.txt", "b.txt"], ambient: ["tools"], names: ["package.json"], command: ["bun", "x.ts"], environment: {K: "V=1"}, rounds: 3, timeoutMs: 30000, backend: "auto", compare: "output"});
    expect(() => readOracleArgumentsV1(["--root", "/w/ws", "--allow-list", list, "--", "x"])).toThrow(/--language/u);
  } finally {
    rmSync(dirname(list), {recursive: true, force: true});
  }
});

const availability: Record<ReadOracleBackendV1, {available: boolean; reason?: string; trace?: boolean}> = {
  seatbelt: seatbeltAvailableV1(),
  landlock: linuxAvailableV1(join(here, "../🐧️linux/🐍️.py"), process.env.SEMIO_PYTHON ?? "python3"),
  mirror: {available: true},
};

/** 🌍️ The same live corpus runs on every backend that works on this machine: a backend that cannot run here is reported with its reason, never silently skipped. */
for (const backend of ["seatbelt", "landlock", "mirror"] as const) {
  test(`live corpus on the ${backend} backend (${availability[backend].available ? "available" : `unavailable: ${availability[backend].reason}`})`, async () => {
    if (!availability[backend].available) {
      expect(availability[backend].reason).toBeTruthy();
      return;
    }
    for (const row of corpus.runs) {
      const expectation = (row.expect as Record<string, {clean: boolean; passed: boolean; learned: string[]; firstDenied?: string[]; firstExit?: number; needsTrace?: boolean}>)[backend];
      if (!expectation || (expectation.needsTrace && !availability.landlock.trace)) continue;
      const root = realpathSync(mkdtempSync(join(tmpdir(), "ro-live-")));
      try {
        for (const [path, text] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); }
        const command = row.shell === "cat" ? ["sh", "-c", `cat ${row.reads.join(" ")}`] : [process.execPath, "tools/read.js", ...row.reads];
        const report = await runReadOracleV1({version: 1, language: "en", root, cwd: "", allowed: row.allowed, ambient: row.ambient, names: row.names, command, environment: {ORACLE_MODE: row.mode}, rounds: row.rounds, timeoutMs: 120000, backend, compare: "output"}, {profileCapacity: (row as {capacity?: number}).capacity});
        expect(report.backend, row.id).toBe(backend);
        expect(report.clean, `${row.id} clean`).toBe(expectation.clean);
        expect(report.passed, `${row.id} passed`).toBe(expectation.passed);
        expect([...report.learned].sort(), `${row.id} learned`).toEqual(expectation.learned);
        if (expectation.firstDenied) expect(report.rounds[0]!.denied, `${row.id} first round`).toEqual(expectation.firstDenied as never);
        if (expectation.firstExit !== undefined) expect(report.rounds[0]!.exit, `${row.id} first exit`).toBe(expectation.firstExit);
      } finally {
        rmSync(root, {recursive: true, force: true});
      }
    }
  }, 600000);
}

test("an aborted run is cancelled before it starts a command", async () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "ro-abort-"))), controller = new AbortController();
  try {
    controller.abort();
    await expect(runReadOracleV1({version: 1, language: "de", root, cwd: "", allowed: [], ambient: [], names: [], command: [process.execPath, "-e", "1"], environment: {}, rounds: 1, timeoutMs: 5000, backend: "mirror", compare: "exit"}, {signal: controller.signal})).rejects.toThrow("abgebrochen");
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
});

/** 🔀️ A plan that needs more profiles than the request allows hands over to the mirror (no profile limit) and says so, instead of failing or relaxing the plan. */
test.skipIf(process.platform !== "darwin" || !availability.seatbelt.available)("a plan that needs more profiles than allowed falls back to the mirror backend", async () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "ro-fall-"))), said: string[] = [];
  try {
    for (const name of ["m1", "m2", "m3", "m4"]) { mkdirSync(join(root, "many"), {recursive: true}); writeFileSync(join(root, "many", name), name); }
    const request = {version: 1, language: "en", root, cwd: "", allowed: ["many/m1", "many/m3"], ambient: [], names: [], command: ["sh", "-c", "cat many/m1 many/m3"], environment: {}, rounds: 1, timeoutMs: 60000, backend: "auto", compare: "exit"};
    const report = await runReadOracleV1({...request, maxProfiles: 1}, {profileCapacity: 1, progress: (text) => said.push(text)});
    expect(report.backend).toBe("mirror");
    expect(report.strength).toBe("output-equivalence");
    expect(report.clean).toBe(true);
    expect(said.some((text) => text.includes("seatbelt") && text.includes("mirror"))).toBe(true);
    const split = await runReadOracleV1({...request, allowed: ["many/m1", "many/m2", "many/m3"], maxProfiles: 4}, {profileCapacity: 2, progress: (text) => said.push(text)});
    expect(split.backend).toBe("seatbelt");
    expect(split.rounds[0]!.notes).toContain("profiles:1");
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
}, 300000);

test("a command started with an already aborted signal is stopped at once", async () => {
  const controller = new AbortController();
  controller.abort();
  const done = await captureV1(["sleep", "30"], {cwd: process.cwd(), env: {PATH: process.env.PATH}, timeoutMs: 60000, signal: controller.signal});
  expect(done.exit).toBe(124);
  expect(done.seconds).toBeLessThan(10);
});

/** 🚏️ The log is read only after the kernel has flushed: a marker denial must show up before the denials of a run are read. */
test.skipIf(process.platform !== "darwin" || !availability.seatbelt.available)("the unified log is awaited until a marker denial shows", async () => {
  const scratch = realpathSync(mkdtempSync(join(tmpdir(), "ro-flush-")));
  try {
    expect(await awaitLogFlushV1(scratch, new Date(Date.now() - 2000), {env: {PATH: process.env.PATH}})).toBe(true);
    const controller = new AbortController();
    controller.abort();
    expect(await awaitLogFlushV1(scratch, new Date(Date.now() - 2000), {env: {PATH: process.env.PATH}, signal: controller.signal}, 1500)).toBe(false);
  } finally {
    rmSync(scratch, {recursive: true, force: true});
  }
}, 120000);
