#!/usr/bin/env bun
/** @emoji 🏛️ One-off: nest kit `types`/`designs` under `typologies[]` and set type/design `typology` refs. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = join(import.meta.dir, "../../../../../../");
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

function migrateKitObject(kit: Record<string, unknown>): void {
  if (kit.typologies && itemsOf(kit.typologies).length > 0) return;
  const types = itemsOf(kit.types);
  const designs = itemsOf(kit.designs);
  const typologyBuckets = new Map<string, { types: unknown[]; designs: unknown[] }>();

  const ensureTopo = (name: string) => {
    const key = name.toLowerCase();
    if (!typologyBuckets.has(key)) typologyBuckets.set(key, { types: [], designs: [] });
    return typologyBuckets.get(key)!;
  };

  for (const t of types) {
    if (!t || typeof t !== "object") continue;
    const row = t as Record<string, unknown>;
    const nm = String(row.name ?? "").toLowerCase();
    const topoName = METABOLOGY_TOPO_NAMES.find((x) => nm === x || nm.includes(x)) ?? "default";
    const bucket = ensureTopo(topoName);
    row.typology = { id: newTypologyId(topoName) };
    bucket.types.push(row);
  }

  for (const d of designs) {
    if (!d || typeof d !== "object") continue;
    const row = d as Record<string, unknown>;
    const nm = String(row.name ?? "").toLowerCase();
    let topoName = "default";
    for (const x of METABOLOGY_TOPO_NAMES) {
      if (nm.includes(x)) {
        topoName = x;
        break;
      }
    }
    const bucket = ensureTopo(topoName);
    row.typology = { id: newTypologyId(topoName) };
    bucket.designs.push(row);
  }

  if (typologyBuckets.size === 0) {
    typologyBuckets.set("default", { types, designs });
  }

  const typologies = [...typologyBuckets.entries()].map(([name, bucket]) => ({
    id: newTypologyId(name),
    name: name === "default" ? "Default" : name.charAt(0).toUpperCase() + name.slice(1),
    types: blockFrom(bucket.types),
    designs: blockFrom(bucket.designs),
  }));

  kit.typologies = blockFrom(typologies);
  delete kit.types;
  delete kit.designs;
}

function walkJsonFiles(dir: string, out: string[] = []): string[] {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walkJsonFiles(p, out);
    else if (ent.name.endsWith(".semio.json")) out.push(p);
  }
  return out;
}

const targets = [
  ...walkJsonFiles(join(ROOT, "semio/fixtures")),
];
const metabolismAssets = join(ROOT, "semio/assets/semio/metabolism");
try {
  targets.push(...walkJsonFiles(metabolismAssets));
} catch {
  /* optional metabolism tree */
}

let changed = 0;
for (const path of targets) {
  let doc: Record<string, unknown>;
  try {
    const raw = readFileSync(path, "utf8");
    doc = JSON.parse(raw) as Record<string, unknown>;
  } catch (err) {
    console.warn("[DEBUG] skip", path, err);
    continue;
  }
  let touched = false;

  const visit = (obj: Record<string, unknown>) => {
    if (obj.types || obj.designs) {
      migrateKitObject(obj);
      touched = true;
    }
    if (obj.wip && typeof obj.wip === "object") visit(obj.wip as Record<string, unknown>);
    if (obj.initialKit && typeof obj.initialKit === "object") visit(obj.initialKit as Record<string, unknown>);
  };

  visit(doc);
  if (touched) {
    writeFileSync(path, `${JSON.stringify(doc, null, 4)}\n`);
    changed++;
    console.log("[DEBUG] migrated", path);
  }
}

console.log(`[DEBUG] migrated ${changed} kit documents`);
