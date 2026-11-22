/** 🔬️ Diagnostic: per bone, the 3D body projection extent against the 2D drawing extent of one view and scale (`bun author-diag.ts <scale> <view> [pose]`).
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-validate.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const S = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const sil = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const scale = Number(process.argv[2] ?? 50);
const view = (process.argv[3] ?? "plan") as "plan";
const pose = process.argv[4] ?? asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
const worlds = sil.worldSolids(asset, state.bones, scale, "body");
const box = () => ({ x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity });
const grow = (b: ReturnType<typeof box>, p: { x: number; y: number }) => { b.x0 = Math.min(b.x0, p.x); b.x1 = Math.max(b.x1, p.x); b.y0 = Math.min(b.y0, p.y); b.y1 = Math.max(b.y1, p.y); };
const b3: Record<string, ReturnType<typeof box>> = {};
for (const w of worlds) {
  const key = `${w.bone}`;
  const b = (b3[key] ??= box());
  for (let i = 0; i < w.positions.length; i += 3) grow(b, sil.project(view, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
}
const drawn = sil.drawnFor(asset, state, view, scale);
const b2: Record<string, ReturnType<typeof box>> = {};
for (const d of drawn) {
  if (!d.filled || !d.closed) continue;
  const b = (b2[d.bone] ??= box());
  for (const r of d.rings) for (const p of r) grow(b, p);
}
const f = (v: number) => (v * 1000).toFixed(0).padStart(6);
for (const k of new Set([...Object.keys(b3), ...Object.keys(b2)])) {
  const a = b3[k], b = b2[k];
  console.log(`[DEBUG] ${k.padEnd(24)} 3D ${a ? [a.x0, a.x1, a.y0, a.y1].map(f).join("") : "  none"} | 2D ${b ? [b.x0, b.x1, b.y0, b.y1].map(f).join("") : "  none"}`);
}
const ref = sil.referenceOf(view, worlds.filter((w: { texture?: boolean }) => w.texture !== true), undefined, drawn).cut;
const all = [...ref.flatMap((g: unknown[]) => [g]), ...drawn.map((d: { rings: unknown }) => d.rings)];
let [mx0, mx1, my0, my1] = [Infinity, -Infinity, Infinity, -Infinity];
for (const g of all) { const b = sil.boundsOf(g); mx0 = Math.min(mx0, b[0]); mx1 = Math.max(mx1, b[1]); my0 = Math.min(my0, b[2]); my1 = Math.max(my1, b[3]); }
const cell = Number(process.env.CELL ?? 0.004);
const frame = { minX: mx0 - 0.02, minY: my0 - 0.02, maxX: mx1 + 0.02, maxY: my1 + 0.02 };
const m3 = new sil.Mask(frame, cell);
for (const g of ref) m3.fill(g, true);
m3.seal();
const m2 = new sil.Mask(frame, cell);
for (const d of drawn) if (d.closed && d.filled) m2.fill(d.rings);
const only3: Record<string, number> = {}, only2: Record<string, number> = {};
for (let j = 0; j < m3.height; j++) for (let i = 0; i < m3.width; i++) {
  const k = j * m3.width + i;
  const key = `${Math.floor((frame.minX + i * cell) / 0.1) / 10},${Math.floor((frame.minY + j * cell) / 0.1) / 10}`;
  if (m3.cells[k] && !m2.cells[k]) only3[key] = (only3[key] ?? 0) + 1;
  if (!m3.cells[k] && m2.cells[k]) only2[key] = (only2[key] ?? 0) + 1;
}
const top = (o: Record<string, number>) => Object.entries(o).sort((a, b) => b[1] - a[1]).slice(0, 12).map(([k, v]) => `${k}:${v}`).join("  ");
console.log(`[DEBUG] only 3D (cell ${cell * 1000} mm, 0.1 m bins): ${top(only3)}`);
console.log(`[DEBUG] only 2D: ${top(only2)}`);
const evals = (await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts")));
let k2 = 0;
for (const d of drawn) {
  if (!d.closed || !d.filled) continue;
  const m = new sil.Mask(frame, cell);
  m.fill(d.rings);
  let n = 0, miss = 0;
  for (let i = 0; i < m.cells.length; i++) if (m.cells[i]) { n++; if (!m3.cells[i]) miss++; }
  if (miss > 3) console.log(`[DEBUG] drawn ${k2} bone ${d.bone} mat ${d.material} cells ${n} missing-in-3D ${miss} box ${sil.boundsOf(d.rings).map((v: number) => v.toFixed(3)).join(",")}`);
  k2++;
}
for (const idx of [1, 4]) {
  const d = drawn.filter((x: { closed: boolean; filled: boolean }) => x.closed && x.filled)[idx];
  const m = new sil.Mask(frame, cell);
  m.fill(d.rings);
  const pts: string[] = [];
  for (let j = 0; j < m.height; j++) for (let i = 0; i < m.width; i++) { const k = j * m.width + i; if (m.cells[k] && !m3.cells[k]) pts.push(`(${(frame.minX + i * cell).toFixed(3)},${(frame.minY + j * cell).toFixed(3)})`); }
  console.log(`[DEBUG] missing of ${idx}: ${pts.slice(0, 80).join(" ")}`);
}
const feat = ({ 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 } as Record<number, number>)[scale];
const mv = sil.measureView(asset, state, worlds.filter((w: { texture?: boolean }) => w.texture !== true), view, scale, undefined, feat);
console.log(`[DEBUG] measureView iou ${mv.iou.toFixed(4)} p95 ${(mv.p95M * 1000).toFixed(1)} mm haus ${(mv.hausdorffM * 1000).toFixed(1)} counts ${JSON.stringify(mv.counts)}`);
