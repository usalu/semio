/** 🩹️ A pad of flesh or cloth in the volume where two parts touch in life and the solids leave a slit narrower than the smallest feature: a capsule between two points of a view at the depth of the middle of a host solid, in the frame of the bone of the host, with its material, role, tone and thermal region. Merges the ops (`solid-add` to `body` and to `thermal`) into an ops file.
 *  bun plug.ts <slug|file.json> <scale> <ops.json|-> <host solid> <new id> <radius mm> <view> <x0,y0> <x1,y1> [--base ops.json] [--lateral <front x>: with the side elevation as the view, the lateral place of the pad as the front elevation shows it, instead of the depth of the host] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf } from "./people.ts";

const args = process.argv.slice(2);
const baseAt = args.indexOf("--base");
const base = baseAt < 0 ? undefined : args.splice(baseAt, 2)[1];
const latAt = args.indexOf("--lateral");
const lateral = latAt < 0 ? undefined : Number(args.splice(latAt, 2)[1]);
const [slug, sc, out, hostId, id, radius, view, a0, a1] = args as string[];
const asset = (await Bun.file(fileOf(slug!)).json()).asset;
if (base !== undefined) applyOps(asset, await readOps(base));
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const w = (worldSolids(asset, state.bones, Number(sc)) as any[]).find((x) => x.id === hostId);
if (w === undefined) throw new Error(`no solid ${hostId}`);
const c = [0, 0, 0];
for (let i = 0; i < w.positions.length; i += 3) for (let k = 0; k < 3; k++) c[k]! += w.positions[i + k] / (w.positions.length / 3);
const axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].map((e) => project(view as never, e[0]!, e[1]!, e[2]!));
const world = (spec: string): number[] => {
  const [x, y] = spec.split(",").map(Number) as [number, number];
  const p = c.slice();
  for (let k = 0; k < 3; k++) { const e = axes[k]!; if (Math.abs(e.x) > 0.5) p[k] = x / e.x; else if (Math.abs(e.y) > 0.5) p[k] = y / e.y; }
  if (lateral !== undefined && view === "elevation") p[2] = lateral / project("front" as never, 0, 0, 1).x;
  return p;
};
const host = (asset.volumes[sc!].body as any[]).find((x) => x.id === hostId);
const f = state.bones[host.bone].frame as { o: any; u: any; v: any; w: any };
const local = (p: number[]): number[] => { const d = [p[0]! - f.o.x, p[1]! - f.o.y, p[2]! - f.o.z]; return [f.u, f.v, f.w].map((e) => +(d[0]! * e.x + d[1]! * e.y + d[2]! * e.z).toFixed(5)); };
const r = Number(radius) / 1000;
const solid = { id, kind: "capsule", bone: host.bone, material: host.material, role: host.role, tone: host.tone, p0: local(world(a0!)), p1: local(world(a1!)), r0: r, r1: r };
const thermal = (asset.volumes[sc!].thermal as any[]).find((t) => t.of === hostId);
const ops: Op[] = [{ op: "solid-add", scale: sc, after: hostId, solid }];
if (thermal?.region !== undefined) ops.push({ op: "solid-add", scale: sc, list: "thermal", after: thermal.id, solid: { ...thermal, id: `${thermal.id.replace(hostId, id) === thermal.id ? `th-${id}` : thermal.id.replace(hostId, id)}`, of: id } });
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops));
console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
