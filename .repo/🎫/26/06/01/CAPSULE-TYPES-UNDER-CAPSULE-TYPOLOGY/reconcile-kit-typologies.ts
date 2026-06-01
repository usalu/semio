#!/usr/bin/env bun
/** @emoji 🏛️ Reconcile kit types/designs into typology buckets; no typology-default on metabolism kits. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const METABOLOGY_TOPO_NAMES = ["base", "capsule", "tambour", "capital", "bridge", "tower"] as const;
/** @emoji 🗼 Design-name typology match order (`tower` before `capsule` so "Nakagin Capsule Tower" → Tower). */
const METABOLOGY_DESIGN_TOPO_PRIORITY = ["tower", "bridge", "capital", "tambour", "capsule", "base"] as const;
/** @emoji 🗼 Nakagin tower variant designs (children of Nakagin Capsule Tower). */
const TOWER_VARIANT_DESIGN_NAMES = new Set(["slanted", "twisted", "dancing", "flat"]);

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

function typeHasCapsuleRepresentation(row: Record<string, unknown>, files: Map<string, string>): boolean {
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const fileId = String((rep as { file?: { id?: string } }).file?.id ?? "");
    const name = files.get(fileId) ?? "";
    if (/capsule/i.test(name)) return true;
  }
  return false;
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
  if (typeHasCapsuleRepresentation(row, files)) return "typology-capsule";
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

const targets = walkJsonFiles(join(import.meta.dir, "../../../../../../semio/fixtures"));
let changed = 0;
for (const path of targets) {
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
    changed++;
    console.log("[DEBUG] reconciled", path);
  }
}
console.log(`[DEBUG] reconciled ${changed} kit documents`);
