/** 🔬️ The evaluated extents of the drawn shapes of an asset at a scale in a pose, per view and shape (id, bone, kind, solid), with the ops of `ops/<id>.json` applied in memory unless `--no-ops`; `--bones a,b` limits it to bones, `--rig` prints bones, poses and anchors instead.
 *  bun probe.ts <id> <scale> <pose> [--bones a,b] [--no-ops] | bun probe.ts <id> --rig */
import { applyAll } from "./apply.ts";
import { S, catalogFile, opsFile } from "./files.ts";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { restFrames } = await import(`${S}/🔨️modules/🦴️rig/🟦️.ts`);
const args = process.argv.slice(2);
const slug = args[0]!;
const asset = (await Bun.file(catalogFile(slug)).json()).asset;
if (!args.includes("--no-ops") && (await Bun.file(opsFile(slug)).exists())) applyAll(asset, JSON.parse(await Bun.file(opsFile(slug)).text()));
const f = (n: number): string => n.toFixed(3);
if (args.includes("--rig")) {
  console.log("dims", JSON.stringify(asset.dimensions), "default", asset.presentation.defaultPose);
  const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
  for (const bone of asset.rig.bones) console.log("bone", JSON.stringify(bone), "o", frames[bone.id]!.frame.o.map(f).join(","), "u", frames[bone.id]!.frame.u.map(f).join(","));
  for (const pose of asset.poses) console.log("pose", JSON.stringify({ id: pose.id, bones: pose.bones, root: pose.root, ground: pose.ground, contacts: pose.contacts }));
  for (const anchor of asset.anchors) console.log("anchor", JSON.stringify({ ...anchor, name: undefined, description: undefined }));
  for (const clip of asset.clips ?? []) console.log("clip", clip.id, JSON.stringify(clip).length);
  process.exit(0);
}
const at = args.indexOf("--bones");
const bones = at < 0 ? undefined : new Set(args[at + 1]!.split(","));
const state: any = evaluate(asset, { kind: "pose", pose: args[2]! }, { kind: "scale", scale: { denominator: Number(args[1]) } });
for (const view of ["elevation", "front", "plan"]) {
  let lo = [Infinity, Infinity], hi = [-Infinity, -Infinity];
  const lines: string[] = [];
  for (const s of state.shapes[view]) {
    const pts = s.rings.flat();
    if (pts.length === 0) continue;
    const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    const box = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
    lo = [Math.min(lo[0]!, box[0]!), Math.min(lo[1]!, box[2]!)]; hi = [Math.max(hi[0]!, box[1]!), Math.max(hi[1]!, box[3]!)];
    if (bones !== undefined && !bones.has(s.shape.bone)) continue;
    lines.push(`   ${s.shape.id} [${s.shape.bone}] ${s.shape.kind}${s.shape.solid ? ` ${s.shape.solid.kind}${s.shape.solid.sweep ? `:${s.shape.solid.sweep}` : `:${s.shape.solid.axis}`}` : ""}${s.shape.followTip ? " followTip" : ""} x ${f(box[0]!)} ${f(box[1]!)} y ${f(box[2]!)} ${f(box[3]!)}`);
  }
  console.log(`-- ${view} 1:${args[1]} ${args[2]} all x ${f(lo[0]!)} ${f(hi[0]!)} y ${f(lo[1]!)} ${f(hi[1]!)}`);
  for (const line of lines) console.log(line);
}
