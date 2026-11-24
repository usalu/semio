/** 🔎️ Measures a candidate document of the walking commuter against the guards that name it, without running their suites: the stride pin of the plan projection (the plan capsule against the 2D kinematics of that test), the three-view sums of the multiview coverage at four instants of the walk, and the print suitability of every scale: bun wc-probe.ts <asset json> [capsule id] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { extentOf, toleranceM } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const pin = process.argv[3] ?? "shoe-toe-l";
const radians = (degrees: number): number => (degrees * Math.PI) / 180;
type M = number[][];
const mul = (a: M, b: M): M => a.map((row) => [0, 1, 2].map((k) => row[0]! * b[0]![k]! + row[1]! * b[1]![k]! + row[2]! * b[2]![k]!));
const rw = (deg: number): M => [[Math.cos(radians(deg)), -Math.sin(radians(deg)), 0], [Math.sin(radians(deg)), Math.cos(radians(deg)), 0], [0, 0, 1]];
const rv = (deg: number): M => [[Math.cos(radians(deg)), 0, -Math.sin(radians(deg))], [0, 1, 0], [Math.sin(radians(deg)), 0, Math.cos(radians(deg))]];
const ru = (deg: number): M => [[1, 0, 0], [0, Math.cos(radians(deg)), Math.sin(radians(deg))], [0, -Math.sin(radians(deg)), Math.cos(radians(deg))]];
const FLAT = process.argv.includes("--flat");
function pointOf(pose: string, boneId: string, along: number): number {
  const offsets = asset.poses.find((candidate: { id: string }) => candidate.id === pose)?.bones ?? {};
  const chain: any[] = [];
  for (let bone = asset.rig.bones.find((b: any) => b.id === boneId); bone !== undefined; bone = asset.rig.bones.find((b: any) => b.id === bone.parent)) chain.unshift(bone);
  let r: M = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  let o = [0, 0, 0];
  for (const bone of chain) {
    const offset = offsets[bone.id] ?? {};
    const angle = Math.min(Math.max(offset.angleDeg ?? 0, bone.joint.minDeg ?? -Infinity), bone.joint.maxDeg ?? Infinity);
    const local = [bone.x, bone.y, FLAT ? 0 : (bone.z ?? 0)];
    o = o.map((value, k) => value + r[k]![0]! * local[0]! + r[k]![1]! * local[1]! + r[k]![2]! * local[2]!);
    const rest = FLAT ? rw(bone.angleDeg) : mul(mul(rw(bone.angleDeg), rv(bone.abductionDeg ?? 0)), ru(bone.twistDeg ?? 0));
    r = mul(mul(r, rest), mul(mul(rw(angle), rv(FLAT ? 0 : (offset.abductionDeg ?? 0))), ru(FLAT ? 0 : (offset.twistDeg ?? 0))));
  }
  const length = asset.rig.bones.find((bone: any) => bone.id === boneId).lengthM * along;
  return o[0]! + r[0]![0]! * length;
}
const capsule = asset.representations["50"].geometry.plan.find((shape: any) => shape.id === pin);
if (capsule?.kind === "capsule") {
  const tops: number[] = [];
  for (const pose of ["contact-left", "passing-right"]) {
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } });
    const points = state.shapes.plan.find((evaluated) => evaluated.shape.id === pin)!.rings.flat();
    const [dx0, dx1] = [pointOf(pose, capsule.bone, 0) - pointOf("", capsule.bone, 0), pointOf(pose, capsule.bone, 1) - pointOf("", capsule.bone, 1)];
    const [maxX, minX] = [Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.x))];
    const [wantMax, wantMin] = [Math.max(capsule.x0 + dx0 + capsule.r0, capsule.x1 + dx1 + capsule.r1), Math.min(capsule.x0 + dx0 - capsule.r0, capsule.x1 + dx1 - capsule.r1)];
    tops.push(maxX);
    console.log(`stride ${pin} ${pose}: maxX ${maxX.toFixed(4)} against ${wantMax.toFixed(4)} (${Math.abs(maxX - wantMax) < 0.005 ? "ok" : "FAIL"}), minX ${minX.toFixed(4)} against ${wantMin.toFixed(4)} (${Math.abs(minX - wantMin) < 0.005 ? "ok" : "FAIL"})`);
  }
  console.log(`stride travel ${Math.abs(tops[0]! - tops[1]!).toFixed(4)} (> 0.2 ${Math.abs(tops[0]! - tops[1]!) > 0.2 ? "ok" : "FAIL"})`);
} else console.log(`stride: plan shape ${pin} of 1:50 is ${capsule?.kind ?? "missing"}, not a capsule: FAIL`);
const tolerance = toleranceM(asset);
for (const timeS of [0, 0.3, 0.6, 0.9]) {
  const state = evaluate(asset, { kind: "clip", clip: "walk-cycle", timeS }, { kind: "scale", scale: { denominator: 100 } });
  const [e, f, p] = [extentOf(state.shapes.elevation)!, extentOf(state.shapes.front)!, extentOf(state.shapes.plan)!];
  const sums = [Math.abs(e.minX - p.minX) + Math.abs(e.maxX - p.maxX), Math.abs(e.minY - f.minY) + Math.abs(e.maxY - f.maxY), Math.abs(f.minX - p.minY) + Math.abs(f.maxX - p.maxY)];
  console.log(`walk ${timeS}: ${sums.map((sum) => `${sum.toFixed(4)} ${sum <= 2 * tolerance ? "ok" : "FAIL"}`).join(", ")} (limit ${(2 * tolerance).toFixed(3)})`);
}
if (!process.argv.includes("--no-print"))
  for (const scale of [50, 100, 200, 500]) {
    const suitability = measureSuitability(asset, asset.representations[String(scale)], scale, DEFAULT_PRINT_PROFILE) as any;
    console.log(`print 1:${scale}: ${suitability.suitable ? "ok" : "FAIL"} min feature ${Number(suitability.minFeatureMm).toFixed(3)} mm, min gap ${suitability.minGapMm}, ${suitability.violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${Number(v.measuredMm).toFixed(3)}<${v.requiredMm}`).join("; ")}`);
  }
