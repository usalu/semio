// #region 🔖Header
// [👤semio📚js🥼semiobenchmark](semiorepo://p/u/semio/b/l/js/f/semio.benchmark.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

// #region 🔖Benchmarks
// Performance benchmarks for kit roundtrip, diff, flatten and validation operations.
// MUST run each benchmark for at least ITERATIONS cycles.

import DiffForward from "../assets/semio/diff_kit_metabolism.json";
import DiffInverse from "../assets/semio/diff_kit_metabolism_inverted.json";
import InvalidKit from "../assets/semio/kit_invalid.json";
import MetabolismKit from "../assets/semio/kit_metabolism.json";
import { applyKitDiff, exportKit, flattenDesign, importKit, Kit, KitDiff, validateKit } from "./semio.js";

// Number of iterations per benchmark run.
// MUST be at least 1 for meaningful timing.
const ITERATIONS = 3;

// Runs a function multiple times, measures elapsed time and logs CSV output.
// MUST await async functions within the iteration loop.
async function bench(name: string, fn: () => Promise<void> | void) {
  const start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    await fn();
  }
  const end = performance.now();
  const durationSec = (end - start) / 1000 / ITERATIONS;
  console.log(`${name},${durationSec.toFixed(6)}`);
}

// Finds a design by name and optional parent name within a kit.
// MUST throw when the design or parent is not found.
function findDesign(kit: Kit, name: string, parentName?: string) {
  let parentGuid: string | undefined;
  if (parentName) {
    const p = kit.designs?.find((d) => d.name === parentName);
    if (!p) throw new Error(`Parent ${parentName} not found`);
    parentGuid = p.guid;
  }
  const d = kit.designs?.find((d) => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
  if (!d) throw new Error(`Design ${name} not found`);
  return d;
}

// Typed metabolism kit fixture for benchmark runs.
// MUST be cast from the imported JSON.
const kitMetabolism = MetabolismKit as unknown as Kit;

// Typed invalid kit fixture for validation benchmarks.
// MUST be cast from the imported JSON.
const kitInvalid = InvalidKit as unknown as Kit;

// Typed forward diff fixture for diff benchmarks.
// MUST be cast from the imported JSON.
const diffForward = DiffForward as unknown as KitDiff;

// Typed inverse diff fixture for diff benchmarks.
// MUST be cast from the imported JSON.
const diffInverse = DiffInverse as unknown as KitDiff;

bench("Roundtrip/Metabolism", async () => {
  const fs = await import("fs");
  const path = await import("path");
  const zipPath = path.resolve("../assets/semio/metabolism.zip");
  const zipBuffer = fs.readFileSync(zipPath);

  const { kit } = await importKit(zipBuffer);

  const blob = await exportKit(kit);
});

bench("Diff/Metabolism", () => {
  const k2 = applyKitDiff(kitMetabolism, diffForward);
  applyKitDiff(k2, diffInverse);
});

const d1 = findDesign(kitMetabolism, "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower", () => {
  flattenDesign(kitMetabolism, d1.guid);
});

const d2 = findDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Slanted", () => {
  flattenDesign(kitMetabolism, d2.guid);
});

const d3 = findDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Twisted", () => {
  flattenDesign(kitMetabolism, d3.guid);
});

const d4 = findDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Dancing", () => {
  flattenDesign(kitMetabolism, d4.guid);
});

const d5 = findDesign(kitMetabolism, "Capsule Dream");
bench("Flatten Design/Capsule Dream", () => {
  flattenDesign(kitMetabolism, d5.guid);
});

bench("Validation/Invalid Kit", () => {
  validateKit(kitInvalid);
});

bench("Validation/Metabolism", () => {
  validateKit(kitMetabolism);
});

// #endregion 🔖Benchmarks
