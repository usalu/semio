// Validates every dry-run fixture pair: both sides import cleanly, before≠after differs (equal:false),
// and identical (before,before) compares equal:true — the gate proven in both directions per-fixture.
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const dir = process.argv[2]!;
const probesScript = process.argv[3]!;
const ids = readdirSync(dir).sort();
let failures = 0;
let acceptGood = 0;
let rejectBad = 0;
for (const id of ids) {
  const before = join(dir, id, "before.gltf");
  const after = join(dir, id, "after.gltf");
  const runImport = (path: string) => JSON.parse(spawnSync("bun", [probesScript, "gltf-import", "--input", path], { encoding: "utf8" }).stdout);
  const bi = runImport(before);
  const ai = runImport(after);
  if (bi.status !== "ok" || ai.status !== "ok") {
    console.log(`[IMPORT-FAIL] ${id}: before=${bi.status} after=${ai.status}`);
    failures += 1;
    continue;
  }
  const runCompare = (a: string, b: string) => JSON.parse(spawnSync("bun", [probesScript, "gltf-compare", "--input", a, "--input", b], { encoding: "utf8" }).stdout);
  const selfCompare = runCompare(before, before);
  const diffCompare = runCompare(before, after);
  const goodOk = selfCompare.measurements.equal === true && selfCompare.measurements.diffCount === 0;
  const badOk = diffCompare.measurements.equal === false && diffCompare.measurements.diffCount > 0;
  if (goodOk) acceptGood += 1;
  if (badOk) rejectBad += 1;
  if (!goodOk || !badOk) {
    console.log(`[GATE-FAIL] ${id}: self-equal=${selfCompare.measurements.equal} diff-equal=${diffCompare.measurements.equal} diffCount=${diffCompare.measurements.diffCount}`);
    failures += 1;
  }
}
console.log(`\n${ids.length} fixtures checked. import-ok for all: ${failures === 0 ? "yes" : "no"}. accept-known-good: ${acceptGood}/${ids.length}. reject-known-bad: ${rejectBad}/${ids.length}. failures: ${failures}`);
process.exit(failures > 0 ? 1 : 0);
