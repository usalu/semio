import { readFileSync } from "node:fs";
import { join } from "node:path";
import { inspectPortableMsvcTreeV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📦️inventory/🔎️verify/🟦️.ts";
const root = process.argv[2]!;
const inventory = JSON.parse(readFileSync(join(root, ".semio-inventory.json"), "utf8"));
const control = { signal: new AbortController().signal, remainingMilliseconds: () => null, publish: () => {}, yieldContinuation: async () => {} } as any;
for (let i = 0; i < 2; i++) {
  const start = performance.now(), cpu = process.cpuUsage();
  const row = await inspectPortableMsvcTreeV1(root, inventory, control, { inventoryFile: true });
  const used = process.cpuUsage(cpu);
  console.error("[DEBUG] msvc inspect", i, "ms", Math.round(performance.now() - start), "cpu-ms", Math.round((used.user + used.system) / 1000), "files", row.files, "bytes", row.bytes);
}
