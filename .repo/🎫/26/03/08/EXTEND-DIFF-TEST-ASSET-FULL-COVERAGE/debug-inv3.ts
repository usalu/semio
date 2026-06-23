import { readFileSync } from "fs";
import { join } from "path";
import {
  getKitDiff, inverseKitDiff, applyKitDiff,
  areKitsEqual, Kit,
} from "/workspaces/semio/compose/js/compose";

const ASSETS_DIR = "/workspaces/semio/compose/assets/compose";
const kitRaw: Kit = JSON.parse(readFileSync(join(ASSETS_DIR, "kit_metabolism.json"), "utf-8"));
const kitBefore: Kit = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };

let uuidCounter = 0;
function newGuid(): string {
  uuidCounter++;
  const hex = uuidCounter.toString(16).padStart(12, "0");
  return `019caa00-0000-7000-a000-${hex}`;
}

function deepCompare(a: any, b: any, path: string = ""): string[] {
  const diffs: string[] = [];
  if (a === b) return diffs;
  if (a == null && b == null) return diffs;
  if (a == null || b == null) { diffs.push(`${path}: ${JSON.stringify(a)?.slice(0,80)} vs ${JSON.stringify(b)?.slice(0,80)}`); return diffs; }
  if (typeof a !== typeof b) { diffs.push(`${path}: type ${typeof a} vs ${typeof b}`); return diffs; }
  if (typeof a !== 'object') { diffs.push(`${path}: ${JSON.stringify(a)?.slice(0,80)} vs ${JSON.stringify(b)?.slice(0,80)}`); return diffs; }
  if (Array.isArray(a) !== Array.isArray(b)) { diffs.push(`${path}: array vs non-array`); return diffs; }
  if (Array.isArray(a)) {
    if (a.length !== b.length) diffs.push(`${path}: length ${a.length} vs ${b.length}`);
    for (let i = 0; i < Math.max(a.length, b.length); i++) {
      diffs.push(...deepCompare(a[i], b[i], `${path}[${i}]`));
    }
    return diffs;
  }
  const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
  for (const k of keys) {
    diffs.push(...deepCompare(a[k], b[k], path ? `${path}.${k}` : k));
  }
  return diffs;
}

// Test 1: types-update-scalars
{
  const before = JSON.parse(JSON.stringify(kitBefore));
  const after = JSON.parse(JSON.stringify(kitBefore));
  const t = after.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
  t.name = "Base Mod"; t.description = "Mod"; t.virtual = true; t.unit = "cm";
  t.isAbstract = true; t.stock = 42; t.folder = "f"; t.icon = "i"; t.image = "im";
  
  const diff = getKitDiff(before, after);
  const inv = inverseKitDiff(before, diff);
  const applied = applyKitDiff(after, inv);
  
  const origType = before.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73");
  const appliedType = applied.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73");
  
  console.log("\n=== types-update-scalars ===");
  const diffs = deepCompare(origType, appliedType);
  for (const d of diffs.slice(0, 10)) console.log("  " + d);
  if (diffs.length > 10) console.log(`  ... and ${diffs.length - 10} more`);
}

// Test 2: types-update-connectors
{
  uuidCounter = 0;
  const before = JSON.parse(JSON.stringify(kitBefore));
  const after = JSON.parse(JSON.stringify(kitBefore));
  const t = after.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
  t.connectors.splice(0, 1);
  const c = t.connectors[0];
  c.name = "mod"; c.t = 0.75; c.point = {x:10,y:20,z:30}; c.direction = {x:1,y:0,z:0};
  t.connectors.push({guid: newGuid(), name: "new", point:{x:0,y:0,z:0}, direction:{x:0,y:1,z:0}, t: 0.5});
  
  const diff = getKitDiff(before, after);
  const inv = inverseKitDiff(before, diff);
  const applied = applyKitDiff(after, inv);
  
  const origType = before.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
  const appliedType = applied.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
  
  console.log("\n=== types-update-connectors ===");
  // Sort connectors by guid for comparison
  const origC = [...(origType.connectors || [])].sort((a:any,b:any) => a.guid.localeCompare(b.guid));
  const applC = [...(appliedType.connectors || [])].sort((a:any,b:any) => a.guid.localeCompare(b.guid));
  const diffs = deepCompare(origC, applC);
  for (const d of diffs.slice(0, 10)) console.log("  " + d);
  if (diffs.length > 10) console.log(`  ... and ${diffs.length - 10} more`);
}

// Test 3: designs-update-concepts
{
  uuidCounter = 0;
  const before = JSON.parse(JSON.stringify(kitBefore));
  const after = JSON.parse(JSON.stringify(kitBefore));
  const d = after.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
  d.concepts = [{guid: before.concepts![0].guid}];
  
  const diff = getKitDiff(before, after);
  const inv = inverseKitDiff(before, diff);
  const applied = applyKitDiff(after, inv);
  
  const origDesign = before.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
  const appliedDesign = applied.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
  
  console.log("\n=== designs-update-concepts ===");
  const diffs = deepCompare(origDesign, appliedDesign);
  for (const d of diffs.slice(0, 10)) console.log("  " + d);
  if (diffs.length > 10) console.log(`  ... and ${diffs.length - 10} more`);
}
