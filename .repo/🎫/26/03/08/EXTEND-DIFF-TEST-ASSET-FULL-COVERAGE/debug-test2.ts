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

// Check each top-level key using JSON deep comparison
const topKeys = [...new Set([...Object.keys(computedDiff), ...Object.keys(kitDiff)])];
for (const k of topKeys) {
  const cv = (computedDiff as any)[k];
  const av = (kitDiff as any)[k];
  const cvj = JSON.stringify(cv);
  const avj = JSON.stringify(av);
  
  if (cvj === avj) {
    console.log(`${k}: MATCH (JSON)`);
  } else if (cv === undefined && av === undefined) {
    console.log(`${k}: MATCH (both undefined)`);
  } else if (cv === undefined || av === undefined) {
    console.log(`${k}: DIFF (one undefined) computed=${typeof cv} asset=${typeof av}`);
  } else {
    console.log(`${k}: DIFF`);
    // Show truncated diff
    if (typeof cv !== 'object' || typeof av !== 'object') {
      console.log(`  computed: ${cvj?.slice(0,200)}`);
      console.log(`  asset:   ${avj?.slice(0,200)}`);
    } else {
      // For objects, find which sub-key differs
      const subKeys = [...new Set([...Object.keys(cv), ...Object.keys(av)])];
      for (const sk of subKeys) {
        const scv = JSON.stringify(cv[sk]);
        const sav = JSON.stringify(av[sk]);
        if (scv !== sav) {
          console.log(`  sub-key "${sk}" differs:`);
          console.log(`    computed: ${scv?.slice(0,300)}`);
          console.log(`    asset:   ${sav?.slice(0,300)}`);
        }
      }
    }
  }
}
