#!/usr/bin/env bun
/** @emoji 🏷️ One-off: nakagin 3d fixture object kinds + catalog from kit types and 2d document names. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");

const METABOLISM_CAPSULE_KIND_SPECIFICITY_PREFIXES = ["Capsule With Balcony ", "Trapezoid Capsule "] as const;

function puzzle3dPreferSpecificMetabolismKindName(kindName: string, availableKindNames: ReadonlySet<string>): string {
  const name = kindName.trim();
  if (name === "") return name;
  if (METABOLISM_CAPSULE_KIND_SPECIFICITY_PREFIXES.some((prefix) => name.startsWith(prefix))) return name;
  const plain = /^Capsule (.+)$/.exec(name);
  if (!plain) return name;
  const tail = plain[1]!;
  for (const prefix of METABOLISM_CAPSULE_KIND_SPECIFICITY_PREFIXES) {
    const candidate = `${prefix}${tail}`;
    if (availableKindNames.has(candidate)) return candidate;
  }
  return name;
}
const typesDir = join(repoRoot, "compose/fixtures/kit/dev/metabolism/wip/initialKit/types");
const designPath = join(repoRoot, "compose/fixtures/kit/dev/metabolism/wip/initialKit/designs/nakagin-capsule-tower.design.compose.json");
const fixture2dPath = join(repoRoot, "puzzle/2d/fixture/nakagin-capsule-tower.2d.json");
const fixture3dPath = join(repoRoot, "puzzle/3d/fixture/nakagin-capsule-tower.3d.json");

function itemsOf(block: unknown): unknown[] {
  if (Array.isArray(block)) return block;
  if (block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)) {
    return (block as { items: unknown[] }).items;
  }
  return [];
}

function primaryGlbFileName(row: Record<string, unknown>): string {
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const name = String((rep as { name?: string }).name ?? "");
    if (name.toLowerCase().endsWith(".glb") && !name.toLowerCase().includes("collider") && !/_1to\d+/i.test(name)) {
      return name;
    }
  }
  return "";
}

const kitTypeNameById = new Map<string, string>();
const meshByKindName = new Map<string, string>();
const availableKindNames = new Set<string>();

for (const fileName of readdirSync(typesDir)) {
  if (!fileName.endsWith(".type.compose.json")) continue;
  const row = JSON.parse(readFileSync(join(typesDir, fileName), "utf8")) as Record<string, unknown>;
  const id = String(row.id ?? "");
  const name = String(row.name ?? "").trim();
  if (!id || !name) continue;
  kitTypeNameById.set(id, name);
  availableKindNames.add(name);
  const glb = primaryGlbFileName(row);
  if (glb) meshByKindName.set(name, `/meshes/${glb}`);
}

const design = JSON.parse(readFileSync(designPath, "utf8")) as Record<string, unknown>;
const pieceTypeNameById = new Map<string, string>();
for (const piece of itemsOf(design.pieces)) {
  if (!piece || typeof piece !== "object") continue;
  const p = piece as Record<string, unknown>;
  const pieceId = String(p.id ?? "");
  const typeId = String((p.type as { id?: string } | undefined)?.id ?? "");
  const typeName = kitTypeNameById.get(typeId) ?? "";
  if (pieceId && typeName) pieceTypeNameById.set(pieceId, typeName);
}

const fixture2d = JSON.parse(readFileSync(fixture2dPath, "utf8")) as Record<string, unknown>;
const paletteNames = new Set<string>();
for (const row of itemsOf((fixture2d.meta as { kindCatalogs?: { nodes?: unknown[] } } | undefined)?.kindCatalogs?.nodes)) {
  if (!row || typeof row !== "object") continue;
  const name = String((row as { name?: string }).name ?? "").trim();
  if (name === "" || name.startsWith("compose.")) continue;
  paletteNames.add(name);
}

const fixture3d = JSON.parse(readFileSync(fixture3dPath, "utf8")) as Record<string, unknown>;
const oldCatalog = itemsOf((fixture3d.meta as { kindCatalogs?: { objects?: unknown[] } }).kindCatalogs?.objects);
const catalogTemplateById = new Map<string, Record<string, unknown>>();
const catalogTemplateByMesh = new Map<string, Record<string, unknown>>();
for (const row of oldCatalog) {
  if (!row || typeof row !== "object") continue;
  const entry = row as Record<string, unknown>;
  const id = String(entry.id ?? "");
  const mesh = String(entry.meshUrl ?? "");
  if (id && !catalogTemplateById.has(id)) catalogTemplateById.set(id, entry);
  if (mesh && !catalogTemplateByMesh.has(mesh)) catalogTemplateByMesh.set(mesh, entry);
}

function catalogTemplateForKindName(kindName: string): Record<string, unknown> | undefined {
  if (catalogTemplateById.has(kindName)) return catalogTemplateById.get(kindName);
  const mesh = meshByKindName.get(kindName);
  if (mesh && catalogTemplateByMesh.has(mesh)) return catalogTemplateByMesh.get(mesh);
  const plain = /^Capsule (.+)$/.exec(kindName);
  if (plain) {
    const fallback = `Capsule ${plain[1]}`;
    if (catalogTemplateById.has(fallback)) return catalogTemplateById.get(fallback);
    const fallbackMesh = meshByKindName.get(fallback);
    if (fallbackMesh && catalogTemplateByMesh.has(fallbackMesh)) return catalogTemplateByMesh.get(fallbackMesh);
  }
  return undefined;
}

const newCatalogObjects: Record<string, unknown>[] = [];
for (const kindName of [...paletteNames].sort((a, b) => a.localeCompare(b))) {
  const template = catalogTemplateForKindName(kindName);
  const meshUrl = meshByKindName.get(kindName) ?? String(template?.meshUrl ?? "");
  newCatalogObjects.push({
    ...(template ?? {}),
    id: kindName,
    label: kindName,
    name: kindName,
    ...(meshUrl ? { meshUrl } : {}),
  });
}

const objects = itemsOf(fixture3d.objects);
let objectUpdates = 0;
for (const row of objects) {
  if (!row || typeof row !== "object") continue;
  const object = row as Record<string, unknown>;
  const objectId = String(object.id ?? "");
  const designTypeName = pieceTypeNameById.get(objectId) ?? String(object.objectKind ?? "");
  const kindName = puzzle3dPreferSpecificMetabolismKindName(designTypeName, availableKindNames);
  if (kindName !== object.objectKind) objectUpdates++;
  object.objectKind = kindName;
  const meshUrl = meshByKindName.get(kindName);
  if (meshUrl) object.meshUrl = meshUrl;
  const label = String(object.label ?? "");
  const caption = label.includes("·") ? label.split("·").slice(1).join("·").trim() : "";
  object.label = caption ? `${kindName} · ${caption}` : kindName;
}

(fixture3d.meta as Record<string, unknown>).kindCatalogs = {
  ...((fixture3d.meta as Record<string, unknown>).kindCatalogs as Record<string, unknown>),
  objects: newCatalogObjects,
};
fixture3d.objects = objects;

writeFileSync(fixture3dPath, `${JSON.stringify(fixture3d, null, 2)}\n`, "utf8");
console.log(
  `[sync-nakagin-3d] catalog rows ${newCatalogObjects.length}, object kind updates ${objectUpdates}, sample placed ${puzzle3dPreferSpecificMetabolismKindName("Capsule J", availableKindNames)}`,
);
