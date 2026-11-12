/** ✅️ Ticket-local validation of an authored volume: Ajv schema conformance, owned `validateAsset` volume rules, and a pose walk (every pose and clip key: detachment of neighbouring solids, wrong side).
 * Usage: bun veg-validate.ts <slug> [--poses] */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { STAFFAGE, frames, loadAsset, meshes, resolveAsset, toWorld, type Json, type V3 } from "./volume-lib.ts";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { carry } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const slug = process.argv[2]!;
const withPoses = process.argv.includes("--poses");
const entry = resolveAsset(slug);
const asset = loadAsset(entry.path);
const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
const ok = validate(JSON.parse(readFileSync(entry.path, "utf8")).asset);
const badBone = new Set<number>((asset.volume?.solids ?? []).map((s: Json, i: number) => (/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(s.bone) ? -1 : i)).filter((i: number) => i >= 0));
const allVol = ok ? [] : (validate.errors ?? []).filter((e) => e.instancePath.startsWith("/volume"));
const schemaErrors = allVol.filter((e) => { const m = /^\/volume\/solids\/(\d+)/.exec(e.instancePath); return !(m && badBone.has(Number(m[1]))); }).map((e) => `${e.instancePath} ${e.message}`);
const attributed = allVol.length - schemaErrors.length;
const issues = validateAsset(asset).filter((i: any) => String(i.code).startsWith("volume-"));
console.log(`${slug}: schema errors (volume) ${schemaErrors.length} (plus ${attributed} on ${badBone.size} solids whose bone id is not a slug in the asset's own rig), volume rule issues ${issues.length}, whole-asset schema ${ok ? "ok" : "errors outside volume " + ((validate.errors ?? []).filter((e) => !e.instancePath.startsWith("/volume")).length)}`);
for (const e of schemaErrors.slice(0, 10)) console.log("  schema:", e);
for (const i of issues.slice(0, 20)) console.log("  rule:", (i as any).code, (i as any).path, (i as any).message?.en);

