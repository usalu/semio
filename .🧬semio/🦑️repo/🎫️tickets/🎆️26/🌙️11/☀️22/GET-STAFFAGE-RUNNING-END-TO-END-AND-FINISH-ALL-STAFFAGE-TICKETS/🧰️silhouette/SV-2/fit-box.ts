/** 📦️ Makes the body of every authored scale of a plant fill the same declared box: per axis the declared dimension stays where the fewest scales would have to move (else it becomes the extent of the finest scale, to the millimetre), and at every scale whose extent then stands further off than `--share` of its smallest feature (default 0.5) the solids that reach beyond the side of the reference scale are pushed back to it (each by its own overshoot, along the world axis, as a translation in the frame of its bone), or the solid that reaches furthest is pushed out where the body falls short. Writes `dims` and `solid-set` ops (absolute values, idempotent) to `repair/<slug>.json`; the catalog file is written by `apply.ts`.
 *  bun fit-box.ts <slug> [--dry] [--share 0.5] [--keep widthM,heightM,depthM] [--declare widthM=1.23,…] */
import { parseArgs } from "node:util";
import { bodyOf, featureOf, load, opsFile, stateOf } from "./lib.ts";
import { applyOp, mergeOps, writeOps, type Op } from "./ops.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { dry: { type: "boolean" }, share: { type: "string" }, keep: { type: "string" }, declare: { type: "string" } } });
const slug = positionals[0]!;
const share = Number(values.share ?? 0.5);
const NAMES = ["widthM", "heightM", "depthM"] as const;
const { asset, ops: startOps } = await load(slug);
const made: Op[] = [];
const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
type Reach = { id: string; bone: string; lo: number[]; hi: number[] };
const reachOf = (scale: number): { per: Reach[]; lo: number[]; hi: number[]; state: any } => {
  const state = stateOf(asset, scale);
  const per = bodyOf(asset, state, scale).map((s: any) => { const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity]; for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) { const v = s.positions[i + k]; if (v < lo[k]!) lo[k] = v; if (v > hi[k]!) hi[k] = v; } return { id: s.id as string, bone: s.bone as string, lo, hi }; });
  return { per, lo: [0, 1, 2].map((k) => Math.min(...per.map((p) => p.lo[k]!))), hi: [0, 1, 2].map((k) => Math.max(...per.map((p) => p.hi[k]!))), state };
};
const forced = Object.fromEntries((values.declare ?? "").split(",").filter(Boolean).map((x) => { const [k, v] = x.split("="); return [k!, Number(v)]; }));
const set: Record<string, number> = {};
const shift = (scale: number, state: any, id: string, k: number, by: number): void => {
  const list = asset.volumes[String(scale)].body as any[];
  const solid = list.find((x) => x.id === id);
  if (solid === undefined) throw new Error(`${slug} 1:${scale}: solid ${id} is not a body solid (a copy of a repeat?)`);
  if (solid.weights !== undefined || solid.repeat !== undefined) throw new Error(`${slug} 1:${scale}: solid ${id} has weights or a repeat`);
  const f = state.bones[solid.bone].frame as { u: any; v: any; w: any };
  const axis = ["x", "y", "z"][k]!;
  const was = solid.transform?.translation ?? [0, 0, 0];
  const translation = [was[0] + by * f.u[axis], was[1] + by * f.v[axis], was[2] + by * f.w[axis]].map((v: number) => Number(v.toFixed(6)));
  const op: Op = { op: "solid-set", scale: String(scale), id, set: { transform: { ...(solid.transform ?? {}), translation } } };
  applyOp(asset, op);
  made.push(op);
};
for (let k = 0; k < 3; k++) {
  const name = NAMES[k]!;
  let reach = Object.fromEntries(scales.map((d) => [d, reachOf(d)]));
  const size = (d: number): number => reach[d]!.hi[k]! - reach[d]!.lo[k]!;
  const off = (target: number): number[] => scales.filter((d) => Math.abs(size(d) - target) > share * featureOf(d));
  const declared = asset.dimensions[name] as number, finest = Number(size(scales[0]!).toFixed(3));
  const target = forced[name] ?? ((values.keep ?? "").split(",").includes(name) || off(declared).length <= off(finest).length ? declared : finest);
  if (target !== declared) { set[name] = target; asset.dimensions[name] = target; }
  const moving = off(target);
  console.log(`[DEBUG] ${slug} ${name}: declared ${declared}${target !== declared ? ` -> ${target}` : ""}; ${scales.map((d) => `1:${d} ${size(d).toFixed(4)}`).join(", ")}; to move: ${moving.map((d) => `1:${d}`).join(", ") || "none"}`);
  const ref = scales.find((d) => !moving.includes(d)) ?? scales[0]!;
  const spare = target - size(ref);
  const [loT, hiT] = [reach[ref]!.lo[k]! - (k === 1 ? 0 : spare / 2), reach[ref]!.hi[k]! + (k === 1 ? spare : spare / 2)];
  for (const d of moving) {
    for (let pass = 0; pass < 4; pass++) {
      const r = (reach[d] = reachOf(d));
      const tol = 0.1 * featureOf(d);
      let moved = 0;
      for (const [sign, edge, goal] of [[1, r.hi[k]!, hiT], [-1, r.lo[k]!, loT]] as const) {
        const over = sign * (edge - goal);
        if (over > tol) for (const p of r.per) { const mine = sign * ((sign === 1 ? p.hi[k]! : p.lo[k]!) - goal); if (mine > 0) { shift(d, r.state, p.id, k, -sign * mine); moved++; console.log(`[DEBUG]   1:${d} ${name} ${sign === 1 ? "high" : "low"}: ${p.id} [${p.bone}] back by ${(mine * 1000).toFixed(1)} mm`); } }
        else if (over < -tol) { const p = r.per.slice().sort((a, b) => sign === 1 ? b.hi[k]! - a.hi[k]! : a.lo[k]! - b.lo[k]!)[0]!; shift(d, r.state, p.id, k, -sign * over); moved++; console.log(`[DEBUG]   1:${d} ${name} ${sign === 1 ? "high" : "low"}: ${p.id} [${p.bone}] out by ${(-over * 1000).toFixed(1)} mm`); }
      }
      if (moved === 0) break;
    }
    reach[d] = reachOf(d);
  }
  const still = off(target);
  console.log(`[DEBUG] ${slug} ${name}: now ${scales.map((d) => `1:${d} ${size(d).toFixed(4)}`).join(", ")} against ${target}${still.length > 0 ? ` STILL OFF at ${still.map((d) => `1:${d}`).join(", ")}` : ""}`);
}
if (Object.keys(set).length > 0) made.unshift({ op: "dims", set });
if (!values.dry && made.length > 0) await writeOps(opsFile(slug), mergeOps(startOps, made));
console.log(`[DEBUG] ${slug}: ${Object.keys(set).length} dimensions redeclared ${JSON.stringify(set)}, ${made.filter((o) => o.op === "solid-set").length} solids moved${values.dry ? " (dry)" : ""}`);
