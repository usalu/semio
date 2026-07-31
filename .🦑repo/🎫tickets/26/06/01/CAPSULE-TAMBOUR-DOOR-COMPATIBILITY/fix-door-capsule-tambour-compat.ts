#!/usr/bin/env bun
/** @emoji 🚪 Same-side doors: capsule east ↔ tambour east; capsule west ↔ tambour west. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const DOOR_CAPSULE_EAST = "019ab243-21f4-73df-8cb4-4ac2be8fc645";
const DOOR_CAPSULE_WEST = "019ab243-21f4-73df-8cb4-4f2766b68b25";
const DOOR_TAMBOUR_EAST = "019ab243-21f4-73df-8cb4-50266f6860b8";
const DOOR_TAMBOUR_WEST = "019ab243-21f4-73df-8cb4-57263e420e70";

const HANDLE_CAPSULE_EAST = `compose.metabolism.light.handle.${DOOR_CAPSULE_EAST}`;
const HANDLE_CAPSULE_WEST = `compose.metabolism.light.handle.${DOOR_CAPSULE_WEST}`;
const HANDLE_TAMBOUR_EAST = `compose.metabolism.light.handle.${DOOR_TAMBOUR_EAST}`;
const HANDLE_TAMBOUR_WEST = `compose.metabolism.light.handle.${DOOR_TAMBOUR_WEST}`;

function itemsOf(block: unknown): { id?: string }[] {
  if (Array.isArray(block)) return block as { id?: string }[];
  if (block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)) {
    return (block as { items: { id?: string }[] }).items;
  }
  return [];
}

let portFixes = 0;
let compatRuleFixes = 0;

function setSingleDoorCompat(port: Record<string, unknown>, tambourOrCapsuleId: string): void {
  const items = itemsOf(port.compatiblePorts);
  const doorIds = new Set([DOOR_CAPSULE_EAST, DOOR_CAPSULE_WEST, DOOR_TAMBOUR_EAST, DOOR_TAMBOUR_WEST]);
  let replaced = false;
  for (const item of items) {
    const id = String(item.id ?? "");
    if (!doorIds.has(id)) continue;
    if (id !== tambourOrCapsuleId) {
      item.id = tambourOrCapsuleId;
      portFixes++;
      replaced = true;
    }
  }
  if (!replaced && items.length === 1 && doorIds.has(String(items[0]!.id ?? ""))) {
    const id = String(items[0]!.id ?? "");
    if (id !== tambourOrCapsuleId) {
      items[0]!.id = tambourOrCapsuleId;
      portFixes++;
    }
  }
}

function fixPortCompatibleRefs(port: Record<string, unknown>): void {
  const name = String(port.name ?? "");
  if (name === "door capsule east") setSingleDoorCompat(port, DOOR_TAMBOUR_EAST);
  if (name === "door capsule west") setSingleDoorCompat(port, DOOR_TAMBOUR_WEST);
  if (name === "door tambour east") setSingleDoorCompat(port, DOOR_CAPSULE_EAST);
  if (name === "door tambour west") setSingleDoorCompat(port, DOOR_CAPSULE_WEST);
}

function fixKindCompatEntry(entry: Record<string, unknown>): void {
  const source = String(entry.source ?? "");
  const target = String(entry.target ?? "");
  const fixNamed = (s: string, t: string, wantTarget: string) => {
    if (s === source && t === target) {
      entry.target = wantTarget;
      compatRuleFixes++;
    }
  };
  if (source === "door capsule east" && target !== "door tambour east") {
    entry.target = "door tambour east";
    compatRuleFixes++;
  } else if (source === "door capsule west" && target !== "door tambour west") {
    entry.target = "door tambour west";
    compatRuleFixes++;
  } else if (source === "door tambour east" && target !== "door capsule east") {
    entry.target = "door capsule east";
    compatRuleFixes++;
  } else if (source === "door tambour west" && target !== "door capsule west") {
    entry.target = "door capsule west";
    compatRuleFixes++;
  } else if (source === HANDLE_CAPSULE_EAST && target !== HANDLE_TAMBOUR_EAST) {
    entry.target = HANDLE_TAMBOUR_EAST;
    compatRuleFixes++;
  } else if (source === HANDLE_CAPSULE_WEST && target !== HANDLE_TAMBOUR_WEST) {
    entry.target = HANDLE_TAMBOUR_WEST;
    compatRuleFixes++;
  } else if (source === HANDLE_TAMBOUR_EAST && target !== HANDLE_CAPSULE_EAST) {
    entry.target = HANDLE_CAPSULE_EAST;
    compatRuleFixes++;
  } else if (source === HANDLE_TAMBOUR_WEST && target !== HANDLE_CAPSULE_WEST) {
    entry.target = HANDLE_CAPSULE_WEST;
    compatRuleFixes++;
  }
  fixNamed("door capsule east", "door tambour west", "door tambour east");
  fixNamed("door capsule west", "door tambour east", "door tambour west");
  fixNamed("door tambour east", "door capsule west", "door capsule east");
  fixNamed("door tambour west", "door capsule east", "door capsule west");
}

function walk(value: unknown): void {
  if (Array.isArray(value)) {
    for (const item of value) walk(item);
    return;
  }
  if (!value || typeof value !== "object") return;
  const obj = value as Record<string, unknown>;
  if (typeof obj.name === "string" && obj.compatiblePorts !== undefined) fixPortCompatibleRefs(obj);
  if (typeof obj.source === "string" && typeof obj.target === "string") fixKindCompatEntry(obj);
  for (const child of Object.values(obj)) walk(child);
}

function walkJsonFiles(dir: string, out: string[] = []): string[] {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walkJsonFiles(p, out);
    else if (ent.name.endsWith(".json") || ent.name.endsWith(".compose.json")) out.push(p);
  }
  return out;
}

const roots = [
  join(import.meta.dir, "../../../../../../compose/fixtures"),
  join(import.meta.dir, "../../../../../../puzzle/3d/fixture"),
  join(import.meta.dir, "../../../../../../puzzle/2d/fixture"),
  join(import.meta.dir, "../../../../../../.storybook/fixtures"),
];

let filesChanged = 0;
for (const root of roots) {
  for (const path of walkJsonFiles(root)) {
    let doc: unknown;
    try {
      doc = JSON.parse(readFileSync(path, "utf8"));
    } catch {
      continue;
    }
    const beforePorts = portFixes;
    const beforeRules = compatRuleFixes;
    walk(doc);
    if (portFixes > beforePorts || compatRuleFixes > beforeRules) {
      const indent = path.includes("shallow") || path.includes("nakagin-capsule-tower.filtered") ? 4 : 2;
      writeFileSync(path, `${JSON.stringify(doc, null, indent)}\n`);
      filesChanged++;
      console.log("[DEBUG] fixed", path);
    }
  }
}

console.log(`[DEBUG] port ref fixes: ${portFixes}, kindCompatibility fixes: ${compatRuleFixes}, files: ${filesChanged}`);
