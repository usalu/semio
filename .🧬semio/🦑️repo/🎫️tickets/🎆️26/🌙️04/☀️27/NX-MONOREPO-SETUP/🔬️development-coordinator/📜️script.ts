import assert from "node:assert/strict";
import { appendFileSync, existsSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import ts from "typescript";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url))), fixture = join(ticket, "🗑️generated", "development-coordinator-" + Date.now());
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const put = (path: string, value: unknown): void => { const target = join(fixture, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, typeof value === "string" ? value : JSON.stringify(value)); };
put("nx.json", { useDaemonProcess: true, plugins: [{ plugin: `./${library}/🟨️.mjs` }], cacheDirectory: ".nx/cache", namedInputs: { sharedGlobals: [] } });
put("package.json", { name: "development-coordinator-proof", private: true, scripts: { nx: "bun ./📜️script.ts nx" }, nx: { includedScripts: [] } });
put(".gitignore", "node_modules\n.nx\nstate\n**/dist\n*.log\n");
put(".nxignore", `!🔨️core/source.json\n!🧩️app/source.json\n!${library}/🟨️.mjs\n!${library}/⚡️caching/🔣️policy.json\n!${library}/🔣️taxonomy.json\n!${library}/🕸️dependencies/🧩️runtime/🟨️.mjs\n`);
for (const path of ["🟨️.mjs", "🔣️taxonomy.json", "⚡️caching/🔣️policy.json", "🕸️dependencies/🧩️runtime/🟨️.mjs"]) put(`${library}/${path}`, readFileSync(join(root, library, path), "utf8"));
for (const [name, directory, dependencies] of [["probe-core", "🔨️core", []], ["probe-app", "🧩️app", ["probe-core"]]] as const) {
  put(`${directory}/📋️project.json`, { name, implicitDependencies: dependencies, targets: { build: { cache: true, inputs: ["default", "^default"], outputs: ["{projectRoot}/dist"], dependsOn: ["^build"], options: { command: `bun ./📜️script.ts build ${directory}`, cwd: "." } }, ...(name === "probe-app" ? { serve: { cache: false, continuous: true, outputs: [], dependsOn: ["build"], options: { command: "bun ./📜️script.ts server", cwd: "." } } } : {}) } });
  put(`${directory}/source.json`, { value: 42 });
}
const source = ts.createSourceFile("root.ts", readFileSync(join(root, library, "⚡️caching/🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "NxScript")!;
const wrapper = ts.transpileModule(declaration.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
put("📜️script.ts", String.raw`import { createRequire } from "node:module";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawn, spawnSync as stopNxProcessTree } from "node:child_process";
import { join } from "node:path";
mkdirSync("state", { recursive: true });
if (process.argv[2] === "build") {
  const directory = process.argv[3];
  mkdirSync(directory + "/dist", { recursive: true });
  writeFileSync(directory + "/dist/result.json", readFileSync("🔨️core/source.json"));
  appendFileSync("state/builds.jsonl", JSON.stringify({ directory }) + "\n");
} else if (process.argv[2] === "server") {
  const child = spawn("bun", ["./📜️script.ts", "child"], { stdio: "inherit" });
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => new Response(readFileSync("🧩️app/dist/result.json")) });
  writeFileSync("state/server.json", JSON.stringify({ port: server.port, pid: process.pid, child: child.pid }));
} else if (process.argv[2] === "child") {
  process.on("SIGTERM", () => {});
  writeFileSync("state/child-ready", "ready");
  setInterval(() => {}, 1000);
}
` + String.raw`
class Script { root = process.cwd(); }
const resolveNxInvocation = (args) => ({ args, env: {}, ...(args[0] === "run" && args[1] === "probe-app:serve" ? { watch: "probe-app:build" } : {}) });
const devToolingEnv = (extra) => ({ ...process.env, ...extra });
const orchestratorBudgetOpts = () => ({});
const spawnNxProcess = (...args) => { const child = spawn(...args); appendFileSync("state/children.jsonl", JSON.stringify({ pid: child.pid }) + "\n"); return child; };
${wrapper}
if (process.argv[2] === "nx") await new NxScript().run(process.argv.slice(3));
`);
const runtimeModules = process.env.SEMIO_NX_QUALIFICATION_MODULES ?? join(root, "node_modules");
symlinkSync(runtimeModules, join(fixture, "node_modules"), process.platform === "win32" ? "junction" : "dir");
const env = { ...process.env, NX_DAEMON: "true", NX_ISOLATE_PLUGINS: process.env.SEMIO_NX_QUALIFICATION_ISOLATION ?? "false", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_WORKSPACE_DATA_DIRECTORY: join(fixture, ".nx/workspace-data"), npm_lifecycle_event: undefined, npm_lifecycle_script: undefined };
for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_SKIP_NX_CACHE"].includes(key)) delete env[key];
const child = spawn("bun", ["./📜️script.ts", "nx", "run", "probe-app:serve", "--output-style=stream"], { cwd: fixture, env });
for (const stream of [child.stdout!, child.stderr!]) stream.on("data", (chunk) => appendFileSync(join(fixture, "coordinator.log"), chunk));
let closed = false;
const done = new Promise<number | null>((accept) => child.once("close", (code) => { closed = true; accept(code); }));
const until = async (condition: () => boolean, label: string): Promise<void> => { const deadline = Date.now() + 120000; while (!condition()) { assert.ok(Date.now() < deadline && !closed, label); await new Promise((accept) => setTimeout(accept, 100)); } };
const alive = (pid: number): boolean => { try { process.kill(pid, 0); return true; } catch { return false; } };
try {
  if (process.argv[2] === "startup") {
    const launches = join(fixture, "state/children.jsonl");
    await until(() => existsSync(launches), "Nx watcher must be launched before cancellation");
    child.kill("SIGTERM");
    assert.equal(await done, 143, "Startup cancellation must retain its signal exit status");
    for (const row of readFileSync(launches, "utf8").trim().split("\n").map((line) => JSON.parse(line))) assert.equal(alive(row.pid), false);
    console.log("[DEBUG] Nx watcher startup cancellation: exit 143 and no surviving child PASS", fixture);
  } else {
  await until(() => existsSync(join(fixture, "state/child-ready")) && existsSync(join(fixture, "state/server.json")), "Nx-owned server must become ready");
  const server = JSON.parse(readFileSync(join(fixture, "state/server.json"), "utf8")), url = `http://127.0.0.1:${server.port}`;
  assert.deepEqual(await fetch(url).then((response) => response.json()), { value: 42 });
  put("🔨️core/source.json", { value: 43 });
  await until(() => JSON.parse(readFileSync(join(fixture, "🧩️app/dist/result.json"), "utf8")).value === 43, "Nx watch must rebuild the dependency closure");
  assert.deepEqual(await fetch(url).then((response) => response.json()), { value: 43 });
  await new Promise((accept) => setTimeout(accept, 1000));
  const builds = readFileSync(join(fixture, "state/builds.jsonl"), "utf8");
  put("state/report.json", { generated: true });
  await new Promise((accept) => setTimeout(accept, 1000));
  assert.equal(readFileSync(join(fixture, "state/builds.jsonl"), "utf8"), builds);
  const daemonPid = JSON.parse(readFileSync(join(fixture, ".nx/workspace-data/d/server-process.json"), "utf8")).processId;
  child.kill("SIGTERM");
  const code = await done;
  assert.equal(code, 143);
  assert.equal(alive(server.pid), false);
  assert.equal(alive(server.child), false);
  await assert.rejects(fetch(url));
  assert.equal(alive(daemonPid), true, "Cancellation must preserve the shared Nx daemon");
  console.log("[DEBUG] Nx outer watcher plus server: source dependency rebuild, ignored report, HTTP readiness, descendant and port cancellation PASS", fixture);
  writeFileSync(join(ticket, "📓️development-coordinator.md"), "# Development Coordinator Verification\n\nThe actual root Nx coordinator class passed an isolated real Nx workspace. It registered source watching before starting the server graph, rebuilt the transitive dependency after an emoji-path edit, served the changed artifact over HTTP, ignored generated reports, and stopped the server plus an ignoring descendant and released its port on SIGTERM. The observed parent exit was 143. This validates the coordinator on macOS; complete product preparation remains separately tracked.\n");
  }
} finally {
  if (!closed) { child.kill("SIGTERM"); await done; }
  const pids = join(fixture, "state/children.jsonl");
  if (existsSync(pids)) for (const row of readFileSync(pids, "utf8").trim().split("\n").filter(Boolean).map((line) => JSON.parse(line))) if (process.platform === "win32") Bun.spawnSync(["taskkill", "/pid", String(row.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" }); else try { process.kill(-row.pid, "SIGKILL"); } catch {}
  const stop = spawn("node", [join(fixture, "node_modules", "nx", JSON.parse(readFileSync(join(runtimeModules, "nx/package.json"), "utf8")).bin.nx), "daemon", "--stop"], { cwd: fixture, env, stdio: "ignore" });
  await new Promise((accept) => stop.once("close", accept));
}
