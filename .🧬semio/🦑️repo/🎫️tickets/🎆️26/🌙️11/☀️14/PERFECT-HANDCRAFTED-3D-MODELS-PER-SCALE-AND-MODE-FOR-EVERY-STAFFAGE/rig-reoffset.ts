/** 🔁️ Re-solves the pose and clip offsets of an asset that was authored before the rest placement became a full rotation, so every bone keeps the posed frame it had.
 *
 * Before 2026-11-22 a bone with a rest `abductionDeg` a took an offset (θ, φ, τ) as `Rw(rest + θ) · Rv(a + φ) · Ru(τ)`: the offset angle turned it about the joint axis of its PARENT. Now it is `Rw(rest) · Rv(a) · R(offset)`, about its own rest axes.
 * The offset that gives the same frame is `R(offset') = Rv(a)ᵀ · Rw(θ) · Rv(a + φ) · Ru(τ)`, read back as `angleDeg = atan2(M10, M00)`, `abductionDeg = asin(M20)`, `twistDeg = atan2(-M21, M22)`.
 * Only bones with a rest abduction and no rest twist are touched (a rest twist did not exist before); clip keys that override such a bone are re-solved from the merged offset (pose, then key), as the animation merges them.
 *
 * Usage: `bun rig-reoffset.ts <category>/<slug> [--restore] [--candidate | --write]` (`--candidate` writes the re-solved document to `🗑️generated/rig-verify/<slug>/🔣️.json` for `r3-sheet.ts --file` and `r3-2d.ts --file`). Without `--write` it only reports: per pose and clip key the largest move of a bone tip between the old and the new reading of the stored offsets (what the rig change did), and after the re-solve (must be ~0).
 * The re-solve must happen exactly once per asset: `--write` records the asset in `rig-reoffset.done.json` and every later run refuses it. An asset whose poses were authored after the rig change must never be run.
 * `--restore --write` undoes a recorded re-solve (the inverse rotation; angles come back to 1e-4 degree, abduction and twist offsets under 5e-4 degree are dropped) for an asset whose stored offsets shall stay as their author wrote them.
 * `--write` re-reads the catalog file, replaces only the text of `asset.poses` and `asset.clips` and leaves every other byte as it is.
 */
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

type M3 = number[][];
type Offset = { angleDeg?: number; abductionDeg?: number; twistDeg?: number; dx?: number; dy?: number };
type Bone = { id: string; parent: string | null; x: number; y: number; z?: number; angleDeg: number; abductionDeg?: number; twistDeg?: number; lengthM: number; joint: { type: string; minDeg?: number; maxDeg?: number; twistMinDeg?: number; twistMaxDeg?: number; continuousTwist?: boolean } };
const RAD = Math.PI / 180;
const mul = (a: M3, b: M3): M3 => a.map((row) => [0, 1, 2].map((j) => row[0]! * b[0]![j]! + row[1]! * b[1]![j]! + row[2]! * b[2]![j]!));
const transpose = (a: M3): M3 => [0, 1, 2].map((i) => [0, 1, 2].map((j) => a[j]![i]!));
const Mw = (deg: number): M3 => [[Math.cos(deg * RAD), -Math.sin(deg * RAD), 0], [Math.sin(deg * RAD), Math.cos(deg * RAD), 0], [0, 0, 1]];
const Mv = (deg: number): M3 => [[Math.cos(deg * RAD), 0, -Math.sin(deg * RAD)], [0, 1, 0], [Math.sin(deg * RAD), 0, Math.cos(deg * RAD)]];
const Mu = (deg: number): M3 => [[1, 0, 0], [0, Math.cos(deg * RAD), Math.sin(deg * RAD)], [0, -Math.sin(deg * RAD), Math.cos(deg * RAD)]];
const clamp = (v: number, lo?: number, hi?: number): number => Math.min(hi ?? Infinity, Math.max(lo ?? -Infinity, v));
const round = (v: number): number => Math.round(v * 1e4) / 1e4 + 0;

const [spec, ...flags] = process.argv.slice(2);
const RESTORE = flags.includes("--restore");
if (spec === undefined) { process.stderr.write("rig-reoffset.ts <category>/<slug> [--write]\n"); process.exit(2); }
const [category, slug] = spec.split("/") as [string, string];
const CATALOG = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const folder = readdirSync(CATALOG).find((name) => name.endsWith(category))!;
const file = join(CATALOG, folder, slug, "🔣️.json");

