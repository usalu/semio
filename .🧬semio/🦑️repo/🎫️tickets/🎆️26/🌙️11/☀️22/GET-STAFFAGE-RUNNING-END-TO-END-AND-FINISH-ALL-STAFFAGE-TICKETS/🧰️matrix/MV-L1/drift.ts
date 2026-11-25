/** 📐️ The facts behind a foliage repair: `bun drift.ts <id|file.json> [--strip plates|all] [--scales 100] [--bones a,b]` prints the tolerance, per bone that draws a filled body shape in the front or the plan its rest direction in the sagittal plane, its lateral axis and the largest turn about the lateral axis over every subject of the matrix, and per scale, view and bone the largest distance between the extent of the bone in the view and in the side elevation (height for the front, forward for the plan) with its subject and the value in the default pose. `--strip plates` measures without the depth plates `mvc-*` of `../MV-1/carry.ts`, `--strip all` also without the suffix `-mvs` of the sheets (the state before batch MV-1).
 * @see ../MV-1/carry.ts — the plates this round replaces */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const strip = flag("strip");
const scales = flag("scales")?.split(",");
const bones = flag("bones")?.split(",");
if (strip !== undefined)
  for (const representation of Object.values(asset.representations) as any[])
    for (const view of ["front", "plan"]) {
      if (representation.geometry === undefined) continue;
      representation.geometry[view] = representation.geometry[view].filter((shape: any) => !String(shape.id).startsWith("mvc-"));
      if (strip === "all") for (const shape of representation.geometry[view]) if (String(shape.id).endsWith("-mvs")) shape.id = shape.id.slice(0, -4);
    }
const tolerance: number = engine.toleranceM(asset);
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const deg = (r: number): number => (r * 180) / Math.PI;
const turn = new Map<string, [number, number]>();
const worst = new Map<string, { m: number; subject: string; rest: number; signed: string }>();
for (const key of authored)
  for (const subject of engine.subjects(asset, Number(key))) {
    const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
    const range = (view: string, axis: "x" | "y"): Map<string, [number, number]> => {
      const ranges = new Map<string, [number, number]>();
      for (const shape of state.shapes[view]) {
        if (shape.shape.role === "ground") continue;
        const r = ranges.get(shape.shape.bone) ?? ranges.set(shape.shape.bone, [Infinity, -Infinity]).get(shape.shape.bone)!;
        for (const ring of shape.rings) for (const p of ring) { r[0] = Math.min(r[0], p[axis]); r[1] = Math.max(r[1], p[axis]); }
      }
      return ranges;
    };
    for (const [view, axis] of [["front", "y"], ["plan", "x"]] as const) {
      const e = range("elevation", axis);
      for (const [bone, r] of range(view, axis)) {
        const a = e.get(bone);
        if (a === undefined || (bones !== undefined && !bones.includes(bone))) continue;
        const m = Math.max(Math.abs(a[0] - r[0]), Math.abs(a[1] - r[1]));
        const k = `${key}|${view}|${bone}`;
        const known = worst.get(k) ?? worst.set(k, { m: -1, subject: subject.id, rest: 0, signed: "" }).get(k)!;
        if (subject.isDefault) known.rest = m;
        if (m > known.m) Object.assign(known, { m, subject: subject.id, signed: `low ${((r[0] - a[0]) * 1000).toFixed(0)} high ${((r[1] - a[1]) * 1000).toFixed(0)}` });
        const now = state.bones[bone].frame;
        const u0 = rest[bone]!.frame.u;
        const angle = deg(Math.atan2(now.u.y, now.u.x) - Math.atan2(u0[1]!, u0[0]!));
        const t = turn.get(bone) ?? turn.set(bone, [0, 0]).get(bone)!;
        t[0] = Math.min(t[0], angle); t[1] = Math.max(t[1], angle);
      }
    }
  }
console.log(`${asset.id}: tolerance ${(tolerance * 1000).toFixed(0)} mm`);
for (const [bone, t] of [...turn].sort()) {
  const f = rest[bone]!.frame;
  console.log(`bone ${bone}: rest u at ${deg(Math.atan2(f.u[1]!, f.u[0]!)).toFixed(1)} deg in the sagittal plane (u.z ${f.u[2]!.toFixed(3)}), w (${f.w.map((n) => n.toFixed(3)).join(", ")}), turn ${t[0].toFixed(1)} … ${t[1].toFixed(1)} deg`);
}
for (const [k, v] of [...worst].sort()) console.log(`${k.replaceAll("|", " ")}: ${(v.m * 1000).toFixed(0)} mm (${(v.m / tolerance).toFixed(2)} tol) @ ${v.subject} (${v.signed}); default pose ${(v.rest * 1000).toFixed(0)} mm`);
