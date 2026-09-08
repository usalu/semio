import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, symlinkSync, existsSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";

const workspace = process.cwd(), ticket = resolve(import.meta.dir, "../.."), require = createRequire(import.meta.url);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const cases = JSON.parse(readFileSync(join(workspace, library, "⚡️caching/🧫️fixtures/bun-dependencies/🔣️.json"), "utf8"));
const root = mkdtempSync(join(ticket, "🗑️generated/bun-dependencies-cache-"));
const put = (path: string, content: string | Buffer) => { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, content); };
for (const path of [library + "/🟨️.mjs", library + "/⚡️caching/🔣️policy.json"]) put(path, readFileSync(join(workspace, path)));
put("bun.lock", JSON.stringify(cases.lock));
for (const [path, content] of Object.entries(cases.patches)) put(path, String(content));
put("package.json", '{"name":"locked-input-fixture","private":true}');
put("nx.json", JSON.stringify({ pluginsConfig: { "@nx/js": { analyzePackageJson: false, analyzeSourceFiles: false, analyzeLockfile: false } }, plugins: [{ plugin: "./" + library + "/🟨️.mjs", options: { analyzeLockfile: true } }], maxCacheSize: "64MB", parallel: 1 }));
put(".nxignore", ".nx\nstate\n**/dist\nnode_modules\n");
symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
for (const [name, key] of [["hoisted", "api"], ["nested", "@fixture/document/api"]]) {
  put(name + "/📜️script.ts", 'import { mkdirSync, writeFileSync, appendFileSync } from "node:fs"; mkdirSync("dist", {recursive:true}); mkdirSync("../state", {recursive:true}); writeFileSync("dist/result", "owned result\\n"); appendFileSync("../state/' + name + '", "run\\n");');
  put(name + "/📋️project.json", JSON.stringify({ name, targets: { build: { cache: true, inputs: ["{projectRoot}/📜️script.ts", { externalDependencies: ["npm:" + key] }], outputs: ["{projectRoot}/dist"], options: { command: "bun ./📜️script.ts build", forwardAllArgs: false } } } }));
}
const env: Record<string, string | undefined> = { ...process.env, NX_DAEMON: "true", NX_ISOLATE_PLUGINS: "true", NX_NATIVE_COMMAND_RUNNER: "false", NX_TUI: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), FORCE_COLOR: "0", NX_VERBOSE_LOGGING: "false", NX_NATIVE_LOGGING: undefined };
for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_SKIP_NX_CACHE", "npm_lifecycle_event", "npm_lifecycle_script", "NO_COLOR"].includes(key)) delete env[key];
const count = (name: string) => existsSync(join(root, "state", name)) ? readFileSync(join(root, "state", name), "utf8").trim().split("\n").length : 0;
let index = 0;
const run = async (scenario: string, project: string, executions: number) => {
  const child = Bun.spawn(["node", require.resolve("nx/bin/nx.js"), "run", project + ":build", "--output-style=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  const cancel = () => child.kill(); process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  try {
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    writeFileSync(join(ticket, "🗑️generated", `bun-cache-${index++}.log`), stdout + stderr);
    assert.equal(code, 0, `${scenario}: ${root}`); assert.equal(count(project), executions, scenario);
    assert.equal(readFileSync(join(root, project, "dist/result"), "utf8"), "owned result\n");
    console.log(`[DEBUG] Bun actual Nx ${scenario}: ${project} producer executions=${executions}`);
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
};
try {
  await run("cold", "hoisted", 1); await run("warm", "hoisted", 1); await run("nested cold", "nested", 1);
  rmSync(join(root, "hoisted/dist"), { recursive: true }); await run("deleted output restore", "hoisted", 1);
  const counts: Record<string, number> = { hoisted: 1, nested: 1 };
  for (const row of cases.mutations) {
    const lock = structuredClone(cases.lock); lock.packages[row.package][3] += "-changed";
    put("bun.lock", JSON.stringify(lock));
    const project = row.root === "api" ? "hoisted" : "nested";
    if (row.changed) counts[project]++;
    await run(row.name, project, counts[project]);
  }
  put("bun.lock", JSON.stringify(cases.lock)); put("patches/core.patch", "changed patch bytes\n");
  await run("patch byte change", "hoisted", ++counts.hoisted);
  await run("unrelated version patch", "nested", counts.nested);
  console.log("[DEBUG] Actual daemon-backed Nx Bun dependency cache and clean restoration PASS");
} finally {
  const stop = Bun.spawn(["node", require.resolve("nx/bin/nx.js"), "reset", "--onlyDaemon"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  await Promise.all([new Response(stop.stdout).text(), new Response(stop.stderr).text(), stop.exited]);
}
