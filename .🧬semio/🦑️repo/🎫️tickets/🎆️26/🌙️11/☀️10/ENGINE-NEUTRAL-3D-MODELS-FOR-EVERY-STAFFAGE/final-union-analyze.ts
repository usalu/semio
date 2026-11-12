import { readFileSync } from "node:fs";
import polygonClipping from "polygon-clipping";
import { unionPolygons } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
type P = { x: number; y: number };
const area = (r: P[]) => r.reduce((s, p, i) => s + (p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y), 0) / 2;
const perim = (r: P[]) => r.reduce((s, p, i) => s + Math.hypot(p.x - r[(i + 1) % r.length]!.x, p.y - r[(i + 1) % r.length]!.y), 0);
const data = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/final/union-inputs3.json", "utf8")) as { asset: string; scale: number; input: P[][] }[];
for (const d of data) {
  console.log("==", d.asset, d.scale, d.input.length);
  const widths = d.input.map((r) => (2 * Math.abs(area(r))) / perim(r));
  console.log("min widths", widths.map((w) => w.toExponential(2)).sort().slice(0, 6));
  console.log("areas", d.input.map((r) => area(r).toExponential(2)).slice(0, 8), "...", d.input.slice(-2).map((r) => area(r)));
  const clip = polygonClipping.union(...d.input.map((r) => [[...r.map((p) => [p.x, p.y] as [number, number])]] as any) as any);
  const aClip = clip.reduce((s: number, poly: any) => s + poly.reduce((t: number, ring: any, i: number) => t + (i === 0 ? 1 : -1) * Math.abs(area(ring.map(([x, y]: number[]) => ({ x, y })))), 0), 0);
  console.log("polygon-clipping area", aClip, "polys", clip.length);
  for (let k = 0; k < d.input.length; k++) { try { unionPolygons(d.input.filter((_, i) => i !== k)); console.log("removing", k, "fixes it", widths[k]); } catch {} }
}
