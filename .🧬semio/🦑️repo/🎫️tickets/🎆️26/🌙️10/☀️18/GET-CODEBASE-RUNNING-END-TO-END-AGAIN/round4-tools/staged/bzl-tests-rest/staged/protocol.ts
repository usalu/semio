/** 🚦️ The tool-neutral test protocol: a build tool names a shard and a result file through the environment, and every spawn of a known test tool applies them, so Bazel's `test.xml`, shard status and shard assignment and the Nx lane share one implementation. Node built-ins only; the closure of the test-execution module stays inside the framework. */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
import protocol from "./🔣️.json" with { type: "json" };

export type TestEnvironmentV1 = Readonly<Record<string, string | undefined>>;
export type TestShardV1 = Readonly<{ index: number; total: number }>;
export type TestSuiteV1 = Readonly<{ name: string; tests: number; failures: number; errors: number; skipped: number; time: number; xml: string }>;
export type TestOutcomeV1 = Readonly<{ code: number | null; signal: string | null; reason?: string; durationMs: number }>;
export type PreparedTestCommandV1 = Readonly<{ args: readonly string[]; environment: Readonly<Record<string, string>>; finish: (outcome: TestOutcomeV1) => void }>;

type TestToolV1 = Readonly<{ id: string; match: Readonly<{ commandBasenames?: string[]; firstArgument?: string; argumentBasenames?: string[]; alsoContains?: string; skipToolchain?: boolean }>; shardFlag?: string; junitFlags?: string[]; junitRefusedBy?: string[]; shardRefusedBy?: string[] }>;
const names = protocol.environment;
const last = (value: string): string => value.split(/[\\/]/u).pop() ?? value;
const refusing = (args: readonly string[], flags: readonly string[]): boolean => args.some((argument) => flags.some((flag) => argument === flag || argument.startsWith(flag + "=")));

/** 🔢️ Reads the shard a build tool assigned (`index/total`, 1-based), or none; any other text is refused. */
export function readTestShardV1(environment: TestEnvironmentV1): TestShardV1 | null {
  const value = environment[names.shard];
  if (value === undefined || value === "") return null;
  const match = /^([1-9][0-9]*)\/([2-9][0-9]*)$/u.exec(value);
  if (!match) throw Error(`Closed test shard required: ${value}`);
  const shard = { index: Number(match[1]), total: Number(match[2]) };
  if (shard.index > shard.total || shard.total > protocol.shard.maximumTotal) throw Error(`Closed test shard required: ${value}`);
  return shard;
}

/** 🍰️ Assigns sorted unique files to shards round-robin, so shard sizes differ by at most one file and equal inputs always split equally. */
export function testShardFilesV1(files: readonly string[], shard: TestShardV1 | null): string[] {
  if (!shard) return [...files];
  return [...new Set(files)].sort().filter((_file, position) => position % shard.total === shard.index - 1);
}

/** 🧭️ Finds which known test tool a command line starts, or none. */
export function matchTestToolV1(command: string, args: readonly string[]): TestToolV1 | null {
  for (const tool of protocol.tools as readonly TestToolV1[]) {
    const match = tool.match;
    if (match.commandBasenames && !match.commandBasenames.includes(last(command))) continue;
    if (match.firstArgument !== undefined && (match.skipToolchain && args[0]?.startsWith("+") ? args[1] : args[0]) !== match.firstArgument) continue;
    if (match.argumentBasenames && !args.some((argument) => match.argumentBasenames!.includes(last(argument)))) continue;
    if (match.alsoContains !== undefined && !args.includes(match.alsoContains)) continue;
    return tool;
  }
  return null;
}

const insertion = (args: readonly string[], flags: readonly string[]): string[] => {
  const stop = args.indexOf("--");
  return stop < 0 ? [...args, ...flags] : [...args.slice(0, stop), ...flags, ...args.slice(stop)];
};

/** 🛠️ Rewrites one command line for a shard and a JUnit file when its tool is known and the caller did not choose either itself; unknown tools pass through unchanged. */
export function planTestCommandV1(command: string, args: readonly string[], shard: TestShardV1 | null, junitFile: string | null): { tool: string | null; args: string[]; sharded: boolean; junit: string | null } {
  const tool = matchTestToolV1(command, args);
  if (!tool) return { tool: null, args: [...args], sharded: false, junit: null };
  const flags: string[] = [];
  const sharded = shard !== null && tool.shardFlag !== undefined && !refusing(args, tool.shardRefusedBy ?? []);
  if (sharded) flags.push(tool.shardFlag!.replace("${index}", String(shard!.index)).replace("${total}", String(shard!.total)));
  const junit = junitFile !== null && tool.junitFlags !== undefined && !refusing(args, tool.junitRefusedBy ?? []) ? junitFile : null;
  if (junit !== null) flags.push(...tool.junitFlags!.map((flag) => flag.replace("${file}", junit)));
  return { tool: tool.id, args: flags.length ? insertion(args, flags) : [...args], sharded, junit };
}

