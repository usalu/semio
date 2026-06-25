import { readFileSync } from "fs";
import { join } from "path";
import {
  getKitDiff,
  inverseKitDiff,
  applyKitDiff,
  areKitsEqual,
  areTypesEqual,
  areDesignsEqual,
  areTagsEqual,
  areConceptsEqual,
  arePortsEqual,
  areQualitiesEqual,
  areAuthorsEqual,
  areFilesEqual,
  areFoldersEqual,
  areAttributesEqual,
  Kit,
} from "/workspaces/semio/compose/js/compose";

const ASSETS_DIR = "/workspaces/semio/assets/compose";
const kitRaw: Kit = JSON.parse(readFileSync(join(ASSETS_DIR, "kit_metabolism.json"), "utf-8"));
const kitBefore: Kit = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };

// Rebuild kitAfter the same way
const kitAfter: Kit = JSON.parse(JSON.stringify(kitBefore));
let uuidCounter = 0;
function newGuid(): string {
  uuidCounter++;
  const hex = uuidCounter.toString(16).padStart(12, "0");
  return `019caa00-0000-7000-a000-${hex}`;
}

// Apply all the same changes...
// Kit scalars
kitAfter.name = "Metabolism Modified"; kitAfter.version = "r25.08-1";
kitAfter.description = "Modified version for comprehensive diff testing";
kitAfter.icon = "modified-icon.svg"; kitAfter.image = "modified-image.png";
kitAfter.remote = "https://modified.example.com/archive.tar.gz";
kitAfter.homepage = "https://modified.example.com";
kitAfter.license = "MIT-Modified"; kitAfter.preview = "modified-preview.png";

// We don't need the full generator. Let me just test with smaller changes.
// Computing the full diff and inverse, then apply inverse.
const diff = getKitDiff(kitBefore, kitAfter);
const inverseDiff = inverseKitDiff(kitBefore, diff);
const appliedInverse = applyKitDiff(kitAfter, inverseDiff);

// Check each top-level field
console.log("[DEBUG] name eq:", kitBefore.name === appliedInverse.name, kitBefore.name, appliedInverse.name);

// Check types
const bt = (kitBefore.types ?? []).sort((a:any,b:any)=>a.guid.localeCompare(b.guid));
const it = (appliedInverse.types ?? []).sort((a:any,b:any)=>a.guid.localeCompare(b.guid));
console.log("[DEBUG] types count:", bt.length, "vs", it.length);

// Check designs
const bd = (kitBefore.designs ?? []).sort((a:any,b:any)=>a.guid.localeCompare(b.guid));
const id = (appliedInverse.designs ?? []).sort((a:any,b:any)=>a.guid.localeCompare(b.guid));
console.log("[DEBUG] designs count:", bd.length, "vs", id.length);
