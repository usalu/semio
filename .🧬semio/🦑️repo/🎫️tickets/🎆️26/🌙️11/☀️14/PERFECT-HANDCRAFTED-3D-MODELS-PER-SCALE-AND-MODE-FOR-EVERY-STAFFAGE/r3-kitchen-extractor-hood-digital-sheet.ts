/** 🛰️ Round 3, appliances/kitchen-extractor-hood: six shaded views of one list of a digital scale (a scale without a drawing, which `r3-sheet.ts` cannot open). Usage: bun r3-kitchen-extractor-hood-digital-sheet.ts <scale> [body|thermal|structure] */
import { join } from "node:path";
import { Canvas } from "./volumes-raster.ts";
import { bodyItems, thermalItems, structureItems } from "./volumes-render.ts";
import { catalogEntries, loadAsset, GENERATED } from "./volumes-lib.ts";
import { Panel, cameraAt, listsIn, paperBox, pointsOf, rigAt, savePng } from "./r3-lib.ts";
const [denominator = "500", list = "body"] = process.argv.slice(2);
const scale = Number(denominator);
const entry = catalogEntries().find((e) => e.category === "appliances" && e.slug === "kitchen-extractor-hood")!;
const asset = loadAsset(entry.path);
const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
const lists = listsIn(asset, scale, rig.frames);
const items = (list === "thermal" ? thermalItems : list === "structure" ? structureItems : bodyItems)(asset, lists, asset.presentation.defaultScale);
const views: readonly (readonly [number, number, string])[] = [[90, 0, "AZ 90 EL 0"], [0, 0, "AZ 0 EL 0"], [0, 89, "AZ 0 EL 89"], [42, 21, "AZ 42 EL 21"], [222, 21, "AZ 222 EL 21"], [42, -28, "AZ 42 EL -28"]];
const [w, h] = [800, 760];
const sheet = new Canvas(w * 3, (h + 24) * 2, [255, 255, 255]);
for (const [i, [az, el, title]] of views.entries()) {
  const cam = cameraAt(az, el);
  const bounds = paperBox(cam, pointsOf(lists.body));
  const place = { cam, scale: Math.min((w - 60) / (bounds.maxX - bounds.minX), (h - 60) / (bounds.maxY - bounds.minY)), cx: (bounds.maxX + bounds.minX) / 2, cy: (bounds.maxY + bounds.minY) / 2, ox: w / 2, oy: h / 2 };
  const panel = new Panel(w, h, [244, 244, 240], 2);
  panel.draw(place, items, Infinity);
  panel.shade(place);
  const [x, y] = [(i % 3) * w, Math.floor(i / 3) * (h + 24)];
  sheet.blit(panel.canvas(), x, y);
  sheet.text(x + 6, y + h + 4, `1:${scale} ${list.toUpperCase()} ${title}`, [20, 20, 20], 2);
}
if (lists.errors.length || lists.unresolved.length) console.error(lists.errors, lists.unresolved);
const out = join(GENERATED, "r3-kitchen-extractor-hood", `digital.s${scale}.${list}.png`);
savePng(out, sheet);
console.log(out);
