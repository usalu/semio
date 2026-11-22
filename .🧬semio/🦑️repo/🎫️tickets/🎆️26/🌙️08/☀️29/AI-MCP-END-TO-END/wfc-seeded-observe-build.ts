import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { open } from "node:fs/promises";
import { join, resolve } from "node:path";

const [buildLog, builtGateway] = process.argv.slice(2).map(path => resolve(path));
assert(buildLog && builtGateway, "Build log and rebuilt native MCP gateway are required");
const baseline = "a1766c3f0e87cfee365c24c8461d1ad8aeec4983bb2a217806e2b22e576e004b";
const deadline = Date.now() + 60 * 60_000;
console.log("[DEBUG] Waiting for native gateway Cargo build", buildLog);
while (Date.now() < deadline) {
  const log = await open(buildLog, "r");
  let tail: string;
  try {
    const size = (await log.stat()).size;
    const buffer = Buffer.alloc(Math.min(size, 128 * 1024));
    const { bytesRead } = await log.read(buffer, 0, buffer.length, size - buffer.length);
    tail = buffer.toString("utf8", 0, bytesRead);
  } finally {
    await log.close();
  }
  assert(!/^error(?:\[|:)|\bcould not compile\b/mu.test(tail), "The combined native gateway Cargo build returned an error");
  if (/^\s*Finished .+ target\(s\)/mu.test(tail)) {
    const hash = createHash("sha256");
    for await (const chunk of createReadStream(builtGateway)) hash.update(chunk);
    const sha256 = hash.digest("hex");
    assert.notEqual(sha256, baseline, "Cargo completion did not publish a rebuilt executable");
    console.log("[DEBUG] Combined executable SHA-256", sha256);
    const outcomes: Record<string, number | string> = {};
    for (const task of ["solve", "cancel"]) {
      try {
        const options = [join(import.meta.dirname, "wfc-seeded-sdk.ts"), task, `--gateway=${builtGateway}`, "--schema-probe", "--label=combined"];
        const child = Bun.spawn([process.execPath, ...options], { stdout: "inherit", stderr: "inherit", signal: AbortSignal.timeout(180_000) });
        outcomes[task] = await child.exited;
      } catch (error) {
        outcomes[task] = String(error);
      }
      console.log("[DEBUG] Combined witness outcome", task, outcomes[task]);
    }
    process.exit(Object.values(outcomes).some(status => status !== 0) ? 1 : 0);
  }
  await Bun.sleep(5_000);
}
throw Error("The combined native gateway Cargo build did not finish within the observation window");
