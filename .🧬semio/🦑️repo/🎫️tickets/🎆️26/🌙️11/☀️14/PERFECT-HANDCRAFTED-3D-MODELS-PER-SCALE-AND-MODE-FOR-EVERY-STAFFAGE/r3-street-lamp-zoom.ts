/** 🔦️ One shaded panel of `objects/street-lamp` (source merged over the catalog) zoomed onto a world window, so the luminaire head, the arm joint and the base of a 6 m column can be judged.
 *
 * Usage: `bun r3-street-lamp-zoom.ts --scale N --list body|thermal|structure --turn DEG --tilt DEG --at X,Y,Z --span M [--pose ID] [--cut] [--name tag]`
 * Output: `🗑️generated/r3-street-lamp/zoom/s<N>.<list>.<name>.png` (1600 x 1200 px), bar = 0.1 m
 * @see ./r3-lib.ts
 * @see ./r3-tower-crane-zoom.ts */
import { join } from "node:path";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { Panel, cameraAt, cli, listsIn, load, queryOf, rigAt, savePng, toScreen, type Item, type Placement } from "./r3-lib.ts";
import type { ListName } from "./volumes-lib.ts";

const { flag, has } = cli(["--scale", "--list", "--turn", "--tilt", "--at", "--span", "--pose", "--name"]);
const { asset } = load("objects/street-lamp", "r3-street-lamp-zoom.ts");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as ListName;
const at = (flag("--at") ?? "0.7,5.8,0").split(",").map(Number);
const span = Number(flag("--span") ?? 1.5);
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
const body: Item[] = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
const cam = cameraAt(Number(flag("--turn") ?? 0), Number(flag("--tilt") ?? 0));
const [W, H] = [1600, 1200];
const dot = (a: readonly number[], b: readonly number[]) => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const place: Placement = { cam, scale: W / span, cx: dot(at, cam.r), cy: dot(at, cam.u), ox: W / 2, oy: H / 2 };
const panel = new Panel(W, H, [247, 247, 244], 2);
const clip = has("--cut") ? dot(at, cam.e) : Infinity;
if (list !== "body") {
  panel.draw(place, body, clip);
  panel.shade(place);
  panel.ghost(0.28);
}
panel.draw(place, items, clip);
panel.shade(place);
const c = panel.canvas();
const gy = Math.round(toScreen(place, [0, 0, 0])[1]);
if (cam.u[1] > 0.9 && gy > 0 && gy < H) c.rect(0, gy, W, 1, [120, 120, 120]);
c.rect(14, H - 20, Math.round((W / span) * 0.1), 4, [20, 20, 20]);
c.text(18, H - 40, `0.1 M  1:${scale} ${list} ${sm[list].length} SOLIDS TURN ${flag("--turn") ?? 0} TILT ${flag("--tilt") ?? 0}`, [20, 20, 20], 2);
process.stdout.write(`${savePng(join(import.meta.dir, "🗑️generated/r3-street-lamp/zoom", `s${scale}.${list}.${flag("--name") ?? "view"}.png`), c)}\n`);
