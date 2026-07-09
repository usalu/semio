import { readFileSync } from "fs";
import { getKitDiff, areKitDiffsEqual } from "/workspaces/semio/compose/js/compose";

const ASSETS = "/workspaces/semio/assets/compose";
const kitRaw = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism.json`, "utf-8"));
const kitOriginal = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitDiffed = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism_diffed.json`, "utf-8"));
const kitDiff = JSON.parse(readFileSync(`${ASSETS}/diff_kit_metabolism.json`, "utf-8"));
const computedDiff = getKitDiff(kitOriginal, kitDiffed);

const td = (computedDiff as any).types.updated[0].diff;
const ta = (kitDiff as any).types.updated[0].diff;

// Check models.removed
console.log("models.removed match:", JSON.stringify(td.models?.removed) === JSON.stringify(ta.models?.removed));

// Check models.updated (already tested individually)
console.log("models.updated match:", JSON.stringify(td.models?.updated) === JSON.stringify(ta.models?.updated));

// Check models.added
const addedC = td.models?.added ?? [];
const addedA = ta.models?.added ?? [];
console.log("models.added count:", addedC.length, "vs", addedA.length);
for (let i = 0; i < Math.max(addedC.length, addedA.length); i++) {
  const ac = addedC[i];
  const aa = addedA[i];
  const match = JSON.stringify(ac) === JSON.stringify(aa);
  if (!match) {
    console.log(`  added[${i}] DIFF`);
    console.log(`    computed: ${JSON.stringify(ac)?.slice(0, 200)}`);
    console.log(`    asset:   ${JSON.stringify(aa)?.slice(0, 200)}`);
  }
}

// Now test models sub-section isolation
for (const sub of ["removed", "updated", "added"]) {
  const p1: any = { types: { updated: [{ type: { guid: td.__typeGuid || "277768b5-9220-4312-bf0d-ab82d9fb6a73" }, diff: { models: { [sub]: td.models?.[sub] } } }] } };
  const p2: any = { types: { updated: [{ type: { guid: "277768b5-9220-4312-bf0d-ab82d9fb6a73" }, diff: { models: { [sub]: ta.models?.[sub] } } }] } };
  console.log(`models.${sub} areKitDiffsEqual:`, areKitDiffsEqual(p1, p2));
}
