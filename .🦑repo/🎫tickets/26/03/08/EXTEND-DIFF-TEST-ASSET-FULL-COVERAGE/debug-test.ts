import { readFileSync } from "fs";
import { getKitDiff, areKitDiffsEqual, Kit, KitDiff } from "/workspaces/semio/compose/js/compose";

const ASSETS = "/workspaces/semio/assets/compose";

const kitRaw = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism.json`, "utf-8"));
const kitOriginal = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitDiffed = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism_diffed.json`, "utf-8"));
const kitDiff = JSON.parse(readFileSync(`${ASSETS}/diff_kit_metabolism.json`, "utf-8"));

const computedDiff = getKitDiff(kitOriginal, kitDiffed);
const ok = areKitDiffsEqual(computedDiff, kitDiff);
console.log("areKitDiffsEqual:", ok);

if (!ok) {
  // Compare top-level keys
  const cKeys = Object.keys(computedDiff);
  const aKeys = Object.keys(kitDiff);
  const allKeys = [...new Set([...cKeys, ...aKeys])];

  for (const k of allKeys) {
    const cv = JSON.stringify((computedDiff as any)[k]);
    const av = JSON.stringify((kitDiff as any)[k]);
    if (cv !== av) {
      console.log(`\nKey "${k}" differs:`);
      console.log(`  computed: ${cv?.slice(0, 200)}`);
      console.log(`  asset:   ${av?.slice(0, 200)}`);
    }
  }
}
