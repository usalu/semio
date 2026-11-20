/** 🖼️ Draws the evaluated 2D shapes of the city bus (elevation, front, plan) of one scale and pose, filled even-odd in layer order with their pens, so the drawing itself can be judged: `bun r3-city-bus-view2d.ts --scale 50 [--pose ID] [--asset file.json] [--zoom x0,y0,x1,y1 --view elevation]`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { GENERATED, entryOf } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string, d: string): string => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]!);
const scale = Number(flag("--scale", "50"));
const file = flag("--asset", "");
const asset = file === "" ? entryOf("vehicles/city-bus").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const pose = flag("--pose", asset.presentation.defaultPose);
const state = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const zoom = flag("--zoom", "");
const only = flag("--view", "");
type Frame = { view: string; ox: number; oy: number; k: number };
const [W, H] = [2400, zoom === "" ? 1180 : 1400];
const canvas = new Canvas(W, H, [250, 250, 246]);
const frames: Frame[] = [];
if (zoom === "") {
  const k = 150;
  frames.push({ view: "elevation", ox: 60, oy: 560, k }, { view: "front", ox: 60 + 12.3 * k + 1.7 * k, oy: 560, k }, { view: "plan", ox: 60, oy: 880, k });
} else {
  const [x0, y0, x1, y1] = zoom.split(",").map(Number) as [number, number, number, number];
  const k = Math.min((W - 80) / (x1 - x0), (H - 80) / (y1 - y0));
  frames.push({ view: only === "" ? "elevation" : only, ox: 40 - x0 * k, oy: H - 40 + y0 * k, k });
}
function fill(rings: { x: number; y: number }[][], f: Frame, c: RGB): void {
  const edges: [number, number, number, number][] = [];
  for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) edges.push([f.ox + r[j]!.x * f.k, f.oy - r[j]!.y * f.k, f.ox + r[i]!.x * f.k, f.oy - r[i]!.y * f.k]);
  const ys = edges.flatMap((e) => [e[1], e[3]]);
  for (let y = Math.max(0, Math.floor(Math.min(...ys))); y <= Math.min(H - 1, Math.ceil(Math.max(...ys))); y++) {
    const xs: number[] = [];
    for (const e of edges) if (e[1] <= y + 0.5 !== e[3] <= y + 0.5) xs.push(e[0] + ((y + 0.5 - e[1]) / (e[3] - e[1])) * (e[2] - e[0]));
    xs.sort((a, b) => a - b);
    for (let i = 0; i + 1 < xs.length; i += 2) for (let x = Math.max(0, Math.round(xs[i]!)); x < Math.min(W, Math.round(xs[i + 1]!)); x++) canvas.set(x, y, c);
  }
}
for (const f of frames) {
  const shapes = [...(state.shapes[f.view] as any[])].sort((a, b) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  for (const s of shapes) {
    const tone = Math.round(255 * (s.shape.tone ?? 0.6));
    const glass = s.shape.role === "glazing";
    if (s.closed && s.shape.filled !== false && s.rings.length > 0) fill(s.rings, f, glass ? [Math.round(tone * 0.75), Math.round(tone * 0.85), tone] : [tone, tone, tone]);
    const pen: RGB = s.shape.role === "body" ? [0, 0, 0] : s.shape.role === "detail" ? [70, 70, 70] : s.shape.role === "glazing" ? [40, 60, 110] : [150, 110, 70];
    for (const r of s.rings as { x: number; y: number }[][]) for (let i = s.closed ? 0 : 1, j = s.closed ? r.length - 1 : 0; i < r.length; j = i++) canvas.line(f.ox + r[j]!.x * f.k, f.oy - r[j]!.y * f.k, f.ox + r[i]!.x * f.k, f.oy - r[i]!.y * f.k, pen);
  }
  canvas.text(Math.max(8, f.ox - 40), 8 + (f.view === "plan" ? 640 : 0), `${f.view.toUpperCase()} 1:${scale} ${pose} (${shapes.length} SHAPES)`, [20, 20, 20], 2);
}
const out = join(GENERATED, "r3-city-bus", `v2d.s${scale}.${pose}${zoom === "" ? "" : `.${frames[0]!.view}.zoom`}.png`);
mkdirSync(join(GENERATED, "r3-city-bus"), { recursive: true });
writeFileSync(out, canvas.png());
console.log(out);
