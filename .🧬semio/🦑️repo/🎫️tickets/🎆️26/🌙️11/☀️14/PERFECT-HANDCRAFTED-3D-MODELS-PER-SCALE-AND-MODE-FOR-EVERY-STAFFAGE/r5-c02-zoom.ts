/** 🔬 Round 5 (C02 walls and beam): one shaded view of one list of one scale, zoomed onto a world box, so a bearing, a lintel end or a sill nose of a long component is judged at full size.
 *
 * Usage: `bun r5-c02-zoom.ts <category>/<slug> --scale N --box x0,y0,z0,x1,y1,z1 [--list body|thermal|structure] [--turn DEG --tilt DEG] [--pose ID] [--clip ID --t S] [--cut] [--name TAG]`
 * Output: `🗑️generated/r5-<slug>/zoom/<slug>.s<N>.<list>.<TAG>.png`
 * @see ./r3-sheet.ts
 * @see ./r5-brief.md */
import { mkdirSync } from "node:fs";
import { Canvas } from "./volumes-raster.ts";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import type { ListName } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, listsIn, load, paperBox, queryOf, rigAt, savePng, type Item, type Placement } from "./r3-lib.ts";

const USAGE = "r5-c02-zoom.ts <category>/<slug> --scale N --box x0,y0,z0,x1,y1,z1 [--list L] [--turn D --tilt D] [--pose ID] [--clip ID --t S] [--cut] [--name TAG]";
const { spec, flag, has } = cli(["--scale", "--list", "--pose", "--clip", "--t", "--turn", "--tilt", "--box", "--name"]);
const { entry, asset } = load(spec, USAGE);
const scale = Number(flag("--scale"));
const list = (flag("--list") ?? "body") as ListName;
const box = (flag("--box") ?? "").split(",").map(Number);
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")));
const sm = listsIn(asset, scale, rig.frames);
for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
const body: Item[] = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
const ghost = list === "structure" || (list === "thermal" && has("--cut")) ? body : [];
const cam = cameraAt(Number(flag("--turn") ?? 0), Number(flag("--tilt") ?? 0));
const corners = [0, 1, 2, 3, 4, 5, 6, 7].map((i) => [box[i & 1 ? 3 : 0]!, box[i & 2 ? 4 : 1]!, box[i & 4 ? 5 : 2]!]);
const b = paperBox(cam, corners);
const [W, H] = [2400, 1500];
const zoom = Math.min((W - 40) / (b.maxX - b.minX || 1), (H - 60) / (b.maxY - b.minY || 1));
const place: Placement = { cam, scale: zoom, cx: (b.minX + b.maxX) / 2, cy: (b.minY + b.maxY) / 2, ox: W / 2, oy: 30 + (H - 30) / 2 };
const centre = [0, 1, 2].reduce((n, q) => n + ((box[q]! + box[q + 3]!) / 2) * cam.e[q]!, 0);
const panel = new Panel(W, H, [247, 247, 244], 2);
if (ghost.length > 0) {
  panel.draw(place, ghost, has("--cut") ? centre : Infinity);
  panel.shade(place);
  panel.ghost(0.28);
}
panel.draw(place, items, has("--cut") ? centre : Infinity);
panel.shade(place);
const sheet = new Canvas(W, H + 40, [255, 255, 255]);
sheet.blit(panel.canvas(), 0, 0);
const bar = Math.round(0.1 * zoom);
sheet.rect(14, H + 20, bar, 4, [20, 20, 20]);
sheet.text(14, H + 4, `0.1 M   ${entry.slug} 1:${scale} ${list} turn ${flag("--turn") ?? 0} tilt ${flag("--tilt") ?? 0} box ${box.join(",")}`, [20, 20, 20], 2);
const dir = `🗑️generated/r5-${entry.slug}/zoom`;
mkdirSync(dir, { recursive: true });
process.stdout.write(`${savePng(`${dir}/${entry.slug}.s${scale}.${list}.${flag("--name") ?? "z"}${has("--cut") ? ".cut" : ""}.png`, sheet)}\n`);
