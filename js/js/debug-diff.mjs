import { readFileSync, writeFileSync } from "fs";
import { getKitDiff, deepEqual } from "./semio.ts";

const kitOriginal = JSON.parse(readFileSync("../../assets/semio/kit_metabolism.json", "utf-8"));
const kitDiff = JSON.parse(readFileSync("../../assets/semio/diff_kit_metabolism.json", "utf-8"));
const kitDiffed = JSON.parse(readFileSync("../../assets/semio/kit_metabolism_diffed.json", "utf-8"));

console.log("Computing diff...");
const computedDiff = getKitDiff(kitOriginal, kitDiffed);

// Write computed diff for inspection
writeFileSync("../../temp/computed-diff.json", JSON.stringify(computedDiff, null, 2));
writeFileSync("../../temp/expected-diff.json", JSON.stringify(kitDiff, null, 2));

console.log("Comparing diffs...");
const areEqual = deepEqual(computedDiff, kitDiff);

console.log(`Diffs equal: ${areEqual}`);

if (!areEqual) {
  // Find first difference
  const findDiff = (path, a, b) => {
    if (a === b) return null;
    if (typeof a !== typeof b) return `${path}: type mismatch (${typeof a} vs ${typeof b})`;
    if (a === null || a === undefined || b === null || b === undefined) {
      if (a !== b) return `${path}: ${JSON.stringify(a)} vs ${JSON.stringify(b)}`;
      return null;
    }
    if (Array.isArray(a)) {
      if (!Array.isArray(b)) return `${path}: array vs non-array`;
      if (a.length !== b.length) return `${path}: length ${a.length} vs ${b.length}`;
      for (let i = 0; i < a.length; i++) {
        const diff = findDiff(`${path}[${i}]`, a[i], b[i]);
        if (diff) return diff;
      }
      return null;
    }
    if (typeof a === 'object') {
      const keysA = Object.keys(a).sort();
      const keysB = Object.keys(b).sort();
      if (keysA.length !== keysB.length) {
        return `${path}: key count ${keysA.length} vs ${keysB.length}. A keys: ${keysA.join(', ')}. B keys: ${keysB.join(', ')}`;
      }
      for (const key of keysA) {
        if (!keysB.includes(key)) return `${path}: missing key ${key}`;
        const diff = findDiff(`${path}.${key}`, a[key], b[key]);
        if (diff) return diff;
      }
      return null;
    }
    return `${path}: ${a} vs ${b}`;
  };

  const firstDiff = findDiff('diff', computedDiff, kitDiff);
  if (firstDiff) {
    console.log(`\nFirst difference: ${firstDiff}`);
    
    // Show designs diff specifically
    console.log(`\nComputed designs diff keys: ${Object.keys(computedDiff.designs || {}).join(', ')}`);
    console.log(`Expected designs diff keys: ${Object.keys(kitDiff.designs || {}).join(', ')}`);
    console.log(`\nComputed designs.removed: ${JSON.stringify(computedDiff.designs?.removed)}`);
    console.log(`Expected designs.removed: ${JSON.stringify(kitDiff.designs?.removed)}`);
  }
}