const invalid = /[^\u0009\u000A\u000D -퟿-�]/gu;
const escape = (value: string): string => value.replace(invalid, "").replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");
const unescape = (value: string): string => value.replace(/&lt;/gu, "<").replace(/&gt;/gu, ">").replace(/&quot;/gu, '"').replace(/&apos;/gu, "'").replace(/&#(\d+);/gu, (_m, code: string) => String.fromCodePoint(Number(code))).replace(/&amp;/gu, "&");
const attribute = (tag: string, name: string): string | undefined => new RegExp(`\\s${name}="([^"]*)"`, "u").exec(tag)?.[1];
const count = (tag: string, name: string, block: string, fallback: RegExp): number => {
  const value = attribute(tag, name);
  return value !== undefined && /^\d+$/u.test(value) ? Number(value) : (block.match(fallback) ?? []).length;
};

/** 🔬️ Extracts the top-level `<testsuite>` elements of a JUnit document with their counts; nested suites stay inside their parent. */
export function junitSuitesV1(xml: string): TestSuiteV1[] {
  const suites: TestSuiteV1[] = [];
  const tags = /<(\/?)testsuite(?=[\s>/])[^>]*>/gu;
  let depth = 0;
  let start = -1;
  let open = "";
  for (let match = tags.exec(xml); match; match = tags.exec(xml)) {
    const closing = match[1] === "/";
    const selfClosing = !closing && match[0].endsWith("/>");
    if (!closing && depth === 0) { start = match.index; open = match[0]; }
    if (closing) depth -= 1;
    else if (!selfClosing) depth += 1;
    if (depth === 0 && start >= 0 && (closing || selfClosing)) {
      const block = xml.slice(start, match.index + match[0].length);
      const time = Number(attribute(open, "time") ?? "0");
      suites.push({
        name: unescape(attribute(open, "name") ?? "suite"),
        tests: count(open, "tests", block, /<testcase[\s>/]/gu),
        failures: count(open, "failures", block, /<failure[\s>/]/gu),
        errors: count(open, "errors", block, /<error[\s>/]/gu),
        skipped: count(open, "skipped", block, /<skipped[\s>/]/gu),
        time: Number.isFinite(time) && time >= 0 ? time : 0,
        xml: block,
      });
      start = -1;
    }
  }
  return suites;
}

/** 🧪️ Describes a command that wrote no JUnit of its own as one suite with one case, so every test run has a result even for tools that cannot report. */
export function syntheticTestSuiteV1(input: { name: string; project?: string; durationMs: number; failure?: string }): TestSuiteV1 {
  const name = input.name.slice(0, protocol.report.maximumNameCodeUnits);
  const time = Math.max(0, input.durationMs) / 1000;
  const message = input.failure === undefined ? undefined : input.failure.slice(0, protocol.report.maximumMessageCodeUnits);
  const body = message === undefined ? "" : `<failure message="${escape(message)}" type="exit"/>`;
  const xml = `<testsuite name="${escape(name)}" tests="1" failures="${message === undefined ? 0 : 1}" errors="0" skipped="0" time="${time.toFixed(3)}"><testcase name="${escape(name)}" classname="${escape(input.project ?? "command")}" time="${time.toFixed(3)}">${body}</testcase></testsuite>`;
  return { name, tests: 1, failures: message === undefined ? 0 : 1, errors: 0, skipped: 0, time, xml };
}

/** 📄️ Renders suites as one `<testsuites>` document whose root counts are the sums of its suites. */
export function renderTestReportV1(suites: readonly TestSuiteV1[]): string {
  const sum = (key: "tests" | "failures" | "errors" | "skipped" | "time"): number => suites.reduce((total, suite) => total + suite[key], 0);
  return `<?xml version="1.0" encoding="UTF-8"?>\n<testsuites tests="${sum("tests")}" failures="${sum("failures")}" errors="${sum("errors")}" skipped="${sum("skipped")}" time="${sum("time").toFixed(3)}">\n${suites.map((suite) => "  " + suite.xml).join("\n")}\n</testsuites>\n`;
}

const slug = (value: string): string => value.replace(/[^A-Za-z0-9._-]/gu, "_").slice(0, protocol.report.maximumSlugCodeUnits);

/** 📍️ Names the result file: the tool's explicit file when given, else a stable per-task path under the results root; opt out with `SEMIO_TEST_REPORT=0`. */
export function testReportPathV1(environment: TestEnvironmentV1): string | null {
  const explicit = environment[names.xmlOutputFile];
  if (explicit !== undefined && explicit !== "") {
    if (!isAbsolute(explicit) || explicit.includes("\0")) throw Error("Absolute test result file required");
    return explicit;
  }
  if (environment[names.report] === "0") return null;
  const targets = new RegExp(protocol.report.defaultTargets, "u");
  const root = environment[protocol.report.root];
  if (!root || !isAbsolute(root)) return null;
  for (const row of protocol.report.identity) {
    const project = environment[row.project];
    const target = environment[row.target];
    if (project && target && targets.test(target)) return join(root, protocol.report.resultsRoot, `${slug(project)}-${createHash("sha256").update(project).digest("hex").slice(0, 8)}`, slug(target), protocol.report.fileName);
  }
  return null;
}

const sleep = (milliseconds: number): void => { Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds); };

