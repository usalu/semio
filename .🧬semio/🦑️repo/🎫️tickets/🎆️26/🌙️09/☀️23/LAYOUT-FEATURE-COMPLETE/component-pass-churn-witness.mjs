import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const [runDirectory, laneDirectory, pass, target, failingPackage] = process.argv.slice(2);
const repository = process.cwd();
const driver = resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts");
const recovery = resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🟦️.ts");
const { announcedUnitIdentityV1, unitGraphIdentityV1 } = await import(driver);
const { failedDependencyDriftV1 } = await import(recovery);
const { readPhysicalFileV1 } = await import(resolve("🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts"));
const graph = JSON.parse(await readFile(resolve(runDirectory, `unit-graph-${pass}.json`), "utf8"));
const messages = (await readFile(resolve(runDirectory, `cargo-${pass}.jsonl`), "utf8")).trim().split("\n").map(line => JSON.parse(line));
const failed = new Map(), artifacts = [];
for (const message of messages) {
  if (message.reason === "compiler-artifact") artifacts.push({ ...message, identity: announcedUnitIdentityV1(message, target) });
  if (message.reason !== "compiler-message" || message.message.level !== "error" || (failingPackage !== "*" && !message.package_id.includes(`#${failingPackage}@`)) || !message.message.code?.code) continue;
  const codes = failed.get(message.package_id) ?? new Set();
  codes.add(message.message.code.code);
  failed.set(message.package_id, codes);
}
assert.ok(failed.size, "The actual Cargo log must contain the selected error");
const lane = resolve(laneDirectory);
const deadline = performance.now() + 10_000;
const witness = await failedDependencyDriftV1({
  workspace: repository,
  build: lane,
  target: lane,
  graph: { units: graph.units.map(unit => ({ ...unit, identity: unitGraphIdentityV1(unit) })) },
  failed,
  artifacts,
  read: async path => {
    try {
      const remainingMs = () => deadline - performance.now();
      if (remainingMs() <= 0) return;
      return (await readPhysicalFileV1(path, {
        maxBytes: 262_144, chunkBytes: 65_536, maxWork: 9,
        cancelled: () => false, remainingMs, onProgress: () => {},
      })).bytes;
    } catch {
      return;
    }
  },
});
assert.ok(witness, "The actual compiled dependencies must expose a source checksum mismatch");
console.log(JSON.stringify(witness));
