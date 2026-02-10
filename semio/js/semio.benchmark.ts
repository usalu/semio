import DiffForward from "../assets/semio/diff_kit_metabolism.json";
import DiffInverse from "../assets/semio/diff_kit_metabolism_inverted.json";
import InvalidKit from "../assets/semio/kit_invalid.json";
import MetabolismKit from "../assets/semio/kit_metabolism.json";
import {
    applyKitDiff,
    exportKit,
    flattenDesign,
    importKit,
    Kit,
    KitDiff,
    validateKit
} from "./semio.js";

const ITERATIONS = 3;

async function bench(name: string, fn: () => Promise<void> | void) {
  const start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    await fn();
  }
  const end = performance.now();
  const durationSec = (end - start) / 1000 / ITERATIONS;
  console.log(`${name},${durationSec.toFixed(6)}`);
}

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

const kitMetabolism = MetabolismKit as unknown as Kit;
const kitInvalid = InvalidKit as unknown as Kit;
const diffForward = DiffForward as unknown as KitDiff;
const diffInverse = DiffInverse as unknown as KitDiff;

bench("Roundtrip/Metabolism", async () => {
    const fs = await import("fs");
    const path = await import("path");
    const zipPath = path.resolve("../assets/semio/metabolism.zip");
    const zipBuffer = fs.readFileSync(zipPath);

    const { kit, files } = await importKit(zipBuffer);

    const blob = await exportKit(kit, files);

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
