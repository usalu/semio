/** 💃️ Builds `dancer` from the archive's `jogger` (same athletic 1.82 m, 72 kg body and rig): a contemporary and ballet dancer in rehearsal wear with arabesque, relevé, passé and preparation poses solved through `evaluate` for balance over the supporting foot, a pirouette that turns the whole figure once about the ball of the supporting foot, a looping plié-and-relevé exercise and a breathing loop.
 *  bun build-dancer.ts [--resolve]
 * @see ./texts-dancer.ts — texts, thermal and structural model */
import { existsSync } from "node:fs";
import { add, at, evalOffsets, fitDimensions, frameOf, len, loadDoc, massOf, mul, rd, revolveLimbs, saveDoc, solidifyFlat, solvePose, sub, TICKET, type Any, type V3 } from "./lib.ts";
import { dancerTexts, DANCER_DECISIONS } from "./texts-dancer.ts";
import { applyDecisions } from "./decisions.ts";

const CACHE = `${TICKET}/🧰️scripts/HW-12/dancer-poses.json`;
const resolve = process.argv.includes("--resolve") || !existsSync(CACHE);
const doc = await loadDoc("jogger");
const a = doc.asset;
a.id = "dancer";
const bone = (id: string) => a.rig.bones.find((b: Any) => b.id === id);
bone("pelvis").joint = { ...bone("pelvis").joint, twistMinDeg: -30, twistMaxDeg: 360 };
for (const s of ["l", "r"]) {
  bone(`thigh-${s}`).joint = { ...bone(`thigh-${s}`).joint, minDeg: -50, maxDeg: 140 };
  bone(`foot-${s}`).joint = { ...bone(`foot-${s}`).joint, minDeg: -62 };
}

/** ⚖️ The whole-body centre of mass in the world. */
export function comOf(asset: Any, state: Any): V3 {
  let m = 0, c: V3 = [0, 0, 0];
  for (const b of asset.rig.bones) if (b.mass) { const f = frameOf(state, b.id); c = add(c, mul(at(f, b.mass.comX, b.mass.comY, 0), b.mass.kg)); m += b.mass.kg; }
  return mul(c, 1 / m);
}
const soft = (x: number, lo: number, hi: number): number => (x < lo ? (lo - x) ** 2 : x > hi ? (x - hi) ** 2 : 0);
const ball = (s: Any, side: "l" | "r"): V3 => [s.anchors[`contact-${side}`].at.x, s.anchors[`contact-${side}`].at.y, s.anchors[`contact-${side}`].z];
const heel = (s: Any, side: "l" | "r"): V3 => [s.anchors[`heel-${side}`].at.x, s.anchors[`heel-${side}`].at.y, s.anchors[`heel-${side}`].z];

/** 🦶️ Solves the supporting left leg (and given extra unknowns) so that the ball of the left foot lies under the pelvis axis and the centre of mass over the support, with the foot flat or on demi-pointe. */
function balanceOnLeft(bones: Record<string, Any>, relevé: boolean, extra: readonly (readonly [string, string])[] = [], start?: number[]) {
  const unknowns = [["thigh-l", "angleDeg"], ["thigh-l", "abductionDeg"], ["shank-l", "angleDeg"], ["foot-l", "angleDeg"], ["toe-l", "angleDeg"], ["foot-l", "abductionDeg"], ...extra] as const;
  return solvePose(a, bones, unknowns, [...(start ?? [bones["thigh-l"]?.angleDeg ?? 0, 0, 0, relevé ? -35 : 0, relevé ? 35 : 0]), 0, ...extra.map(() => 0)], (s, x) => {
    const P = frameOf(s, "pelvis");
    const b = ball(s, "l"), h = heel(s, "l");
    const support: V3 = relevé ? b : mul(add(b, h), 0.5);
    const com = comOf(a, s);
    const toeY = s.anchors["contact-l"].at.y;
    const heelUp = h[1] - toeY;
    return 3000 * ((b[0] - P.o[0]) ** 2 + (b[2] - P.o[2]) ** 2) + 200 * ((com[0] - support[0]) ** 2 + (com[2] - support[2]) ** 2) + (relevé ? 30 * (heelUp - 0.075) ** 2 : 30 * heelUp ** 2) + 0.05 * soft(x[2]!, -5, -1) + 1e-3 * (soft(x[3]!, -60, 36) + soft(x[4]!, 0, 68)) + 1e-6 * x.reduce((q, v) => q + v * v, 0);
  }, 6, 1200);
}

