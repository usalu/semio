#!/usr/bin/env bun
/**
 * 🏗️ WP-19 fixture migration (run once, kept as the record of the change): every committed JSON document of the BIM model subset moves from the old frame vocabulary to the new one.
 *  - a beam `{ start, end }` becomes `axis: { Line: { start, end } }` (also in `Created` diff entries, `Patched` entries and `Beam` placements);
 *  - a curtain wall `{ u_spacing, v_spacing, mullion, panel_material, mullion_material }` becomes a reference `curtain_wall_type` to a curtain wall type of the same unit
 *    (one type per distinct combination, ids `cwt-1`, `cwt-2` … in sorted order), and every snapshot of the unit gains the `curtain_wall_types` it needs.
 * A unit is the quintet directory of a mutation case, otherwise one file. `bun r12-w2-wp19-migrate.ts [--dry]`. Documents are rewritten only when they change.
 */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset } from "./r3-f1-paths.ts";

type Json = any;
const dry = process.argv.includes("--dry");
const roots = readdirSync(subset).filter((name) => /fixtures|assets|examples|tests/.test(name) || name.endsWith("io")).map((name) => join(subset, name));
const files: string[] = [];
const walk = (dir: string) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      if (name !== "node_modules") walk(path);
    } else if (name.endsWith(".json")) files.push(path);
  }
};
roots.forEach(walk);

const isBeam = (v: Json) => v && typeof v === "object" && typeof v.beam_type === "string" && "start" in v && "end" in v;
const isCurtain = (v: Json) => v && typeof v === "object" && typeof v.u_spacing === "number" && "mullion" in v;
const isSnapshot = (v: Json) => v && typeof v === "object" && typeof v.schema === "string" && "project" in v;
const typeKey = (v: Json) => JSON.stringify({ u: v.u_spacing, v: v.v_spacing, m: v.mullion, p: v.panel_material, q: v.mullion_material });

const insertAfter = (record: Json, after: string, key: string, value: Json): Json => {
  const out: Json = {};
  for (const [k, v] of Object.entries(record)) {
    out[k] = v;
    if (k === after) out[key] = value;
  }
  return out;
};
const beamRecord = (v: Json): Json => {
  const { start, end, ...rest } = v;
  return insertAfter(rest, "beam_type", "axis", { Line: { start, end } });
};
const curtainRecord = (v: Json, ids: Map<string, string>): Json => {
  const { u_spacing, v_spacing, mullion, panel_material, mullion_material, ...rest } = v;
  return insertAfter(rest, "storey", "curtain_wall_type", ids.get(typeKey(v)));
};
const typeRecord = (v: Json): Json => ({
  name: `Curtain wall ${v.u_spacing} x ${v.v_spacing}`,
  u_grid: { Spacing: { spacing: v.u_spacing } },
  v_grid: { Spacing: { spacing: v.v_spacing } },
  interior_mullion: v.mullion,
  border_mullion: v.mullion,
  panel: "Glass",
  panel_material: v.panel_material,
  mullion_material: v.mullion_material,
});

const collect = (value: Json, found: Map<string, Json>) => {
  if (Array.isArray(value)) value.forEach((item) => collect(item, found));
  else if (value && typeof value === "object") {
    if (isCurtain(value)) found.set(typeKey(value), value);
    Object.values(value).forEach((item) => collect(item, found));
  }
};

const convert = (value: Json, key: string, ids: Map<string, string>): Json => {
  if (Array.isArray(value)) return value.map((item) => convert(item, "", ids));
  if (!value || typeof value !== "object") return value;
  if (isBeam(value)) return beamRecord(value);
  if (isCurtain(value)) return curtainRecord(value, ids);
  if (key === "Beam" && "start" in value && "end" in value && !("beam_type" in value)) return { axis: { Line: { start: value.start, end: value.end } } };
  return Object.fromEntries(Object.entries(value).map(([k, v]) => [k, convert(v, k, ids)]));
};

const patchBeams = (diff: Json, oldBefore: Json | undefined): Json => {
  const entries = diff?.beams;
  if (!entries || typeof entries !== "object" || diff.schema !== undefined) return diff;
  for (const [id, entry] of Object.entries<Json>(entries)) {
    if (entry?.entry === "Patched" && ("start" in entry || "end" in entry)) {
      const base = oldBefore?.beams?.[id];
      const { start, end, ...rest } = entry;
      entries[id] = { ...rest, axis: { Line: { start: start ?? base.start, end: end ?? base.end } } };
    }
  }
  return diff;
};

const withTypes = (doc: Json, found: Map<string, Json>, ids: Map<string, string>): Json => {
  if (Array.isArray(doc)) return doc.map((item) => withTypes(item, found, ids));
  if (!doc || typeof doc !== "object") return doc;
  if (!isSnapshot(doc)) return Object.fromEntries(Object.entries(doc).map(([key, value]) => [key, withTypes(value, found, ids)]));
  if (found.size === 0) return doc;
  const types = Object.fromEntries([...found].map(([key, record]) => [ids.get(key)!, typeRecord(record)]).sort(([a], [b]) => (a < b ? -1 : 1)));
  return "door_types" in doc ? insertAfter(doc, "door_types", "curtain_wall_types", types) : { ...doc, curtain_wall_types: types };
};

const unitOf = (path: string): string => {
  const parts = path.split(/[\\/]/);
  const at = parts.findIndex((part, index) => index > 0 && parts[index - 1].endsWith("fixtures") && part.endsWith("mutations"));
  return at >= 0 && parts.length > at + 3 ? parts.slice(0, at + 3).join("/") : path;
};
const groups = new Map<string, string[]>();
for (const path of files) groups.set(unitOf(path), [...(groups.get(unitOf(path)) ?? []), path]);

let changed = 0;
for (const [, paths] of groups) {
  const texts = new Map(paths.map((path) => [path, readFileSync(path, "utf8")] as const));
  if (![...texts.values()].some((text) => /"beam_type"|"u_spacing"|"Beam"/.test(text))) continue;
  const docs = new Map([...texts].map(([path, text]) => [path, JSON.parse(text)] as const));
  const found = new Map<string, Json>();
  docs.forEach((doc) => collect(doc, found));
  const ids = new Map([...found.keys()].sort().map((key, index) => [key, `cwt-${index + 1}`] as const));
  const oldBefore = [...docs].find(([path]) => /before[\\/]/.test(path))?.[1];
  for (const [path, doc] of docs) {
    const snapshotUnit = isSnapshot(doc);
    const own = snapshotUnit && paths.length === 1 ? (() => { const m = new Map<string, Json>(); collect(doc, m); return m; })() : found;
    const next = withTypes(patchBeams(convert(doc, "", ids), oldBefore), own, ids);
    if (JSON.stringify(next) !== JSON.stringify(doc)) {
      changed++;
      if (!dry) writeFileSync(path, JSON.stringify(next, null, 2) + "\n");
    }
  }
}
console.log(`${dry ? "would migrate" : "migrated"} ${changed} documents of ${files.length}`);
