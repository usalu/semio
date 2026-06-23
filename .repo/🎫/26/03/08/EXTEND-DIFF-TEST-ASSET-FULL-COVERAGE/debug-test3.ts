import { readFileSync } from "fs";
import {
  getKitDiff,
  areKitDiffsEqual,
  Kit,
  KitDiff,
  TypesDiff,
  DesignsDiff,
} from "/workspaces/semio/compose/js/compose";

const ASSETS = "/workspaces/semio/compose/assets/compose";
const kitRaw = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism.json`, "utf-8"));
const kitOriginal = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitDiffed = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism_diffed.json`, "utf-8"));
const kitDiff = JSON.parse(readFileSync(`${ASSETS}/diff_kit_metabolism.json`, "utf-8"));
const computedDiff = getKitDiff(kitOriginal, kitDiffed);

// Test each key independently by creating partial diffs
const allKeys = [...new Set([...Object.keys(computedDiff), ...Object.keys(kitDiff)])];
for (const key of allKeys) {
  const partialComputed: any = {};
  const partialAsset: any = {};
  partialComputed[key] = (computedDiff as any)[key];
  partialAsset[key] = (kitDiff as any)[key];
  
  const result = areKitDiffsEqual(partialComputed as KitDiff, partialAsset as KitDiff);
  if (!result) {
    console.log(`FAIL: key "${key}"`);
    // Try to narrow down further for collection types
    const cv = (computedDiff as any)[key];
    const av = (kitDiff as any)[key];
    if (cv && typeof cv === 'object' && av && typeof av === 'object') {
      // Check removed/updated/added if it's a collection diff
      if ('removed' in cv || 'updated' in cv || 'added' in cv) {
        for (const subKey of ['removed', 'updated', 'added']) {
          const subComputed: any = {};
          const subAsset: any = {};
          subComputed[key] = { [subKey]: cv[subKey] };
          subAsset[key] = { [subKey]: av[subKey] };
          const subResult = areKitDiffsEqual(subComputed as KitDiff, subAsset as KitDiff);
          if (!subResult) {
            console.log(`  FAIL sub: ${key}.${subKey}`);
            if (subKey === 'updated' && Array.isArray(cv[subKey])) {
              for (let i = 0; i < cv[subKey].length; i++) {
                const itemC: any = {};
                const itemA: any = {};
                itemC[key] = { [subKey]: [cv[subKey][i]] };
                itemA[key] = { [subKey]: [av[subKey][i]] };
                const itemResult = areKitDiffsEqual(itemC as KitDiff, itemA as KitDiff);
                if (!itemResult) {
                  const entityKey = Object.keys(cv[subKey][i]).find(k => k !== 'diff');
                  const entityId = entityKey ? cv[subKey][i][entityKey] : 'unknown';
                  console.log(`    FAIL item[${i}]: ${JSON.stringify(entityId)} diff keys: ${Object.keys(cv[subKey][i].diff || {}).join(',')}`);
                  // Compare diff fields
                  const dc = cv[subKey][i].diff || {};
                  const da = av[subKey][i].diff || {};
                  const diffKeys = [...new Set([...Object.keys(dc), ...Object.keys(da)])];
                  for (const dk of diffKeys) {
                    if (JSON.stringify(dc[dk]) !== JSON.stringify(da[dk])) {
                      console.log(`      DIFF field "${dk}": computed=${JSON.stringify(dc[dk])?.slice(0,100)} asset=${JSON.stringify(da[dk])?.slice(0,100)}`);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  } else {
    console.log(`PASS: key "${key}"`);
  }
}
