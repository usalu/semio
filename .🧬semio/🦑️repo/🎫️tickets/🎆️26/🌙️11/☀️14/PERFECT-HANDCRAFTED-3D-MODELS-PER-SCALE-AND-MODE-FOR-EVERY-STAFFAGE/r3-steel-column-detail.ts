/** 🔬️ Close-up of the steel column for looking at base, head and section details that the whole-height sheet cannot show:
 * `bun r3-steel-column-detail.ts --scale N [--list body|thermal|structure] [--turn DEG] [--tilt DEG] [--at HEIGHT_M] [--span M] [--pose ID] [--name TAG]`;
 * writes `🗑️generated/r3-steel-column/detail.s<N>.<list>.<tag>.png` (1600 x 1200) with the same shading and colours as `r3-sheet.ts`.
 *
 * @see ./r3-tools.md
 * @see ./r3-components-steel-column.md */
import { join } from "node:path";

import { bodyItems, structureItems, thermalItems } from "./volumes-render.ts";
import { GENERATED } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, listsIn, load, queryOf, rigAt, savePng, type Item, type Placement } from "./r3-lib.ts";

const { flag } = cli(["--scale", "--list", "--turn", "--tilt", "--at", "--span", "--pose", "--name"]);
const { asset } = load("components/steel-column", "bun r3-steel-column-detail.ts --scale N");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as "body" | "thermal" | "structure";
const turn = Number(flag("--turn") ?? 50);
const tilt = Number(flag("--tilt") ?? 28);
const at = Number(flag("--at") ?? 0.08);
const span = Number(flag("--span") ?? 0.5);
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
for (const e of [...sm.errors, ...sm.unresolved]) process.stderr.write(`NOT DRAWN ${e}\n`);
const body: Item[] = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
const [W, H] = [1600, 1200];
const cam = cameraAt(turn, tilt);
const place: Placement = { cam, scale: W / span, cx: 0, cy: at * cam.u[1], ox: W / 2, oy: H / 2 };
const panel = new Panel(W, H, [247, 247, 244], 2);
if (list === "structure") {
  panel.draw(place, body);
  panel.shade(place);
  panel.ghost(0.28);
}
panel.draw(place, items);
panel.shade(place);
const canvas = panel.canvas();
canvas.text(8, 8, `STEEL-COLUMN 1:${scale} ${list.toUpperCase()} TURN ${turn} TILT ${tilt} AT ${at} M SPAN ${span} M`, [30, 30, 30], 2);
console.log(savePng(join(GENERATED, "r3-steel-column", `detail.s${scale}.${list}.${flag("--name") ?? `t${turn}-e${tilt}-h${at}`}.png`), canvas));
