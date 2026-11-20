/** 🔎️ Round 3 tied canopy: a zoomed shaded view of a world window of one list, coloured by material (body) or by region / part, to judge the wall anchors, the strut node, the glass bearings and the layer shells. Usage: bun r3-tied-canopy-look.ts <scale> <list> <turn> <tilt> <worldX> <worldY> <halfM> (window centred on the world point at z 0) [hide-prefixes,comma] [pose] [cutZ] */
import { Panel, cameraAt, listsIn, load, rigAt, savePng, type Item } from "./r3-lib.ts";
import type { RGB } from "./volumes-raster.ts";

const [scale, list, turn, tilt, cx, cy, half] = process.argv.slice(2, 9).map((v, i) => (i === 1 ? v : Number(v))) as [number, "body" | "thermal" | "structure", number, number, number, number, number];
const hide = (process.argv[9] ?? "").split(",").filter((s) => s !== "" && s !== "-");
const pose = process.argv[10] && process.argv[10] !== "-" ? process.argv[10] : undefined;
const cutZ = process.argv[11] !== undefined ? Number(process.argv[11]) : undefined;
const { asset } = load("components/tied-canopy", "r3-tied-canopy-look.ts");
const rig = rigAt(asset, scale, { kind: "pose", pose: pose ?? asset.presentation.defaultPose });
const lists = listsIn(asset, scale, rig.frames);
const MATERIAL: Record<string, RGB> = { steel: [120, 130, 145], concrete: [175, 170, 160], "mineral-wool": [215, 200, 120], render: [235, 232, 222], glass: [150, 200, 210], "anchor-pad": [90, 90, 90], epdm: [40, 40, 40] };
const KEY: Record<string, RGB> = { "wall-face": [200, 180, 150], "wall-concrete": [170, 170, 170], "wall-wool": [230, 205, 90], "wall-render": [250, 245, 235], "wall-plate": [200, 90, 80], "plate-concrete": [140, 140, 150], "plate-pad": [60, 60, 60], "plate-steel": [200, 90, 80], "canopy-steel": [90, 120, 190], "canopy-glass": [120, 210, 220], "rod-steel": [210, 140, 60], member: [80, 110, 200], joint: [230, 140, 40], support: [60, 170, 80], mass: [200, 60, 180] };
const items: Item[] = lists[list].flatMap((mesh, i) => {
  const s = lists.solids[list][i]!;
  if (hide.some((h) => String(s.id).startsWith(h))) return [];
  const key = list === "body" ? null : list === "thermal" ? (s.layer ?? s.region) : s.part;
  const color: RGB = (key !== null ? KEY[key] : MATERIAL[s.material]) ?? [160, 120, 130];
  if (cutZ !== undefined) {
    let minZ = Infinity;
    for (let k = 2; k < mesh.world.length; k += 3) minZ = Math.min(minZ, mesh.world[k]!);
    if (minZ < cutZ && list !== "body") return [{ mesh, color }];
  }
  return [{ mesh, color }];
});
const size = 1600;
const panel = new Panel(size, size);
const cam = cameraAt(turn, tilt);
const world = [cx, cy, 0];
const p = { cam, scale: size / (2 * half), cx: world[0]! * cam.r[0] + world[1]! * cam.r[1] + world[2]! * cam.r[2], cy: world[0]! * cam.u[0] + world[1]! * cam.u[1] + world[2]! * cam.u[2], ox: size / 2, oy: size / 2 };
panel.draw(p, items, cutZ);
panel.shade(p);
if (lists.errors.length) console.error("[DEBUG] NOT DRAWN", lists.errors.join("; "));
console.log(savePng(`🗑️generated/r3-tied-canopy/look.s${scale}.${list}.${turn}-${tilt}.${cx}-${cy}-${half}${hide.length ? ".h" : ""}${pose ? `.${pose}` : ""}${cutZ !== undefined ? ".cut" : ""}.png`, panel.canvas()));
