/** 🏗️ Assembles the round 3 sleeping person: the catalog document with the root fixes (`rooted()`), the body, thermal model and structure of every authored scale from the hand-stated lists, the dimensions from the 1:50 body in the default pose, the provenance note, and the redrawn 2D geometry with its drawing decisions when `🗑️generated/r3-sleeping-person/geometry.json` exists. Writes the candidate document `🗑️generated/r3-sleeping-person/🔣️.json`; `--source` also writes `volumes/humans/sleeping-person.json`.
 * Usage: `bun r3-sleeping-person-build.ts [--source]`
 * @see ./r3-sleeping-person-roots.ts */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, type Json } from "./r3-sleeping-person-kit.ts";
import { body50 } from "./r3-sleeping-person-s50.ts";
import { body100, body200, body500 } from "./r3-sleeping-person-coarse.ts";
import { structure50, structureCoarse, thermal } from "./r3-sleeping-person-lists.ts";
import { rooted } from "./r3-sleeping-person-roots.ts";
import { DECISIONS, NOTES, REP_DECISIONS } from "./r3-sleeping-person-texts.ts";
import { meshesIn, rigAt } from "./r3-lib.ts";

export const SCALES = [50, 100, 200, 500] as const;
export const OUT = join(import.meta.dir, "🗑️generated", "r3-sleeping-person");
export const CANDIDATE = join(OUT, "🔣️.json");
const BODY: Record<number, (k: Kit) => void> = { 50: body50, 100: body100, 200: body200, 500: body500 };

/** 🧱️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): { volume: Json; kit: Kit } {
  const kit = new Kit();
  BODY[scale]!(kit);
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale), structure: scale === 50 ? structure50(asset) : structureCoarse(asset, scale), decisions: DECISIONS[scale] } };
}

/** 📦️ The world box of the non-slender body solids of a scale in the default pose. */
export function boxOf(asset: Json, scale: number): number[][] {
  const frames = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  const box = [[Infinity, -Infinity], [Infinity, -Infinity], [Infinity, -Infinity]];
  for (const m of meshesIn(asset, frames, (asset.volumes[String(scale)].body as Json[]).filter((s) => s.slender !== true))) for (let i = 0; i < m.world.length; i++) box[i % 3] = [Math.min(box[i % 3]![0]!, m.world[i]!), Math.max(box[i % 3]![1]!, m.world[i]!)];
  return box;
}

/** 📄️ The candidate document: root fixes, volumes of every scale, and the 2D geometry when it has been drawn. */
export function candidate(report: string[] = [], draw = true): Json {
  const document = rooted(report);
  const asset = document.asset as Json;
  for (const s of SCALES) asset.volumes[String(s)] = build(asset, s).volume;
  const box = boxOf(asset, 50);
  const span = (k: number): number => Math.round((box[k]![1]! - box[k]![0]!) * 1000) / 1000;
  asset.dimensions = { ...asset.dimensions, heightM: span(1), widthM: span(0), depthM: span(2), massKg: Math.round((asset.rig.bones as Json[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0) * 1e4) / 1e4 };
  const geometry = join(OUT, "geometry.json");
  if (draw && existsSync(geometry)) {
    const drawn = JSON.parse(readFileSync(geometry, "utf8")) as Json;
    for (const [scale, views] of Object.entries(drawn)) for (const [view, shapes] of Object.entries(views as Json)) asset.representations[scale].geometry[view] = shapes;
    for (const s of SCALES) asset.representations[String(s)].decisions = { ...asset.representations[String(s)].decisions, ...REP_DECISIONS[s] };
  }
  asset.provenance.notes = NOTES;
  return document;
}

if (import.meta.main) {
  const document = candidate();
  mkdirSync(OUT, { recursive: true });
  writeFileSync(CANDIDATE, JSON.stringify(document));
  const volumes = document.asset.volumes as Json;
  if (process.argv.includes("--source")) writeFileSync(join(import.meta.dir, "volumes", "humans", "sleeping-person.json"), `${JSON.stringify({ volumes })}\n`);
  console.log("dimensions", JSON.stringify(document.asset.dimensions));
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
