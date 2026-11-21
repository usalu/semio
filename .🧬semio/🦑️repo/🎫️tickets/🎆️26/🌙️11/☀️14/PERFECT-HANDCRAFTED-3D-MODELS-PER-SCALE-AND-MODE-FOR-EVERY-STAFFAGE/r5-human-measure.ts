/** 📏️ Round 5: measures a rider/kneeler figure in the rest pose of its rig against stature shares (DIN 33402 / ISO 7250 / Winter): joint heights from the rig, and from the body solids of every authored scale stature, head height, bideltoid and hip breadth, hand and foot length, thigh and calf girth.
 * Usage: `bun r5-human-measure.ts <category/slug> <upper-arm bone prefix: upper-arm|arm> [--file <asset json>]`
 * @see ./r5-brief.md
 * @see ./r3-lib.ts */
import { readFileSync } from "node:fs";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { meshesIn, type Frame } from "./r3-lib.ts";

type Json = Record<string, any>;
const [spec, armPrefix] = [process.argv[2]!, process.argv[3] ?? "upper-arm"];
const fileAt = process.argv.indexOf("--file");
const { entry, asset } = entryOf(spec);
const m: Json = fileAt > 0 ? JSON.parse(readFileSync(process.argv[fileAt + 1]!, "utf8")).asset : merged(asset, loadSource(entry)!);
const rest = restFrames(m.rig) as Record<string, { frame: Frame }>;
const frames: Record<string, Frame> = Object.fromEntries(Object.entries(rest).map(([k, b]) => [k, b.frame]));
const o = (b: string): number[] => frames[b]!.o as number[];
const SKIP = /hat|brim|band|helmet|hair|bun|pony|strap|plume|breath|collar|trowel|trug|pad|weed|leaf|stalk|saddle|bike|tyre|wheel|fender|frame|crank|pedal/;

function extent(solids: Json[], axis: number, filter: (p: number[]) => boolean = () => true): [number, number] {
  let [lo, hi] = [Infinity, -Infinity];
  for (const mesh of meshesIn(m, frames, solids)) {
    for (let i = 0; i < mesh.world.length; i += 3) {
      const p = [mesh.world[i]!, mesh.world[i + 1]!, mesh.world[i + 2]!];
      if (!filter(p)) continue;
      lo = Math.min(lo, p[axis]!);
      hi = Math.max(hi, p[axis]!);
    }
  }
  return [lo, hi];
}

function girth(solids: Json[], y: number): number {
  const pts: number[][] = [];
  for (const mesh of meshesIn(m, frames, solids)) for (let i = 0; i < mesh.world.length; i += 3) if (Math.abs(mesh.world[i + 1]! - y) < 0.02) pts.push([mesh.world[i]!, mesh.world[i + 2]!]);
  if (pts.length < 3) return NaN;
  const [x0, x1] = [Math.min(...pts.map((p) => p[0]!)), Math.max(...pts.map((p) => p[0]!))];
  const [z0, z1] = [Math.min(...pts.map((p) => p[1]!)), Math.max(...pts.map((p) => p[1]!))];
  const [a, b] = [(x1 - x0) / 2, (z1 - z0) / 2];
  return Math.PI * (3 * (a + b) - Math.sqrt((3 * a + b) * (a + 3 * b)));
}

const row = (name: string, model: number, ref: number): string => `| ${name} | ${model.toFixed(3)} | ${ref.toFixed(3)} | ${(((model - ref) / ref) * 100).toFixed(1)} % |`;
for (const scale of ["50", "100", "200", "500"]) {
  const vol = m.volumes[scale];
  if (vol?.status !== "authored") continue;
  const body = (vol.body as Json[]).filter((s) => !SKIP.test(s.id) && frames[s.bone] !== undefined);
  const on = (re: RegExp): Json[] => body.filter((s) => re.test(s.bone));
  const sole = extent(on(/^(foot|toe)-l$/), 1)[0];
  const top = extent(on(/^head$/), 1)[1];
  const H = top - sole;
  const chin = extent(on(/^head$/).filter((s) => !/neck/.test(s.id)), 1)[0];
  const shoulderY = o(`${armPrefix}-l`)[1]!;
  const bideltoid = extent(on(new RegExp(`^(torso|${armPrefix}-[lr])$`)), 2, (p) => p[1]! > shoulderY - 0.12);
  const hipY = o("thigh-l")[1]!;
  const hips = extent(on(/^(pelvis|thigh-[lr])$/), 2, (p) => Math.abs(p[1]! - (hipY - 0.06)) < 0.04);
  const hand = extent(on(/^hand-l$/), 1);
  const foot = extent(on(/^(foot|toe)-l$/), 0);
  const thighG = girth(on(/^thigh-l$/), hipY - 0.06);
  const kneeY = o("shank-l")[1]!;
  const calfG = Math.max(...[0.08, 0.1, 0.12, 0.14].map((d) => girth(on(/^shank-l$/), kneeY - d)));
  console.log(`\n### ${spec} 1:${scale} (rest pose, ${body.length} figure solids; H = ${H.toFixed(3)} m)\n\n| quantity | model m | reference m | deviation |\n|---|---|---|---|`);
  const ref = Number(process.env.STATURE ?? H);
  console.log(row("stature (sole to vertex)", H, ref));
  console.log(row("hip joint height 0.530 H", hipY - sole, 0.53 * ref));
  console.log(row("knee joint height 0.285 H", kneeY - sole, 0.285 * ref));
  console.log(row("ankle joint height 0.039 H (+ sole)", o("foot-l")[1]! - sole, 0.039 * ref + Number(process.env.SOLE ?? 0)));
  console.log(row("shoulder joint height 0.818 H", shoulderY - sole, 0.818 * ref));
  console.log(row("elbow height 0.630 H", o("forearm-l")[1]! - sole, 0.63 * ref));
  console.log(row("wrist height 0.485 H", o("hand-l")[1]! - sole, 0.485 * ref));
  console.log(row("head height chin to vertex H/7.5", top - chin, ref / 7.5));
  console.log(row("bideltoid breadth 0.259 H (clothed)", bideltoid[1] - bideltoid[0], 0.259 * ref));
  console.log(row("hip breadth (clothed)", hips[1] - hips[0], Number(process.env.HIPS ?? 0.2 * ref)));
  console.log(row("hand length 0.108 H (glove)", hand[1] - hand[0], 0.108 * ref + Number(process.env.GLOVE ?? 0)));
  console.log(row("foot length 0.152 H (+ shoe)", foot[1] - foot[0], 0.152 * ref + Number(process.env.SHOE ?? 0)));
  console.log(row("upper thigh girth 0.335 H (clothed)", thighG, 0.335 * ref));
  console.log(row("calf girth 0.212 H (clothed)", calfG, 0.212 * ref));
}
