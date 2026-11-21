/** 🔎 Round-5 zoom look tool of `humans/parent-with-stroller` and `humans/parent-carrying-toddler`: the catalog asset with its volumes source merged over it (as `r3-sheet.ts` reads it), one PNG with several directions of one list, a subset of solids, a zoom box and the rig.
 *
 * Usage: `bun r5-parent-look.ts <category/slug> --scale N [--list body|thermal|structure] [--dirs turn:tilt,...] [--pose id | --clip id --t s] [--ids regex] [--box x0,x1,y0,y1,z0,z1] [--rig] [--cut] [--w 400 --h 900] [--out name]`
 * Turn 0 = right side (product elevation), 90 = front, 180 = left side, 270 = back; tilt 90 = plan. Output: `🗑️generated/r5-<slug>/<name>.png`
 * @see ./r3-parent-carrying-toddler-look.ts */
import { join } from "node:path";

import { Canvas, type RGB } from "./volumes-raster.ts";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { Panel, cameraAt, cli, listsIn, load, paperBox, pointsOf, queryOf, rigAt, savePng, thickLine, dot, toScreen, type Item, type Placement } from "./r3-lib.ts";
import { GENERATED, type SolidMesh } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string, d?: string) => (args.includes(n) ? args[args.indexOf(n) + 1] : d);
const { spec } = cli(["--scale", "--list", "--dirs", "--pose", "--clip", "--t", "--ids", "--box", "--w", "--h", "--out"]);
const { entry, asset: m } = load(spec, "bun r5-parent-look.ts <category/slug> --scale N ...");
const OUT = join(GENERATED, `r5-${entry.slug}`);
const scale = Number(flag("--scale", "50"));
const query = queryOf(m, flag("--pose"), flag("--clip"), flag("--t"));
const tag = query.kind === "pose" ? (flag("--pose") === undefined ? "" : `-${query.pose}`) : `-${query.clip}-t${query.timeS}`;
const rig = rigAt(m, scale, query);

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
