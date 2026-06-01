#!/usr/bin/env bun
/** @emoji 🏷️ Reconcile kind names from representation stems and sync puzzle nakagin fixtures. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileStemForNaming, typeNameFromFileName } from "../../repo/lib/js/src/kind-name-from-representation.ts";

const METABOLOGY_TOPO_NAMES = ["base", "capsule", "tambour", "capital", "bridge", "tower"] as const;
const METABOLOGY_DESIGN_TOPO_PRIORITY = ["tower", "bridge", "capital", "tambour", "capsule", "base"] as const;
const TOWER_VARIANT_DESIGN_NAMES = new Set(["slanted", "twisted", "dancing", "flat"]);
const SKIP_FIXTURES = new Set(["architect.harness.kit.semio.json"]);
const FIXTURES_ROOT = join(import.meta.dir);
const TYPE_FILES_ROOT = join(FIXTURES_ROOT, "kit/dev/metabolism");

function itemsOf(block: unknown): unknown[] {
  if (Array.isArray(block)) return block;
  if (block && typeof block === "object" && Array.isArray((block as { items?: unknown[] }).items)) {
    return (block as { items: unknown[] }).items;
  }
  return [];
}

function blockFrom(items: unknown[]): Record<string, unknown> {
  return { hash: "typology-migration-stub", items };
}

function newTypologyId(name: string): string {
  return `typology-${name.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`;
}

function typologyNameFromId(id: string): string {
  const tail = id.replace(/^typology-/, "");
  return tail.charAt(0).toUpperCase() + tail.slice(1);
}

function fileNameById(kit: Record<string, unknown>): Map<string, string> {
  const out = new Map<string, string>();
  for (const f of itemsOf(kit.files)) {
    if (!f || typeof f !== "object") continue;
    const row = f as Record<string, unknown>;
    const id = String(row.id ?? "");
    if (id) out.set(id, String(row.name ?? ""));
  }
  return out;
}

function familyNameById(kit: Record<string, unknown>): Map<string, string> {
  const out = new Map<string, string>();
  for (const f of itemsOf(kit.families)) {
    if (!f || typeof f !== "object") continue;
    const row = f as Record<string, unknown>;
    const id = String(row.id ?? "");
    if (id) out.set(id, String(row.name ?? ""));
  }
  return out;
}

export function iconOrImageFileStem(row: Record<string, unknown>): string {
  for (const key of ["icon", "image"] as const) {
    const path = String(row[key] ?? "");
    const base = path.split("/").pop() ?? "";
    if (!base) continue;
    return fileStemForNaming(base);
  }
  return "";
}

export function primaryRepresentationFileName(row: Record<string, unknown>, files: Map<string, string>): string {
  const glbs: string[] = [];
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const fileId = String((rep as { file?: { id?: string } }).file?.id ?? "");
    const fromFile = files.get(fileId) ?? "";
    const fromRep = String((rep as { name?: string }).name ?? "");
    const name = fromFile || fromRep;
    if (name.toLowerCase().endsWith(".glb") && !name.toLowerCase().includes("collider")) glbs.push(name);
  }
  return glbs.find((n) => !/_1to\d+/i.test(n)) ?? glbs[0] ?? "";
}

function primaryFromRepresentationNames(row: Record<string, unknown>): string {
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const raw = String((rep as { name?: string }).name ?? "");
    if (!raw || /collider/i.test(raw)) continue;
    const stem = fileStemForNaming(raw.includes(".") ? raw : `${raw}.glb`);
    if (stem && !/_1to\d+$/i.test(stem)) return stem;
  }
  return "";
}

function namingSourceForType(row: Record<string, unknown>, files: Map<string, string>): string {
  return primaryRepresentationFileName(row, files) || primaryFromRepresentationNames(row) || iconOrImageFileStem(row);
}

function syncRepresentationNames(row: Record<string, unknown>, files: Map<string, string>): void {
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const r = rep as Record<string, unknown>;
    const fileId = String((r.file as { id?: string } | undefined)?.id ?? "");
    const fileName = files.get(fileId) ?? String(r.name ?? "");
    if (!fileName) continue;
    const stem = fileStemForNaming(fileName);
    if (stem) r.name = stem;
  }
}

export function renameTypeRow(row: Record<string, unknown>, files: Map<string, string>): string | undefined {
  const before = String(row.name ?? "");
  const primary = namingSourceForType(row, files);
  if (!primary) return undefined;
  const derivedName = typeNameFromFileName(primary.includes(".") ? primary : `${primary}.glb`);
  if (derivedName) row.name = derivedName;
  syncRepresentationNames(row, files);
  const after = String(row.name ?? "");
  return after !== before ? before : undefined;
}

function typologyIdFromRepresentationFile(fileName: string): string | null {
  const n = fileName.toLowerCase();
  if (!n) return null;
  if (n.includes("tambour") && !n.includes("capsule")) return "typology-tambour";
  if (
    n.includes("ellipsoid-capsule") ||
    n.includes("trapezoid-capsule") ||
    n.includes("capsule-with-balcony") ||
    /^capsule[_-]/.test(n) ||
    (n.includes("capsule") && !n.includes("tambour"))
  ) {
    return "typology-capsule";
  }
  if (n.includes("capital")) return "typology-capital";
  if (n.includes("bridge")) return "typology-bridge";
  if (n.includes("base")) return "typology-base";
  return null;
}

function typologyIdFromFamilies(row: Record<string, unknown>, families: Map<string, string>): string | null {
  for (const fam of itemsOf(row.families)) {
    if (!fam || typeof fam !== "object") continue;
    const famId = String((fam as { id?: string }).id ?? "");
    const famName = (families.get(famId) ?? "").toLowerCase();
    if (!famName) continue;
    if (/nakagin|tower/i.test(famName)) return "typology-tower";
    const match = METABOLOGY_TOPO_NAMES.find((x) => famName.includes(x));
    if (match) return newTypologyId(match);
  }
  return null;
}

function intendedTypologyIdForType(
  row: Record<string, unknown>,
  files: Map<string, string>,
  families: Map<string, string>,
): string {
  const primary = namingSourceForType(row, files);
  if (primary) {
    renameTypeRow(row, files);
    const fromFile = typologyIdFromRepresentationFile(primary);
    if (fromFile) return fromFile;
  }
  const nm = String(row.name ?? "").toLowerCase();
  if (nm.includes("sketchpad") && nm.includes("default")) return "typology-base";
  const fromName = METABOLOGY_TOPO_NAMES.find((x) => nm === x || nm.includes(x));
  if (fromName) return newTypologyId(fromName);
  const fromFamily = typologyIdFromFamilies(row, families);
  if (fromFamily) return fromFamily;
  if (/storey|tambour|cylindric/i.test(nm)) return "typology-tambour";
  if (/ellipsoid|trapezoid|balcony/i.test(nm)) return "typology-capsule";
  return "typology-base";
}

function intendedTypologyIdForDesign(row: Record<string, unknown>): string {
  const nm = String(row.name ?? "").toLowerCase();
  if (TOWER_VARIANT_DESIGN_NAMES.has(nm) || nm.includes("tower")) return "typology-tower";
  const fromName = METABOLOGY_DESIGN_TOPO_PRIORITY.find((x) => nm.includes(x));
  if (fromName) return newTypologyId(fromName);
  return "typology-base";
}

function reconcileKitObject(kit: Record<string, unknown>): boolean {
  const topos = itemsOf(kit.typologies);
  if (topos.length === 0) return false;
  const files = fileNameById(kit);
  const families = familyNameById(kit);
  const topoById = new Map<string, Record<string, unknown>>();
  for (const topo of topos) {
    if (!topo || typeof topo !== "object") continue;
    const row = topo as Record<string, unknown>;
    const id = String(row.id ?? "");
    if (!id) continue;
    topoById.set(id, row);
  }

  const allTypes: Record<string, unknown>[] = [];
  const allDesigns: Record<string, unknown>[] = [];
  for (const topo of topoById.values()) {
    allTypes.push(...(itemsOf(topo.types) as Record<string, unknown>[]));
    allDesigns.push(...(itemsOf(topo.designs) as Record<string, unknown>[]));
  }
  for (const row of allTypes) {
    row.typology = { id: intendedTypologyIdForType(row, files, families) };
  }
  for (const row of allDesigns) {
    row.typology = { id: intendedTypologyIdForDesign(row) };
  }

  for (const topo of topoById.values()) {
    topo.types = blockFrom([]);
    topo.designs = blockFrom([]);
  }

  const ensureBucket = (tid: string) => {
    let bucket = topoById.get(tid);
    if (!bucket) {
      bucket = { id: tid, name: typologyNameFromId(tid), types: blockFrom([]), designs: blockFrom([]) };
      topoById.set(tid, bucket);
    }
    return bucket;
  };

  for (const row of allTypes) {
    const tid = String((row.typology as { id?: string }).id ?? "typology-base");
    itemsOf(ensureBucket(tid).types).push(row);
  }
  for (const row of allDesigns) {
    const tid = String((row.typology as { id?: string }).id ?? "typology-base");
    itemsOf(ensureBucket(tid).designs).push(row);
  }

  for (const topo of topoById.values()) {
    topo.types = blockFrom(itemsOf(topo.types));
    topo.designs = blockFrom(itemsOf(topo.designs));
  }

  topoById.delete("typology-default");
  const finalTopos = [...topoById.values()].filter((row) => {
    const id = String(row.id ?? "");
    if (id === "typology-default") return false;
    return itemsOf(row.types).length > 0 || itemsOf(row.designs).length > 0;
  });

  kit.typologies = blockFrom(finalTopos);
  delete kit.types;
  delete kit.designs;
  return true;
}

function walkJsonFiles(dir: string, out: string[] = []): string[] {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walkJsonFiles(p, out);
    else if (ent.name.endsWith(".semio.json")) out.push(p);
  }
  return out;
}

function loadTypeNameMaps(): { byId: Map<string, string>; renames: Map<string, string> } {
  const byId = new Map<string, string>();
  const renames = new Map<string, string>();
  for (const path of walkJsonFiles(TYPE_FILES_ROOT)) {
    if (!path.endsWith(".type.semio.json")) continue;
    const row = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    const id = String(row.id ?? "");
    const before = String(row.name ?? "");
    const files = new Map<string, string>();
    const previous = renameTypeRow(row, files);
    const after = String(row.name ?? "");
    if (id) byId.set(id, after);
    if (previous) renames.set(previous, after);
    else if (before !== after) renames.set(before, after);
    if (previous || before !== after) {
      writeFileSync(path, `${JSON.stringify(row, null, 4)}\n`);
      console.log("[DEBUG] type file", path.split("/").pop(), previous ?? before, "→", after);
    }
  }
  return { byId, renames };
}

function replaceKindNamesInJson(value: unknown, renames: Map<string, string>): unknown {
  if (typeof value === "string") {
    return renames.get(value) ?? value;
  }
  if (Array.isArray(value)) {
    return value.map((entry) => replaceKindNamesInJson(entry, renames));
  }
  if (value && typeof value === "object") {
    const row = value as Record<string, unknown>;
    const out: Record<string, unknown> = {};
    for (const [key, child] of Object.entries(row)) {
      if (key === "id" || key === "name" || key === "label" || key === "objectKind" || key === "partKind" || key === "nodeKind") {
        if (typeof child === "string" && renames.has(child)) {
          out[key] = renames.get(child);
          continue;
        }
        if (key === "nodeKind" && typeof child === "string" && child.startsWith("semio.metabolism.light.node.")) {
          out[key] = child;
          continue;
        }
      }
      out[key] = replaceKindNamesInJson(child, renames);
    }
    return out;
  }
  return value;
}

const MESH_STEM_KIND_NAME: Record<string, string> = {
  "tambour_last-storey": "Last Storey Tambour",
  "tambour_first-storey": "First Storey Tambour",
  "tambour_single-storey": "Single Storey Tambour",
  "cylindric-tambour_last-storey": "Cylindric Last Storey Tambour",
  "cylindric-tambour_first-storey": "Cylindric First Storey Tambour",
  "cylindric-tambour_single-storey": "Cylindric Single Storey Tambour",
};

function kindNameFromMeshUrl(meshUrl: unknown): string | undefined {
  if (typeof meshUrl !== "string") return undefined;
  const base = meshUrl.split("/").pop() ?? "";
  const stem = fileStemForNaming(base);
  return MESH_STEM_KIND_NAME[stem];
}

function syncPuzzle3dKindCatalog(doc: Record<string, unknown>, renames: Map<string, string>): boolean {
  const meta = doc.meta as Record<string, unknown> | undefined;
  const catalogs = meta?.kindCatalogs as Record<string, unknown> | undefined;
  const objects = catalogs?.objects as unknown[] | undefined;
  if (!Array.isArray(objects)) return false;
  let touched = false;
  for (const entry of objects) {
    if (!entry || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    const fromMesh = kindNameFromMeshUrl(row.meshUrl);
    const current = String(row.id ?? row.name ?? "");
    const next = fromMesh ?? renames.get(current);
    if (!next || next === current) continue;
    if (row.id === current) row.id = next;
    if (row.name === current || row.name === undefined) row.name = next;
    if (row.label === current || row.label === undefined) row.label = next;
    renames.set(current, next);
    touched = true;
  }
  return touched;
}

function syncPuzzleNodeCatalogNames(doc: Record<string, unknown>, byId: Map<string, string>): boolean {
  const meta = doc.meta as Record<string, unknown> | undefined;
  const catalogs = meta?.kindCatalogs as Record<string, unknown> | undefined;
  const nodes = catalogs?.nodes as unknown[] | undefined;
  if (!Array.isArray(nodes)) return false;
  let touched = false;
  for (const entry of nodes) {
    if (!entry || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    const id = String(row.id ?? "");
    const suffix = id.replace(/^semio\.metabolism\.light\.node\./, "");
    const name = byId.get(suffix);
    if (!name || row.name === name) continue;
    row.name = name;
    touched = true;
  }
  return touched;
}

function syncPuzzleFixtures(byId: Map<string, string>, renames: Map<string, string>): void {
  const puzzlePaths = [
    join(FIXTURES_ROOT, "../../puzzle/2d/fixture/nakagin-capsule-tower.2d.json"),
    join(FIXTURES_ROOT, "../../puzzle/3d/fixture/nakagin-capsule-tower.3d.json"),
    join(FIXTURES_ROOT, "../../puzzle/5d/fixture/nakagin-capsule-tower.5d.json"),
    join(FIXTURES_ROOT, "../../.storybook/fixtures/nakagin-capsule-tower.board.json"),
  ];
  for (const path of puzzlePaths) {
    const doc = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    const nodeTouched = syncPuzzleNodeCatalogNames(doc, byId);
    const catalogTouched = path.includes("/3d/") ? syncPuzzle3dKindCatalog(doc, renames) : false;
    const replaced = replaceKindNamesInJson(doc, renames) as Record<string, unknown>;
    if (nodeTouched || catalogTouched || JSON.stringify(replaced) !== JSON.stringify(doc)) {
      writeFileSync(path, `${JSON.stringify(replaced, null, 2)}\n`);
      console.log("[DEBUG] puzzle fixture", path);
    }
  }
}

let kitChanged = 0;
for (const path of walkJsonFiles(FIXTURES_ROOT)) {
  if (SKIP_FIXTURES.has(path.split("/").pop() ?? "")) continue;
  let doc: Record<string, unknown>;
  try {
    doc = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
  } catch {
    continue;
  }
  let touched = false;
  const visit = (obj: Record<string, unknown>) => {
    if (itemsOf(obj.typologies).length > 0 && reconcileKitObject(obj)) touched = true;
    if (obj.wip && typeof obj.wip === "object") visit(obj.wip as Record<string, unknown>);
    if (obj.initialKit && typeof obj.initialKit === "object") visit(obj.initialKit as Record<string, unknown>);
  };
  visit(doc);
  if (touched) {
    writeFileSync(path, `${JSON.stringify(doc, null, 4)}\n`);
    kitChanged++;
    console.log("[DEBUG] kit", path);
  }
}
console.log(`[DEBUG] reconciled ${kitChanged} kit documents`);

const { byId, renames } = loadTypeNameMaps();
syncPuzzleFixtures(byId, renames);
console.log(`[DEBUG] ${renames.size} kind renames, ${byId.size} type ids`);
