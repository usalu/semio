import * as fs from "fs";
import * as path from "path";
import { applyKitDiff, getKitDiff, inverseKitDiff, Kit, KitDiff } from "../js/js/semio";

const assetsPath = path.join(process.cwd(), "assets/semio");

const kitJson = fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8");
const kit = JSON.parse(kitJson) as Kit;

console.log("Generating kit diffs...");

const testChanges: KitDiff = {
    name: "Metabolism (Modified)",
    types: {
        updated: [
            {
                id: kit.types?.[0]?.guid || "",
                diff: {
                    description: "Modified description for testing"
                }
            }
        ]
    }
};

const kitDiffed = applyKitDiff(kit, testChanges);
fs.writeFileSync(
    path.join(assetsPath, "kit_metabolism_diffed.json"),
    JSON.stringify(kitDiffed, null, 2)
);
console.log("✓ Wrote kit_metabolism_diffed.json");

const computedDiff = getKitDiff(kit, kitDiffed);
fs.writeFileSync(
    path.join(assetsPath, "diff_kit_metabolism.json"),
    JSON.stringify(computedDiff, null, 2)
);
console.log("✓ Wrote diff_kit_metabolism.json");

const invertedDiff = inverseKitDiff(kit, computedDiff);
fs.writeFileSync(
    path.join(assetsPath, "diff_kit_metabolism_inverted.json"),
    JSON.stringify(invertedDiff, null, 2)
);
console.log("✓ Wrote diff_kit_metabolism_inverted.json");

console.log("\nDone!");
