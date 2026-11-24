/** 📷️ The nearness the product gives the shapes of one view of an asset in a pose (`shapeDepth`, larger is nearer) beside the nearest point of their evaluated solid measured on the body solids of the same bone, to see which shapes are ranked by a depth their solid does not have; ops applied in memory unless `--no-ops`.
 *  bun depth.ts <id> <scale> <pose> <view> <id-substring> [--no-ops] */
import { applyAll } from "./apply.ts";
import { S, catalogFile, opsFile } from "./files.ts";
const { evaluate, shapeDepth } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const args = process.argv.slice(2);
const asset = (await Bun.file(catalogFile(args[0]!)).json()).asset;
if (!args.includes("--no-ops") && (await Bun.file(opsFile(args[0]!)).exists())) applyAll(asset, JSON.parse(await Bun.file(opsFile(args[0]!)).text()));
const view = args[3]!;
const state: any = evaluate(asset, { kind: "pose", pose: args[2]! }, { kind: "scale", scale: { denominator: Number(args[1]) } });
const twin = new Map<string, any>(state.shapes[view === "plan" ? "elevation" : "plan"].map((s: any) => [`${s.shape.id} ${s.shape.bone}`, s]));
for (const s of state.shapes[view]) {
  if (!String(s.shape.id).includes(args[4] ?? "")) continue;
  const other = twin.get(`${s.shape.id} ${s.shape.bone}`);
  const reach = other === undefined ? undefined : view === "plan" ? Math.max(...other.rings.flat().map((p: any) => p.y)) : Math.max(...other.rings.flat().map((p: any) => p.x));
  console.log(`${s.shape.id} [${s.shape.bone}]${s.shape.solid ? ` ${s.shape.solid.kind}:${s.shape.solid.sweep ?? s.shape.solid.axis}` : " flat"} shapeDepth ${shapeDepth(state, s.shape, view).toFixed(4)}${reach === undefined ? "" : ` | ${view === "plan" ? "top in the side elevation" : "front end in the plan"} ${reach.toFixed(4)}`}`);
}
