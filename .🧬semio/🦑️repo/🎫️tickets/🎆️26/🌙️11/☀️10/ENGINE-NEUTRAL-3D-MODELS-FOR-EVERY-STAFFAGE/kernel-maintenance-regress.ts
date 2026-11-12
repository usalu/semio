/** 🔁️ Compares `tessellateSolid` of the maintained kernel against the frozen pre-maintenance copy for every solid of the volume snapshot (positions, normals, indices exact; reports index-order-only differences separately). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, QUALITY_FOR } from "./volume-lib.ts";
import { tessellateSolid as now } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { tessellateSolid as before } from "./🗑️generated/kernel-maintenance/old/model/🔺️tessellation/🟦️.ts";

const assets = JSON.parse(readFileSync(join(GENERATED, "kernel-maintenance", "volumes-snapshot.json"), "utf8")) as Record<string, any>;
const tally: Record<string, { solids: number; positions: number; indices: number }> = {};
const detail: string[] = [];
for (const [key, asset] of Object.entries(assets)) {
  const d = asset.dimensions ?? {};
  const max = Math.max(d.widthM ?? 1, d.heightM ?? 1, d.depthM ?? 1);
  const quality = QUALITY_FOR(max);
  for (const solid of asset.volume.solids) {
    const a = now(solid, quality), b = before(solid, quality);
    const t = (tally[solid.kind] ??= { solids: 0, positions: 0, indices: 0 });
    t.solids++;
    const samePos = a.positions.length === b.positions.length && a.positions.every((v, i) => v === b.positions[i]) && a.normals.every((v, i) => v === b.normals[i]);
    const sameIdx = a.indices.length === b.indices.length && a.indices.every((v, i) => v === b.indices[i]);
    if (!samePos) { t.positions++; detail.push(`${key} ${solid.id} ${solid.kind}: positions/normals differ`); }
    else if (!sameIdx) { t.indices++; detail.push(`${key} ${solid.id} ${solid.kind}: indices differ (${a.indices.length / 3} vs ${b.indices.length / 3} triangles)`); }
  }
}
console.log(tally);
for (const line of detail.slice(0, 40)) console.log(line);
