/** 🦶 Round 5 root fix of `humans/elderly-with-cane`: the ankle joint centre stands 84 mm above the ground in the shoe (lateral malleolus 0.039 H = 64 mm barefoot for H 1.65 m, DIN 33402 / Winter 2009, plus the 25 mm heel block and insole), not 70 mm. The foot bone runs from the ankle to the ball contact (158 mm ahead, at the ground) and the toe bone stays level; heel and ball anchors keep their place on the shoe. Everything above the ankle rises by 14 mm and the cane-gait cycle advances by its new planted travel.
 *
 * Usage: `bun r5-elderly-with-cane-foot.ts [--write]`; afterwards `bun r3-elderly-with-cane-roots.ts --write` re-solves the cane arm of every pose and clip key.
 * @see ./r3-humans-elderly-with-cane.md */
import { readFileSync, writeFileSync } from "node:fs";
import { entryOf, type Json } from "./volumes-lib.ts";

const WRITE = process.argv.includes("--write");
const { entry } = entryOf("humans/elderly-with-cane");
const FILE: string = (entry as Json).file ?? (entry as Json).path;
const text = readFileSync(FILE, "utf8");
const asset = JSON.parse(text).asset as Json;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const rad = (d: number): number => (d * Math.PI) / 180;
const ANKLE = 0.084;
const BALL_AHEAD = 0.158;
/** 🚶 The planted travel of one cane-gait cycle with the 14 mm longer leg (measured by `validateAsset`, rule clip-advance). */
const GAIT_ADVANCE = 0.9619;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const old = bone("foot-l");
const oldDeg = -90 + old.angleDeg;
const oldU: [number, number] = [Math.cos(rad(oldDeg)), Math.sin(rad(oldDeg))];
const newDeg = (Math.atan2(-ANKLE, BALL_AHEAD) * 180) / Math.PI;
const newLen = Math.hypot(BALL_AHEAD, ANKLE);
const nu: [number, number] = [Math.cos(rad(newDeg)), Math.sin(rad(newDeg))];
const nv: [number, number] = [-nu[1], nu[0]];
const ov: [number, number] = [-oldU[1], oldU[0]];
for (const side of ["l", "r"]) {
  const f = bone(`foot-${side}`);
  const t = bone(`toe-${side}`);
  console.log(side, "foot", f.angleDeg, f.lengthM, "toe", t.x, t.angleDeg);
  f.angleDeg = r5(90 + newDeg);
  f.lengthM = r5(newLen);
  t.x = r5(newLen);
  t.angleDeg = r5(-newDeg);
  console.log(side, "->", f.angleDeg, f.lengthM, t.x, t.angleDeg);
  for (const id of [`heel-${side}`, `ball-${side}`]) {
    const a = asset.anchors.find((q: Json) => q.id === id);
    const w: [number, number] = [id.startsWith("ball") ? BALL_AHEAD : r5(a.x * oldU[0] + a.y * ov[0]), -ANKLE];
    const before = [a.x, a.y];
    a.x = r5(w[0] * nu[0] + w[1] * nu[1]);
    a.y = r5(w[0] * nv[0] + w[1] * nv[1]);
    console.log(id, before, "->", [a.x, a.y], "world from ankle", w.map(r5));
  }
}
const gait = asset.clips.find((c: Json) => c.id === "cane-gait");
console.log("cane-gait advanceM", gait.advanceM, "->", GAIT_ADVANCE);
gait.advanceM = GAIT_ADVANCE;
function memberRange(source: string, name: string): [number, number] {
  const open = source.indexOf(`\n    "${name}": `) + `\n    "${name}": `.length;
  let depth = 0;
  let quoted = false;
  for (let i = open; i < source.length; i++) {
    const ch = source[i]!;
    if (quoted) { if (ch === "\\") i++; else if (ch === '"') quoted = false; } else if (ch === '"') quoted = true;
    else if (ch === "{" || ch === "[") depth++;
    else if (ch === "}" || ch === "]") { depth--; if (depth === 0) return [open, i + 1]; }
  }
  throw new Error("unbalanced");
}
const dump = (value: Json): string => JSON.stringify(value, null, 2).replace(/\n/g, "\n    ");
let out = text;
const original = JSON.parse(text).asset as Json;
for (const name of ["rig", "anchors", "clips"]) {
  const [a, b] = memberRange(out, name);
  if (out.slice(a, b) !== dump(original[name])) throw new Error(`member ${name} is not in the expected style`);
  out = out.slice(0, a) + dump(asset[name]) + out.slice(b);
}
if (WRITE) writeFileSync(FILE, out);
console.log(WRITE ? `written ${FILE}` : "dry run");
