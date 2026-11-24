/** 🔝️ The drawn shapes that reach furthest in one direction of a view (default pose), with their extents, beside the extent of the body solids: bun tops.ts <slug> <scale> <view> <maxY|minY|maxX|minX> [count] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";

const [slug, sc, view, side, count] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(slug!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const key = (p: { x: number; y: number }): number => (side!.endsWith("Y") ? p.y : p.x) * (side!.startsWith("max") ? 1 : -1);
const rows: { name: string; reach: number }[] = [];
for (const s of state.shapes[view!]) { const pts = s.rings.flat(); if (pts.length > 0) rows.push({ name: `drawn ${s.shape.id}[${s.shape.bone}] ${s.shape.role}${s.shape.filled === false ? " unfilled" : ""}${s.shape.solid ? " " + s.shape.solid.kind : ""}`, reach: Math.max(...pts.map(key)) }); }
for (const w of worldSolids(asset, state.bones, Number(sc))) { let m = -Infinity; for (let i = 0; i < w.positions.length; i += 3) m = Math.max(m, key(project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!))); rows.push({ name: `solid ${w.id}[${w.bone}]${(w as any).texture ? " texture" : ""}`, reach: m }); }
rows.sort((a, b) => b.reach - a.reach);
for (const r of rows.slice(0, Number(count ?? 8))) console.log(`${(Math.abs(r.reach)).toFixed(4)} ${r.name}`);
