/** 🔍 Close-up of one list of the wheelchair ramp: one shaded view around a world point, because the 11.7 m ramp leaves 5 mm per pixel on the shared sheet.
 *
 * Usage: `bun r3-wheelchair-ramp-zoom.ts --scale N [--list body|thermal|structure] --at x,y,z --span M [--turn DEG --tilt DEG] [--cut] --name TAG`
 * Output: `🗑️generated/r3-wheelchair-ramp/zoom.s<N>.<list>.<TAG>.png`
 * @see ./r3-sheet.ts */
import { join } from "node:path";
import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { type ListName } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, listsIn, load, queryOf, rigAt, savePng, type Item, type Placement } from "./r3-lib.ts";

if (import.meta.main) {
  const { flag, has } = cli(["--scale", "--list", "--at", "--span", "--turn", "--tilt", "--name"]);
  const { asset } = load("components/wheelchair-ramp", "r3-wheelchair-ramp-zoom.ts");
  const scale = Number(flag("--scale"));
  const list = (flag("--list") ?? "body") as ListName;
  const rig = rigAt(asset, scale, queryOf(asset, undefined, undefined, undefined));
  const sm = listsIn(asset, scale, rig.frames);
  for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
  const body = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
  const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
  const cam = cameraAt(Number(flag("--turn") ?? 50), Number(flag("--tilt") ?? 28));
  const at = (flag("--at") ?? "0,0,0").split(",").map(Number);
  const [w, h] = [1600, 1100];
  const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
  const place: Placement = { cam, scale: w / Number(flag("--span") ?? 1), cx: dot(at, cam.r), cy: dot(at, cam.u), ox: w / 2, oy: h / 2 };
  const panel = new Panel(w, h, [247, 247, 244], 2);
  const centre = dot(at, cam.e);
  if (list !== "body") {
    panel.draw(place, body, has("--cut") ? centre : Infinity);
    panel.shade(place);
    panel.ghost(0.28);
  }
  panel.draw(place, items, has("--cut") ? centre : Infinity);
  panel.shade(place);
  const path = join(import.meta.dir, "🗑️generated", "r3-wheelchair-ramp", `zoom.s${scale}.${list}.${flag("--name") ?? "view"}.png`);
  process.stdout.write(`${savePng(path, panel.canvas())}\n`);
}
