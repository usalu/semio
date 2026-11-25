/** 🧩️ Which view draws a part wrong: `bun parts.ts <id|file.json> <scale> [--all] [--pose <id>]` evaluates one pose (default: the default pose) and prints, for every bone with a hard per-bone finding of the matrix (`--all`: for every drawn bone), the extent of its shapes in the three views beside the extent of the body solids of that bone (`volumes.<scale>.body`, as the silhouette guard places them): ahead (elevation x, plan x, body x), up (elevation y, front y, body y) and left (front x, plan y, body z), each with the distance of the drawn extent from the body in millimetres. The view whose extent leaves the body is the one to redraw.
 * @see ../MR/gate.ts — the measurement of the matrix */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { worldSolids, project } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volumes", "🌓️silhouette", "🟦️.ts"));

const args = process.argv.slice(2);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(args[1]);
const poseId = args.includes("--pose") ? args[args.indexOf("--pose") + 1]! : asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose: poseId }, engine.scaleSelector(scale));
const findings = engine.geometryFindings(asset, state, poseId === asset.presentation.defaultPose).filter((finding: any) => finding.tier === "hard" && finding.bone !== undefined);
const bones: string[] = args.includes("--all") ? [...new Set(["elevation", "front", "plan"].flatMap((view) => state.shapes[view].map((shape: any) => shape.shape.bone as string)))] : [...new Set(findings.map((finding: any) => finding.bone as string))];
const body = new Map<string, any>((asset.volumes[String(scale)].body as any[]).map((solid) => [solid.id, solid]));
const worlds = (worldSolids(asset, state.bones, scale) as any[]).filter((world) => world.texture !== true);
type R = [number, number] | undefined;
const range = (values: number[]): R => (values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)]);
const drawn = (view: string, bone: string, axis: "x" | "y"): R => range(state.shapes[view].filter((shape: any) => shape.shape.bone === bone && shape.shape.role !== "ground").flatMap((shape: any) => shape.rings.flat().map((p: any) => p[axis] as number)));
const solid = (bone: string, view: string, axis: "x" | "y"): R => {
  const values: number[] = [];
  for (const world of worlds) {
    if (body.get(world.id)?.bone !== bone) continue;
    for (let i = 0; i < world.positions.length; i += 3) values.push(project(view, world.positions[i], world.positions[i + 1], world.positions[i + 2])[axis]);
  }
  return range(values);
};
const show = (r: R): string => (r === undefined ? "-" : `[${r[0].toFixed(3)}, ${r[1].toFixed(3)}]`);
const off = (r: R, b: R): string => (r === undefined || b === undefined ? "" : ` (${(Math.max(Math.abs(r[0] - b[0]), Math.abs(r[1] - b[1])) * 1000).toFixed(0)})`);
console.log(`${asset.id} 1:${scale} pose ${poseId}: ${findings.length} hard per-bone findings, tolerance ${(engine.toleranceM(asset) * 1000).toFixed(0)} mm`);
for (const bone of bones.sort()) {
  const ids = (view: string): string => state.shapes[view].filter((shape: any) => shape.shape.bone === bone).map((shape: any) => shape.shape.id).join(",").slice(0, 60);
  const [bx, by, bz] = [solid(bone, "elevation", "x"), solid(bone, "elevation", "y"), solid(bone, "front", "x")];
  const [ex, px] = [drawn("elevation", bone, "x"), drawn("plan", bone, "x")];
  const [ey, fy] = [drawn("elevation", bone, "y"), drawn("front", bone, "y")];
  const [fx, py] = [drawn("front", bone, "x"), drawn("plan", bone, "y")];
  console.log(`${bone}: ${findings.filter((finding: any) => finding.bone === bone).map((finding: any) => `${finding.check} ${finding.delta?.deltaMm?.toFixed(0)}`).join(", ")}`);
  console.log(`   ahead  elevation ${show(ex)}${off(ex, bx)} plan ${show(px)}${off(px, bx)} body ${show(bx)}`);
  console.log(`   up     elevation ${show(ey)}${off(ey, by)} front ${show(fy)}${off(fy, by)} body ${show(by)}`);
  console.log(`   left   front ${show(fx)}${off(fx, bz)} plan ${show(py)}${off(py, solid(bone, "plan", "y"))} body ${show(bz)} (plan axis ${show(solid(bone, "plan", "y"))})`);
  console.log(`   shapes elevation ${ids("elevation")} | front ${ids("front")} | plan ${ids("plan")}`);
}
