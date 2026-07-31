import { readFileSync } from "fs";
import { getKitDiff, areKitDiffsEqual } from "/workspaces/semio/compose/js/compose";

const ASSETS = "/workspaces/semio/assets/compose";
const kitRaw = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism.json`, "utf-8"));
const kitOriginal = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitDiffed = JSON.parse(readFileSync(`${ASSETS}/kit_metabolism_diffed.json`, "utf-8"));
const kitDiff = JSON.parse(readFileSync(`${ASSETS}/diff_kit_metabolism.json`, "utf-8"));
const computedDiff = getKitDiff(kitOriginal, kitDiffed);

// Get the model updated diff
const typeUpdate = (computedDiff as any).types.updated[0];
const typeUpdateAsset = (kitDiff as any).types.updated[0];
const modelUpdated = typeUpdate.diff.models.updated[0];
const modelUpdatedAsset = typeUpdateAsset.diff.models.updated[0];

console.log("Model diff computed:", JSON.stringify(modelUpdated));
console.log("\nModel diff asset:", JSON.stringify(modelUpdatedAsset));

// Check each model diff field
const dc = modelUpdated.diff;
const da = modelUpdatedAsset.diff;
console.log("\nname match:", dc.name === da.name);
console.log("file match:", JSON.stringify(dc.file) === JSON.stringify(da.file));
console.log("tags computed:", JSON.stringify(dc.tags));
console.log("tags asset:", JSON.stringify(da.tags));
console.log("tags match:", JSON.stringify(dc.tags) === JSON.stringify(da.tags));
console.log("description computed:", JSON.stringify(dc.description));
console.log("description asset:", JSON.stringify(da.description));
console.log("attributes computed:", JSON.stringify(dc.attributes));
console.log("attributes asset:", JSON.stringify(da.attributes));

// Now test the model diff comparison in isolation
const partial1: any = { types: { updated: [{ type: typeUpdate.type, diff: { models: { updated: [modelUpdated] } } }] } };
const partial2: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { models: { updated: [modelUpdatedAsset] } } }] } };
console.log("\nModel update diff equal:", areKitDiffsEqual(partial1, partial2));

// Try with only tags
const partial3: any = { types: { updated: [{ type: typeUpdate.type, diff: { models: { updated: [{ model: modelUpdated.model, diff: { tags: dc.tags } }] } } }] } };
const partial4: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { models: { updated: [{ model: modelUpdatedAsset.model, diff: { tags: da.tags } }] } } }] } };
console.log("Model tags diff equal:", areKitDiffsEqual(partial3, partial4));

// Try with only name
const partial5: any = { types: { updated: [{ type: typeUpdate.type, diff: { models: { updated: [{ model: modelUpdated.model, diff: { name: dc.name } }] } } }] } };
const partial6: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { models: { updated: [{ model: modelUpdatedAsset.model, diff: { name: da.name } }] } } }] } };
console.log("Model name diff equal:", areKitDiffsEqual(partial5, partial6));

// Try with only attributes
const partial7: any = { types: { updated: [{ type: typeUpdate.type, diff: { models: { updated: [{ model: modelUpdated.model, diff: { attributes: dc.attributes } }] } } }] } };
const partial8: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { models: { updated: [{ model: modelUpdatedAsset.model, diff: { attributes: da.attributes } }] } } }] } };
console.log("Model attributes diff equal:", areKitDiffsEqual(partial7, partial8));

// Try with only description
const partial9: any = { types: { updated: [{ type: typeUpdate.type, diff: { models: { updated: [{ model: modelUpdated.model, diff: { description: dc.description } }] } } }] } };
const partial10: any = { types: { updated: [{ type: typeUpdateAsset.type, diff: { models: { updated: [{ model: modelUpdatedAsset.model, diff: { description: da.description } }] } } }] } };
console.log("Model description diff equal:", areKitDiffsEqual(partial9, partial10));