function spanOf(raw: string, key: string): [number, number] {
  let depth = 0;
  for (let i = 0; i < raw.length; i++) {
    const c = raw[i]!;
    if (c === '"') {
      let j = i + 1;
      while (raw[j] !== '"') j += raw[j] === "\\" ? 2 : 1;
      if (depth === 2 && raw.slice(i + 1, j) === key && /^\s*:/u.test(raw.slice(j + 1, j + 8))) {
        let k = raw.indexOf(":", j) + 1;
        while (/\s/u.test(raw[k]!)) k++;
        let d = 0, e = k;
        for (; ; e++) {
          const ch = raw[e]!;
          if (ch === '"') { e++; while (raw[e] !== '"') e += raw[e] === "\\" ? 2 : 1; }
          else if (ch === "[" || ch === "{") d++;
          else if (ch === "]" || ch === "}") { d--; if (d === 0) break; }
        }
        return [k, e + 1];
      }
      i = j;
    } else if (c === "{" || c === "[") depth++;
    else if (c === "}" || c === "]") depth--;
  }
  throw new Error(`no asset.${key} in ${file}`);
}

function plan(raw: string) {
  const asset = JSON.parse(raw).asset as { rig: { root: string; bones: Bone[] }; poses: { id: string; bones: Record<string, Offset> }[]; clips: { id: string; keys: { t: number; pose: string; bones?: Record<string, Offset> }[] }[] };
  const bones = new Map(asset.rig.bones.map((bone) => [bone.id, bone]));
  const touched = (bone: Bone): boolean => (bone.abductionDeg ?? 0) !== 0 && (bone.twistDeg ?? 0) === 0 && bone.joint.type === "revolute" && !bone.joint.continuousTwist;
  const effective = (bone: Bone, o: Offset | undefined) => bone.joint.type !== "revolute" ? { a: 0, b: 0, t: 0 } : bone.joint.continuousTwist ? { a: 0, b: 0, t: o?.twistDeg ?? 0 } : { a: clamp(o?.angleDeg ?? 0, bone.joint.minDeg, bone.joint.maxDeg), b: clamp(o?.abductionDeg ?? 0, -180, 180), t: clamp(o?.twistDeg ?? 0, bone.joint.twistMinDeg ?? -180, bone.joint.twistMaxDeg ?? 180) };
  const oldLocal = (bone: Bone, o: Offset | undefined): M3 => { const e = effective(bone, o); return mul(mul(Mw(bone.angleDeg + e.a), Mv((bone.abductionDeg ?? 0) + e.b)), Mu(e.t)); };
  const warnings: string[] = [];
  const resolve = (bone: Bone, o: Offset, where: string): Offset => {
    const e = effective(bone, o);
    if (e.a === 0 && e.t === 0 && !(RESTORE && e.b !== 0)) return o;
    const spread = bone.abductionDeg ?? 0;
    const m = RESTORE ? mul(Mv(spread), mul(mul(Mw(e.a), Mv(e.b)), Mu(e.t))) : mul(transpose(Mv(spread)), mul(mul(Mw(e.a), Mv(spread + e.b)), Mu(e.t)));
    const snap = (v: number): number => (RESTORE && Math.abs(v) < 5e-4 ? 0 : v);
    const next = { angleDeg: round(Math.atan2(m[1]![0]!, m[0]![0]!) / RAD), abductionDeg: snap(round(Math.asin(clamp(m[2]![0]!, -1, 1)) / RAD - (RESTORE ? spread : 0))), twistDeg: snap(round(Math.atan2(-m[2]![1]!, m[2]![2]!) / RAD)) };
    if (next.angleDeg !== clamp(next.angleDeg, bone.joint.minDeg, bone.joint.maxDeg)) warnings.push(`${where} ${bone.id}: angleDeg ${next.angleDeg} leaves ${bone.joint.minDeg}..${bone.joint.maxDeg}`);
    if (next.twistDeg !== clamp(next.twistDeg, bone.joint.twistMinDeg ?? -180, bone.joint.twistMaxDeg ?? 180)) warnings.push(`${where} ${bone.id}: twistDeg ${next.twistDeg} leaves ${bone.joint.twistMinDeg ?? -180}..${bone.joint.twistMaxDeg ?? 180}`);
    const { angleDeg: _a, abductionDeg: _b, twistDeg: _t, ...rest } = o;
    return { ...(next.angleDeg === 0 ? {} : { angleDeg: next.angleDeg }), ...(next.abductionDeg === 0 ? {} : { abductionDeg: next.abductionDeg }), ...(next.twistDeg === 0 ? {} : { twistDeg: next.twistDeg }), ...rest };
  };
  const poses = asset.poses.map((pose) => ({ ...pose, bones: Object.fromEntries(Object.entries(pose.bones).map(([id, o]) => [id, touched(bones.get(id)!) ? resolve(bones.get(id)!, o, `pose ${pose.id}`) : o])) }));
  const clips = asset.clips.map((clip) => ({ ...clip, keys: clip.keys.map((key) => {
    if (key.bones === undefined) return key;
    const base = asset.poses.find((pose) => pose.id === key.pose)!.bones;
    return { ...key, bones: Object.fromEntries(Object.entries(key.bones).map(([id, over]) => {
      const bone = bones.get(id)!;
      if (!touched(bone)) return [id, over];
      const solved = resolve(bone, { ...base[id], ...over }, `clip ${clip.id} t ${key.t}`);
      const under = poses.find((pose) => pose.id === key.pose)!.bones[id] ?? {};
      const part = (name: "angleDeg" | "abductionDeg" | "twistDeg") => ((solved[name] ?? 0) === 0 && (under[name] ?? 0) === 0 ? {} : { [name]: solved[name] ?? 0 });
      return [id, { ...part("angleDeg"), ...part("abductionDeg"), ...part("twistDeg"), ...(solved.dx === undefined ? {} : { dx: solved.dx }), ...(solved.dy === undefined ? {} : { dy: solved.dy }) }];
    })) };
  }) }));
  const order: Bone[] = [];
  const visit = (bone: Bone): void => { if (order.includes(bone)) return; if (bone.parent !== null) visit(bones.get(bone.parent)!); order.push(bone); };
  asset.rig.bones.forEach(visit);
  const oldTips = (offsets: Record<string, Offset>): Record<string, number[]> => {
    const frames = new Map<string, { o: number[]; m: M3 }>();
    const out: Record<string, number[]> = {};
    for (const bone of order) {
      const parent = bone.parent === null ? { o: [0, 0, 0], m: Mw(0) } : frames.get(bone.parent)!;
      const local = [bone.x, bone.y, bone.z ?? 0];
      const o = [0, 1, 2].map((i) => parent.o[i]! + parent.m[i]![0]! * local[0]! + parent.m[i]![1]! * local[1]! + parent.m[i]![2]! * local[2]!);
      const m = mul(parent.m, oldLocal(bone, offsets[bone.id]));
      frames.set(bone.id, { o, m });
      out[bone.id] = [0, 1, 2].map((i) => o[i]! + m[i]![0]! * bone.lengthM);
    }
    return out;
  };
  const newTips = (offsets: Record<string, Offset>): Record<string, number[]> => Object.fromEntries(Object.entries(solveChain(asset.rig as never, offsets as never, { x: 0, y: 0, angleDeg: 0 })).map(([id, solved]) => [id, [0, 1, 2].map((i) => solved.frame.o[i]! + solved.frame.u[i]! * bones.get(id)!.lengthM)]));
  const prismatic = asset.rig.bones.some((bone) => bone.joint.type === "prismatic");
  const far = (a: Record<string, number[]>, b: Record<string, number[]>): { d: number; bone: string } => Object.keys(a).reduce((worst, id) => { const d = Math.hypot(a[id]![0]! - b[id]![0]!, a[id]![1]! - b[id]![1]!, a[id]![2]! - b[id]![2]!); return d > worst.d ? { d, bone: id } : worst; }, { d: 0, bone: "" });
  const merged = (list: typeof asset.poses, key: { pose: string; bones?: Record<string, Offset> }): Record<string, Offset> => { const base = list.find((pose) => pose.id === key.pose)!.bones; return Object.fromEntries(asset.rig.bones.map((bone) => [bone.id, { ...base[bone.id], ...key.bones?.[bone.id] }])); };
  const rows: string[] = [];
  let before = 0, after = 0;
  const row = (label: string, stored: Record<string, Offset>, solved: Record<string, Offset>): void => {
    const was = RESTORE ? newTips(stored) : oldTips(stored);
    const [b, a] = RESTORE ? [far(was, newTips(solved)), far(was, oldTips(solved))] : [far(was, newTips(stored)), far(was, newTips(solved))];
    before = Math.max(before, b.d); after = Math.max(after, a.d);
    if (b.d > 5e-4) rows.push(`${label}: moved ${(b.d * 1000).toFixed(1)} mm at ${b.bone}, after the re-solve ${(a.d * 1000).toFixed(2)} mm`);
  };
  for (const [i, pose] of asset.poses.entries()) row(`pose ${pose.id}`, pose.bones, poses[i]!.bones);
  for (const [c, clip] of asset.clips.entries()) for (const [k, key] of clip.keys.entries()) row(`clip ${clip.id} key ${key.t} s`, merged(asset.poses, key), merged(poses, clips[c]!.keys[k]!));
  return { asset, poses, clips, rows, before, after, warnings, folds: prismatic || ((asset.rig as { radialFolds?: unknown[] }).radialFolds?.length ?? 0) > 0, touched: asset.rig.bones.filter(touched).length };
}

