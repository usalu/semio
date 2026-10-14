import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

if (!import.meta.main || process.argv.length !== 3 || process.argv[2] !== "capture-root-dispatched") throw Error("Layout GUI1 requires Root's explicit registered Source dispatch");
const root = resolve(import.meta.dir, "../../../../../../../.."), ticket = resolve(import.meta.dir, "..");
const output = resolve(ticket, "🗑️generated/layout-media-adoption-gui1"), inherited = { ...process.env };
const command = "bun repo run @semio-tech/layout-layout-rs:test-media-adoption-source --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype";
const name = "🎟️Layout Media Adoption Source 🦀️", argv = command.split(" "), arguments_ = argv.slice(1);
const privateKeys = new Set([
  "SEMIO_SCRIPT_PROCESS_INVOCATION", "SEMIO_SCRIPT_CAPABILITIES", "SEMIO_OWNER_ARGUMENTS", "SEMIO_NX_COMMAND_DIRECTORY", "SEMIO_NX_COMMAND_RUNNER",
  "SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP", "SEMIO_NX_SELECTED_GRAPH_RECEIPT", "SEMIO_NX_GRAPH_SEAL_REUSE", "SEMIO_BUILD_GRAPH_PATH", "SEMIO_BUILD_GRAPH_DIGEST", "SEMIO_BUILD_REUSE_GRAPH",
  "SEMIO_CARGO_PREPARATION_ACTIVE", "SEMIO_CARGO_PREPARATION_OBSERVATION", "SEMIO_CARGO_PREPARATION_STORAGE", "SEMIO_BUILD_INVOCATION_PID", "SEMIO_OWNER_POLICY_RECEIPT",
  "SEMIO_PROCESS_OWNER_CONTEXT", "SEMIO_NX_NATIVE_SOURCE_SNAPSHOT", "SEMIO_COMPILER_RESOURCE_ROOT", "SEMIO_BAZEL_SELECTION_CHAIN",
]);
for (const key of Object.keys(inherited)) if (privateKeys.has(key) || key.startsWith("SEMIO_SOURCE_CLOSURE_") || key.startsWith("SEMIO_CARGO_LOCK_") || key.startsWith("NX_TASK_") || /^(?:SEMIO|NX)_.*(?:PRIVATE|CUSTODY)/u.test(key)) throw Error(`Layout GUI1 refuses inherited issued authority or private custody: ${key}`);
if (existsSync(output)) throw Error("Fresh Layout GUI1 output ownership required; no overwrite or resumed authority");

/** 🪟️ Reads one actual registered row without importing an executable or modifying an inventory. */
function sourceRow(path: string): Record<string, any> {
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
  throw Error("Registered Layout Source row incomplete");
}

const launch = sourceRow(".vscode/🧩️launch.seed.jsonc"), mirror = sourceRow(".vscode/launch.json");
assert.deepEqual(launch, mirror); assert.equal(launch.command, command); assert.equal(launch.cwd, "${workspaceFolder}");
assert.equal(launch.env.SEMIO_BUILD_MODE, "prototype"); assert.equal(launch.env.SEMIO_SCRIPT_CAPABILITIES, undefined);
const policy = JSON.parse(launch.env.SEMIO_SCRIPT_POLICY);
assert.deepEqual(policy, { version: 1, owner: "layout:media-adoption:source", maximumElapsedMilliseconds: 135000 });
const packageRoot = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust";
const editor = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor", exporter = editor + "/⚙️engine/📤️export";
const project = JSON.parse(readFileSync(resolve(root, packageRoot, "📋️project.json"), "utf8")), target = project.targets["test-media-adoption-source"];
assert.equal(project.name, "@semio-tech/layout-layout-rs"); assert.deepEqual(target.metadata.semio.execution, { version: 1, kind: "source" });
assert.equal(target.options.command, "bun ./📜️script.ts test-media-adoption-source");
const handler = readFileSync(resolve(root, packageRoot, "📜️script.ts"), "utf8");
assert.equal(handler.includes('{ name: "strict", budget: 120_000'), true); assert.equal(handler.includes('{ name: "source", budget: 15_000'), true);
const initialRowEnvironment = Object.fromEntries(Object.entries(launch.env as Record<string, string>).map(([key, value]) => [key, value.replaceAll("${workspaceFolder}", root.replaceAll("\\", "/"))]));
const environment = { ...inherited, ...initialRowEnvironment, SEMIO_TEST_ARTIFACT_DIR: output };
const paths = [
  packageRoot + "/📋️project.json", packageRoot + "/📜️script.ts", editor + "/🦀️.rs", exporter + "/🦀️.rs",
  exporter + "/🧪️testing/📥️source/🟦️.ts", exporter + "/🧪️testing/📥️source/🧬️schema/🔣️.json", exporter + "/🧪️tests/🟦️.ts", exporter + "/🧪️tests/🧫️fixtures/🔣️.json",
];

