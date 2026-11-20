/** 🏗️ Assembles the round 3 wheelchair user: the catalog document with the root fixes (`rooted()`), the body, thermal model and structure of every authored scale from the hand-stated lists, the provenance note, and the redrawn 2D geometry with its drawing decisions when `🗑️generated/r3-wheelchair-user/geometry.json` exists. Writes the candidate document `🗑️generated/r3-wheelchair-user/🔣️.json`; `--source` also writes `volumes/humans/wheelchair-user.json`.
 * Usage: `bun r3-wheelchair-user-build.ts [--source]`
 * @see ./r3-wheelchair-user-roots.ts
 * @see ./r3-wheelchair-user-lists.ts */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, type Json } from "./r3-wheelchair-user-kit.ts";
import { BODY, DECISIONS, EXTRA, FACE, finish, structure, thermal } from "./r3-wheelchair-user-lists.ts";
import { rooted } from "./r3-wheelchair-user-roots.ts";
import { NOTES, REP_DECISIONS } from "./r3-wheelchair-user-texts.ts";
import { meshesIn, rigAt } from "./r3-lib.ts";

export const SCALES = [50, 100, 200, 500] as const;
export const OUT = join(import.meta.dir, "🗑️generated", "r3-wheelchair-user");
export const CANDIDATE = join(OUT, "🔣️.json");
export type Built = { volume: Json; kit: Kit };

/** 🧱️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  BODY[scale](kit);
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale, FACE[scale], EXTRA[scale]), structure: structure(asset, scale, kit.solids), decisions: DECISIONS[scale] } };
}

/** 📄️ The candidate document: root fixes, volumes of every scale, and the 2D geometry when it has been drawn. */
export function candidate(report: string[] = [], draw = true): Json {
  const document = rooted(report);
  const asset = document.asset as Json;
  finish(asset);
  for (const s of SCALES) asset.volumes[String(s)] = build(asset, s).volume;
  const frames = rigAt(asset, 50, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  const box = [[Infinity, -Infinity], [Infinity, -Infinity], [Infinity, -Infinity]];
  for (const m of meshesIn(asset, frames, (asset.volumes["50"].body as Json[]).filter((s) => s.slender !== true))) for (let i = 0; i < m.world.length; i++) box[i % 3] = [Math.min(box[i % 3]![0]!, m.world[i]!), Math.max(box[i % 3]![1]!, m.world[i]!)];
  const span = (k: number): number => Math.round((box[k]![1]! - box[k]![0]!) * 1000) / 1000;
  asset.dimensions = { ...asset.dimensions, heightM: Math.round(box[1]![1]! * 1000) / 1000, widthM: span(0), depthM: span(2) };
  const geometry = join(OUT, "geometry.json");
  if (draw && existsSync(geometry)) {
    const drawn = JSON.parse(readFileSync(geometry, "utf8")) as Json;
    for (const [scale, views] of Object.entries(drawn)) for (const [view, shapes] of Object.entries(views as Json)) asset.representations[scale].geometry[view] = shapes;
    for (const s of SCALES) asset.representations[String(s)].decisions = { ...asset.representations[String(s)].decisions, ...REP_DECISIONS[s] };
  }
  if (NOTES !== null) asset.provenance.notes = NOTES;
  for (const s of [100, 200]) asset.representations[String(s)].pens.texture = 0.1;
  return document;
}

if (import.meta.main) {
  const document = candidate();
  mkdirSync(OUT, { recursive: true });
  writeFileSync(CANDIDATE, JSON.stringify(document));
  const volumes = document.asset.volumes as Json;
  if (process.argv.includes("--source")) writeFileSync(join(import.meta.dir, "volumes", "humans", "wheelchair-user.json"), `${JSON.stringify({ volumes })}\n`);
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