const locked = <T>(path: string, body: () => T): T => {
  const lock = path + ".lock";
  const deadline = Date.now() + protocol.report.lockWaitMilliseconds;
  mkdirSync(dirname(path), { recursive: true });
  for (;;) {
    try { mkdirSync(lock); break; } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      try { if (Date.now() - statSync(lock).mtimeMs > protocol.report.lockStaleMilliseconds) { rmSync(lock, { recursive: true, force: true }); continue; } } catch { continue; }
      if (Date.now() > deadline) throw Error("Test result file lock timed out: " + lock);
      sleep(20);
    }
  }
  try { return body(); } finally { rmSync(lock, { recursive: true, force: true }); }
};

const written = new Set<string>();

/** ➕️ Appends suites to the result file under a directory lock and replaces the file atomically; the first write of the process that owns the report starts a fresh file. */
export function appendTestReportV1(path: string, suites: readonly TestSuiteV1[], fresh: boolean): void {
  locked(path, () => {
    const previous = !fresh && existsSync(path) ? junitSuitesV1(readFileSync(path, "utf8")) : [];
    const temporary = `${path}.${process.pid}.tmp`;
    writeFileSync(temporary, renderTestReportV1([...previous, ...suites]));
    renameSync(temporary, path);
  });
}

let sequence = 0;

/** 🏁️ Prepares one test command: applies the shard and a JUnit file to a known tool, and returns the finish step that appends the command's result to the report and tells the tool that sharding was honoured. A command line with neither a shard nor a result file is returned unchanged. */
export function prepareTestCommandV1(command: string, args: readonly string[], environment: TestEnvironmentV1, scratch: string): PreparedTestCommandV1 {
  const shard = readTestShardV1(environment);
  const report = testReportPathV1(environment);
  if (!shard && !report) return { args, environment: {}, finish: () => undefined };
  const owner = environment[names.reportOwner] === undefined;
  const junitFile = report ? join(scratch, `junit-${process.pid}-${sequence++}.xml`) : null;
  const plan = planTestCommandV1(command, args, shard, junitFile);
  if (plan.tool === null) return { args, environment: {}, finish: () => undefined };
  const additions: Record<string, string> = owner && report ? { [names.reportOwner]: String(process.pid) } : {};
  const finish = (outcome: TestOutcomeV1): void => {
    try {
      if (report) {
        const failed = outcome.code !== 0 || outcome.signal !== null || outcome.reason !== undefined;
        const text = `${last(command)} ${args.slice(0, 4).join(" ")}`.trim();
        const failure = outcome.reason ?? (outcome.signal ? `killed by ${outcome.signal}` : `exit status ${outcome.code}`);
        const parsed = plan.junit && existsSync(plan.junit) ? junitSuitesV1(readFileSync(plan.junit, "utf8")) : [];
        const reported = parsed.reduce((total, suite) => total + suite.failures + suite.errors, 0);
        const suites = parsed.length ? [...parsed] : [syntheticTestSuiteV1({ name: text, project: environment[protocol.report.identity[0]!.project], durationMs: outcome.durationMs, ...(failed ? { failure } : {}) })];
        if (parsed.length && failed && reported === 0) suites.push(syntheticTestSuiteV1({ name: text, project: environment[protocol.report.identity[0]!.project], durationMs: outcome.durationMs, failure }));
        const fresh = owner && !written.has(report);
        written.add(report);
        appendTestReportV1(report, suites, fresh);
      }
      if (plan.sharded) touchTestShardStatusV1(environment);
    } finally {
      if (plan.junit) rmSync(plan.junit, { force: true });
    }
  };
  return { args: plan.args, environment: additions, finish };
}

/** ✅️ Tells the build tool that this run honoured its shard; a tool that requested shards fails a run that never says so. */
export function touchTestShardStatusV1(environment: TestEnvironmentV1): void {
  const file = environment[names.shardStatusFile];
  if (file === undefined || file === "") return;
  if (!isAbsolute(file)) throw Error("Absolute test shard status file required");
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, "");
}

/** 📣️ Renders an operator-facing text in every supported language, joined, because no language is a default. */
export function testProtocolMessageV1(key: keyof typeof protocol.messages, values: Readonly<Record<string, string>>): string {
  const fill = (text: string): string => text.replace(/\$\{(\w+)\}/gu, (_all, name: string) => values[name] ?? "");
  return `${fill(protocol.messages[key].en)} | ${fill(protocol.messages[key].de)}`;
}

/** 🧾️ The names a build tool may set, for tools and tests that must not repeat them. */
export const TEST_PROTOCOL_NAMES_V1 = names;
