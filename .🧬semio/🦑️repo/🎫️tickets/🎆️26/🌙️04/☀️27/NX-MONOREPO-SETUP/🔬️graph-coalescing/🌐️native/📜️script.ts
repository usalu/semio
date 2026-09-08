import assert from "node:assert/strict";
import { appendFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

const root = process.cwd(), ticket = dirname(dirname(import.meta.dir)), generated = join(ticket, "🗑️generated");
const candidate = process.argv[2], isolation = process.argv[3] ?? "true", label = `${candidate ? "candidate" : "baseline"}-${isolation === "true" ? "isolated" : "in-process"}`;
assert.ok(["true", "false"].includes(isolation));
const fixture = mkdtempSync(join(generated, `graph-native-${label}-`)), nxRoot = join(fixture, "node_modules/nx");
const require = createRequire(join(root, "package.json")), installedNx = dirname(require.resolve("nx/package.json"));
mkdirSync(dirname(nxRoot), { recursive: true });
cpSync(installedNx, nxRoot, { recursive: true });
if (candidate) writeFileSync(join(nxRoot, "dist/src/daemon/server/project-graph-incremental-recomputation.js"), readFileSync(candidate));
const put = (name: string, value: unknown) => { const file = join(fixture, name); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, typeof value === "string" ? value : JSON.stringify(value)); };
put("nx.json", { useDaemonProcess: true, plugins: ["./plugin/📜️script.ts"], neverConnectToCloud: true });
put("package.json", { name: "graph-coalescing-fixture", private: true });
put(".gitignore", "node_modules\n.nx\nstate\n");
put(".nxignore", "state/**\n!fixture/source.json\n!fixture/project.json\n!plugin/📜️script.ts\n");
put("fixture/project.json", { name: "fixture", targets: {} });
put("fixture/source.json", { revision: 0 });
put("state/release", "ready");
put("plugin/📜️script.ts", String.raw`import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
export const createNodesV2 = ["fixture/project.json", async (files, _options, context) => {
  const root = context.workspaceRoot, value = JSON.parse(readFileSync(join(root, "fixture/source.json"), "utf8"));
  const event = (phase) => appendFileSync(join(root, "state/events.jsonl"), JSON.stringify({ phase, value, pid: process.pid }) + "\n");
  event("start");
  const deadline = Date.now() + 45000;
  while (!existsSync(join(root, "state/release"))) { if (Date.now() > deadline) throw new Error("Fixture gate timeout"); await new Promise(accept => setTimeout(accept, 20)); }
  event("end");
  return files.map(file => [file, { projects: { fixture: { name: "fixture", root: "fixture", metadata: value } } }]);
}];
`);
const env = { ...process.env, NX_WORKSPACE_ROOT: fixture, NX_WORKSPACE_DATA_DIRECTORY: join(fixture, ".nx/workspace-data"), NX_DAEMON: "true", NX_ISOLATE_PLUGINS: isolation, NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_VERBOSE_LOGGING: "false", NX_NO_CLOUD: "true" };
for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_SKIP_NX_CACHE", "npm_lifecycle_event", "npm_lifecycle_script"].includes(key)) delete env[key];
const cli = join(nxRoot, "dist/bin/nx.js"), children = new Set<ReturnType<typeof Bun.spawn>>();
const run = async (name: string, args: string[], overrides = {}) => {
  const log = join(fixture, "state", name + ".log"), child = Bun.spawn(["node", cli, ...args], { cwd: fixture, env: { ...env, ...overrides }, stdout: "pipe", stderr: "pipe" });
  children.add(child);
  const drain = async (stream: ReadableStream<Uint8Array>) => { for await (const bytes of stream) appendFileSync(log, bytes); };
  const timeout = setTimeout(() => child.kill("SIGTERM"), 90000);
  try { const [,,status] = await Promise.all([drain(child.stdout), drain(child.stderr), child.exited]); assert.equal(status, 0, `${name}: ${readFileSync(log, "utf8").slice(-6000)}`); }
  finally { clearTimeout(timeout); children.delete(child); }
};
const events = () => existsSync(join(fixture, "state/events.jsonl")) ? readFileSync(join(fixture, "state/events.jsonl"), "utf8").trim().split("\n").filter(Boolean).map(line => JSON.parse(line)) : [];
const until = async (test: () => boolean) => { const deadline = Date.now() + 60000; while (!test()) { assert.ok(Date.now() < deadline, "Native discovery did not reach its gate"); await new Promise(accept => setTimeout(accept, 50)); } };
const graph = (name: string, overrides = {}) => run(name, ["graph", `--file=state/${name}.json`], overrides);
const cancel = () => { put("state/release", "cancelled"); for (const child of children) child.kill("SIGTERM"); };
process.on("SIGINT", cancel); process.on("SIGTERM", cancel);
console.log(`[DEBUG] Native ${label} graph fixture: ${fixture}`);
try {
  await graph("initial");
  const baseline = events().length;
  rmSync(join(fixture, "state/release"));
  put("fixture/source.json", { revision: 1 });
  const first = graph("first");
  await until(() => events().slice(baseline).some(row => row.phase === "start"));
  const peers = Array.from({ length: 6 }, (_, index) => graph(`peer-${index}`));
  for (let revision = 2; revision <= 9; revision++) { put("fixture/source.json", { revision }); await new Promise(accept => setTimeout(accept, 300)); }
  await new Promise(accept => setTimeout(accept, 1500));
  put("state/release", "ready");
  await Promise.all([first, ...peers]);
  const sequence = events().slice(baseline);
  let active = 0, maximum = 0;
  for (const event of sequence) { active += event.phase === "start" ? 1 : -1; maximum = Math.max(maximum, active); }
  assert.equal(active, 0);
  const computations = sequence.filter(event => event.phase === "start").length;
  await graph("oracle", { NX_DAEMON: "false" });
  const metadata = (name: string) => JSON.parse(readFileSync(join(fixture, `state/${name}.json`), "utf8")).graph.nodes.fixture.data.metadata;
  const expected = metadata("oracle");
  assert.deepEqual(expected, { revision: 9 });
  for (const name of ["first", ...peers.map((_, index) => `peer-${index}`)]) assert.deepEqual(metadata(name), expected, name + ": stale graph");
  if (candidate) { assert.equal(maximum, 1); assert.equal(computations, 2); }
  console.log(`[DEBUG] Native ${label}: ${computations} computations, concurrency ${maximum}; all seven clients match daemon-disabled Nx at revision 9 PASS`);
} finally {
  put("state/release", "ready");
  await run("stop", ["daemon", "--stop"]);
  process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
}
