import { readFileSync } from "fs";
import {
  getKitDiff,
  areKitDiffsEqual,
} from "/workspaces/semio/compose/js/compose";

const ASSETS = "/workspaces/semio/compose/assets/compose";
const kitRaw = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism.json`, "utf-8"));
const kitOriginal = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitDiffed = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism_diffed.json`, "utf-8"));
const kitDiff = JSON.parse(readFileSync(`${ASSETS}/diff_kit_metabolism.json`, "utf-8"));
const computedDiff = getKitDiff(kitOriginal, kitDiffed);

// Get the Base type update diff
const typeUpdate = (computedDiff as any).types.updated[0];
const typeUpdateAsset = (kitDiff as any).types.updated[0];

console.log("Type guid match:", typeUpdate.type.guid === typeUpdateAsset.type.guid);

// Test each field in the type diff independently
const diffKeys = [...new Set([...Object.keys(typeUpdate.diff), ...Object.keys(typeUpdateAsset.diff)])];
for (const dk of diffKeys) {
  const partial1: any = { types: { updated: [{ type: typeUpdate.type, diff: { [dk]: typeUpdate.diff[dk] } }] } };
  const partial2: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { [dk]: typeUpdateAsset.diff[dk] } }] } };
  const result = areKitDiffsEqual(partial1, partial2);
  if (!result) {
    console.log(`FAIL: type diff field "${dk}"`);
    console.log(`  computed: ${JSON.stringify(typeUpdate.diff[dk])?.slice(0, 200)}`);
    console.log(`  asset:   ${JSON.stringify(typeUpdateAsset.diff[dk])?.slice(0, 200)}`);
  } else {
    console.log(`PASS: type diff field "${dk}"`);
  }
}
