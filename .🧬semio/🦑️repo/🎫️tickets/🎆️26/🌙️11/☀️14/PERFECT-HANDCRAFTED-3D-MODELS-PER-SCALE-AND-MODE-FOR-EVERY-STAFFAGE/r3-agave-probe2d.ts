/** 🔎️ Round 3, vegetation/agave: finds the 2D shapes of the candidate document that the product cannot place (union failures) by evaluating every pose and clip sample with halves of the shape list. Usage: bun r3-agave-probe2d.ts [scale ...] */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, STAFFAGE } from "./volumes-lib.ts";
const { evaluate } = await import(join(STAFFAGE, "🔨️modules/🧮️evaluation/🟦️.ts"));
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-agave", "🔣️.json"), "utf8")).asset;
const queries: any[] = [...asset.poses.map((p: any) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c: any) => [0, 0.25, 0.5, 0.75, 1].map((f) => ({ kind: "clip", clip: c.id, timeS: f * (c.durationS ?? 1) })))];
const scales = process.argv.slice(2).length ? process.argv.slice(2).map(Number) : [50, 100, 200, 500];
for (const scale of scales) {
  const full = asset.representations[String(scale)].geometry;
  const fails = (keep: (id: string) => boolean, q: any): boolean => {
    const a = { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: Object.fromEntries(Object.entries(full).map(([v, list]) => [v, (list as any[]).filter((s) => keep(s.id))])) } } };
    try { const st = evaluate(a, q, { kind: "scale", scale: { denominator: scale } }); JSON.stringify(st.shapes ?? st); return false; } catch (e) { return String(e).includes("union"); }
  };
  for (const q of queries) {
    if (!fails(() => true, q)) continue;
    let ids: string[] = full.elevation.map((s: any) => s.id);
    while (ids.length > 1) {
      const half = ids.slice(0, ids.length >> 1), set = new Set(half);
      ids = fails((id) => set.has(id), q) ? half : ids.slice(ids.length >> 1);
    }
    console.log(`[DEBUG] 1:${scale} ${JSON.stringify(q)} first failing shape ${ids[0]}`);
  }
}
