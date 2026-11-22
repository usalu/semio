import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { CurrentPhysicalOwnerV1 } from "../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import { cargoProvenancePhysicalControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";

const sizes = [500, 2000, 8000];
for (const size of sizes) {
  const dir = join("G:/t/prov/en", String(size));
  mkdirSync(dir, { recursive: true });
  for (let index = 0; index < size; index++) writeFileSync(join(dir, `f${index}-${(index * 7919) % 10007}.txt`), "x");
  const owner = new CurrentPhysicalOwnerV1("G:/t/prov", cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => undefined }));
  const cpu0 = process.cpuUsage(), start = performance.now();
  const rows = await owner.directoryEntries(dir);
  const cpu = process.cpuUsage(cpu0);
  console.log(JSON.stringify({ size, rows: rows!.length, wallMs: Math.round(performance.now() - start), cpuMs: Math.round((cpu.user + cpu.system) / 1000) }));
}
