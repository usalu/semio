/** 🚛 Draws the evaluated 2D shapes of `machinery/concrete-mixer-truck` (elevation, front, plan) of one scale and pose or clip instant in their layer order with tones and pens, so the drawing itself can be judged: `bun r3-concrete-mixer-truck-view2d.ts --scale 50 [--pose ID | --clip ID --t S] [--asset file.json] [--view elevation|front|plan] [--rig]`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { GENERATED, entryOf } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string, d: string): string => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]!);
const scale = Number(flag("--scale", "50"));
const file = flag("--asset", "");
const asset = file === "" ? entryOf("machinery/concrete-mixer-truck").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const pose = flag("--pose", asset.presentation.defaultPose);
const clip = flag("--clip", "");
const only = flag("--view", "");
const query = clip === "" ? { kind: "pose", pose } : { kind: "clip", clip, timeS: Number(flag("--t", "0")) };
const state = evaluate(asset, query as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const views = (only === "" ? ["elevation", "plan", "front"] : [only]) as string[];
const k = only === "" ? 150 : only === "front" ? 300 : 200;
const cell = { elevation: [13.2 * k, 5.2 * k], plan: [13.2 * k, 3.6 * k], front: [4.4 * k, 5.2 * k] } as Record<string, [number, number]>;
const origin = { elevation: [7.6 * k, 4.9 * k], plan: [7.6 * k, 1.8 * k], front: [2.2 * k, 4.9 * k] } as Record<string, [number, number]>;
const place: Record<string, [number, number]> = only === "" ? { elevation: [0, 0], plan: [0, cell.elevation![1]], front: [cell.elevation![0], 0] } : { [only]: [0, 0] };
const W = only === "" ? cell.elevation![0] + cell.front![0] : cell[only]![0];
const H = only === "" ? cell.elevation![1] + cell.plan![1] : cell[only]![1];
const canvas = new Canvas(Math.round(W), Math.round(H), [250, 250, 246]);
function fill(rings: { x: number; y: number }[][], o: [number, number], c: RGB): void {
  const edges: [number, number, number, number][] = [];
  for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) edges.push([o[0] + r[j]!.x * k, o[1] - r[j]!.y * k, o[0] + r[i]!.x * k, o[1] - r[i]!.y * k]);
  const ys = edges.flatMap((e) => [e[1], e[3]]);
  for (let y = Math.max(0, Math.floor(Math.min(...ys))); y <= Math.min(canvas.height - 1, Math.ceil(Math.max(...ys))); y++) {
    const xs: number[] = [];
    for (const e of edges) if (e[1] <= y + 0.5 !== e[3] <= y + 0.5) xs.push(e[0] + ((y + 0.5 - e[1]) / (e[3] - e[1])) * (e[2] - e[0]));
    xs.sort((a, b) => a - b);
    for (let i = 0; i + 1 < xs.length; i += 2) for (let x = Math.round(xs[i]!); x < Math.round(xs[i + 1]!); x++) canvas.set(x, y, c);
  }
}
for (const view of views) {
  const o: [number, number] = [place[view]![0] + origin[view]![0], place[view]![1] + origin[view]![1]];
  const shapes = [...(state.shapes[view] as any[])].sort((a, b) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  for (const s of shapes) {
    const tone = Math.round(255 * (0.12 + 0.86 * (s.shape.tone ?? 0.6)));
    if (s.closed && s.shape.filled !== false && s.rings.length > 0) fill(s.rings, o, [tone, tone, tone]);
    const pen: RGB = s.shape.role === "body" ? [0, 0, 0] : s.shape.role === "detail" ? [40, 40, 150] : s.shape.role === "glazing" ? [0, 110, 140] : [150, 90, 40];
    for (const r of s.rings as { x: number; y: number }[][]) for (let i = s.closed ? 0 : 1, j = s.closed ? r.length - 1 : 0; i < r.length; j = i++) canvas.line(o[0] + r[j]!.x * k, o[1] - r[j]!.y * k, o[0] + r[i]!.x * k, o[1] - r[i]!.y * k, pen);
  }
  if (args.includes("--rig"))
    for (const bone of asset.rig.bones as any[]) {
      const b = state.bones[bone.id];
      const p = (q: any): [number, number] => (view === "elevation" ? [q.x, q.y] : view === "front" ? [q.z ?? 0, q.y] : [q.x, q.z ?? 0]);
      const [a, t] = [p(b.frame.o), p(b.tip)];
      canvas.line(o[0] + a[0] * k, o[1] - a[1] * k, o[0] + t[0] * k, o[1] - t[1] * k, [220, 0, 0]);
      canvas.rect(Math.round(o[0] + a[0] * k) - 2, Math.round(o[1] - a[1] * k) - 2, 5, 5, [220, 0, 0]);
    }
  canvas.line(place[view]![0], o[1], place[view]![0] + cell[view]![0], o[1], [170, 170, 170]);
  canvas.text(place[view]![0] + 8, place[view]![1] + 8, `${view.toUpperCase()} 1:${scale} ${clip === "" ? pose : clip + "@" + flag("--t", "0")}`, [20, 20, 20], 2);
}
const out = join(GENERATED, "r3-concrete-mixer-truck", `v2d.s${scale}.${clip === "" ? pose : clip + flag("--t", "0")}${only === "" ? "" : "." + only}.png`);
mkdirSync(join(GENERATED, "r3-concrete-mixer-truck"), { recursive: true });
writeFileSync(out, canvas.png());
console.log(out);
