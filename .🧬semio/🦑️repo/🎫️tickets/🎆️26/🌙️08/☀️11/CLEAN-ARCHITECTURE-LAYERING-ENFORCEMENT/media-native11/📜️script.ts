import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

if (!import.meta.main || process.argv.length !== 3 || process.argv[2] !== "capture-root-dispatched") throw Error("Native11 requires Root's explicit capture dispatch");
const root = resolve(import.meta.dir, "../../../../../../../.."), ticket = resolve(import.meta.dir, "..");
const output = resolve(ticket, "🗑️generated/catalog-current/media-native11"), previous = resolve(ticket, "🗑️generated/catalog-current/media-native10");
const command = "bun repo run @semio-tech/framework-plugin:test-media-turn-native --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype";
const argv = command.split(" "), arguments_ = argv.slice(1), name = "🎟️Original Media Turn Native 🦀️";
const inherited = { ...process.env };
const privateKeys = new Set([
  "SEMIO_SCRIPT_PROCESS_INVOCATION", "SEMIO_SCRIPT_CAPABILITIES", "SEMIO_OWNER_ARGUMENTS", "SEMIO_NX_COMMAND_DIRECTORY", "SEMIO_NX_COMMAND_RUNNER",
  "SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP", "SEMIO_NX_SELECTED_GRAPH_RECEIPT", "SEMIO_NX_GRAPH_SEAL_REUSE", "SEMIO_BUILD_GRAPH_PATH", "SEMIO_BUILD_GRAPH_DIGEST", "SEMIO_BUILD_REUSE_GRAPH",
  "SEMIO_CARGO_PREPARATION_ACTIVE", "SEMIO_CARGO_PREPARATION_OBSERVATION", "SEMIO_CARGO_PREPARATION_STORAGE",
  "SEMIO_BUILD_INVOCATION_PID", "SEMIO_OWNER_POLICY_RECEIPT", "SEMIO_PROCESS_OWNER_CONTEXT", "SEMIO_NX_NATIVE_SOURCE_SNAPSHOT", "SEMIO_COMPILER_RESOURCE_ROOT", "SEMIO_BAZEL_SELECTION_CHAIN",
]);
for (const key of Object.keys(inherited)) if (privateKeys.has(key) || key.startsWith("SEMIO_SOURCE_CLOSURE_") || key.startsWith("SEMIO_CARGO_LOCK_") || key.startsWith("NX_TASK_") || /^(?:SEMIO|NX)_.*(?:PRIVATE|CUSTODY)/u.test(key)) throw Error(`Native11 refuses inherited issued authority or private custody: ${key}`);
if (existsSync(output)) throw Error("Native11 output already exists; this recorder never overwrites or resumes an original capture");

/** 🪟️ Reads the exact native GUI row without loading any executable or rewriting launch inventories. */
function nativeRow(path: string): Record<string, any> {
  const source = readFileSync(resolve(root, path), "utf8"), marker = `"name": "${name}"`;
  assert.equal(source.split(marker).length - 1, 1);
  const start = source.lastIndexOf("{", source.indexOf(marker));
  let depth = 0, quoted = false, escaped = false;
  for (let end = start; end < source.length; end++) {
    const value = source[end]!;
    if (quoted) {
      if (escaped) escaped = false;
      else if (value === "\\") escaped = true;
      else if (value === '"') quoted = false;
    } else if (value === '"') quoted = true;
    else if (value === "{") depth++;
    else if (value === "}" && --depth === 0) return JSON.parse(source.slice(start, end + 1));
  }
  throw Error("Exact native GUI row is incomplete");
}

