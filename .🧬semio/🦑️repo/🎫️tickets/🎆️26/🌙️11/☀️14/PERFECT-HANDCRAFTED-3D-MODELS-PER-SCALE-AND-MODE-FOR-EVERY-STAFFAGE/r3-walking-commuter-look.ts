/** 👁️ Round-3 look tool of `humans/walking-commuter` on the candidate asset: one PNG with several directions of one list (pose or clip frame, a subset of solids, a zoom box, the rig), or the 2D drawings of a scale side by side.
 *
 * Usage: `bun r3-walking-commuter-look.ts --scale N [--list body|thermal|structure] [--dirs turn:tilt,...] [--pose id | --clip id --t s] [--ids regex] [--box x0,x1,y0,y1,z0,z1] [--rig] [--cut] [--w 400 --h 900] [--out name]`
 *        `bun r3-walking-commuter-look.ts --scale N --draw2d [--pose id] [--ppm 600] [--out name]`
 * Turn 0 = right side (product elevation), 90 = front, 180 = left side, 270 = back; tilt 90 = plan. Output: `🗑️generated/r3-walking-commuter/<name>.png`
 * @see ./r3-lib.ts */
import { join } from "node:path";

import { Canvas, type RGB } from "./volumes-raster.ts";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { Panel, cameraAt, listsIn, paperBox, pointsOf, queryOf, rigAt, savePng, thickLine, dot, toScreen, type Item, type Placement } from "./r3-lib.ts";
import { type Json, type SolidMesh } from "./volumes-lib.ts";
import { OUT, candidate } from "./r3-walking-commuter-io.ts";

const args = process.argv.slice(2);
const flag = (n: string, d?: string) => (args.includes(n) ? args[args.indexOf(n) + 1] : d);
const m: Json = candidate();
const scale = Number(flag("--scale", "50"));
const query = queryOf(m, flag("--pose"), flag("--clip"), flag("--t"));
const tag = query.kind === "pose" ? (flag("--pose") === undefined ? "" : `-${query.pose}`) : `-${query.clip}-t${query.timeS}`;
const rig = rigAt(m, scale, query);

if (args.includes("--draw2d")) {
  const st = rig.state.shapes as Json;
  const ppm = Number(flag("--ppm", "600"));
  const pens = m.representations[String(scale)].pens;
  const views = ["elevation", "front", "plan"] as const;
  const boxes = views.map((v) => {
    let b = [1e9, 1e9, -1e9, -1e9];
    for (const s of st[v]) for (const r of s.rings) for (const p of r) b = [Math.min(b[0]!, p.x), Math.min(b[1]!, p.y), Math.max(b[2]!, p.x), Math.max(b[3]!, p.y)];
    return b;
  });
  const pad = 0.04;
  const dims = boxes.map((b) => [Math.ceil((b[2]! - b[0]! + 2 * pad) * ppm), Math.ceil((b[3]! - b[1]! + 2 * pad) * ppm)]);
  const W = dims.reduce((a, d) => a + d[0]! + 8, 0);
  const H = Math.max(...dims.map((d) => d[1]!)) + 24;
  const AA = 2;
  const c = new Canvas(W * AA, H * AA, [255, 255, 255]);
  let x0 = 0;
  views.forEach((v, i) => {
    const b = boxes[i]!;
    const px = (x: number) => (x0 + (x - b[0]! + pad) * ppm) * AA;
    const py = (y: number) => (H - 4 - (y - b[1]! + pad) * ppm) * AA;
    for (let g = Math.ceil(b[1]! * 10) / 10; g <= b[3]!; g += 0.1) for (let x = px(b[0]! - pad); x < px(b[2]! + pad); x++) c.set(Math.round(x), Math.round(py(g)), [225, 232, 250]);
    for (let g = Math.ceil(b[0]! * 10) / 10; g <= b[2]!; g += 0.1) for (let y = py(b[3]! + pad); y < py(b[1]! - pad); y++) c.set(Math.round(px(g)), Math.round(y), [225, 232, 250]);
    for (const s of st[v]) {
      if (s.shape.role === "ground") continue;
      const tone = Math.round(60 + (s.shape.tone ?? 0.5) * 190);
      const filled = s.closed && s.shape.filled !== false;
      const penMm = s.shape.penMm ?? pens[s.shape.role] ?? 0.13;
      const half = Math.max(0.6, ((penMm * scale) / 1000) * ppm * AA * 0.5);
      for (const ring of s.rings as { x: number; y: number }[][]) {
        if (filled) {
          const xs = ring.map((p) => px(p.x));
          const ys = ring.map((p) => py(p.y));
          for (let y = Math.floor(Math.min(...ys)); y <= Math.ceil(Math.max(...ys)); y++) {
            const cut: number[] = [];
            for (let a = 0, k = ring.length - 1; a < ring.length; k = a++) if (ys[a]! <= y !== ys[k]! <= y) cut.push(xs[k]! + ((y - ys[k]!) / (ys[a]! - ys[k]!)) * (xs[a]! - xs[k]!));
            cut.sort((p, q) => p - q);
            for (let a = 0; a + 1 < cut.length; a += 2) for (let x = Math.round(cut[a]!); x <= Math.round(cut[a + 1]!); x++) c.set(x, y, [tone, tone, tone]);
          }
        }
        const n = ring.length;
        for (let a = 0; a < (s.closed ? n : n - 1); a++) {
          const p = ring[a]!;
          const q = ring[(a + 1) % n]!;
          thickLine(c, px(p.x), py(p.y), px(q.x), py(q.y), [15, 15, 15], Math.max(1, Math.round(half * 2)));
        }
      }
    }
    x0 += dims[i]![0]! + 8;
  });
  const out = c.downsample(AA);
  out.text(6, 4, `2D 1:${scale}${tag}  elevation | front | plan  grid 0.1 m`, [40, 40, 120], 1);
  process.stdout.write(`${savePng(join(OUT, `${flag("--out", `draw2d-s${scale}${tag}`)}.png`), out)}\n`);
  process.exit(0);
}

