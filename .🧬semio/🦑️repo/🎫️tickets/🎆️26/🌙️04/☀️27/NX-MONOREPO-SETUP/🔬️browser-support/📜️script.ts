import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
if (process.argv[2] === "inspect") {
  const shim = await import(pathToFileURL(join(process.argv[3], "io.js")).href);
  assert.ok(shim.poll && shim.streams);
  console.log("[DEBUG] Restored Preview2 IO module imported with Node");
  process.exit(0);
}
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", "browser-support-" + Date.now());
mkdirSync(evidence, { recursive: true });
const artifactRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules");
const outputs = ["🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim", "🧵️shard"];
const snapshot = (): Record<string, unknown> => {
  const result: Record<string, unknown> = {};
  const walk = (path: string): void => {
    for (const entry of readdirSync(path, { withFileTypes: true })) {
      const child = join(path, entry.name);
      if (entry.isDirectory()) walk(child);
      else result[relative(artifactRoot, child)] = { sha256: createHash("sha256").update(readFileSync(child)).digest("hex"), mode: lstatSync(child).mode & 0o777 };
    }
  };
  for (const path of outputs) walk(join(artifactRoot, path));
  return result;
};
const run = async (label: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/framework-plugin-web:support-dev", "--output-style=stream"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, label + ".log"), stdout + stderr);
  assert.equal(code, 0, stdout + stderr);
  return stdout;
};
await run("warm");
const before = snapshot();
for (const [index, path] of outputs.entries()) renameSync(join(artifactRoot, path), join(evidence, "backup-" + index));
try {
  assert.match(await run("restore"), /local cache|existing outputs match the cache/);
  assert.deepEqual(snapshot(), before);
} finally {
  for (const [index, path] of outputs.entries()) if (!existsSync(join(artifactRoot, path))) renameSync(join(evidence, "backup-" + index), join(artifactRoot, path));
}
const oracle = Bun.spawn(["node", fileURLToPath(import.meta.url), "inspect", join(artifactRoot, outputs[0])], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, code] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
writeFileSync(join(evidence, "oracle.log"), stdout + stderr);
assert.equal(code, 0, stdout + stderr);
writeFileSync(join(ticket, "📓️browser-support-artifacts.md"), "# Browser Support Restoration\n\nNx restored both deleted support directories with identical hashes and modes. The independent Node runtime imported the restored Preview2 IO module and exposed poll/streams interfaces. The shard worker was restored byte-identically; this probe does not claim browser worker activation.\n\n" + Object.keys(before).length + " files verified.\n");
console.log("[DEBUG] Browser support Nx restoration and independent Node module consumer: PASS");
