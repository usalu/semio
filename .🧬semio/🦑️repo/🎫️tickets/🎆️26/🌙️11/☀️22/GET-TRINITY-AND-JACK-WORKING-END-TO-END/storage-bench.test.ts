import { test } from "bun:test";
import { withIssuedLawStorageV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";

test("storage recheck cost", async () => {
  await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "storage-bench", maximumElapsedMilliseconds: 0 }, {}, Date.now()), (original: any) =>
    withIssuedLawStorageV1(original, new AbortController().signal, async (held) => {
      const cpu0 = process.cpuUsage(), start = performance.now();
      for (let index = 0; index < 300; index++) await held.storage.recheck();
      const cpu = process.cpuUsage(cpu0);
      console.log(JSON.stringify({ calls: 300, wallMs: Math.round(performance.now() - start), cpuMs: Math.round((cpu.user + cpu.system) / 1000) }));
    }),
  );
}, 180000);
