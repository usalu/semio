import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, existsSync, lstatSync, mkdirSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", `wgpu-restore-${Date.now()}`);
mkdirSync(evidence, { recursive: true });
const output = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/dist/native-dev");
const snapshot = async (): Promise<unknown> => Object.fromEntries(await Promise.all(readdirSync(output).sort().map(async (path) => {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(join(output, path))) hash.update(chunk);
  const stat = lstatSync(join(output, path));
  return [path, { hash: hash.digest("hex"), mode: stat.mode & 0o777, size: stat.size }];
})));
const run = async (name: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--output-style=stream"], { cwd: root, env: { ...process.env, npm_lifecycle_event: "", npm_lifecycle_script: "" }, stdout: "pipe", stderr: "pipe" });
  const [out, err, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, name + ".log"), out + err);
  assert.equal(code, 0, out + err); return out;
};
await run("warm");
const before = await snapshot(), backup = join(evidence, "backup");
renameSync(output, backup);
try {
  assert.match(await run("restore"), /local cache|existing outputs match the cache/);
  assert.deepEqual(await snapshot(), before);
} finally { if (!existsSync(output)) renameSync(backup, output); }
const executable = join(output, "semio-wgpu-native" + (process.platform === "win32" ? ".exe" : ""));
const env: Record<string, string | undefined> = {};
for (const key of ["PATH", "HOME", "SystemRoot", "USERPROFILE", "TMPDIR", "TEMP", "TMP"]) env[key] = process.env[key];
env.SEMIO_DIRECT_CHILD_BENIGN = "preserved";
const consumer = Bun.spawn([executable, "--credential-probe"], { cwd: root, env, stdout: "pipe", stderr: "pipe", timeout: 30000 });
const [stdout, stderr, code] = await Promise.all([new Response(consumer.stdout).text(), new Response(consumer.stderr).text(), consumer.exited]);
writeFileSync(join(evidence, "consumer.log"), stdout + stderr);
assert.equal(code, 0, stdout + stderr);
assert.match(stdout, /native-credential-probe-ok/);
writeFileSync(join(ticket, "📓️wgpu-artifacts.md"), "# WGPU Native Artifact Restoration\n\nThe native development producer restored a deleted output directory from Nx with identical hashes, sizes and modes. The restored executable passed its real credential process probe and launched its own child successfully. The native binary producer no longer compiles plugin catalogs. Development and release have disjoint staged outputs; only development was executed here. The ordinary application runner still owns plugin preparation, which remains part of the development graph refactor.\n\n" + JSON.stringify(before, null, 2) + "\n");
console.log("[DEBUG] WGPU native Nx restoration and executable child consumer: PASS");
