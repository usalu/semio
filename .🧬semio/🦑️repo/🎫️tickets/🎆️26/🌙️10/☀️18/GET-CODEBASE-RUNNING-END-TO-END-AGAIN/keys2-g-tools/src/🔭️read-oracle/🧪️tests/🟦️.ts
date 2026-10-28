import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {backendOrderV1, readOracleRequestV1, readOracleTextV1, runReadOracleV1, windowsPlanV1, type ReadOracleBackendV1} from "../🟦️.ts";
import {readOracleArgumentsV1} from "../🚪️cli/🟦️.ts";
import {landlockGrantsV1, linuxAvailableV1, straceReadsV1, straceStringV1} from "../🐧️linux/🟦️.ts";
import {buildMirrorV1, missingInMirrorV1, rebaseV1, type MirrorFileSystemV1} from "../🪞️mirror/🟦️.ts";
import {coverV1, namedFilesV1, planGrantsV1, planV1} from "../🪢️policy/🟦️.ts";
import {seatbeltAvailableV1, seatbeltDenialsV1, seatbeltProfileV1, seatbeltRefusalV1} from "../🍎️seatbelt/🟦️.ts";
import document from "../🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type: "json"};

const tree = corpus.tree as Record<string, string>, files = Object.keys(tree).sort();
const directories = new Set(files.flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
const lister = (directory: string) => {
  if (directory !== "" && !directories.has(directory)) return undefined;
  const prefix = directory ? `${directory}/` : "";
  return [...new Set(files.filter((path) => path.startsWith(prefix)).map((path) => path.slice(prefix.length).split("/")[0]!))];
};
const ROOT = "/w/ws", here = import.meta.dirname, oracle = join(here, "🐍️oracle/🐍️.py");

const policyOf = (row: {allowed: string[]; ambient: string[]; names: string[]}) => planV1(ROOT, row, lister);
const linuxFixture = (row: (typeof corpus.linuxGrants)[number]) => ({list: (absolute: string) => (row.tree as Record<string, string[]>)[absolute], isDirectory: (absolute: string) => row.directories.includes(absolute), real: (absolute: string) => absolute});
const probe = {
  root: ROOT,
  policy: Object.fromEntries(corpus.policy.map((row) => [row.id, {profile: seatbeltProfileV1(policyOf(row)), rules: policyOf(row).rules}])),
  linuxGrants: Object.fromEntries(corpus.linuxGrants.map((row) => [row.id, landlockGrantsV1({root: row.root, rules: coverV1(row.allowed, (directory) => (row.tree as Record<string, string[]>)[directory ? join(row.root, directory) : row.root]), ambient: row.ambient, names: row.names}, linuxFixture(row))])),
};
const python = () => {
  const scratch = mkdtempSync(join(tmpdir(), "ro-oracle-")), probePath = join(scratch, "probe.json");
  try {
    writeFileSync(probePath, JSON.stringify(probe));
    const done = spawnSync(testPythonV1() ?? "python3", ["-I", oracle, join(here, "🔣️.json"), probePath], {encoding: "utf8"});
    expect(done.status, done.stderr).toBe(0);
    return JSON.parse(done.stdout);
  } finally {
    rmSync(scratch, {recursive: true, force: true});
  }
};

/** 🔬️ The corpus, the document of texts and a request satisfy their strict schemas; a request outside the schema is refused. */
test("the corpus, the document and the request satisfy their strict schemas", () => {
  const ajv = new Ajv({strict: true, allErrors: true});
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, document), JSON.stringify(ajv.errors)).toBe(true);
  const request = {version: 1, language: "en", root: "/w/ws", cwd: "", allowed: ["a.txt"], ambient: [], names: [], command: ["true"], environment: {}, rounds: 1, timeoutMs: 1000, backend: "auto", compare: "output"};
  expect(readOracleRequestV1(request)).toEqual(request as never);
  for (const bad of [{...request, language: "fr"}, {...request, rounds: 0}, {...request, command: []}, {...request, backend: "bwrap"}, {...request, extra: 1}, {...request, allowed: ["../x"]}]) expect(() => readOracleRequestV1(bad), JSON.stringify(bad)).toThrow(/refused/u);
  expect(new Set(corpus.runs.map((row) => row.id)).size).toBe(corpus.runs.length);
});

test("the cover collapses complete directories, keeps partial ones literal, and the plan grants what the policy says", () => {
  const evaluated = python().policy as {id: string; direct: string[]; seatbelt: string[]; rules: string[]}[];
  corpus.policy.forEach((row, index) => {
    const plan = policyOf(row);
    expect(plan.rules, row.id).toEqual(row.rules as never);
    expect(files.filter((path) => planGrantsV1(plan, path)), row.id).toEqual(row.readable.slice().sort());
    expect(evaluated[index]!.direct, `${row.id} python direct`).toEqual(row.readable.slice().sort());
    expect(evaluated[index]!.seatbelt, `${row.id} python reads the rendered profile`).toEqual(row.readable.slice().sort());
    expect(evaluated[index]!.rules, `${row.id} python rules`).toEqual(row.readable.slice().sort());
  });
  expect(namedFilesV1(["package.json"], ["node_modules"], lister, (path) => directories.has(path))).toEqual(["pkg/package.json"]);
});

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
  const made = new Map<string, string>(), links = new Map<string, string>(), plan = policyOf({allowed: ["dir/c.txt"], ambient: [], names: ["package.json"]});
  const fs: MirrorFileSystemV1 = {copy: (from, to) => void made.set(to, from), link: (target, at) => void links.set(at, target), exists: (path) => path.startsWith(`${ROOT}/`) && (files.includes(path.slice(ROOT.length + 1)) || directories.has(path.slice(ROOT.length + 1))), make: () => undefined, list: (absolute) => lister(absolute === ROOT ? "" : absolute.slice(ROOT.length + 1)), isDirectory: (absolute) => directories.has(absolute.slice(ROOT.length + 1))};
  expect(buildMirrorV1(plan, ["dir/c.txt"], "/m", fs)).toBe(2);
  expect([...made.keys()].sort()).toEqual(["/m/dir/c.txt", "/m/pkg/package.json"]);
  expect([...links]).toEqual([["/m/node_modules", `${ROOT}/node_modules`]]);
  expect(rebaseV1(`cat ${ROOT}/a.txt ${ROOT}`, ROOT, "/m")).toBe("cat /m/a.txt /m");
  const output = "denied:/m/b.txt:ENOENT\nok:/m/a.txt\nError: ENOENT: no such file or directory, open '/m/dir/c.txt'\nunrelated /m/zzz";
  expect(missingInMirrorV1(output, "/m", ROOT, (path) => path === `${ROOT}/b.txt` || path === `${ROOT}/dir/c.txt`)).toEqual(["b.txt", "dir/c.txt"]);
});

test("the kernel refusing to compile a profile is told apart from a failing command", () => {
  expect(seatbeltRefusalV1(65, "sandbox-exec: data object length 75339 exceeds maximum (65535)\n")).toBe(true);
  expect(seatbeltRefusalV1(65, "\nsandbox-exec: profile.sb:3: syntax error\n")).toBe(true);
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

/** 🌐️ The same live corpus runs on every backend that works on this machine: a backend that cannot run here is reported with its reason, never silently skipped. */
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
        const report = await runReadOracleV1({version: 1, language: "en", root, cwd: "", allowed: row.allowed, ambient: row.ambient, names: row.names, command, environment: {ORACLE_MODE: row.mode}, rounds: row.rounds, timeoutMs: 120000, backend, compare: "output"});
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