let P: Record<string, Any>;
if (resolve) {
  const armsFirst = { "upper-arm-l": { angleDeg: 38, abductionDeg: 12, twistDeg: 25 }, "forearm-l": { angleDeg: 62, abductionDeg: -15 }, "hand-l": { angleDeg: 5 }, "upper-arm-r": { angleDeg: 38, abductionDeg: -12, twistDeg: -25 }, "forearm-r": { angleDeg: 62, abductionDeg: 15 }, "hand-r": { angleDeg: 5 } };
  const armsLow = { "upper-arm-l": { angleDeg: 8, abductionDeg: 6, twistDeg: 20 }, "forearm-l": { angleDeg: 30, abductionDeg: -10 }, "hand-l": { angleDeg: 5 }, "upper-arm-r": { angleDeg: 8, abductionDeg: -6, twistDeg: -20 }, "forearm-r": { angleDeg: 30, abductionDeg: 10 }, "hand-r": { angleDeg: 5 } };
  const armsHigh = { "upper-arm-l": { angleDeg: 150, abductionDeg: 18, twistDeg: 30 }, "forearm-l": { angleDeg: 38, abductionDeg: -20 }, "hand-l": { angleDeg: 5 }, "upper-arm-r": { angleDeg: 150, abductionDeg: -18, twistDeg: -30 }, "forearm-r": { angleDeg: 38, abductionDeg: 20 }, "hand-r": { angleDeg: 5 } };
  const armsArabesque = { "upper-arm-l": { angleDeg: 95, abductionDeg: 8, twistDeg: 10 }, "forearm-l": { angleDeg: 8 }, "hand-l": { angleDeg: -5 }, "upper-arm-r": { angleDeg: 12, abductionDeg: -82, twistDeg: -15 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: -5 } };
  const armsSecond = { "upper-arm-l": { angleDeg: 18, abductionDeg: 78, twistDeg: 15 }, "forearm-l": { angleDeg: 12 }, "hand-l": { angleDeg: 0 }, "upper-arm-r": { angleDeg: 18, abductionDeg: -78, twistDeg: -15 }, "forearm-r": { angleDeg: 12 }, "hand-r": { angleDeg: 0 } };
  const turnout = (deg: number) => ({ "thigh-l": { angleDeg: 0, twistDeg: -deg }, "thigh-r": { angleDeg: 0, twistDeg: deg } });

  const arabesqueBase = { pelvis: { angleDeg: -36 }, torso: { angleDeg: 12 }, chest: { angleDeg: 5 }, neck: { angleDeg: 2 }, head: { angleDeg: 4 }, "thigh-r": { angleDeg: -50, twistDeg: 20 }, "shank-r": { angleDeg: -2 }, "foot-r": { angleDeg: -45 }, "toe-r": { angleDeg: 0 }, ...armsArabesque };
  const arabesque = balanceOnLeft(arabesqueBase, false, [], [36, 0, -2, 0, 0]);
  const passeBase = { "upper-arm-l": { angleDeg: 38, abductionDeg: 12, twistDeg: 25 }, "thigh-r": { angleDeg: 72, abductionDeg: -42, twistDeg: 40 }, "shank-r": { angleDeg: -118 }, "foot-r": { angleDeg: -40 }, ...armsFirst };
  const passe = balanceOnLeft(passeBase, true, [], [2, 0, -1, -35, 35]);
  const prepBase = { "thigh-r": { angleDeg: -22, twistDeg: 25 }, "shank-r": { angleDeg: -22 }, "foot-r": { angleDeg: -5 }, ...{ "upper-arm-l": armsFirst["upper-arm-l"], "forearm-l": armsFirst["forearm-l"], "hand-l": armsFirst["hand-l"], "upper-arm-r": armsSecond["upper-arm-r"], "forearm-r": armsSecond["forearm-r"], "hand-r": armsSecond["hand-r"] } };
  const prep = solvePose(a, prepBase, [["thigh-l", "angleDeg"], ["thigh-l", "abductionDeg"], ["shank-l", "angleDeg"], ["foot-l", "angleDeg"], ["thigh-r", "angleDeg"], ["shank-r", "angleDeg"], ["foot-r", "angleDeg"], ["toe-r", "angleDeg"]], [22, 0, -38, 16, -22, -22, -5, 0], (s, x) => {
    const Pf = frameOf(s, "pelvis"), bl = ball(s, "l"), hl = heel(s, "l"), br = ball(s, "r"), hr = heel(s, "r");
    const com = comOf(a, s);
    return 3000 * ((bl[0] - Pf.o[0]) ** 2 + (bl[2] - Pf.o[2]) ** 2) + 30 * ((hl[1] - bl[1]) ** 2 + (hr[1] - br[1]) ** 2 + (br[1] - bl[1]) ** 2) + 5 * (com[0] - (bl[0] + hl[0] + br[0] + hr[0]) / 4) ** 2 + 2 * (Pf.o[1] - 0.86) ** 2 + 1e-3 * (soft(x[2]!, -150, -1) + soft(x[5]!, -150, -1)) + 1e-6 * x.reduce((q, v) => q + v * v, 0);
  }, 6, 1200);
  const firstFlat = { ...turnout(28), "shank-l": { angleDeg: -1 }, "shank-r": { angleDeg: -1 }, "foot-l": { angleDeg: 1 }, "foot-r": { angleDeg: 1 } };
  const plie = { ...turnout(28), "thigh-l": { angleDeg: 30, twistDeg: -28, abductionDeg: 6 }, "thigh-r": { angleDeg: 30, twistDeg: 28, abductionDeg: -6 }, "shank-l": { angleDeg: -58 }, "shank-r": { angleDeg: -58 }, "foot-l": { angleDeg: 28 }, "foot-r": { angleDeg: 28 } };
  const releve = { ...turnout(28), "shank-l": { angleDeg: 0 }, "shank-r": { angleDeg: 0 }, "foot-l": { angleDeg: -38 }, "foot-r": { angleDeg: -38 }, "toe-l": { angleDeg: 38 }, "toe-r": { angleDeg: 38 } };
  P = { arabesque: arabesque.bones, passe: passe.bones, prep: prep.bones, armsFirst, armsLow, armsHigh, armsSecond, firstFlat, plie, releve };
  for (const [k, r] of Object.entries({ arabesque, passe, prep })) {
    const s = evalOffsets(a, r.bones);
    const c = comOf(a, s), pel = frameOf(s, "pelvis"), b = ball(s, "l");
    console.log(`[DEBUG] ${k}: residual ${r.residual.toExponential(2)}, ball off axis ${(Math.hypot(b[0] - pel.o[0], b[2] - pel.o[2]) * 1000).toFixed(1)} mm, com-ball ${(Math.hypot(c[0] - b[0], c[2] - b[2]) * 1000).toFixed(1)} mm, heel ${(s.anchors["heel-l"].at.y * 1000).toFixed(0)} mm`);
  }
  await Bun.write(CACHE, JSON.stringify(P, null, 1));
} else P = await Bun.file(CACHE).json();

dancerTexts(a, P);
revolveLimbs(a, new Set(["upper-arm-l", "forearm-l", "upper-arm-r", "forearm-r", "thigh-l", "shank-l", "thigh-r", "shank-r"]));
solidifyFlat(a);
applyDecisions(a, DANCER_DECISIONS);
a.dimensions.massKg = massOf(a);
fitDimensions(a);
void len; void sub; void rd;
console.log("[DEBUG] written", await saveDoc(doc));
