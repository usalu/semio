/** 🧪️ Runs the authored transport worker with its genuine issued birth and independent native stdin. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, scriptInvocationEnvironment, withScriptProcessEnvelope } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { captureOwnedCommandResultV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";

const worker = resolve("🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/📜️script.ts");
const input = "ordinary stdin α🙂";
const modes = process.argv.includes("--capture") ? ["inherit"] : ["inherit", "large", "beyond", "early-close", "cancel", "deadline", "consumer-primary", "status", "large-stream"];
for (const mode of modes) {
  await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "process:auxiliary-fallback", maximumElapsedMilliseconds: 20000 }, {}, Date.now()), async original => {
    if (process.argv.includes("--capture")) {
      const result = await captureOwnedCommandResultV1(process.execPath, [worker, mode], process.cwd(), {
        input: { version: 1, text: input },
        control: createIssuedResourceControlV1(original, "process:auxiliary-fixture-worker", 20000),
        env: scriptInvocationEnvironment(original, process.env),
        output: { maximumBytes: 8192, maximumLines: 64 },
      });
      console.error("[DEBUG] captured worker", result);
      return;
    }
    const child = Bun.spawn([process.execPath, worker, mode], {
      stdin: Buffer.from(input),
      stdout: "pipe",
      stderr: "pipe",
      env: scriptInvocationEnvironment(original, process.env),
    });
    const [stdout, stderr, status] = await Promise.all([child.stdout.text(), child.stderr.text(), child.exited]);
    assert.equal(status, 0, stderr);
    const row = JSON.parse(stdout);
    console.error("[DEBUG]", row);
    assert.equal(row.mode, mode);
    assert.equal(row.birthMilliseconds, 20000);
    if (!["early-close", "deadline"].includes(mode)) assert(row.live);
    if (["inherit", "large", "beyond", "status"].includes(mode)) {
      const text = mode === "beyond" ? "a".repeat(33554433) : mode === "large" ? "α".repeat(1048577) : "Source🙂";
      assert.deepEqual(row.ordinary, { ordinary: input, bytes: Buffer.byteLength(text), sha256: createHash("sha256").update(text).digest("hex") });
    }
    if (mode === "large-stream") {
      assert.equal(row.streamedBytes, 30 * 1048576);
      assert.deepEqual(row.summary, { ordinary: input, bytes: 2097154 });
    }
    assert.equal(row.failed, !["inherit", "large", "beyond", "large-stream"].includes(mode));
    if (mode === "early-close") assert.match(row.message, /pipe|EPIPE|stream|ECONNRESET/iu);
    if (["cancel", "consumer-primary"].includes(mode)) assert(row.samePrimary);
    if (mode === "deadline") assert(row.sameDeadline);
    if (mode === "status") assert.equal(row.status, 7);
  });
}
