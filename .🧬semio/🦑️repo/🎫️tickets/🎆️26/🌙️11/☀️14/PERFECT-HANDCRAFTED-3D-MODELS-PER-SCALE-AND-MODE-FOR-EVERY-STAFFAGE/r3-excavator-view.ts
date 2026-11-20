/** 🔭️ Round 3 excavator inspection: `draw <asset file> <scale> <viewpoint> <pose|clip@t> [view]` writes the 2D drawing as SVG under 🗑️generated/r3-excavator/2d; `bbox <asset file> <scale>` prints the body extent in the default pose; `closure <asset file>` prints the largest pin gap of every cylinder and of the bucket linkage over the poses and clips (30 samples per second).
 *
 * @see ./r3-excavator-build.py — writes the draft asset this tool reads
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { extentOf, poseOf, scaleMeshes } from "./volumes-lib.ts";

const [command, file, ...rest] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const out = join(import.meta.dir, "🗑️generated", "r3-excavator", "2d");

const queryOf = (spec: string): any => {
  if (spec.includes("@")) {
    const [clip, t] = spec.split("@");
    return { kind: "clip", clip, timeS: Number(t) };
  }
  return { kind: "pose", pose: spec };
};

if (command === "draw") {
  const [scale, viewpoint, spec, view] = rest;
  const state = evaluate(asset, queryOf(spec ?? asset.presentation.defaultPose), { kind: "scale", scale: { denominator: Number(scale) } });
  const scene = depict(state, { ...defaultDepictionOptions((view ?? "light-solid") as any, "en", Number(scale), viewpoint as any) } as any, {});
  const path = join(out, `s${scale}.${viewpoint}.${(spec ?? "default").replace("@", "-")}.${view ?? "light-solid"}.svg`);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, writeSvg(scene));
  console.log(path);
}

if (command === "bbox") {
  const scale = Number(rest[0]);
  const posed = { ...asset, rig: asset.rig };
  const m = scaleMeshes(posed, scale, asset.volumes[String(scale)]);
  const e = extentOf(m.body)!;
  console.log("pose", poseOf(asset), "min", e.min.map((v: number) => v.toFixed(3)).join(" "), "max", e.max.map((v: number) => v.toFixed(3)).join(" "), "size", e.max.map((v: number, i: number) => (v - e.min[i]!).toFixed(3)).join(" "));
}

if (command === "closure") {
  const bones = Object.fromEntries(asset.rig.bones.map((b: any) => [b.id, b]));
  const pin = (state: any, bone: string, x: number, y: number): [number, number] => {
    const w = state.bones[bone].world;
    return [w[0] * x + w[2] * y + w[4], w[1] * x + w[3] * y + w[5]];
  };
  const targets: [string, string, string, number, number][] = [
    ["boom-rod", "boom", "D", 2.45, 0.66],
    ["stick-rod", "stick", "F", -0.794, 0.457],
    ["bucket-rod", "idler-link", "K", bones["idler-link"].lengthM, 0],
    ["bucket-link", "bucket", "J", -0.06, 0.47],
  ];
  const gap = (state: any): Record<string, number> => Object.fromEntries(targets.map(([rod, bone, name, x, y]) => {
    const a = pin(state, rod, bones[rod].lengthM, 0);
    const b = pin(state, bone, x, y);
    return [name, Math.hypot(a[0] - b[0], a[1] - b[1])];
  }));
  const show = (label: string, worst: Record<string, number>): void => console.log(label, Object.entries(worst).map(([k, v]) => `${k} ${(v * 1000).toFixed(1)} mm`).join("  "));
  show("rest", gap(evaluate({ ...asset, poses: [...asset.poses, { ...asset.poses[0], id: "r3-rest", bones: {} }] }, { kind: "pose", pose: "r3-rest" })));
  for (const pose of asset.poses) show(`pose ${pose.id}`, gap(evaluate(asset, { kind: "pose", pose: pose.id })));
  for (const clip of asset.clips) {
    const worst: Record<string, number> = {};
    for (let i = 0; i <= clip.durationS * 30; i++) {
      const g = gap(evaluate(asset, { kind: "clip", clip: clip.id, timeS: i / 30 }));
      for (const [k, v] of Object.entries(g)) worst[k] = Math.max(worst[k] ?? 0, v);
    }
    show(`clip ${clip.id}`, worst);
  }
}