/** 🔎️ Records bounded owned-file observations without asserting complete compiler Source authority. */
function observations(): object {
  return { kind: "finite-owned-layout-source-observation-not-authority-or-complete-compiler-closure", observedAt: new Date().toISOString(), files: paths.map(path => {
    const bytes = readFileSync(resolve(root, path));
    return { path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") };
  }) };
}

/** 🧾️ Keeps original receipts exclusive while the whole ticket remains active. */
function record(name: string, value: unknown): void {
  writeFileSync(resolve(output, name), JSON.stringify(value, null, 2) + "\n", { flag: "wx" });
}

mkdirSync(output, { recursive: false });
let stage = "initial-read-only-observation", descriptor: number | undefined, child: ReturnType<typeof Bun.spawn> | undefined;
const started = performance.now(), startedAt = new Date().toISOString();
try {
  record("caller-declaration.json", { kind: "original-registered-source-gui-declaration-not-factory-receipt", launch, argv, command, initialRowEnvironment, outputRedirectOnly: output, startedAt });
  record("source-sha.json", observations());
  stage = "configured-initial-factory-before-first-envelope";
  const buildPath = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts");
  const { createBuildBootstrapCapabilitiesV1 } = await import(pathToFileURL(buildPath).href);
  const capabilities = await createBuildBootstrapCapabilitiesV1(root, arguments_, environment);
  record("factory.json", capabilities);
  stage = "first-original-envelope";
  const processPath = resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
  const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(pathToFileURL(processPath).href);
  const envelope = createScriptProcessEnvelope(policy, capabilities, Date.now()), childEnvironment = scriptProcessEnvironment(envelope, environment);
  record("caller.json", { kind: "actual-configured-initial-source-caller-not-current-test-acceptance", launch, command, argv, initialRowEnvironment, capabilities, envelope, outputRedirectOnly: output, qualification: "Complete configured factory return recorded before first envelope; no reconstructed raw capabilities or native/resource grant" });
  stage = "original-registered-child";
  descriptor = openSync(resolve(output, "current.log"), "wx");
  child = Bun.spawn(argv, { cwd: root, env: childEnvironment, stdin: "ignore", stdout: descriptor, stderr: descriptor });
  record("started.json", { recorderPid: process.pid, pid: child.pid, startedAt, command, argv, initialPolicy: policy });
  const status = await child.exited;
  stage = "raw-terminal-recording";
  record("terminal.json", { status, signal: child.signalCode, pid: child.pid, command, elapsedMilliseconds: performance.now() - started, startedAt, endedAt: new Date().toISOString(), qualification: "Raw registered Source process result; actual tests/Source acceptance require actual output, no native admission or heap witness" });
  process.exitCode = child.exitCode ?? 1;
} catch (error) {
  record("refusal.json", { stage, message: error instanceof Error ? error.message : String(error), childPid: child?.pid, elapsedMilliseconds: performance.now() - started, qualification: "Recorder/factory/refusal evidence, not a passing test or native result" });
  throw error;
} finally {
  if (descriptor !== undefined) closeSync(descriptor);
  try { record("source-sha-terminal.json", observations()); } catch (error) { record("source-observation-error.json", { message: String(error), qualification: "Read-only observer failure is not compiler Source authority" }); }
}
console.log(`[DEBUG] Registered Layout Source GUI1 recorder completed; original receipts retained at ${output}`);
