/** 🕸️ A web between two body solids of one bone whose flesh the volume models apart (thumb and palm, two fingers, a cuff and a hand): a capsule from a point of the first solid to a point of the second (their centroids, or the fractions given along the line between the centroids), in the frame of the bone, with the material, tone and thermal region of the first; or a plug: with `@<view>:<x>,<y>` in place of the second solid, a ball at that point of the view at the depth of the centroid of the first solid (the tip of a slit between two parts that touch in life). Prints the ops (`solid-add` to `body` and to `thermal`) as JSON for a hand ops file.
 *  bun web.ts <slug> <scale> <solid A> <solid B | @view:x,y> <new id> <radius mm> [from 0..1] [to 0..1] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";

const [slug, sc, idA, idB, id, radius, from, to] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(slug!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const solids = worldSolids(asset, state.bones, Number(sc));
const centroid = (name: string): number[] => { const w = solids.find((x) => x.id === name); if (w === undefined) throw new Error(`no solid ${name}`); const c = [0, 0, 0]; const n = w.positions.length / 3; for (let i = 0; i < w.positions.length; i += 3) for (let k = 0; k < 3; k++) c[k]! += w.positions[i + k]! / n; return c; };
const plug = (spec: string, like: number[]): number[] => {
  const [view, xy] = spec.slice(1).split(":") as [string, string];
  const [x, y] = xy.split(",").map(Number) as [number, number];
  const axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].map((e) => project(view as never, e[0]!, e[1]!, e[2]!));
  const out = like.slice();
  for (let k = 0; k < 3; k++) { const e = axes[k]!; if (Math.abs(e.x) > 0.5) out[k] = x / e.x; else if (Math.abs(e.y) > 0.5) out[k] = y / e.y; }
  return out;
};
const a = centroid(idA!);
const b = idB!.startsWith("@") ? plug(idB!, a) : centroid(idB!);
if (idB!.startsWith("@")) a.splice(0, 3, ...b);
const body = asset.volumes[sc!].body as any[];
const host = body.find((x) => x.id === idA);
const f = state.bones[host.bone].frame as { o: any; u: any; v: any; w: any };
const local = (p: number[]): number[] => { const d = [p[0]! - f.o.x, p[1]! - f.o.y, p[2]! - f.o.z]; return [f.u, f.v, f.w].map((e) => +(d[0]! * e.x + d[1]! * e.y + d[2]! * e.z).toFixed(5)); };
const at = (t: number): number[] => a.map((x, k) => x + (b[k]! - x) * t);
const r = Number(radius) / 1000;
const solid = { id, kind: "capsule", bone: host.bone, material: host.material, role: host.role, tone: host.tone, p0: local(at(Number(from ?? 0))), p1: local(at(Number(to ?? 1))), r0: r, r1: r };
const thermal = (asset.volumes[sc!].thermal as any[]).find((t) => t.of === idA);
const ops: any[] = [{ op: "solid-add", scale: sc, after: idA, solid }];
if (thermal?.region !== undefined) ops.push({ op: "solid-add", scale: sc, list: "thermal", after: thermal.id, solid: { id: `th-${id}`, of: id, region: thermal.region } });
console.log(JSON.stringify(ops));
