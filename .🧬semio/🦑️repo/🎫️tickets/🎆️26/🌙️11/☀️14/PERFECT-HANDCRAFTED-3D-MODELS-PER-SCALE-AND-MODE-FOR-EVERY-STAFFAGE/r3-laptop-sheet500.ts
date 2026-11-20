/** 🔭 Renders the 1:500 digital volume of the laptop (catalog file, default pose): body, thermal and structure, each from the front three-quarter and the back three-quarter, because `r3-sheet.ts` needs a 2D representation of the scale and 1:500 has none.
 *
 * Usage: `bun r3-laptop-sheet500.ts`
 * @see ./r3-tools.md */
import { join } from "node:path";
import { Canvas } from "./volumes-raster.ts";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { GENERATED, catalogEntries, loadAsset } from "./volumes-lib.ts";
import { Panel, cameraAt, listsIn, paperBox, pointsOf, rigAt, savePng } from "./r3-lib.ts";

const entry = catalogEntries().find((e) => e.category === "appliances" && e.slug === "laptop")!;
const asset = loadAsset(entry.path);
const rig = rigAt(asset, 500, { kind: "pose", pose: asset.presentation.defaultPose });
const lists = listsIn(asset, 500, rig.frames);
const w = 800;
const h = 540;
const sheet = new Canvas(w * 3, (h + 20) * 2, [255, 255, 255]);
const makers = [["body", bodyItems], ["thermal", thermalItems], ["structure", structureItems]] as const;
for (const [row, [turn, tilt]] of [[42, 21], [222, 21]].entries()) {
  for (const [column, [name, make]] of makers.entries()) {
    const cam = cameraAt(turn, tilt);
    const bounds = paperBox(cam, pointsOf(lists.body));
    const place = { cam, scale: Math.min((w - 40) / (bounds.maxX - bounds.minX), (h - 40) / (bounds.maxY - bounds.minY)), cx: (bounds.maxX + bounds.minX) / 2, cy: (bounds.maxY + bounds.minY) / 2, ox: w / 2, oy: h / 2 };
    const panel = new Panel(w, h, [244, 244, 240], 2);
    panel.draw(place, make(asset, lists, asset.presentation.defaultScale), Infinity);
    panel.shade(place);
    sheet.blit(panel.canvas(), column * w, row * (h + 20));
    sheet.text(column * w + 4, row * (h + 20) + h + 3, `1:500 ${name} turn ${turn}`, [20, 20, 20], 1);
  }
}
if (lists.errors.length || lists.unresolved.length) console.error(lists.errors, lists.unresolved);
const out = join(GENERATED, "r3-laptop", "laptop.s500.png");
savePng(out, sheet);
console.log(out);
