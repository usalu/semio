import { expect, test } from "bun:test";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests");
const output = join(dirname(import.meta.dir), "🗑️generated/current-library-inline");
mkdirSync(output, { recursive: true });
for (const [name, filter] of [
  ["🧬️mutation-authority", ""],
  ["🕰️historical-json-source-encoding", ""],
  ["🧲️rust-physical-reference-context", "Rust diagnostic references require exact unescaped assertion-message arguments|independent syn parsing"],
  ["🛟️transaction-recovery-authority", ""],
  ["❄️frozen-markdown-coordinates", ""],
  ["🚪️artifact-io-ownership", ""],
  ["🔎️json-reference-owner-lookup", ""],
  ["🛤️typescript-path-collection", ""],
  ["🔬️workspace-contract", "requires a fresh clean terminal verification before mutation apply can commit"],
  ["📈️reference-coordinate-progress", ""],
  ["💥️nested-cargo-collision-authority", ""],
]) test(`actual library inline consumer ${name}`, async () => {
  const child = Bun.spawn([process.execPath, "test", join(library, name!, "🟦️.ts"), ...(filter ? ["-t", filter] : [])], {
    cwd: root,
    env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output, SEMIO_REPO_TEST_ARTIFACT_DIR: output },
    stdout: "pipe", stderr: "pipe",
  });
  const timer = setTimeout(() => child.kill("SIGTERM"), 290000);
  const stop = () => child.kill("SIGTERM");
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  try {
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    writeFileSync(join(output, name + ".log"), stdout + stderr);
    console.log(`[DEBUG] ${name} actual suite exit=${code}: ${stderr.replace(/\u001b\[[0-9;]*m/gu, "").split("\n").filter((line) => /pass|fail|filtered|Ran \d|error:|Error:/u.test(line)).join(" | ")}`);
    expect(code, name).toBe(0);
  } finally {
    clearTimeout(timer);
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
  }
}, 300000);
