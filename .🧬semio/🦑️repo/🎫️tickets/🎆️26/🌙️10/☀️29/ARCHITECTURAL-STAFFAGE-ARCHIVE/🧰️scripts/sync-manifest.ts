/** 🔄️ Ticket script: reconciles manifest rows with the authored assets (pose/clip ids, evaluated default-pose height and width,
 * completion) and merges a scratch category document into its catalog file; prints a diff summary. Usage:
 * `repo tools exec bun -- sync-manifest.ts [--merge <scratch.json>] [--write]` */
import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import type { CategoryDocument, Manifest } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const CATALOG = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const { values } = parseArgs({ options: { merge: { type: "string" }, write: { type: "boolean", default: false } } });
const manifest = JSON.parse(readFileSync(`${CATALOG}/🔣️.json`, "utf8")) as Manifest;
const round = (n: number): number => Math.round(n * 100) / 100;
const json = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;

if (values.merge) {
  const scratch = JSON.parse(readFileSync(values.merge, "utf8")) as CategoryDocument;
  const file = `${CATALOG}/${manifest.categories.find((c) => c.id === scratch.category)!.file}`;
  const target = JSON.parse(readFileSync(file, "utf8")) as CategoryDocument;
  const ids = new Set(scratch.assets.map((a) => a.id));
  const merged = { ...target, assets: [...target.assets.filter((a) => !ids.has(a.id)), ...scratch.assets] };
  const order = manifest.entries.map((e) => e.id);
  merged.assets.sort((a, b) => order.indexOf(a.id) - order.indexOf(b.id));
  console.log(`[DEBUG] merge ${scratch.assets.length} into ${file} → ${merged.assets.length}`);
  if (values.write) writeFileSync(file, json(merged));
}

const assets = new Map(manifest.categories.flatMap((c) => {
  try { return (JSON.parse(readFileSync(`${CATALOG}/${c.file}`, "utf8")) as CategoryDocument).assets; } catch { return []; }
}).map((a) => [a.id, a]));

const entries = manifest.entries.map((entry) => {
  const asset = assets.get(entry.id);
  if (!asset) return entry;
  const b = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bounds;
  const next = { ...entry, heightM: round(b.maxY - Math.min(0, b.minY)), widthM: round(b.maxX - b.minX), poses: asset.poses.filter((p) => !p.key).map((p) => p.id), clips: asset.clips.map((c) => c.id), viewpoints: [...asset.presentation.viewpoints], completion: asset.completion };
  const changes = (Object.keys(next) as (keyof typeof next)[]).filter((k) => JSON.stringify(next[k]) !== JSON.stringify(entry[k]));
  if (changes.length) console.log(`[DEBUG] ${entry.id}: ${changes.map((k) => `${k} ${JSON.stringify(entry[k])} → ${JSON.stringify(next[k])}`).join("; ")}`);
  return next;
});
console.log(`[DEBUG] authored ${assets.size}/${manifest.entries.length}`);
if (values.write) writeFileSync(`${CATALOG}/🔣️.json`, json({ ...manifest, entries }));
