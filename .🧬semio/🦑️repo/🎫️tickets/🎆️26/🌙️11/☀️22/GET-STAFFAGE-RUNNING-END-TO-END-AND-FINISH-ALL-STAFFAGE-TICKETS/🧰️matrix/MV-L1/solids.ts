/** 🧊️ What the two solids of the schema (`extruded`, `revolved`) would do to the front outline of a foliage mass at rest: `bun solids.ts <id|file.json> [--scale 100]` prints, per bone that draws one closed body outline in the front and an outline in the side elevation, the rest direction of the bone, the depth and height of the elevation outline, and
 * - `prism`: the height over which an `extruded` solid along the bone axis nearest to the viewing direction, long enough to span the depth of the mass, smears the front outline at rest (depth × tangent of the angle between that axis and the viewing direction): the drawing of the default pose would change by that much;
 * - `taper`: the bone length against the reach of the outline along the bone (the span of `zTip` is clamped to the bone length, so a taper cannot follow the mass);
 * - `revolved`: how far the front outline is from mirror symmetry about its best vertical axis (half the range of the middle of its lateral sections), which a revolved solid would draw instead.
 * The measurements of section 2 of `../../../multiview-MV-L1.md`.
 * @see ./body.ts — the carriers chosen instead */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const key = args.indexOf("--scale") < 0 ? "100" : args[args.indexOf("--scale") + 1]!;
const asset = readDoc(args[0]!).doc.asset;
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const geometry = asset.representations[key].geometry;
const lengths = new Map<string, number>(asset.rig.bones.map((bone: any) => [bone.id, bone.lengthM]));
const mm = (m: number): string => (m * 1000).toFixed(0);
const worst = { prism: 0, revolved: 0 };
for (const bone of [...new Set<string>(geometry.front.map((shape: any) => shape.bone))].sort()) {
  const sheets = geometry.front.filter((shape: any) => shape.bone === bone && shape.filled === true && shape.role === "body" && shape.solid === undefined && shape.kind !== "member" && !String(shape.id).startsWith("mv"));
  const twins = geometry.elevation.filter((shape: any) => shape.bone === bone && shape.filled === true && shape.role === "body" && shape.solid === undefined);
  if (sheets.length !== 1 || twins.length === 0 || !/^fol/u.test(bone)) continue;
  const f = rest[bone]!.frame;
  const local: { x: number; y: number }[] = twins.flatMap((shape: any) => flattenGeometry(shape).rings.flat());
  const world = local.map((p) => ({ x: f.o[0]! + p.x * f.u[0]! + p.y * f.v[0]!, y: f.o[1]! + p.x * f.u[1]! + p.y * f.v[1]! }));
  const depth = Math.max(...world.map((p) => p.x)) - Math.min(...world.map((p) => p.x));
  const height = Math.max(...world.map((p) => p.y)) - Math.min(...world.map((p) => p.y));
  const reach = Math.max(...local.map((p) => p.x)) - Math.min(...local.map((p) => p.x));
  const tilt = Math.min(Math.atan2(Math.abs(f.u[1]!), Math.abs(f.u[0]!)), Math.atan2(Math.abs(f.v[1]!), Math.abs(f.v[0]!)));
  const rings: { x: number; y: number }[][] = flattenGeometry(sheets[0]).rings;
  const ys = rings.flat().map((p) => p.y);
  const middles: number[] = [];
  for (let i = 1; i < 40; i++) {
    const y = Math.min(...ys) + ((Math.max(...ys) - Math.min(...ys)) * i) / 40;
    const cuts: number[] = [];
    for (const ring of rings) for (let j = 0; j < ring.length; j++) { const [p, q] = [ring[j]!, ring[(j + 1) % ring.length]!]; if (p.y > y !== q.y > y) cuts.push(p.x + ((q.x - p.x) * (y - p.y)) / (q.y - p.y)); }
    if (cuts.length >= 2) middles.push((Math.min(...cuts) + Math.max(...cuts)) / 2);
  }
  const skew = (Math.max(...middles) - Math.min(...middles)) / 2;
  const smear = depth * Math.tan(tilt);
  worst.prism = Math.max(worst.prism, smear); worst.revolved = Math.max(worst.revolved, skew);
  console.log(`${asset.id} 1:${key} ${bone}: rest u ${((Math.atan2(f.u[1]!, f.u[0]!) * 180) / Math.PI).toFixed(1)} deg, elevation outline ${depth.toFixed(2)} m deep and ${height.toFixed(2)} m high; prism smears the front outline at rest over ${mm(smear)} mm (axis ${((tilt * 180) / Math.PI).toFixed(1)} deg off the viewing direction); taper: bone ${mm(lengths.get(bone)!)} mm long, outline reaches ${mm(reach)} mm along it; revolved: front outline ${mm(skew)} mm off mirror symmetry`);
}
console.log(`${asset.id} 1:${key}: worst prism smear at rest ${mm(worst.prism)} mm, worst asymmetry ${mm(worst.revolved)} mm`);
