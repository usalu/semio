/** 🖍️ Draws the evaluated 2D shapes of `humans/courier-with-hand-truck` (elevation, front, plan) of one scale and pose with the rig on top, so the drawing itself can be judged: `bun r3-courier-view2d.ts --scale 50 [--pose waiting] [--asset file.json] [--px 900]`. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { GENERATED, entryOf } from "./volumes-lib.ts";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const args = process.argv.slice(2);
const flag = (n: string, d: string): string => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]!);
const scale = Number(flag("--scale", "50"));
const file = flag("--asset", "");
const asset = file === "" ? entryOf("humans/courier-with-hand-truck").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const pose = flag("--pose", asset.presentation.defaultPose);
const clip = flag("--clip", "");
const query = clip === "" ? { kind: "pose", pose } : { kind: "clip", clip, timeS: Number(flag("--t", "0")) };
const state = evaluate(asset, query as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const H = Number(flag("--px", "900"));
const k = H / 2.0;
const W = [Math.round(1.7 * k), Math.round(1.1 * k), Math.round(1.7 * k)];
const canvas = new Canvas(W[0]! + W[1]! + W[2]!, H, [250, 250, 246]);
const views = ["elevation", "front", "plan"] as const;
const origin = { elevation: [0.45 * k, H - 0.08 * k], front: [W[0]! + 0.55 * k, H - 0.08 * k], plan: [W[0]! + W[1]! + 0.45 * k, H * 0.5] } as Record<string, [number, number]>;
const only = flag("--bone", "");
function fill(rings: { x: number; y: number }[][], o: [number, number], c: RGB): void {
  const edges: [number, number, number, number][] = [];
  for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) edges.push([o[0] + r[j]!.x * k, o[1] - r[j]!.y * k, o[0] + r[i]!.x * k, o[1] - r[i]!.y * k]);
  const ys = edges.flatMap((e) => [e[1], e[3]]);
  for (let y = Math.max(0, Math.floor(Math.min(...ys))); y <= Math.min(H - 1, Math.ceil(Math.max(...ys))); y++) {
    const xs: [number, number][] = [];
    for (const e of edges) if (e[1] <= y + 0.5 !== e[3] <= y + 0.5) xs.push([e[0] + ((y + 0.5 - e[1]) / (e[3] - e[1])) * (e[2] - e[0]), e[3] > e[1] ? 1 : -1]);
    xs.sort((a, b) => a[0] - b[0]);
    let wind = 0;
    for (let i = 0; i < xs.length - 1; i++) {
      wind += xs[i]![1];
      if (wind !== 0) for (let x = Math.round(xs[i]![0]); x < Math.round(xs[i + 1]![0]); x++) canvas.set(x, y, c);
    }
  }
}
for (const view of views) {
  const o = origin[view]!;
  const shapes = [...(state.shapes[view] as any[])].filter((s) => only === "" || s.shape.bone === only).sort((a, b) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  for (const s of shapes) {
    const tone = Math.round(255 * (0.25 + 0.7 * (s.shape.tone ?? 0.6)));
    if (s.closed && s.shape.filled !== false && s.rings.length > 0) fill(s.rings, o, [tone, tone, tone]);
    const pen: RGB = s.shape.role === "body" ? [0, 0, 0] : s.shape.role === "detail" ? [60, 60, 160] : [150, 90, 40];
    for (const r of s.rings as { x: number; y: number }[][]) for (let i = s.closed ? 0 : 1, j = s.closed ? r.length - 1 : 0; i < r.length; j = i++) canvas.line(o[0] + r[j]!.x * k, o[1] - r[j]!.y * k, o[0] + r[i]!.x * k, o[1] - r[i]!.y * k, pen);
  }
  if (!args.includes("--no-rig"))
    for (const bone of asset.rig.bones as any[]) {
      const b = state.bones[bone.id];
      const p = (q: any): [number, number] => (view === "elevation" ? [q.x, q.y] : view === "front" ? [q.z ?? 0, q.y] : [q.x, q.z ?? 0]);
      const [a, t] = [p(b.frame.o), p(b.tip)];
      canvas.line(o[0] + a[0] * k, o[1] - a[1] * k, o[0] + t[0] * k, o[1] - t[1] * k, [220, 0, 0]);
      canvas.rect(Math.round(o[0] + a[0] * k) - 2, Math.round(o[1] - a[1] * k) - 2, 5, 5, [220, 0, 0]);
    }
  canvas.text(o[0] - 0.4 * k, 8, `${view.toUpperCase()} 1:${scale} ${clip === "" ? pose : clip + "@" + flag("--t", "0")}`, [20, 20, 20], 2);
  for (let m = 0; m <= 1.8; m += 0.2) canvas.rect(Math.round(o[0] - 0.42 * k), Math.round(o[1] - m * k), view === "plan" ? 0 : 8, 1, [0, 120, 0]);
}
const out = join(GENERATED, "r3-courier", `v2d.s${scale}.${clip === "" ? pose : clip + flag("--t", "0")}${only === "" ? "" : "." + only}.png`);
mkdirSync(join(GENERATED, "r3-courier"), { recursive: true });
writeFileSync(out, canvas.png());
console.log(out);
