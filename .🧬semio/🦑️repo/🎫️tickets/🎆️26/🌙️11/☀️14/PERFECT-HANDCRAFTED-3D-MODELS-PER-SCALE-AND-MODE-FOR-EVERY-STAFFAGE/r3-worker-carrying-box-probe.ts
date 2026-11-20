/** 🔬️ Probe of the round-3 draft of `humans/worker-carrying-box`: prints as JSON the planted travel of every looping clip (for `advanceM`) and, per pose, the product's 3D frame origins of a few bones (cross-check of the Python rig solver) and the world extent of the contact anchors.
 * Usage: `bun r3-worker-carrying-box-probe.ts [asset document json]` (default: the draft `🗑️generated/r3-worker-carrying-box/asset.json`). */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { sampleClip } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { GENERATED, type Json } from "./volumes-lib.ts";
import { rigAt } from "./r3-lib.ts";

const file = process.argv[2] ?? join(GENERATED, "r3-worker-carrying-box", "asset.json");
const asset = JSON.parse(readFileSync(file, "utf8")).asset as Json;
const travel: Record<string, number> = {};
for (const clip of asset.clips as Json[]) {
  if (clip.rootMotion !== "plant" || clip.loop !== true) continue;
  travel[clip.id] = (sampleClip(asset as never, clip as never, clip.durationS - 1e-9) as Json).root.x - (sampleClip(asset as never, clip as never, 0) as Json).root.x;
}
const poses: Record<string, Json> = {};
for (const pose of asset.poses as Json[]) {
  const rig = rigAt(asset, 50, { kind: "pose", pose: pose.id });
  poses[pose.id] = Object.fromEntries(["pelvis", "hand-l", "hand-r", "box-l", "box-r", "foot-l", "head"].map((b) => [b, { o: rig.frames[b]!.o.map((x) => Math.round(x * 1e5) / 1e5), u: rig.frames[b]!.u.map((x) => Math.round(x * 1e5) / 1e5), w: rig.frames[b]!.w.map((x) => Math.round(x * 1e5) / 1e5) }]));
}
process.stdout.write(JSON.stringify({ travel, poses }));
