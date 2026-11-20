/** 🔍️ Close-ups for appliances/shower-head: four shaded views (turn/tilt pairs) of one list of one scale, zoomed on the solids of the named bones, so that parts of a few centimetres on a 1.1 m set can be judged.
 *
 * Usage: `bun r3-shower-head-zoom.ts --scale N --bones holder,shower,head [--not ID,ID] [--list body|thermal|structure] [--pose ID] [--cut] [--views 0:0,90:0,50:28,230:20] [--name TAG]`
 * Output: `🗑️generated/r3-shower-head/zoom.s<N>.<list>.<TAG>.png`
 * @see ./r3-sheet.ts */
import { join } from "node:path";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { GENERATED, type ListName } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, listsIn, load, paperBox, pointsOf, queryOf, rigAt, savePng, type Item, type Placement } from "./r3-lib.ts";

const { flag, has } = cli(["--scale", "--bones", "--not", "--list", "--pose", "--views", "--name"]);
const { asset } = load("appliances/shower-head", "r3-shower-head-zoom.ts --scale N --bones a,b");
const scale = Number(flag("--scale"));
const list = (flag("--list") ?? "body") as ListName;
const bones = (flag("--bones") ?? "rail,elbow,lever,holder,shower,head").split(",");
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
const body = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
const ghost: Item[] = list === "body" ? [] : body;
const not = (flag("--not") ?? "").split(",");
const inFocus = (s: { id: string; bone: string }) => bones.includes(s.bone) && !not.some((n) => n !== "" && s.id.includes(n));
const focus = [...sm.body.filter((_, i) => inFocus(sm.solids.body[i])), ...sm[list].filter((_, i) => inFocus(sm.solids[list][i]))];
const views = (flag("--views") ?? "0:0,90:0,50:28,230:20").split(",").map((v) => v.split(":").map(Number) as [number, number]);
const [pw, ph] = [1200, 800];
const BG: RGB = [247, 247, 244];
const sheet = new Canvas(pw * 2, ph * Math.ceil(views.length / 2), [255, 255, 255]);
views.forEach(([turn, tilt], k) => {
  const cam = cameraAt(turn, tilt);
  const b = paperBox(cam, pointsOf(focus));
  const zoom = Math.min((pw - 80) / (b.maxX - b.minX || 1), (ph - 80) / (b.maxY - b.minY || 1));
  const place: Placement = { cam, scale: zoom, cx: (b.minX + b.maxX) / 2, cy: (b.minY + b.maxY) / 2, ox: pw / 2, oy: ph / 2 };
  const panel = new Panel(pw, ph, BG, 2);
  const centre = [...pointsOf(focus)].reduce((n, p) => n + p[0] * cam.e[0] + p[1] * cam.e[1] + p[2] * cam.e[2], 0) / Math.max(1, [...pointsOf(focus)].length);
  if (ghost.length > 0) {
    panel.draw(place, ghost, has("--cut") ? centre : Infinity);
    panel.shade(place);
    panel.ghost(0.28);
  }
  panel.draw(place, items, has("--cut") ? centre : Infinity);
  panel.shade(place);
  const c = panel.canvas();
  const bar = Math.round(0.01 * zoom);
  c.rect(14, ph - 20, bar, 4, [20, 20, 20]);
  c.text(14, ph - 40, `10 MM   TURN ${turn} TILT ${tilt}`, [20, 20, 20], 2);
  sheet.blit(c, (k % 2) * pw, Math.floor(k / 2) * ph);
});
process.stdout.write(`${savePng(join(GENERATED, "r3-shower-head", `zoom.s${scale}.${list}.${flag("--name") ?? bones.join("-")}${has("--cut") ? ".cut" : ""}.png`), sheet)}\n`);
