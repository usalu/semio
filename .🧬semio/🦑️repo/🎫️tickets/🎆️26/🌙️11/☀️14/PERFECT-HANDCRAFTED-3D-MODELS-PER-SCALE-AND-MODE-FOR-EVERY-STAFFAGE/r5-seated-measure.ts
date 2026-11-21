/** 📐️ Round 5, seated figures: measures a candidate document against the reference table of the seated office worker or the seated reader (joint heights from the rig, extents of the body solids in world coordinates) in the default pose at any scale, and prints model, reference and deviation per quantity.
 * Usage: `bun r5-seated-measure.ts office|reader [--scale 50] [--pose ID]`
 * @see ./r5-brief.md
 * @see ./r3-humans-seated-office-worker.md
 * @see ./r3-humans-seated-reader.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

type Json = Record<string, any>;
type Box = { min: number[]; max: number[] };
type Ref = { name: string; ref: number; model: (m: Measure) => number; source: string };
type Measure = { box: (re: RegExp) => Box; joint: (bone: string) => number[]; along: (re: RegExp, bone: string) => number; seat: number };

const args = process.argv.slice(2);
const who = args[0] === "reader" ? "reader" : "office";
const flag = (name: string, fallback: string): string => (args.indexOf(name) < 0 ? fallback : args[args.indexOf(name) + 1]!);
const scale = Number(flag("--scale", "50"));
const slug = who === "reader" ? "seated-reader" : "seated-office-worker";
const document = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", `r3-${slug}`, "🔣️.json"), "utf8")) as Json;
const asset = document.asset as Json;
const pose = flag("--pose", asset.presentation.defaultPose);
const frames: Record<string, Frame> = args.includes("--rest") ? Object.fromEntries(Object.entries(restFrames(asset.rig) as Record<string, Json>).map(([id, b]) => [id, b.frame as Frame])) : rigAt(asset, scale, { kind: "pose", pose }).frames;
const solids = (asset.volumes[String(scale)].body as Json[]).filter((s) => s.texture !== true);
const meshes = meshesIn(asset, frames, solids);
const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;

const m: Measure = {
  box(re) {
    const out = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
    for (const mesh of meshes) if (re.test(mesh.id)) for (let i = 0; i < mesh.world.length; i++) {
      out.min[i % 3] = Math.min(out.min[i % 3]!, mesh.world[i]!);
      out.max[i % 3] = Math.max(out.max[i % 3]!, mesh.world[i]!);
    }
    return out;
  },
  joint: (bone) => [...frames[bone]!.o],
  along(re, bone) {
    const f = frames[bone]!;
    let [lo, hi] = [Infinity, -Infinity];
    for (const mesh of meshes) if (re.test(mesh.id)) for (let i = 0; i < mesh.world.length; i += 3) {
      const d = dot([mesh.world[i]! - f.o[0]!, mesh.world[i + 1]! - f.o[1]!, mesh.world[i + 2]! - f.o[2]!], f.u);
      [lo, hi] = [Math.min(lo, d), Math.max(hi, d)];
    }
    return hi - lo;
  },
  seat: 0,
};

const OFFICE: Ref[] = [
  { name: "sitting height (seat top to vertex)", ref: 0.925, model: (q) => q.box(/^(hair|head)$/).max[1]! - q.seat, source: "DIN 33402-2 m P50 0.907 at 1.75 m, scaled to 1.78 m" },
  { name: "head height chin to vertex (H/7.5)", ref: 0.237, model: (q) => q.along(/^(head|hair)$/, "head"), source: "brief; ISO 7250 m P50" },
  { name: "head breadth (hair incl.)", ref: 0.16, model: (q) => q.box(/^(head|hair)$/).max[2]! - q.box(/^(head|hair)$/).min[2]!, source: "ISO 7250 m P50 0.155 + hair" },
  { name: "shoulder joint above seat top", ref: 0.615, model: (q) => q.joint("arm-l")[1]! - q.seat, source: "0.818 H - 0.52 H + hip 0.085 above seat" },
  { name: "shoulder joint spacing", ref: 0.352, model: (q) => q.joint("arm-l")[2]! - q.joint("arm-r")[2]!, source: "biacromial 0.40 - 2 x 0.024" },
  { name: "bideltoid breadth in shirt", ref: 0.48, model: (q) => q.box(/^(sleeve-.|shirt)$/).max[2]! - q.box(/^(sleeve-.|shirt)$/).min[2]!, source: "DIN 33402 m P50 0.477" },
  { name: "upper arm (shoulder-elbow)", ref: 0.335, model: (q) => Math.hypot(...q.joint("forearm-l").map((x, i) => x - q.joint("arm-l")[i]!)), source: "Winter 0.188 H" },
  { name: "forearm (elbow-wrist)", ref: 0.258, model: (q) => Math.hypot(...q.joint("hand-l").map((x, i) => x - q.joint("forearm-l")[i]!)), source: "Winter 0.145 H" },
  { name: "hand length (0.108 H), fingers straightened", ref: 0.192, model: () => 0.096 + 0.045 + 0.027 + 0.022, source: "brief; DIN 33402; knuckle at 0.096 plus the three phalanges of fingerPath (curled on the keys in the model)" },
  { name: "hip breadth seated in trousers", ref: 0.41, model: (q) => q.box(/^(trousers-seat|thigh-.)$/).max[2]! - q.box(/^(trousers-seat|thigh-.)$/).min[2]!, source: "DIN 33402 m P50 0.375 + trousers" },
  { name: "thigh clearance (seat top to thigh top)", ref: 0.16, model: (q) => q.box(/^thigh-l$/).max[1]! - q.seat, source: "DIN 33402 m P50 0.155 + cloth" },
  { name: "knee height (floor to knee top, shoe)", ref: 0.585, model: (q) => q.box(/^shank-l$/).max[1]!, source: "DIN 33402 m P50 0.555 barefoot + 0.023 sole + cloth" },
  { name: "knee joint height (0.285 H + sole)", ref: 0.53, model: (q) => q.joint("shank-l")[1]!, source: "Winter 0.285 H + 0.023 sole" },
  { name: "buttock-knee length", ref: 0.62, model: (q) => q.box(/^(trousers-seat|thigh-l)$/).max[0]! - q.box(/^(trousers-seat|thigh-l)$/).min[0]!, source: "DIN 33402 m P50 0.605 + cloth" },
  { name: "calf breadth in trousers", ref: 0.12, model: (q) => { const b = q.box(/^shank-l$/); return b.max[2]! - b.min[2]!; }, source: "ANSUR II m calf breadth 0.11 + 2 x 5 mm cloth" },
  { name: "shoe length (foot 0.152 H + 0.02)", ref: 0.29, model: (q) => q.box(/^shoe(-toe|-heel|-sole|-toe-sole)?-l$/).max[0]! - q.box(/^shoe(-toe|-heel|-sole|-toe-sole)?-l$/).min[0]!, source: "brief; EU 43" },
  { name: "ankle joint height in shoe", ref: 0.092, model: (q) => q.joint("foot-l")[1]!, source: "Winter 0.039 H + 0.023 sole" },
];
const READER: Ref[] = [
  { name: "sitting height (seat to vertex)", ref: 0.875, model: (q) => q.box(/^(hair|head|bun)$/).max[1]! - q.seat, source: "DIN 33402-2 f P50 0.857 at 1.65 m, scaled to 1.68 m" },
  { name: "head height chin to vertex (H/7.5)", ref: 0.224, model: (q) => q.along(/^(head|hair)$/, "head"), source: "brief; ISO 7250 f P50" },
  { name: "head breadth (hair incl.)", ref: 0.155, model: (q) => q.box(/^(head|hair)$/).max[2]! - q.box(/^(head|hair)$/).min[2]!, source: "ISO 7250 f P50 0.147 + hair" },
  { name: "shoulder joint above seat", ref: 0.59, model: (q) => q.joint("arm-l")[1]! - q.seat, source: "0.818 H - 0.52 H + hip 0.09 above seat" },
  { name: "shoulder joint spacing", ref: 0.33, model: (q) => q.joint("arm-l")[2]! - q.joint("arm-r")[2]!, source: "biacromial 0.365 - 2 x 0.02" },
  { name: "bideltoid breadth in sweater", ref: 0.45, model: (q) => q.box(/^(sleeve-.|sweater)$/).max[2]! - q.box(/^(sleeve-.|sweater)$/).min[2]!, source: "DIN 33402 f P50 0.43 + knit" },
  { name: "upper arm (shoulder-elbow)", ref: 0.312, model: (q) => Math.hypot(...q.joint("forearm-l").map((x, i) => x - q.joint("arm-l")[i]!)), source: "Winter 0.186 H" },
  { name: "forearm (elbow-wrist)", ref: 0.245, model: (q) => Math.hypot(...q.joint("hand-l").map((x, i) => x - q.joint("forearm-l")[i]!)), source: "Winter 0.146 H" },
  { name: "hand length (0.108 H)", ref: 0.181, model: (q) => q.along(/^(palm|finger-2)-l$/, "hand-l") - 0.006, source: "brief; DIN 33402" },
  { name: "hip breadth seated in jeans", ref: 0.405, model: (q) => q.box(/^(jeans-seat|thigh-.)$/).max[2]! - q.box(/^(jeans-seat|thigh-.)$/).min[2]!, source: "DIN 33402 f P50 0.39 + jeans" },
  { name: "thigh clearance (seat to thigh top)", ref: 0.155, model: (q) => q.box(/^thigh-l$/).max[1]! - 0.43, source: "DIN 33402 f P50 0.15 + cloth, over the raked slats (0.43 under mid thigh)" },
  { name: "knee joint height (0.285 H + sole)", ref: 0.491, model: (q) => q.joint("shank-l")[1]!, source: "Winter 0.285 H + 0.012 sole" },
  { name: "buttock-knee length", ref: 0.595, model: (q) => q.box(/^(jeans-seat|thigh-l)$/).max[0]! - q.box(/^(jeans-seat|thigh-l)$/).min[0]!, source: "DIN 33402 f P50 0.585 + cloth" },
  { name: "calf breadth in jeans", ref: 0.115, model: (q) => { const b = q.box(/^shank-l$/); return b.max[2]! - b.min[2]!; }, source: "ANSUR II f calf breadth 0.107 + 2 x 4 mm denim" },
  { name: "shoe length (foot 0.152 H + 0.02)", ref: 0.265, model: (q) => q.box(/^(shoe|sneaker)[a-z-]*-l$/).max[0]! - q.box(/^(shoe|sneaker)[a-z-]*-l$/).min[0]!, source: "brief; EU 39" },
  { name: "ankle joint height in shoe", ref: 0.078, model: (q) => q.joint("foot-l")[1]!, source: "Winter 0.039 H + 0.012 sole" },
];

m.seat = who === "reader" ? 0.414 : 0.485;
console.log(`# ${slug} 1:${scale} ${args.includes("--rest") ? "rest frames" : `pose ${pose}`}`);
console.log("| quantity | reference | model | deviation | source |\n|---|---|---|---|---|");
for (const r of who === "reader" ? READER : OFFICE) {
  let v = Number.NaN;
  try {
    v = r.model(m);
  } catch {}
  const d = (v / r.ref - 1) * 100;
  console.log(`| ${r.name} | ${r.ref.toFixed(3)} | ${v.toFixed(3)} | ${Number.isFinite(d) ? `${d >= 0 ? "+" : ""}${d.toFixed(1)} %${Math.abs(d) > 3 ? " **" : ""}` : "n/a"} | ${r.source} |`);
}
