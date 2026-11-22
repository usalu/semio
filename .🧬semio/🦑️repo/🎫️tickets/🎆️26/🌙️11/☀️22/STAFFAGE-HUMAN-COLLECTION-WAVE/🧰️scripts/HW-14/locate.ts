/** 📍️ Where the silhouette guard's boundary distance peaks: rebuilds the drawn and volume masks of a view as `measureView` does (holes up to four features filled) and prints the worst boundary cells of each mask with their distance to the other boundary. bun locate.ts <file.json> <scale> <view> [n] [--pose id] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { Mask, worldSolids, referenceOf, drawnFor, bodyOutline, strokeQuad, fillSmallHoles, boundaryOf, distanceField, boundsOf, comparisonCell } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const args = process.argv.slice(2);
const [file, sc, view] = args as [string, string, "elevation" | "front" | "plan"];
const n = Number(args[3] && !args[3].startsWith("--") ? args[3] : 6);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const pose = args.includes("--pose") ? args[args.indexOf("--pose") + 1]! : asset.presentation.defaultPose;
const scale = Number(sc);
const feature = featureOf(scale as never);
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } } as never);
const solids = worldSolids(asset, state.bones as never, scale);
const drawn = drawnFor(asset, state as never, view, scale);
const { cut: reference } = referenceOf(view, solids, undefined, drawn);
const evaluated = (state.shapes as any)[view].filter((s: any) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0);
const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
for (const g of [...reference, ...evaluated.map((s: any) => s.rings)]) { const q = boundsOf(g); b.minX = Math.min(b.minX, q[0]); b.maxX = Math.max(b.maxX, q[1]); b.minY = Math.min(b.minY, q[2]); b.maxY = Math.max(b.maxY, q[3]); }
const cell = comparisonCell(feature, Math.max(b.maxX - b.minX, b.maxY - b.minY));
const box = { minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell };
const vol = new Mask(box, cell); for (const g of reference) vol.fill(g, true); vol.seal();
const rep = asset.representations[sc];
const pen = (s: any) => ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
const dr = new Mask(box, cell);
for (const s of evaluated) {
  const fills = s.closed && (s.shape.filled !== false || bodyOutline(view, solids, s.rings));
  if (fills) dr.fill(s.rings);
  else for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) dr.fill([strokeQuad(r[i], r[i + 1], Math.max(pen(s), 1e-4))]); }
}
const [W, H] = [vol.width, vol.height];
const holes = (4 * feature / cell) ** 2;
const [ea, eb] = [boundaryOf(fillSmallHoles(dr.cells, W, H, holes), W, H), boundaryOf(fillSmallHoles(vol.cells, W, H, holes), W, H)];
const [fa, fb] = [distanceField(ea, W, H), distanceField(eb, W, H)];
for (const [E, F, name] of [[ea, fb, "drawn edge"], [eb, fa, "volume edge"]] as const) {
  const hits: [number, number][] = [];
  for (let k = 0; k < E.length; k++) if (E[k]) hits.push([Math.sqrt(F[k]!) * cell, k]);
  hits.sort((p, q) => q[0] - p[0]);
  const shown: number[] = [];
  for (const [d, k] of hits) { if (shown.length >= n) break; if (shown.some((m) => Math.hypot((m % W) - (k % W), Math.floor(m / W) - Math.floor(k / W)) * cell < 0.02)) continue; shown.push(k); console.log(`${name} ${(d * 1000).toFixed(1)} mm @ (${(box.minX + ((k % W) + 0.5) * cell).toFixed(3)}, ${(box.minY + (Math.floor(k / W) + 0.5) * cell).toFixed(3)})`); }
}
console.log(`cell ${(cell * 1000).toFixed(2)} mm, limit ${(2 * feature * 1000).toFixed(0)} mm`);
