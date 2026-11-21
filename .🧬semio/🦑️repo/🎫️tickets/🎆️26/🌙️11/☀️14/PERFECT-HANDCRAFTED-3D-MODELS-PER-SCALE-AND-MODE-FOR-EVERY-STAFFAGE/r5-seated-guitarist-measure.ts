/** 📐️ Round 5 model-vs-reference table of `humans/seated-guitarist`: rig segment lengths and joint spacings of the candidate document, the measured extents of the 1:50 body solids per bone in their rest frames (head, hand, boot, guitar, stool), each against the anthropometric or catalogue reference and its deviation in percent.
 * Usage: `bun r5-seated-guitarist-measure.ts`
 * @see ./r3-humans-seated-guitarist.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { meshesIn } from "./r3-lib.ts";

type Json = Record<string, any>;
const H = 1.76;
const doc = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-seated-guitarist", "🔣️.json"), "utf8")) as Json;
const asset = doc.asset as Json;
const bone = (id: string): Json => (asset.rig.bones as Json[]).find((b) => b.id === id)!;
const frames = restFrames(asset.rig as never) as unknown as Record<string, Json>;
const body = asset.volumes["50"].body as Json[];
/** 📦️ The extent along the rest axes u, v, w of the bone of the named body solids. */
function extent(ids: RegExp, of: string): number[] {
  const f = frames[of]!.frame ?? frames[of]!;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshesIn(asset, Object.fromEntries(Object.entries(frames).map(([k, v]) => [k, (v as Json).frame ?? v])) as never, body.filter((s) => ids.test(s.id) && s.slender !== true))) {
    for (let i = 0; i < m.world.length; i += 3) {
      const d = [m.world[i]! - f.o[0], m.world[i + 1]! - f.o[1], m.world[i + 2]! - f.o[2]];
      const p = [f.u, f.v, f.w].map((a: number[]) => d[0]! * a[0]! + d[1]! * a[1]! + d[2]! * a[2]!);
      p.forEach((x, k) => { lo[k] = Math.min(lo[k]!, x); hi[k] = Math.max(hi[k]!, x); });
    }
  }
  return hi.map((h, k) => h - lo[k]!);
}
const rows: [string, number, number, string][] = [
  ["thigh (hip-knee joint)", bone("thigh-r").lengthM, 0.245 * H, "Drillis-Contini 0.245 H"],
  ["shank (knee-ankle joint)", bone("shank-r").lengthM, 0.246 * H, "Drillis-Contini 0.246 H"],
  ["upper arm (shoulder-elbow joint)", bone("upper-arm-r").lengthM, 0.186 * H, "Drillis-Contini 0.186 H"],
  ["forearm (elbow-wrist joint)", bone("forearm-r").lengthM, 0.146 * H, "Drillis-Contini 0.146 H"],
  ["hand bone", bone("hand-r").lengthM, 0.108 * H, "Drillis-Contini 0.108 H"],
  ["hip joint to shoulder joint", bone("torso").x + bone("upper-arm-r").x, (0.818 - 0.52) * H, "0.818 H - 0.52 H"],
  ["shoulder joint spacing", 2 * Math.abs(bone("upper-arm-r").z), 0.23 * H - 0.025, "biacromial 0.23 H less 2 x 12.5 mm to the joint centres"],
  ["hip joint spacing", 2 * Math.abs(bone("thigh-r").z), 0.18, "Bell et al. 1990, about 0.17-0.18 m"],
  ["head chin-vertex (head + hair solids, u)", extent(/^(head|hair)$/, "head")[0]!, H / 7.5, "H / 7.5"],
  ["head breadth (w)", extent(/^head$/, "head")[2]!, 0.152, "ANSUR II bitragion/head breadth 152 mm"],
  ["head length (v)", extent(/^head$/, "head")[1]!, 0.197, "ANSUR II glabella-opisthocranion 197 mm"],
  ["hand length (palm + middle finger, u)", extent(/^(palm-r|finger-2-r)$/, "hand-r")[0]!, 0.108 * H, "0.108 H (strumming hand flexed, so shorter)"],
  ["boot length (heel to toe cap)", extent(/^boot(-heel)?-r$/, "foot-r")[0]! + extent(/^boot-toe-r$/, "toe-r")[0]!, 0.152 * H + 0.03, "foot 0.152 H + 30 mm of boot"],
  ["guitar overall length", extent(/^guitar-(body|neck|head)$/, "guitar")[0]!, 1.03, "Martin D-28: 1029 mm"],
  ["guitar lower bout", 2 * 0.1985, 0.397, "Martin D-28: 397 mm"],
  ["guitar body depth at the tail", 0.124, 0.124, "Martin D-28: 124 mm"],
  ["stool seat height", bone("stool").lengthM, 0.61, "24 inch guitar stool"],
];
for (const [name, model, ref, source] of rows) console.log(`| ${name} | ${(ref * 1000).toFixed(0)} | ${(model * 1000).toFixed(0)} | ${(((model - ref) / ref) * 100).toFixed(1)} % | ${source} |`);