if (withPoses) {
  const solids = asset.volume.solids as Json[];
  const rest = frames(asset);
  const list = meshes(asset, solids, 1);
  const sample = (m: any): V3[] => {
    const w = m.world as Float64Array, ix = m.indices as Uint32Array;
    const P = (i: number): V3 => [w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!];
    let total = 0;
    const per: number[] = [];
    for (let t = 0; t < ix.length; t += 3) { const a = P(ix[t]!), b = P(ix[t + 1]!), c = P(ix[t + 2]!); const e = Math.max(Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]), Math.hypot(b[0] - c[0], b[1] - c[1], b[2] - c[2]), Math.hypot(a[0] - c[0], a[1] - c[1], a[2] - c[2])); const k = Math.max(1, Math.ceil(e / 0.12)); per.push(k); total += ((k + 1) * (k + 2)) / 2; }
    const shrink = Math.max(1, Math.ceil(total / 4000));
    const out: V3[] = [];
    for (let t = 0, n = 0; t < ix.length; t += 3, n++) { if (n % shrink !== 0) continue; const a = P(ix[t]!), b = P(ix[t + 1]!), c = P(ix[t + 2]!); const k = Math.min(per[n]!, 12); for (let i = 0; i <= k; i++) for (let j = 0; j <= k - i; j++) { const u = i / k, v = j / k, q = 1 - u - v; out.push([a[0] * q + b[0] * u + c[0] * v, a[1] * q + b[1] * u + c[1] * v, a[2] * q + b[2] * u + c[2] * v]); } }
    return out;
  };
  const pts = list.map(sample);
  const bounds = (A: V3[]) => { const lo: V3 = [Infinity, Infinity, Infinity], hi: V3 = [-Infinity, -Infinity, -Infinity]; const c: V3 = [0, 0, 0]; for (const p of A) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); c[k] += p[k]! / A.length; } return { lo, hi, c }; };
  const inside = (c: V3, b: { lo: V3; hi: V3 }) => [0, 1, 2].every((k) => c[k]! >= b.lo[k]! && c[k]! <= b.hi[k]!);
  const minDist = (A: V3[], B: V3[]) => { const a = bounds(A), b = bounds(B); if (inside(a.c, b) || inside(b.c, a)) return 0; return rawDist(A, B); };
  const rawDist = (A: V3[], B: V3[]) => { let best = Infinity; for (const p of A) for (const q of B) { const d = (p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2 + (p[2] - q[2]) ** 2; if (d < best) best = d; } return Math.sqrt(best); };
  const boneOf = (n: number): string => solids[n].bone;
  const byBone = new Map<string, number[]>();
  solids.forEach((sol, n) => { if (sol.weights === undefined) (byBone.get(sol.bone) ?? byBone.set(sol.bone, []).get(sol.bone)!).push(n); });
  const parentWith = (b: string): string | null => { let cur = asset.rig.bones.find((x: Json) => x.id === b)?.parent ?? null; while (cur !== null) { if (byBone.has(cur)) return cur; cur = asset.rig.bones.find((x: Json) => x.id === cur)?.parent ?? null; } return null; };
  const joints: { child: string; parent: string; rest: number }[] = [];
  const gather = (b: string, src: V3[][]) => byBone.get(b)!.flatMap((n) => src[n]!);
  for (const b of byBone.keys()) { const p = parentWith(b); if (p !== null) joints.push({ child: b, parent: p, rest: rawDist(gather(b, pts), gather(p, pts)) }); }
  if (process.env.VEG_DEBUG) console.log(joints.map((j) => `${j.child}~${j.parent} ${j.rest.toFixed(2)}`).join("; "));
  console.log(`  joints checked (bone against nearest ancestor bone with solids): ${joints.length}; largest rest gap ${Math.max(0, ...joints.map((j) => j.rest)).toFixed(3)} m (${joints.slice().sort((x, y) => y.rest - x.rest)[0]?.child ?? "-"})`);
  const queries: { name: string; q: any }[] = [...asset.poses.map((p: any) => ({ name: `pose:${p.id}`, q: { kind: "pose", pose: p.id } })), ...asset.clips.flatMap((c: any) => [0.25, 0.5, 0.75].map((f) => ({ name: `clip:${c.id}@${f}`, q: { kind: "clip", clip: c.id, timeS: f * c.durationS } })))];
  for (const { name, q } of queries) {
    let state: any;
    try { state = evaluate(asset, q, { kind: "scale", scale: { denominator: 50 } }) as any; } catch (e) { console.log(`  ${name}: evaluate failed ${(e as Error).message}`); continue; }
    const arr = (v: any): V3 => [v.x, v.y, v.z];
    const frameOf = (b: string) => { const f = state.bones[b]?.frame; return f === undefined ? undefined : { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) }; };
    const posed = pts.map((ps, n) => {
      const sol = solids[n];
      const list = sol.weights ?? [{ bone: sol.bone, weight: 1 }];
      return ps.map((p) => { const out: V3 = [0, 0, 0]; for (const { bone, weight } of list) { const now = frameOf(bone); const q = now === undefined ? p : carry(rest[bone]!, now, p); out[0] += q[0] * weight; out[1] += q[1] * weight; out[2] += q[2] * weight; } return out; });
    });
    let worst = 0; let worstJoint = "";
    for (const j of joints) { const gap = rawDist(gather(j.child, posed), gather(j.parent, posed)) - j.rest; if (gap > worst) { worst = gap; worstJoint = `${j.child}~${j.parent}`; } }
    let wrong = 0; let wrongId = "";
    posed.forEach((ps, n) => { const c0 = [0, 1, 2].map((k) => pts[n]!.reduce((t, p) => t + p[k]!, 0) / pts[n]!.length); const c1 = [0, 1, 2].map((k) => ps.reduce((t, p) => t + p[k]!, 0) / ps.length); const move = Math.hypot(c1[0]! - c0[0]!, c1[1]! - c0[1]!, c1[2]! - c0[2]!); if (move > wrong) { wrong = move; wrongId = solids[n].id; } });
    let hi = -Infinity, ymin = Infinity;
    for (const ps of posed) for (const p of ps) { if (p[1] < ymin) ymin = p[1]; if (p[1] > hi) hi = p[1]; }
    if (name === `pose:${asset.presentation?.defaultPose}`) { const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity]; for (const ps of posed) for (const p of ps) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); } const d = asset.dimensions; console.log(`  default-pose bbox ${[0, 1, 2].map((k) => (hi[k]! - lo[k]!).toFixed(3)).join(" x ")} vs dimensions ${d.widthM} x ${d.heightM} x ${d.depthM}: error % ${[hi[0]! - lo[0]!, hi[1]! - lo[1]!, hi[2]! - lo[2]!].map((v, k) => (((v - [d.widthM, d.heightM, d.depthM][k]!) / [d.widthM, d.heightM, d.depthM][k]!) * 100).toFixed(1)).join(" / ")}`); }
    console.log(`  ${name}: worst joint gap +${worst.toFixed(3)} m (${worstJoint}); largest centroid move ${wrong.toFixed(2)} m (${wrongId}); y ${ymin.toFixed(2)}..${hi.toFixed(2)}`);
  }
}
