#!/usr/bin/env bun
/** @emoji 🎨 Backfills nakagin 3d kind catalog meshUrl from metabolism naming conventions. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const fixture3dPath = join(repoRoot, "puzzle/3d/fixture/nakagin-capsule-tower.3d.json");

function itemsOf(block: unknown): unknown[] {
  if (Array.isArray(block)) return block;
  if (block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)) {
    return (block as { items: unknown[] }).items;
  }
  return [];
}

function metabolismCapsuleTailStem(tail: string): string {
  const token = tail.trim();
  if (token === "Backslash") return "backslash";
  if (token === "Slash") return "slash";
  if (token.length === 1) return token === "J" || token === "L" ? token : token.toLowerCase();
  return token.toLowerCase().replace(/\s+/g, "_");
}

function inferMeshUrl(kindName: string): string | undefined {
  const name = kindName.trim();
  const trapezoid = /^Trapezoid Capsule (.+)$/.exec(name);
  if (trapezoid) return `/meshes/trapezoid-capsule_${metabolismCapsuleTailStem(trapezoid[1]!)}.glb`;
  const balcony = /^Capsule With Balcony (.+)$/.exec(name);
  if (balcony) return `/meshes/capsule-with-balcony_${metabolismCapsuleTailStem(balcony[1]!)}.glb`;
  const capsule = /^Capsule (.+)$/.exec(name);
  if (capsule) return `/meshes/capsule_${metabolismCapsuleTailStem(capsule[1]!)}.glb`;
  if (name === "Base Blob") return "/meshes/base_blob.glb";
  if (name === "Cylindric Capital") return "/meshes/cylindric-capital.glb";
  if (name === "Cylindric Tambour") return "/meshes/cylindric-tambour.glb";
  if (name === "Cylindric First Storey Tambour") return "/meshes/cylindric-tambour_first-storey.glb";
  if (name === "Cylindric Last Storey Tambour") return "/meshes/cylindric-tambour_last-storey.glb";
  if (name === "Cylindric Single Storey Tambour") return "/meshes/cylindric-tambour_single-storey.glb";
  if (name === "Single Storey Tambour") return "/meshes/tambour_single-storey.glb";
  return undefined;
}

const fixture3d = JSON.parse(readFileSync(fixture3dPath, "utf8")) as Record<string, unknown>;
const catalog = itemsOf((fixture3d.meta as { kindCatalogs?: { objects?: unknown[] } }).kindCatalogs?.objects);
let filled = 0;
for (const row of catalog) {
  if (!row || typeof row !== "object") continue;
  const entry = row as Record<string, unknown>;
  const id = String(entry.id ?? "");
  if (String(entry.meshUrl ?? "").trim()) continue;
  const meshUrl = inferMeshUrl(id);
  if (!meshUrl) continue;
  entry.meshUrl = meshUrl;
  filled += 1;
}
writeFileSync(fixture3dPath, `${JSON.stringify(fixture3d, null, 2)}\n`, "utf8");
console.log(`[backfill-nakagin-3d-catalog-mesh-urls] filled ${filled} catalog meshUrl entries`);
