/** 🪟 Round 3, appliances/room-thermostat: six shaded views of one list at a digital scale (1:200, 1:500), where `r3-sheet.ts` stops because the scale has no drawing. Usage: bun r3-room-thermostat-sheet.ts <200|500> [body|thermal|structure] */
import { join } from "node:path";
import { Canvas } from "./volumes-raster.ts";
import { bodyItems, thermalItems, structureItems } from "./volumes-render.ts";
import { GENERATED } from "./volumes-lib.ts";
import { Panel, cameraAt, listsIn, load, paperBox, pointsOf, rigAt, savePng } from "./r3-lib.ts";

const [denominator, list = "body"] = process.argv.slice(2);
const scale = Number(denominator);
const { asset } = load("appliances/room-thermostat", "bun r3-room-thermostat-sheet.ts <scale> [list]");
const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
const sm = listsIn(asset, scale, rig.frames);
for (const e of [...sm.errors, ...sm.unresolved]) console.error("NOT DRAWN", e);
const body = bodyItems(asset, sm, asset.presentation.defaultScale);
const own = list === "thermal" ? thermalItems(asset, sm, asset.presentation.defaultScale) : list === "structure" ? structureItems(asset, sm, asset.presentation.defaultScale) : body;
const ghost = list === "structure" ? body.map(({ mesh }) => ({ mesh, color: [226, 226, 222] as [number, number, number] })) : [];
const views: [string, number, number][] = [["front", 90, 0], ["elevation", 0, 0], ["back", 270, 0], ["top", 0, 90], ["three-quarter", 50, 28], ["from behind", 215, 20]];
const w = 800, h = 760;
const sheet = new Canvas(w * 3, (h + 24) * 2, [255, 255, 255]);
const extent = Math.max(asset.dimensions.heightM, asset.dimensions.widthM) * 1.6;
for (const [i, [name, turn, tilt]] of views.entries()) {
  const cam = cameraAt(turn, tilt), bounds = paperBox(cam, pointsOf(sm.body));
  const place = { cam, scale: (w - 40) / extent, cx: (bounds.maxX + bounds.minX) / 2, cy: (bounds.maxY + bounds.minY) / 2, ox: w / 2, oy: h / 2 };
  const panel = new Panel(w, h, [246, 246, 242], 2);
  if (ghost.length > 0 && tilt !== 0) panel.draw(place, ghost, Infinity);
  panel.draw(place, own.map(({ mesh, color }) => ({ mesh, color })), Infinity);
  panel.shade(place);
  const x = (i % 3) * w, y = Math.floor(i / 3) * (h + 24);
  sheet.blit(panel.canvas(), x, y);
  sheet.text(x + 6, y + h + 4, `${name} 1:${scale} ${list} ${own.length} solids`, [20, 20, 20], 2);
}
console.log(savePng(join(GENERATED, "r3-room-thermostat", `s${scale}.${list}.png`), sheet));
