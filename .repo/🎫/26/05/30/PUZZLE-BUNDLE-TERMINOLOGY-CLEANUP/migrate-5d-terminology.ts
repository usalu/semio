#!/usr/bin/env bun
/** One-off: puzzle 5d terminology decoupling — grips, fasteners, 2d/3d aspect keys. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");

function migrateObject(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(migrateObject);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const src = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, raw] of Object.entries(src)) {
    let nextKey = key;
    if (key === "anchors") nextKey = "grips";
    else if (key === "anchorKind") nextKey = "gripKind";
    else if (key === "ties") nextKey = "fasteners";
    else if (key === "tieKind") nextKey = "fastenerKind";
    else if (key === "puzzle2d") nextKey = "2d";
    else if (key === "puzzle3d") nextKey = "3d";
    let next = migrateObject(raw);
    if (nextKey === "specificity" && typeof next === "string") {
      if (next === "handle" || next === "vortex" || next === "node" || next === "object") next = "grip";
      else if (next === "edge" || next === "attraction") next = "fastener";
      else if (next === "wire" || next === "cable") next = "rope";
    }
    out[nextKey] = next;
  }
  return out;
}

function migrate5dJson(rel: string): void {
  const full = join(root, rel);
  const doc = migrateObject(JSON.parse(readFileSync(full, "utf8"))) as Record<string, unknown>;
  writeFileSync(full, `${JSON.stringify(doc, null, 2)}\n`);
  console.log(`[DEBUG] migrated ${rel}`);
}

for (const rel of ["puzzle/5d/fixture/nakagin-capsule-tower.5d.json", "puzzle/5d/fixture/concrete-forest.5d.json"]) {
  migrate5dJson(rel);
}