const list = flag("--list", "body")!;
const sm = listsIn(m, scale, rig.frames);
for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
for (const e of sm.unresolved) process.stderr.write(`UNRESOLVED ${e}\n`);
const strip = (xs: readonly { mesh: SolidMesh; color: RGB }[]): Item[] => xs.map(({ mesh, color }) => ({ mesh, color }));
const bodyI = strip(bodyItems(m, sm, scale));
let items = list === "body" ? bodyI : list === "thermal" ? strip(thermalItems(m, sm, scale)) : strip(structureItems(m, sm, scale));
const ids = flag("--ids");
if (ids !== undefined) items = items.filter((it) => new RegExp(ids).test(it.mesh.id));
const box = flag("--box")?.split(",").map(Number);
const ghost = list === "body" || args.includes("--no-ghost") ? [] : bodyI;
const dirs = flag("--dirs", "90:0,0:0,270:0,180:0,50:28,230:55")!.split(",").map((d) => d.split(":").map(Number) as [number, number]);
const w = Number(flag("--w", "400"));
const h = Number(flag("--h", "900"));
const cut = args.includes("--cut");
const sheet = new Canvas(w * dirs.length, h, [255, 255, 255]);
const corners: number[][] = box === undefined ? [] : [0, 1].flatMap((i) => [2, 3].flatMap((j) => [4, 5].map((k) => [box[i]!, box[j]!, box[k]!])));
const fitPoints = box !== undefined ? corners : ids !== undefined ? [...pointsOf(items.map((i) => i.mesh))] : [...pointsOf(sm.body)];
dirs.forEach(([turn, tilt], k) => {
  const cam = cameraAt(turn!, tilt!);
  const b = paperBox(cam, fitPoints);
  const zoom = Math.min((w - 24) / (b.maxX - b.minX || 1), (h - 40) / (b.maxY - b.minY || 1));
  const place: Placement = { cam, scale: zoom, cx: (b.minX + b.maxX) / 2, cy: (b.minY + b.maxY) / 2, ox: w / 2, oy: 14 + h / 2 };
  const panel = new Panel(w, h, [247, 247, 244], 2);
  const centre = fitPoints.reduce((n, p) => n + (p[0]! * cam.e[0] + p[1]! * cam.e[1] + p[2]! * cam.e[2]) / fitPoints.length, 0);
  if (ghost.length > 0) {
    panel.draw(place, ghost, cut ? centre : Infinity);
    panel.shade(place);
    panel.ghost(0.28);
  }
  panel.draw(place, items, cut ? centre : Infinity);
  panel.shade(place);
  const c = panel.canvas();
  if (args.includes("--rig")) {
    for (const bn of rig.bones) {
      const [a, t] = [toScreen(place, bn.o), toScreen(place, bn.tip)];
      thickLine(c, a[0], a[1], t[0], t[1], [235, 20, 120], 2);
      dot(c, a[0], a[1], 3, [255, 225, 0], [30, 30, 30]);
    }
  }
  const gy = Math.round(toScreen(place, [0, 0, 0])[1]);
  if (tilt === 0 && gy > 20 && gy < h - 1) c.rect(0, gy, w, 1, [150, 150, 150]);
  c.text(6, 4, `${list} 1:${scale} turn ${turn} tilt ${tilt}${tag}`, [30, 30, 30], 1);
  c.frame(0, 0, w, h, [190, 190, 185]);
  sheet.blit(c, k * w, 0);
});
process.stdout.write(`${savePng(join(OUT, `${flag("--out", `look-s${scale}-${list}${tag}`)}.png`), sheet)}\n`);
