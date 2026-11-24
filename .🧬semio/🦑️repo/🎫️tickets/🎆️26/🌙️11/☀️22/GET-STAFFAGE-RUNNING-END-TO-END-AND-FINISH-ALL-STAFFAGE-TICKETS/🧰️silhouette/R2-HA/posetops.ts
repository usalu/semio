/** 🔝️ The drawn shapes that reach furthest in one direction of a view in a named pose: bun posetops.ts <slug|file.json> <scale> <view> <pose> <maxY|minY|maxX|minX> [count] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./people.ts";
const [slug, sc, view, pose, side, count] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(slug!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(sc) } });
const key = (p: { x: number; y: number }): number => (side!.endsWith("Y") ? p.y : p.x) * (side!.startsWith("max") ? 1 : -1);
const rows = (state.shapes[view!] as any[]).filter((s) => s.rings.flat().length > 0 && s.shape.role !== "ground").map((s) => ({ name: `${s.shape.id}[${s.shape.bone}]`, reach: Math.max(...s.rings.flat().map(key)) })).sort((a, b) => b.reach - a.reach);
for (const r of rows.slice(0, Number(count ?? 6))) console.log(`${Math.abs(r.reach).toFixed(4)} ${r.name}`);
