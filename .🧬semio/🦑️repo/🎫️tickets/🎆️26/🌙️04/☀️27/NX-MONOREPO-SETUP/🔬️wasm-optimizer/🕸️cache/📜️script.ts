import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
const workspace = resolve(import.meta.dir, "../../../../../../../../.."), ticket = resolve(import.meta.dir, "../.."), output = join(ticket, "🗑️generated");
const tooling = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts"));
const fixture = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🔣️.json"), "utf8"));
const action = process.argv[2];
if (action === "identity") console.log(tooling.binaryenIdentity());
else if (action === "prepare") { await tooling.prepareBinaryen(process.cwd(), AbortSignal.timeout(120000)); appendFileSync("events", "prepare\n"); }
else if (action === "build") {
  mkdirSync("out", { recursive: true });
  const result = Bun.spawnSync([tooling.preparedBinaryen(process.cwd()), "input.wat", "-o", "out/module.wasm", ...fixture.optimizer], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
  assert.equal(result.exitCode, 0, result.stderr.toString()); appendFileSync("events", "build\n");
} else {
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "binaryen-nx-")), require = createRequire(join(workspace, "package.json")), command = `bun ${JSON.stringify(join(import.meta.dir, "📜️script.ts"))}`;
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "optimizer-fixture", private: true }));
  writeFileSync(join(root, "nx.json"), JSON.stringify({ cacheDirectory: ".nx/cache", parallel: 1 }));
  writeFileSync(join(root, "project.json"), JSON.stringify({ name: "optimizer", targets: {
    prepare: { executor: "nx:run-commands", cache: false, outputs: [], options: { command: command + " prepare" } },
    build: { executor: "nx:run-commands", cache: true, dependsOn: ["prepare"], inputs: ["{projectRoot}/input.wat", { runtime: command + " identity" }], outputs: ["{projectRoot}/out"], options: { command: command + " build" } }
  } }));
  writeFileSync(join(root, "input.wat"), fixture.module);
  const run = async (label: string): Promise<void> => {
    const log = Bun.file(join(output, `binaryen-native-${label}.log`)), child = Bun.spawn(["node", require.resolve("nx/bin/nx.js"), "run", "optimizer:build", "--output-style=stream"], { cwd: root, env: { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT_PATH: root, NX_CACHE_PROJECT_GRAPH: "false" }, stdout: log, stderr: log });
    const budget = setTimeout(() => child.kill("SIGKILL"), 180000);
    try { assert.equal(await child.exited, 0, label); } finally { clearTimeout(budget); }
  };
  const hash = (): string => createHash("sha256").update(readFileSync(join(root, "out/module.wasm"))).digest("hex");
  await run("cold"); const expected = hash(); assert.equal(WebAssembly.validate(readFileSync(join(root, "out/module.wasm"))), true);
  await run("warm"); assert.equal(hash(), expected);
  rmSync(join(root, "out"), { recursive: true }); await run("restored"); assert.equal(hash(), expected);
  rmSync(tooling.binaryenDirectory(root), { recursive: true }); rmSync(join(root, "out"), { recursive: true });
  await run("tooling-restored"); assert.equal(hash(), expected);
  assert.equal(readFileSync(join(root, "events"), "utf8").split("\n").filter(line => line === "build").length, 1);
  assert.equal(readFileSync(join(root, "events"), "utf8").split("\n").filter(line => line === "prepare").length, 4);
  appendFileSync(join(root, "input.wat"), "\n"); await run("invalidated");
  assert.equal(readFileSync(join(root, "events"), "utf8").split("\n").filter(line => line === "build").length, 2);
  const binary = tooling.preparedBinaryen(root), nativeHash = Bun.spawnSync(["shasum", "-a", "256", binary], { stdout: "pipe", stderr: "pipe" });
  assert.equal(nativeHash.exitCode, 0);
  assert.equal(nativeHash.stdout.toString().split(/\s+/)[0], createHash("sha256").update(readFileSync(binary)).digest("hex"));
  assert.ok(existsSync(join(tooling.binaryenDirectory(root), ".toolchain.json")));
  console.log("[DEBUG] Native Nx optimizer cold/warm/output restoration/tool reacquisition/invalidation and system checksum parity PASS", JSON.stringify({ root, outputSha256: expected }));
}
