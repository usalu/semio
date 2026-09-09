import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync, mkdtempSync, rmSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const mode = process.argv[2];
if (mode === "fingerprint") {
  process.stdout.write("stable-input\n");
  process.exit(readFileSync("state", "utf8") === "fail" ? 1 : 0);
} else if (mode === "receipt") {
  const state = readFileSync("state", "utf8");
  if (state === "fail") process.exit(1);
  mkdirSync(dirname(process.argv[3]), { recursive: true });
  writeFileSync(process.argv[3], createHash("sha256").update(state).digest("hex"));
} else if (mode === "work" || mode === "receipt-work") {
  const count = existsSync("executions") ? Number(readFileSync("executions", "utf8")) : 0;
  writeFileSync("executions", String(count + 1)); writeFileSync(mode === "work" ? "out.txt" : "receipt-out.txt", readFileSync("state", "utf8"));
} else {
  const workspace = process.cwd(), script = fileURLToPath(import.meta.url), require = createRequire(join(workspace, "package.json"));
  const root = mkdtempSync(resolve(dirname(script), "../🗑️generated/runtime-hash-exit-"));
  const receiptOutput = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8")).generatorInputs["registry-catalog"].output;
  writeFileSync(join(root, ".nxignore"), "/.🧬semio/**\n");
  writeFileSync(join(root, ".gitignore"), ".🧬semio/\n");
  const command = action => `node "${script.replaceAll("\\", "/")}" ${action}`;
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "runtime-hash-fixture", private: true }));
  writeFileSync(join(root, "nx.json"), JSON.stringify({ useInferencePlugins: false }));
  writeFileSync(join(root, "project.json"), JSON.stringify({ name: "fixture", targets: {
    work: { executor: "nx:run-commands", cache: true, inputs: [{ runtime: command("fingerprint") }, { externalDependencies: [] }], outputs: ["{projectRoot}/out.txt"], options: { command: command("work"), cwd: root } },
    guarded: { executor: "nx:run-commands", cache: true, dependsOn: ["guard"], inputs: [{ runtime: command("fingerprint") }, { externalDependencies: [] }], outputs: ["{projectRoot}/out.txt"], options: { command: command("work"), cwd: root } },
    receipt: { executor: "nx:run-commands", cache: false, outputs: ["{workspaceRoot}/" + receiptOutput], options: { command: command("receipt") + ` "${receiptOutput}"`, cwd: root } },
    "receipt-work": { executor: "nx:run-commands", cache: true, dependsOn: ["receipt"], inputs: [{ dependentTasksOutputFiles: receiptOutput }, { externalDependencies: [] }], outputs: ["{projectRoot}/receipt-out.txt"], options: { command: command("receipt-work"), cwd: root } },
    guard: { executor: "nx:run-commands", cache: false, outputs: [], options: { command: command("fingerprint"), cwd: root } }
  }}));
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const env = { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, "workspace-data"), NX_CACHE_DIRECTORY: join(root, "cache"), NX_TASKS_RUNNER_DYNAMIC_OUTPUT: "false", NX_NO_CLOUD: "true", NX_CLOUD: "false" };
  let sequence = 0;
  const run = target => {
    const result = spawnSync("node", [cli, "run", `fixture:${target}`, "--output-style=static"], { cwd: root, env, encoding: "utf8", timeout: 60000 });
    writeFileSync(join(root, `${++sequence}-${target}-${readFileSync(join(root, "state"), "utf8")}.log`), result.stdout + result.stderr);
    return { status: result.status, executions: Number(readFileSync(join(root, "executions"), "utf8")), cacheHit: /local cache|read the output from the cache/.test(result.stdout), error: result.error?.message };
  };
  writeFileSync(join(root, "state"), "ok"); const initial = run("work"); assert.equal(initial.status, 0);
  writeFileSync(join(root, "state"), "fail"); const failedRuntime = run("work"), guarded = run("guarded");
  writeFileSync(join(root, "state"), "ok"); const receiptCold = run("receipt-work"), receiptWarm = run("receipt-work");
  assert.equal(receiptCold.status, 0); assert.equal(receiptWarm.status, 0); assert.equal(receiptWarm.cacheHit, true); assert.equal(receiptWarm.executions, receiptCold.executions);
  writeFileSync(join(root, "state"), "changed"); const receiptChanged = run("receipt-work");
  assert.equal(receiptChanged.status, 0); assert.equal(receiptChanged.executions, receiptCold.executions + 1); assert.equal(readFileSync(join(root, "receipt-out.txt"), "utf8"), "changed");
  rmSync(join(root, "receipt-out.txt")); const receiptRestored = run("receipt-work");
  assert.equal(receiptRestored.status, 0); assert.equal(receiptRestored.cacheHit, true); assert.equal(receiptRestored.executions, receiptChanged.executions); assert.equal(readFileSync(join(root, "receipt-out.txt"), "utf8"), "changed");
  const digest = readFileSync(join(root, receiptOutput), "utf8"); writeFileSync(join(root, "state"), "fail"); const receiptFailed = run("receipt-work");
  assert.notEqual(receiptFailed.status, 0); assert.equal(receiptFailed.executions, receiptChanged.executions); assert.equal(readFileSync(join(root, receiptOutput), "utf8"), digest);
  const receipt = { nx: require("nx/package.json").version, initial, failedRuntime, guarded, receiptCold, receiptWarm, receiptChanged, receiptRestored, receiptFailed };
  writeFileSync(join(root, "receipt.json"), JSON.stringify(receipt, null, 2)); console.log("[DEBUG] Native Nx runtime exit-status qualification", JSON.stringify(receipt));
  assert.notEqual(guarded.status, 0); assert.equal(guarded.executions, initial.executions);
}
