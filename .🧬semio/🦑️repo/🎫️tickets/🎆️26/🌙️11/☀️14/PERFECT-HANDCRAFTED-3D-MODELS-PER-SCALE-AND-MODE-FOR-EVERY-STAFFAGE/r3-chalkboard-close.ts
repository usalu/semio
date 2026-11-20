/** 🔬️ Close-up of one list of one scale of the chalkboard (an 80 mm deep object on a 4 m sheet needs it): one shaded orthographic view centred on a world point with a chosen width, optionally cut at a depth along the viewing direction so the profiles show in section.
 *
 * Usage: `bun r3-chalkboard-close.ts --scale N [--list body|thermal|structure] --at X,Y,Z --span M [--turn DEG --tilt DEG] [--cut D] [--pose ID] [--name TAG]`
 * Output: `🗑️generated/r3-chalkboard/close.s<N>.<list>.<TAG>.png`
 * @see ./r3-tools.md */
import { join } from "node:path";
import { PART_COLOR, bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { GENERATED, type ListName } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, listsIn, load, queryOf, rigAt, savePng, type Item, type Placement } from "./r3-lib.ts";

const { flag } = cli(["--scale", "--list", "--at", "--span", "--turn", "--tilt", "--cut", "--pose", "--name"]);
const { asset } = load("objects/chalkboard", "r3-chalkboard-close.ts --scale N --at X,Y,Z --span M");
const scale = Number(flag("--scale"));
const list = (flag("--list") ?? "body") as ListName;
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
const body = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
const cam = cameraAt(Number(flag("--turn") ?? 90), Number(flag("--tilt") ?? 0));
const at = (flag("--at") ?? "0,0.63,0").split(",").map(Number);
const dot = (a: readonly number[], b: readonly number[]) => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const [W, H] = [1600, 1200];
const place: Placement = { cam, scale: W / Number(flag("--span") ?? 0.2), cx: dot(at, cam.r), cy: dot(at, cam.u), ox: W / 2, oy: H / 2 };
const cut = flag("--cut") === undefined ? Infinity : Number(flag("--cut"));
const panel = new Panel(W, H, [247, 247, 244], 2);
if (list !== "body") {
  panel.draw(place, body, cut);
  panel.shade(place);
  panel.ghost(0.25);
}
panel.draw(place, items, cut);
panel.shade(place);
const c = panel.canvas();
const bar = Number(flag("--span") ?? 0.2) >= 1 ? 0.1 : 0.01;
c.rect(14, H - 20, Math.round(bar * place.scale), 4, [20, 20, 20]);
c.text(14, H - 40, `${bar} M   1:${scale} ${list.toUpperCase()}   AT ${at.join(",")}  TURN ${flag("--turn") ?? 90} TILT ${flag("--tilt") ?? 0}${cut === Infinity ? "" : `  CUT ${cut}`}   ${Object.keys(PART_COLOR).length > 0 && list === "structure" ? "MEMBER BLUE JOINT ORANGE SUPPORT GREEN MASS MAGENTA" : ""}`, [20, 20, 20], 2);
process.stdout.write(`${savePng(join(GENERATED, "r3-chalkboard", `close.s${scale}.${list}.${flag("--name") ?? "view"}.png`), c)}\n`);
