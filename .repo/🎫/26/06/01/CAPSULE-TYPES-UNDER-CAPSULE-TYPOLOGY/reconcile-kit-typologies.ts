#!/usr/bin/env bun
/** @emoji 🏛️ Reconcile kit types/designs into typology buckets by typology ref, capsule reps, and metabolism names. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const METABOLOGY_TOPO_NAMES = ["base", "capsule", "tambour", "capital", "bridge", "tower"] as const;

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
  return tail === "default" ? "Default" : tail.charAt(0).toUpperCase() + tail.slice(1);
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

function typeHasCapsuleRepresentation(row: Record<string, unknown>, files: Map<string, string>): boolean {
  for (const rep of itemsOf(row.representations)) {
    if (!rep || typeof rep !== "object") continue;
    const fileId = String((rep as { file?: { id?: string } }).file?.id ?? "");
    const name = files.get(fileId) ?? "";
    if (/capsule/i.test(name)) return true;
  }
  return false;
}

function intendedTypologyIdForType(row: Record<string, unknown>, files: Map<string, string>): string {
  if (typeHasCapsuleRepresentation(row, files)) return "typology-capsule";
  const explicit = String((row.typology as { id?: string } | undefined)?.id ?? "");
  if (explicit.startsWith("typology-")) return explicit;
  const nm = String(row.name ?? "").toLowerCase();
  const topoName = METABOLOGY_TOPO_NAMES.find((x) => nm === x || nm.includes(x)) ?? "default";
  return newTypologyId(topoName);
}

function intendedTypologyIdForDesign(row: Record<string, unknown>): string {
  const explicit = String((row.typology as { id?: string } | undefined)?.id ?? "");
  if (explicit.startsWith("typology-")) return explicit;
  const nm = String(row.name ?? "").toLowerCase();
  for (const x of METABOLOGY_TOPO_NAMES) {
    if (nm.includes(x)) return newTypologyId(x);
  }
  return "typology-default";
}

function reconcileKitObject(kit: Record<string, unknown>): boolean {
  const topos = itemsOf(kit.typologies);
  if (topos.length === 0) return false;
  const files = fileNameById(kit);
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
    row.typology = { id: intendedTypologyIdForType(row, files) };
  }
  for (const row of allDesigns) {
    row.typology = { id: intendedTypologyIdForDesign(row) };
  }

  for (const topo of topoById.values()) {
    topo.types = blockFrom([]);
    topo.designs = blockFrom([]);
  }

  for (const row of allTypes) {
    const tid = String((row.typology as { id?: string }).id ?? "typology-default");
    let bucket = topoById.get(tid);
    if (!bucket) {
      bucket = { id: tid, name: typologyNameFromId(tid), types: blockFrom([]), designs: blockFrom([]) };
      topoById.set(tid, bucket);
      topos.push(bucket);
    }
    itemsOf(bucket.types).push(row);
  }
  for (const row of allDesigns) {
    const tid = String((row.typology as { id?: string }).id ?? "typology-default");
    let bucket = topoById.get(tid);
    if (!bucket) {
      bucket = { id: tid, name: typologyNameFromId(tid), types: blockFrom([]), designs: blockFrom([]) };
      topoById.set(tid, bucket);
      topos.push(bucket);
    }
    itemsOf(bucket.designs).push(row);
  }

  for (const topo of topoById.values()) {
    topo.types = blockFrom(itemsOf(topo.types));
    topo.designs = blockFrom(itemsOf(topo.designs));
  }

  kit.typologies = blockFrom(topos);
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