const launch = nativeRow(".vscode/🧩️launch.seed.jsonc"), mirror = nativeRow(".vscode/launch.json");
const priorCaller = JSON.parse(readFileSync(resolve(previous, "caller.json"), "utf8"));
assert.deepEqual(launch, mirror); assert.deepEqual(launch, priorCaller.launch); assert.equal(launch.command, command);
const expand = (value: string): string => value.replaceAll("${workspaceFolder}", root.replaceAll("\\", "/"));
const initialRowEnvironment = Object.fromEntries(Object.entries(launch.env as Record<string, string>).map(([key, value]) => [key, expand(value)]));
assert.equal(initialRowEnvironment.SEMIO_BUILD_MODE, "prototype"); assert.equal(initialRowEnvironment.CARGO_BUILD_JOBS, "2"); assert.equal(initialRowEnvironment.CARGO_NET_OFFLINE, "true");
const policy = JSON.parse(initialRowEnvironment.SEMIO_SCRIPT_POLICY!);
assert.deepEqual(policy, { version: 1, owner: "plugin:media-turn:native", maximumElapsedMilliseconds: 3600000 });
const authored = JSON.parse(initialRowEnvironment.SEMIO_SCRIPT_CAPABILITIES!);
assert.deepEqual(authored.arguments, arguments_); assert.deepEqual(authored.selection, arguments_); assert.deepEqual(authored.launches, [arguments_]);
assert.equal(authored.buildMode, "prototype"); assert.equal(resolve(authored.workspaceRoot), root);
assert.equal(resolve(authored.native.artifactDirectory), resolve(initialRowEnvironment.CARGO_TARGET_DIR!));
assert.deepEqual(authored.native.transport, { maximumBytes: 134217728, maximumLines: 1000000 });
assert.deepEqual(authored.native.child, { maximumElapsedMilliseconds: 3600000 }); assert.deepEqual(authored.native.network, { offline: true });
const environment = { ...inherited, ...initialRowEnvironment, SEMIO_TEST_ARTIFACT_DIR: output };
const roster = JSON.parse(readFileSync(resolve(previous, "source-sha.json"), "utf8"));
assert.equal(roster.kind, "read-only-source-observation-not-authority"); assert.equal(roster.files.length, 39);
const sourcePaths: string[] = roster.files.map((row: { path: string }) => row.path);
assert.equal(new Set(sourcePaths).size, 39);
for (const path of sourcePaths) {
  const local = relative(root, resolve(root, path));
  if (isAbsolute(path) || local === ".." || local.startsWith(`..${sep}`)) throw Error("Media source observation escaped its original workspace");
}

/** 🔎️ Observes actual current files without claiming an authoritative Source publication. */
function observe(): object {
  return { kind: "read-only-source-observation-not-authority", observedAt: new Date().toISOString(), files: sourcePaths.map(path => {
    const bytes = readFileSync(resolve(root, path));
    return { path, sha256: createHash("sha256").update(bytes).digest("hex"), bytes: bytes.length };
  }) };
}

/** 🧾️ Preserves complete recorder receipts without overwriting prior output. */
function record(name: string, value: unknown): void {
  writeFileSync(resolve(output, name), JSON.stringify(value, null, 2) + "\n", { flag: "wx" });
}

mkdirSync(output, { recursive: false });
record("source-sha.json", observe());
record("dispatch.json", { kind: "root-dispatched-original-capture", command, argv, initialRowEnvironment, artifactDirectory: output, recordedAt: new Date().toISOString() });
const buildPath = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts");
const processPath = resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const { createBuildBootstrapCapabilitiesV1 } = await import(pathToFileURL(buildPath).href);
const capabilities = await createBuildBootstrapCapabilitiesV1(root, arguments_, environment);
record("factory.json", { kind: "actual-complete-configured-initial-factory-return-before-first-envelope", returnedAt: new Date().toISOString(), capabilities });
const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(pathToFileURL(processPath).href);
const envelope = createScriptProcessEnvelope(policy, capabilities, Date.now());
const childEnvironment = scriptProcessEnvironment(envelope, environment);
record("caller.json", { launch, artifactDirectory: output, command, argv, initialRowEnvironment, captureEnvironmentAdditions: { SEMIO_TEST_ARTIFACT_DIR: output }, capabilities, envelope, lawCount: 9, qualification: "Initial actual configured build factory receipt; native materialization and all assertions remain subject to actual child output" });
const log = openSync(resolve(output, "current.log"), "wx"), started = performance.now(), startedAt = new Date().toISOString();
let child: ReturnType<typeof Bun.spawn> | undefined;
try {
  child = Bun.spawn(argv, { cwd: root, env: childEnvironment, stdin: "ignore", stdout: log, stderr: log });
  record("process.json", { recorderPid: process.pid, pid: child.pid, command, argv, startedAt });
  const status = await child.exited;
  record("terminal.json", { status, signal: child.signalCode, elapsedSeconds: (performance.now() - started) / 1000, pid: child.pid, command, startedAt, endedAt: new Date().toISOString(), qualification: "Raw process result; no inferred native-law acceptance" });
  process.exitCode = child.exitCode ?? 1;
} catch (error) {
  record("recorder-error.json", { message: error instanceof Error ? error.message : String(error), childPid: child?.pid, elapsedSeconds: (performance.now() - started) / 1000, qualification: "Recorder failure is not a native assertion result" });
  throw error;
} finally {
  closeSync(log);
  try { record("source-sha-terminal.json", observe()); } catch (error) { record("source-observation-error.json", { message: String(error), kind: "read-only-observation-failure-not-acceptance" }); }
}
const logText = readFileSync(resolve(output, "current.log"), "utf8");
record("dependency-observation.json", { kind: "actual-log-observation-not-identity-requirement", lines: logText.split(/\r?\n/u).filter(line => /immutable Bun dependencies|dependency generation/iu.test(line)), qualification: "Records actual reported reuse/publication; no expected historical generation, inference, initializer retry or identity gate" });
console.log(`[DEBUG] Native11 original capture recorder completed; raw receipts retained at ${output}`);
