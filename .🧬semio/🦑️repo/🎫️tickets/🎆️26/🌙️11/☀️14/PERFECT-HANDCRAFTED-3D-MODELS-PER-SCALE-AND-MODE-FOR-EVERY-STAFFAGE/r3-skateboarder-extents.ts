/** 📏 Round 3 skateboarder: per scale and view the extents of every evaluated 2D shape of a candidate document in the default pose (or `--pose`), widest first along the asked axis, to find which shapes carry a view beyond the body.
 *
 * Usage: `bun r3-skateboarder-extents.ts <candidate json> --scale N --view elevation|front|plan [--axis x|y] [--pose ID]` */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";
import { cli, queryOf } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--view", "--axis", "--pose"]);
const asset = loadAsset(spec);
const scale = Number(flag("--scale"));
const view = flag("--view") ?? "front";
const axis = flag("--axis") === "y" ? 1 : 0;
const state = evaluate(asset as never, queryOf(asset, flag("--pose"), undefined, undefined) as never, { kind: "scale", scale: { denominator: scale } } as never) as never as { shapes: Record<string, { shape: { id: string; bone: string; role: string }; rings: { x: number; y: number }[][] }[]> };
const rows = state.shapes[view]!.filter((e) => e.shape.role !== "ground").map((e) => {
  const pts = e.rings.flat();
  const xs = pts.map((p) => (axis === 0 ? p.x : p.y));
  return { id: e.shape.id, bone: e.shape.bone, lo: Math.min(...xs), hi: Math.max(...xs) };
});
const lo = Math.min(...rows.map((r) => r.lo));
const hi = Math.max(...rows.map((r) => r.hi));
process.stdout.write(`1:${scale} ${view} axis ${axis === 0 ? "x" : "y"}: ${lo.toFixed(3)} .. ${hi.toFixed(3)} = ${(hi - lo).toFixed(3)} m\n`);
for (const r of rows.sort((a, b) => Math.max(b.hi - hi + 0.05, lo - b.lo + 0.05) - Math.max(a.hi - hi + 0.05, lo - a.lo + 0.05)).slice(0, 8)) process.stdout.write(`  ${r.id.padEnd(18)} ${r.bone.padEnd(12)} ${r.lo.toFixed(3)} .. ${r.hi.toFixed(3)}\n`);