const serialise = (old: string, value: unknown, raw: string, at: number): string => {
  const lineStart = raw.lastIndexOf("\n", at) + 1;
  const base = raw.slice(lineStart, at).match(/^\s*/u)![0];
  const original = JSON.parse(old) as unknown;
  for (const indent of [undefined, 1, 2, 3, 4, "\t"] as const) {
    const write = (v: unknown): string => (indent === undefined ? JSON.stringify(v) : JSON.stringify(v, null, indent).replace(/\n/gu, `\n${base}`));
    if (write(original) === old) return write(value);
  }
  return JSON.stringify(value, null, 1).replace(/\n/gu, `\n${base}`);
};

const LEDGER = join(import.meta.dir, "rig-reoffset.done.json");
const done = JSON.parse(readFileSync(LEDGER, "utf8")) as Record<string, string>;
if (RESTORE && done[spec] === undefined) { console.log(`${spec} was not re-solved (rig-reoffset.done.json): nothing to restore.`); process.exit(0); }
if (!RESTORE && done[spec] !== undefined) { console.log(`${spec} was re-solved on ${done[spec]} (rig-reoffset.done.json): its stored offsets are already about the bones' own rest axes, a second run would turn them again. Nothing done.`); process.exit(0); }
const raw = readFileSync(file, "utf8");
const result = plan(raw);
console.log(`${spec}: ${result.touched} bones with a rest abduction and no rest twist; largest move of a bone tip by the rig change ${(result.before * 1000).toFixed(1)} mm, after the re-solve ${(result.after * 1000).toFixed(2)} mm`);
if (RESTORE) console.log("  restore: the first figure is how far the restored offsets pose a bone tip from where it is now, the second how far the restored offsets, read the old way, are from the present frames (the proof of the inverse)");
for (const line of result.rows) console.log(`  ${line}`);
for (const line of result.warnings) console.log(`  LIMIT ${line}`);
if (result.folds) console.log("  the rig has radial folds or prismatic joints, which the old reading here does not follow: the moves above are not meaningful for their bones");
if (flags.includes("--write") || flags.includes("--candidate")) {
  if (result.warnings.length > 0) { console.log("not written: a re-solved offset leaves its joint limit"); process.exit(1); }
  const fresh = readFileSync(file, "utf8");
  const again = plan(fresh);
  let text = fresh;
  for (const key of ["clips", "poses"] as const) {
    const [from, to] = spanOf(text, key);
    text = text.slice(0, from) + serialise(text.slice(from, to), again[key], text, from) + text.slice(to);
  }
  const check = JSON.parse(text).asset;
  const kept = JSON.parse(fresh).asset;
  for (const key of Object.keys(kept)) if (key !== "poses" && key !== "clips" && JSON.stringify(kept[key]) !== JSON.stringify(check[key])) throw new Error(`asset.${key} would change`);
  if (flags.includes("--candidate")) {
    const out = join(import.meta.dir, "🗑️generated/rig-verify", slug);
    mkdirSync(out, { recursive: true });
    writeFileSync(join(out, "🔣️.json"), text);
    console.log(`candidate: ${join(out, "🔣️.json")}`);
    process.exit(0);
  }
  if (readFileSync(file, "utf8") !== fresh) { console.log("the file changed while solving: run again"); process.exit(1); }
  writeFileSync(file, text);
  const ledger = JSON.parse(readFileSync(LEDGER, "utf8")) as Record<string, string>;
  if (RESTORE) delete ledger[spec];
  else ledger[spec] = new Date().toISOString();
  writeFileSync(LEDGER, `${JSON.stringify(ledger, null, 2)}\n`);
  console.log(`written: asset.poses and asset.clips of ${file} (${fresh.length} -> ${text.length} characters)`);
}
