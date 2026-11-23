/** 🔗️ Closes what `volumes-connected` reports at a scale: two parts of the structure that should meet get a joint capsule between their nearest points (in the frame of the bone of the second part); a loose body solid is moved by the measured gap onto what carries it. The rule is run again after every change, the ops are merged into the repair file of the person.
 *  bun bridge.ts <slug> [--scales 50,100] [--file <asset.json>] [--dry] [--skip idA,idB] [--grip memberId: what is found near this held member is joined to the nearest hand or arm bone instead] */
import { parseArgs } from "node:util";
import { checkVolumes, featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOp, applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

type V = [number, number, number];
const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, file: { type: "string" }, dry: { type: "boolean" }, skip: { type: "string" }, grip: { type: "string" } } });
const slug = positionals[0]!;
const asset = (await Bun.file(values.file ?? catalogFile(slug)).json()).asset;
const existing = await readOps(opsFile(slug));
applyOps(asset, existing);
const skip = new Set((values.skip ?? "").split(",").filter(Boolean));
const grip = new Set((values.grip ?? "").split(",").filter(Boolean));
const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const add = (a: V, b: V, s = 1): V => [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s];
const closestOnTriangle = (p: V, a: V, b: V, c: V): V => {
  const ab = sub(b, a), ac = sub(c, a), ap = sub(p, a);
  const d1 = dot(ab, ap), d2 = dot(ac, ap);
  if (d1 <= 0 && d2 <= 0) return a;
  const bp = sub(p, b), d3 = dot(ab, bp), d4 = dot(ac, bp);
  if (d3 >= 0 && d4 <= d3) return b;
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) return add(a, ab, d1 / (d1 - d3));
  const cp = sub(p, c), d5 = dot(ab, cp), d6 = dot(ac, cp);
  if (d6 >= 0 && d5 <= d6) return c;
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) return add(a, ac, d2 / (d2 - d6));
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) return add(b, sub(c, b), (d4 - d3) / (d4 - d3 + (d5 - d6)));
  const den = 1 / (va + vb + vc);
  return add(add(a, ab, vb * den), ac, vc * den);
};
type W = { id: string; bone?: string; positions: Float64Array; indices: Uint32Array };
const vert = (w: W, i: number): V => [w.positions[i * 3]!, w.positions[i * 3 + 1]!, w.positions[i * 3 + 2]!];
const nearest = (a: W, b: W): { p: V; q: V; d: number } => {
  let best = { p: [0, 0, 0] as V, q: [0, 0, 0] as V, d: Infinity };
  const one = (from: W, to: W, flip: boolean): void => {
    for (let i = 0; i < from.positions.length / 3; i++) {
      const p = vert(from, i);
      for (let k = 0; k < to.indices.length; k += 3) {
        const q = closestOnTriangle(p, vert(to, to.indices[k]!), vert(to, to.indices[k + 1]!), vert(to, to.indices[k + 2]!));
        const d = Math.hypot(...sub(p, q));
        if (d < best.d) best = flip ? { p: q, q: p, d } : { p, q, d };
      }
    }
  };
  one(a, b, false);
  one(b, a, true);
  return best;
};
const boxOf = (w: W): { min: V; max: V } => { const min: V = [Infinity, Infinity, Infinity], max: V = [-Infinity, -Infinity, -Infinity]; for (let i = 0; i < w.positions.length; i++) { const k = i % 3; if (w.positions[i]! < min[k]!) min[k] = w.positions[i]!; if (w.positions[i]! > max[k]!) max[k] = w.positions[i]!; } return { min, max }; };
const r5 = (x: number): number => Number(x.toFixed(5));
const ops: Op[] = [];
for (const scale of (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored")) {
  const feature = featureOf(scale);
  for (let round = 0; round < 8; round++) {
    const findings = checkVolumes(asset, { scale: scale as never, rules: ["volumes-connected"] as never });
    if (findings.length === 0) break;
    const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
    let changed = false;
    for (const f of findings) {
      const list = f.path.endsWith(".structure") ? "structure" : "body";
      const worlds = worldSolids(asset, state.bones as never, scale, list as never) as unknown as W[];
      for (const m of f.message.en.matchAll(/"([^"]+)"(?: \(\+\d+\))? ([\d.]+) mm from "([^"]+)"/gu)) {
        const [, named, reported, idB] = m as unknown as [string, string, string, string];
        if (skip.has(named) || skip.has(idB)) continue;
        const b = worlds.find((w) => w.id === idB);
        let idA = named;
        if (b !== undefined && /\(\+\d+\)/u.test(m[0]!)) {
          const want = Number(reported) / 1000, bb = boxOf(b);
          let bestGap = Infinity;
          for (const w of worlds) {
            if (w.id === idB) continue;
            const wb = boxOf(w);
            if ([0, 1, 2].some((k) => wb.min[k]! - bb.max[k]! > want + 0.002 || bb.min[k]! - wb.max[k]! > want + 0.002)) continue;
            const gap = Math.abs(nearest(w, b).d - want);
            if (gap < bestGap) { bestGap = gap; idA = w.id; }
          }
        }
        if (grip.has(idB) && b !== undefined) {
          const structure = asset.volumes[String(scale)][list] as any[], bodies = asset.volumes[String(scale)].body as any[];
          let bestD = Infinity;
          for (const w of worlds) {
            const so = structure.find((y) => y.id === w.id);
            const bone = String(so?.bone ?? bodies.find((y) => y.id === so?.of)?.bone ?? "");
            if (w.id === idB || !/^(hand|forearm|arm|finger|thumb|index|middle|ring|little)/u.test(bone)) continue;
            const d = nearest(w, b).d;
            if (d < bestD) { bestD = d; idA = w.id; }
          }
        }
        const a = worlds.find((w) => w.id === idA);
        const [sa, sb] = [asset.volumes[String(scale)][list].find((s: any) => s.id === idA), asset.volumes[String(scale)][list].find((s: any) => s.id === idB)];
        if (a === undefined || b === undefined || sa === undefined || sb === undefined) { console.log(`[DEBUG] 1:${scale} ${list} ${idA} / ${idB}: not found`); continue; }
        const bodyList = asset.volumes[String(scale)].body as any[];
        const boneOf = (x: any): string | undefined => x.bone ?? bodyList.find((y) => y.id === x.of)?.bone;
        const host = sb.bone !== undefined ? sb : sa.bone !== undefined ? sa : { ...(asset.volumes[String(scale)][list] as any[]).find((y) => y.material !== undefined && y.part === "member"), bone: boneOf(sb), member: sb.member };
        if (list === "structure" ? state.bones[host.bone] === undefined : state.bones[sa.bone] === undefined) { console.log(`[DEBUG] 1:${scale} ${list} ${idA} (${sa.bone}) / ${idB} (${sb.bone}): bone without a frame`); continue; }
        const { p, q, d } = nearest(a, b);
        const fr = (bone: string) => state.bones[bone].frame as { o: any; u: any; v: any; w: any };
        const local = (bone: string, x: V): V => { const g = fr(bone); const r: V = [x[0] - g.o.x, x[1] - g.o.y, x[2] - g.o.z]; return [dot(r, [g.u.x, g.u.y, g.u.z]), dot(r, [g.v.x, g.v.y, g.v.z]), dot(r, [g.w.x, g.w.y, g.w.z])]; };
        let op: Op;
        if (list === "structure") {
          if (host.weights !== undefined) { console.log(`[DEBUG] 1:${scale} ${host.id} is skinned: no joint added`); continue; }
          const n = d > 1e-9 ? (sub(q, p).map((x) => x / d) as V) : ([0, 1, 0] as V);
          const r = Math.max(0.006, feature / 2 + 0.0005, Math.min(0.012, d / 2));
          const [p0, p1] = [local(host.bone, add(p, n, -0.004)), local(host.bone, add(q, n, 0.004))];
          op = { op: "solid-add", scale: String(scale), list, solid: { id: `joint-${idA}-${idB}`.slice(0, 60), kind: "capsule", bone: host.bone, material: host.material, role: host.role, tone: host.tone, p0: p0.map(r5), p1: p1.map(r5), r0: r5(r), r1: r5(r), member: host.member ?? host.bone, part: "joint" } };
        } else {
          if (sa.weights !== undefined) { console.log(`[DEBUG] 1:${scale} ${idA} is skinned: not moved`); continue; }
          const g = fr(sa.bone);
          const move = sub(q, p).map((x) => x * (1 + 0.0015 / Math.max(d, 1e-6))) as V;
          const delta: V = [dot(move, [g.u.x, g.u.y, g.u.z]), dot(move, [g.v.x, g.v.y, g.v.z]), dot(move, [g.w.x, g.w.y, g.w.z])];
          const t = (sa.transform?.translation ?? [0, 0, 0]) as V;
          op = { op: "solid-set", scale: String(scale), list, id: idA, set: { transform: { ...(sa.transform ?? {}), translation: add(t, delta).map(r5) } } };
        }
        applyOp(asset, op);
        ops.push(op);
        changed = true;
        console.log(`${slug} 1:${scale} ${list}: ${idA} ${(d * 1000).toFixed(1)} mm from ${idB}: ${op.op === "solid-add" ? `joint ${op.solid.id}` : "moved onto it"}`);
      }
    }
    if (!changed) break;
  }
  for (const f of checkVolumes(asset, { scale: scale as never, rules: ["volumes-connected"] as never })) console.log(`   LEFT 1:${scale} ${f.path}: ${f.message.en}`.slice(0, 420));
}
if (!values.dry && ops.length > 0) await writeOps(opsFile(slug), mergeOps(existing, ops));
console.log(`${slug}: ${ops.length} connection ops${values.dry ? " (dry)" : ""}`);
