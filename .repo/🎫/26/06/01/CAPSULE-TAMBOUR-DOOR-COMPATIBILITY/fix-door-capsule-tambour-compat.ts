#!/usr/bin/env bun
/** @emoji 🚪 Capsule east ↔ tambour west; capsule west ↔ tambour east only. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const DOOR_CAPSULE_EAST = "019ab243-21f4-73df-8cb4-4ac2be8fc645";
const DOOR_CAPSULE_WEST = "019ab243-21f4-73df-8cb4-4f2766b68b25";
const DOOR_TAMBOUR_EAST = "019ab243-21f4-73df-8cb4-50266f6860b8";
const DOOR_TAMBOUR_WEST = "019ab243-21f4-73df-8cb4-57263e420e70";

const HANDLE_CAPSULE_EAST = `semio.metabolism.light.handle.${DOOR_CAPSULE_EAST}`;
const HANDLE_CAPSULE_WEST = `semio.metabolism.light.handle.${DOOR_CAPSULE_WEST}`;
const HANDLE_TAMBOUR_EAST = `semio.metabolism.light.handle.${DOOR_TAMBOUR_EAST}`;
const HANDLE_TAMBOUR_WEST = `semio.metabolism.light.handle.${DOOR_TAMBOUR_WEST}`;

function itemsOf(block: unknown): { id?: string }[] {
  if (Array.isArray(block)) return block as { id?: string }[];
  if (block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)) {
    return (block as { items: { id?: string }[] }).items;
  }
  return [];
}

let portFixes = 0;
let compatRuleFixes = 0;

function fixPortCompatibleRefs(port: Record<string, unknown>): void {
  const name = String(port.name ?? "");
  const compat = port.compatiblePorts;
  const items = itemsOf(compat);
  if (items.length === 0) return;
  const replaceId = (from: string, to: string) => {
    for (const item of items) {
      if (String(item.id ?? "") === from) {
        item.id = to;
        portFixes++;
      }
    }
  };
  if (name === "door capsule east") replaceId(DOOR_TAMBOUR_EAST, DOOR_TAMBOUR_WEST);
  if (name === "door capsule west") replaceId(DOOR_TAMBOUR_WEST, DOOR_TAMBOUR_EAST);
}

function fixKindCompatEntry(entry: Record<string, unknown>): void {
  const source = String(entry.source ?? "");
  const target = String(entry.target ?? "");
  const swapNamed = (s: string, t: string, ns: string, nt: string) => {
    if (s === ns && t === nt) {
      entry.target = nt === "door tambour east" ? "door tambour west" : nt;
      if (ns === "door capsule east" && nt === "door tambour east") entry.target = "door tambour west";
      if (ns === "door capsule west" && nt === "door tambour west") entry.target = "door tambour east";
      compatRuleFixes++;
    }
  };
  if (source === "door capsule east" && target === "door tambour east") {
    entry.target = "door tambour west";
    compatRuleFixes++;
    return;
  }
  if (source === "door capsule west" && target === "door tambour west") {
    entry.target = "door tambour east";
    compatRuleFixes++;
    return;
  }
  if (source === HANDLE_CAPSULE_EAST && target === HANDLE_TAMBOUR_EAST) {
    entry.target = HANDLE_TAMBOUR_WEST;
    compatRuleFixes++;
    return;
  }
  if (source === HANDLE_CAPSULE_WEST && target === HANDLE_TAMBOUR_WEST) {
    entry.target = HANDLE_TAMBOUR_EAST;
    compatRuleFixes++;
  }
  swapNamed(source, target, "door capsule east", "door tambour east");
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
    else if (ent.name.endsWith(".json") || ent.name.endsWith(".semio.json")) out.push(p);
  }
  return out;
}

const roots = [
  join(import.meta.dir, "../../../../../../semio/fixtures"),
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
      writeFileSync(path, `${JSON.stringify(doc, null, path.includes("shallow") || path.includes("nakagin-capsule-tower.filtered") ? 4 : 2)}\n`);
      filesChanged++;
      console.log("[DEBUG] fixed", path);
    }
  }
}

console.log(`[DEBUG] port ref fixes: ${portFixes}, kindCompatibility fixes: ${compatRuleFixes}, files: ${filesChanged}`);
