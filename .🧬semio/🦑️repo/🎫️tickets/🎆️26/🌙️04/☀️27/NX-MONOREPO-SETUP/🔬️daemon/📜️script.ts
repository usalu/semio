import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { appendFileSync, copyFileSync, existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import ts from "typescript";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const fixture = join(ticket, "🗑️generated", `daemon-${Date.now()}`);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const put = (path: string, value: unknown): void => { const file = join(fixture, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, typeof value === "string" ? value : JSON.stringify(value, null, 2)); };
put("nx.json", { useDaemonProcess: true, plugins: [{ plugin: `./${library}/🟨️.mjs` }], cacheDirectory: ".nx/cache", namedInputs: { sharedGlobals: [] } });
put("package.json", { name: "daemon-qualification", private: true, nx: { includedScripts: [] } });
put(".gitignore", "node_modules\n.nx\nstate\n**/dist\n*.log\n");
put(".nxignore", `!🔨️core/source.json\n!🧩️app/source.json\n!${library}/🟨️.mjs\n!${library}/⚡️caching/🔣️policy.json\n!${library}/🕸️dependencies/🧩️runtime/🟨️.mjs\n`);
put(`${library}/⚡️caching/🔣️policy.json`, readFileSync(join(root, library, "⚡️caching/🔣️policy.json"), "utf8"));
copyFileSync(join(root, library, "🟨️.mjs"), join(fixture, library, "🟨️.mjs"));
put(`${library}/🕸️dependencies/🧩️runtime/🟨️.mjs`, readFileSync(join(root, library, "🕸️dependencies/🧩️runtime/🟨️.mjs"), "utf8"));
for (const [name, directory, dependencies] of [["probe-core", "🔨️core", []], ["probe-app", "🧩️app", ["probe-core"]]] as const) {
  put(`${directory}/📋️project.json`, { name, implicitDependencies: dependencies, targets: { build: { cache: true, inputs: ["default", "^default"], outputs: ["{projectRoot}/dist"], dependsOn: ["^build"], options: { command: `bun ./📜️script.ts build ${directory}`, cwd: "." } } } });
  put(`${directory}/source.json`, { value: 42 });
}
const rootSource = ts.createSourceFile("📜️script.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const nxClass = rootSource.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "NxScript")!;
const wrapper = ts.transpileModule(nxClass.getText(rootSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const cancellation = JSON.parse(readFileSync(join(root, library, "⚡️caching/🧫️fixtures/nx-contract/🔣️.json"), "utf8")).watchCancellation;
put("📜️script.ts", String.raw`import { createRequire } from "node:module";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { spawn, spawnSync, spawnSync as stopNxProcessTree } from "node:child_process";
import { join } from "node:path";
mkdirSync("state", {recursive:true});
if (process.argv[2] === "build") {
  const path = process.argv[3];
  mkdirSync(path + "/dist", {recursive:true});
  writeFileSync(path + "/dist/result.json", readFileSync(path + "/source.json"));
  appendFileSync("state/builds.jsonl", JSON.stringify({ path }) + "\n");
} else if (process.argv[2] === "stubborn") {
  const child = spawn("bun", ["./📜️script.ts", "child"], {stdio:"inherit"});
  writeFileSync("state/callback-pid", String(child.pid));
  await new Promise((accept) => child.once("close", accept));
} else if (process.argv[2] === "child") {
  process.on("SIGTERM", () => {});
  setInterval(() => {}, 1000);
  writeFileSync("state/child-ready", "ready");
} else if (process.argv[2] === "rebuild") {
  const result = spawnSync("node", [createRequire(import.meta.url).resolve("nx/bin/nx.js"), "run", "probe-app:build", "--output-style=static"], {stdio:"inherit", env: {...process.env, npm_lifecycle_event:undefined, npm_lifecycle_script:undefined}});
  appendFileSync("state/events.jsonl", JSON.stringify({status:result.status,files:process.env.NX_FILE_CHANGES}) + "\n");
  process.exitCode=result.status ?? 1;
}
` + `
class Script { root = process.cwd(); }
const resolveNxInvocation = (args) => ({ args, env: {} });
const devToolingEnv = (extra) => ({ ...process.env, ...extra });
const orchestratorBudgetOpts = () => ({});
const spawnNxProcess = (...args) => { const child = spawn(...args); writeFileSync("state/watcher-pid", String(child.pid)); return child; };
${wrapper}
if (process.argv[2] === "nx") await new NxScript().run(process.argv.slice(3));
`);
symlinkSync(join(root, "node_modules"), join(fixture, "node_modules"), process.platform === "win32" ? "junction" : "dir");
const env = { ...process.env, NX_DAEMON: "true", NX_ISOLATE_PLUGINS: "true", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_WORKSPACE_DATA_DIRECTORY: join(fixture, ".nx/workspace-data"), NX_VERBOSE_LOGGING: "true", npm_lifecycle_event: undefined, npm_lifecycle_script: undefined };
for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_SKIP_NX_CACHE"].includes(key)) delete env[key];
const cli = createRequire(join(root, "package.json")).resolve("nx/bin/nx.js");
const run = async (args: string[], name: string, overrides = {}, wrapper = false): Promise<string> => {
  const child = spawn(wrapper ? "bun" : "node", wrapper ? ["./📜️script.ts", "nx", ...args] : [cli, ...args], { cwd: fixture, env: { ...env, ...overrides } });
  let out = "";
  child.stdout.on("data", (data) => out += data); child.stderr.on("data", (data) => out += data);
  const timeout = setTimeout(() => child.kill("SIGTERM"), 60000);
  const status = await new Promise<number | null>((accept, reject) => { child.once("error", reject); child.once("close", accept); });
  clearTimeout(timeout); writeFileSync(join(fixture, name + ".log"), out);
  assert.equal(status, 0, out); return out;
};
let watcher: ReturnType<typeof spawn> | undefined;
const eventPath = join(fixture, "state/events.jsonl");
const events = (): any[] => existsSync(eventPath) ? readFileSync(eventPath, "utf8").trim().split("\n").filter(Boolean).map((line) => JSON.parse(line)) : [];
const until = async (test: () => boolean, label: string): Promise<void> => { const deadline = Date.now() + 45000; while (!test()) { assert.ok(Date.now() < deadline, label); await new Promise((accept) => setTimeout(accept, 200)); } };
try {
  const graph = JSON.parse(await run(["graph", "--file=stdout"], "graph").then((text) => text.slice(text.indexOf("{\n"))));
  assert.equal(graph.graph.nodes["probe-app"].data.root, "🧩️app");
  assert.equal(graph.graph.nodes["probe-core"].data.root, "🔨️core");
  await run(["run", "probe-app:build", "--output-style=static"], "cold");
  const warm = await run(["run", "probe-app:build", "--output-style=static"], "warm");
  assert.match(warm, /read the output from the cache|local cache/);
  const daemonPid = (): number => JSON.parse(readFileSync(join(fixture, ".nx/workspace-data/d/server-process.json"), "utf8")).processId;
  const nativePid = daemonPid();
  const shared = await run(["run", "probe-app:build", "--output-style=static"], "shared-data", { NX_WORKSPACE_DATA_DIRECTORY: join(fixture, "state", "agent-data") }, true);
  assert.match(shared, /read the output from the cache|local cache/, "The public runner must reuse one workspace database despite caller overrides");
  assert.equal(daemonPid(), nativePid, "Public wrapper and native Nx CLI must use the same daemon socket");
  const implementation = readFileSync(join(fixture, library, "🟨️.mjs"), "utf8");
  assert.ok(implementation.includes("return { ...json, name: json.name, root, namedInputs:"));
  put(`${library}/🟨️.mjs`, implementation.replace("return { ...json, name: json.name, root, namedInputs:", 'return { ...json, metadata: { qualification: "updated-plugin" }, name: json.name, root, namedInputs:'));
  await new Promise((accept) => setTimeout(accept, 1500));
  const reloaded = JSON.parse(await run(["graph", "--file=stdout"], "plugin-reloaded").then((text) => text.slice(text.indexOf("{\n"))));
  assert.equal(reloaded.graph.nodes["probe-app"].data.metadata?.qualification, "updated-plugin", "Daemon must reload changed graph implementation");
  const policy = JSON.parse(readFileSync(join(fixture, library, "⚡️caching/🔣️policy.json"), "utf8"));
  policy.uncached.push("build");
  put(`${library}/⚡️caching/🔣️policy.json`, policy);
  await new Promise((accept) => setTimeout(accept, 1500));
  const repolicy = JSON.parse(await run(["graph", "--file=stdout"], "policy-reloaded").then((text) => text.slice(text.indexOf("{\n"))));
  assert.equal(repolicy.graph.nodes["probe-app"].data.targets.build.cache, false, "Daemon must reload changed cache policy");
  watcher = spawn("node", [cli, "watch", "--projects=probe-app", "--includeDependentProjects", "--initialRun", "--", "bun", "./📜️script.ts", "rebuild"], { cwd: fixture, env });
  const output = (data: Buffer): void => appendFileSync(join(fixture, "watch.log"), data);
  watcher.stdout!.on("data", output); watcher.stderr!.on("data", output);
  await until(() => events().length >= 1, "initial watch execution");
  await new Promise((accept) => setTimeout(accept, 1000));
  put("🔨️core/source.json", { value: 43 });
  await until(() => events().some((event) => event.files?.includes("🔨️core/source.json")), "Unicode dependency change delivered through daemon");
  assert.ok(events().every((event) => event.status === 0));
  assert.equal(JSON.parse(readFileSync(join(fixture, "🔨️core/dist/result.json"), "utf8")).value, 43);
  watcher.kill("SIGTERM");
  await new Promise((accept) => watcher!.once("close", accept));
  watcher = undefined;
  const wrapperProcess = spawn("bun", ["./📜️script.ts", "nx", "watch", "--projects=probe-app", "--initialRun", "--", "bun", "./📜️script.ts", "stubborn"], { cwd: fixture, env });
  wrapperProcess.stdout!.on("data", (data) => appendFileSync(join(fixture, "cancellation.log"), data));
  wrapperProcess.stderr!.on("data", (data) => appendFileSync(join(fixture, "cancellation.log"), data));
  let wrapperClosed = false;
  wrapperProcess.once("close", () => wrapperClosed = true);
  const exists = (pid: number): boolean => { try { process.kill(pid, 0); return true; } catch { return false; } };
  try {
    await until(() => existsSync(join(fixture, "state/child-ready")), "watch callback must start");
    const pid = Number(readFileSync(join(fixture, "state/callback-pid"), "utf8"));
    const deadline = Date.now() + cancellation.shutdownMilliseconds;
    wrapperProcess.kill(cancellation.signal);
    while (Date.now() < deadline && (!wrapperClosed || exists(pid))) await new Promise((accept) => setTimeout(accept, 100));
    assert.ok(wrapperClosed, "Cancelled Nx watcher must close every descendant output pipe");
    assert.equal(exists(pid), false, "Cancelled Nx watcher must stop an ignoring callback descendant");
  } finally {
    const pidPath = join(fixture, "state/watcher-pid");
    if (existsSync(pidPath)) {
      const pid = Number(readFileSync(pidPath, "utf8"));
      if (process.platform === "win32") Bun.spawnSync(["taskkill", "/pid", String(pid), "/t", "/f"], {stdout:"ignore",stderr:"ignore"});
      else try { process.kill(-pid, "SIGKILL"); } catch {}
    }
    if (!wrapperClosed) { wrapperProcess.kill("SIGTERM"); await new Promise((accept) => wrapperProcess.once("close", accept)); }
  }
  console.log("[DEBUG] Nx daemon Unicode graph, cache reuse, dependency watcher and callback cancellation: PASS", fixture);
  appendFileSync(join(ticket, "📓️daemon-verification.md"), `# Nx Daemon and Watch Qualification\n\nNx 21.6.11 on ${process.platform}/${process.arch} passed an isolated workspace using a copy of the actual repository discovery plugin and policy. The graph retained exact emoji project roots, a warm run reused task results, and changing an emoji-path dependency triggered a successful watch rebuild with the exact path preserved. The experiment daemon and watcher were stopped afterward. The root wrapper class was also exercised against a real Nx watcher callback that ignored SIGTERM; cancellation must close descendant pipes and terminate the child. Windows/Linux runtime remains unclaimed.\n\nReferences: [Nx workspace watching](https://nx.dev/docs/kb/workspace-watching), [Nx daemon](https://nx.dev/docs/reference/nx-daemon). Installed Nx 21 source was used to resolve flag and daemon differences from current documentation.\n`);
} finally {
  if (watcher && watcher.exitCode === null) { watcher.kill("SIGTERM"); await new Promise((accept) => watcher!.once("close", accept)); }
  await run(["daemon", "--stop"], "stop-wrapper", {}, true);
  await run(["daemon", "--stop"], "stop");
}
