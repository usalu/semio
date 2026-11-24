/** 🔬️ What the three shapes of one id are as bodies: `bun dump.ts <id|file.json> <scale> <shape id> [--subject <text>]` (from `T`) prints per view the kind, the solid, the ring and point counts, the ranges of the two profile coordinates and the local box (u, v, w) of the body of `solid.ts`, then the frame of the bone at rest and in the default pose, and the extents of the three bodies along the world axes in the default pose and in the subjects whose id contains `--subject`. */
import { readDoc } from "../MR/lib.ts";
import { VIEWS, bodyOf, extent, restOf, ringsOf, subjectsOf, type View } from "./solid.ts";

const args = process.argv.slice(2);
const asset = readDoc(args[0]!).doc.asset;
const g = asset.representations[args[1]!].geometry as Record<View, any[]>;
const rest = restOf(asset);
const subjects = subjectsOf(asset);
const wanted = args.includes("--subject") ? args[args.indexOf("--subject") + 1]! : undefined;
const fmt = (values: readonly number[]): string => `[${values.map((v) => v.toFixed(3)).join(", ")}]`;
for (const twinE of g.elevation.filter((shape) => shape.id === args[2]).concat(g.elevation.some((shape) => shape.id === args[2]) ? [] : [undefined])) {
  const bone = twinE?.bone ?? VIEWS.flatMap((view) => g[view]).find((shape) => shape.id === args[2])?.bone;
  const bodies: Partial<Record<View, any>> = {};
  for (const view of VIEWS) {
    const shape = g[view].find((candidate) => candidate.id === args[2] && candidate.bone === bone);
    if (shape === undefined) continue;
    const rings = ringsOf(shape);
    const points = rings.flat();
    const body = bodyOf(asset, rest, view, shape, twinE);
    bodies[view] = body;
    const box = body !== undefined && "local" in body ? [0, 1, 2].map((i) => fmt([Math.min(...body.local.map((p: any) => p[i])), Math.max(...body.local.map((p: any) => p[i]))])).join(" ") : "-";
    console.log(`${view} ${shape.bone} ${shape.kind} ${JSON.stringify(shape.solid ?? null)} rings ${rings.length} points ${points.length} a ${fmt([Math.min(...points.map((p) => p[0])), Math.max(...points.map((p) => p[0]))])} b ${fmt([Math.min(...points.map((p) => p[1])), Math.max(...points.map((p) => p[1]))])} local uvw ${box}${shape.followTip ? " followTip" : ""}`);
  }
  const r = rest[bone]!;
  console.log(`rest  o ${fmt(r.o)} u ${fmt(r.u)} v ${fmt(r.v)} w ${fmt(r.w)}`);
  for (const subject of subjects) {
    if (!(subject.isDefault || (wanted !== undefined && subject.id.includes(wanted)))) continue;
    const f = subject.frames[bone]!;
    console.log(`${subject.id}${subject.isDefault ? " (default)" : ""}: o ${fmt(f.o)} u ${fmt(f.u)} v ${fmt(f.v)} w ${fmt(f.w)}`);
    for (const view of VIEWS) if (bodies[view] !== undefined) console.log(`   ${view.padEnd(9)} x ${fmt(extent(bodies[view], f, 0))} y ${fmt(extent(bodies[view], f, 1))} z ${fmt(extent(bodies[view], f, 2))}`);
  }
}
