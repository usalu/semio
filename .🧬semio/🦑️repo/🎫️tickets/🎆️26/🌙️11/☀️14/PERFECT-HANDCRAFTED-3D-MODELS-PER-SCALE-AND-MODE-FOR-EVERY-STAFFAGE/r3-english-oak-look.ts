/** 🔍️ Round 3 English oak: a zoomed shaded view of a chosen window of one list, optionally without the foliage, to judge junctions, collars and the root flare. Usage: bun r3-english-oak-look.ts <scale> <list> <turn> <tilt> <cx> <cy> <halfM> [wood|all] [pose] */
import { Panel, cameraAt, listsIn, load, rigAt, savePng, type Item } from "./r3-lib.ts";
import type { RGB } from "./volumes-raster.ts";

const [scale, list, turn, tilt, cx, cy, half] = process.argv.slice(2, 9).map((v, i) => (i === 1 ? v : Number(v))) as [number, "body" | "thermal" | "structure", number, number, number, number, number];
const which = process.argv[9] ?? "wood";
const pose = process.argv[10];
const { entry, asset } = load("vegetation/english-oak", "r3-english-oak-look.ts");
const rig = rigAt(asset, scale, { kind: "pose", pose: pose ?? asset.presentation.defaultPose });
const lists = listsIn(asset, scale, rig.frames);
const PALETTE: RGB[] = [[150, 120, 130], [120, 150, 135], [160, 140, 100], [120, 130, 170], [170, 120, 100], [110, 160, 170]];
const items: Item[] = lists[list].flatMap((mesh, i) => {
  const s = lists.solids[list][i]!;
  const leaf = String(s.material ?? "").includes("leaf") || String(s.bone).startsWith("fol-");
  if (which === "wood" && leaf) return [];
  const color: RGB = list === "body" ? (leaf ? [110, 140, 125] : [150, 120, 130]) : PALETTE[(i * 7) % PALETTE.length]!;
  return [{ mesh, color }];
});
const size = 1600;
const panel = new Panel(size, size);
const p = { cam: cameraAt(turn, tilt), scale: size / (2 * half), cx, cy, ox: size / 2, oy: size / 2 };
panel.draw(p, items);
panel.shade(p);
console.log(savePng(`🗑️generated/r3-english-oak/look.s${scale}.${list}.${which}.${turn}-${tilt}.${cx}-${cy}-${half}${pose ? `.${pose}` : ""}.png`, panel.canvas()));
