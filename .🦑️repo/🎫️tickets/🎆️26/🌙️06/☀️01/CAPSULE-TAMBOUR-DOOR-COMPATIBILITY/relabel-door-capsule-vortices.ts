/** One-off: relabel `door capsule *` vortex kinds from local CAD X (negative → west). */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../..");
const fixturePath = resolve(repoRoot, "puzzle/3d/fixture/nakagin-capsule-tower.3d.json");

function doorCapsuleVortexKindFromPoint(portName: string, position: readonly number[]): string {
  if (!portName.includes("door capsule")) {
    return portName;
  }
  return position[0]! < 0 ? "door capsule west" : "door capsule east";
}

function relabelVortex(v: { vortexKind?: string; label?: string; position?: number[] }): boolean {
  if (!v.vortexKind?.includes("door capsule") || !v.position?.length) {
    return false;
  }
  const next = doorCapsuleVortexKindFromPoint(v.vortexKind, v.position);
  if (next === v.vortexKind) {
    return false;
  }
  v.vortexKind = next;
  if (v.label?.includes("door capsule")) {
    v.label = next;
  }
  return true;
}

const doc = JSON.parse(readFileSync(fixturePath, "utf8")) as {
  meta?: { kindCatalogs?: { objects?: { vortices?: { vortexKind?: string; label?: string; position?: number[] }[] }[] } };
  objects?: { vortices?: { vortexKind?: string; label?: string; position?: number[] }[] }[];
};

let relabeled = 0;
for (const kind of doc.meta?.kindCatalogs?.objects ?? []) {
  for (const vortex of kind.vortices ?? []) {
    if (relabelVortex(vortex)) {
      relabeled += 1;
    }
  }
}
for (const object of doc.objects ?? []) {
  for (const vortex of object.vortices ?? []) {
    if (relabelVortex(vortex)) {
      relabeled += 1;
    }
  }
}

writeFileSync(fixturePath, `${JSON.stringify(doc, null, 2)}\n`);
console.log(`[relabel-door-capsule-vortices] ${relabeled} vortices updated in ${fixturePath}`);
