/** 🏃️ Generator of the jogger's `run-cycle`: 24 key poses 1/32 s apart whose centre of mass follows one physically consistent run, written into the asset file as text.
 *
 * `bun <this file> <asset json> [--write]`: solves, prints the key table and the remaining errors; `--write` replaces the run poses and the `run-cycle` clip in the file (everything else stays byte for byte).
 *
 * Model of one step (the right step mirrors the left one), stance {@link STANCE_S}, flight {@link FLIGHT_S}:
 * - vertical: ground force `W·(F0 + P·sin(π·s) + Q·sin(2π·s))` over the stance fraction `s`; `P` makes the impulse of one step carry the weight, `Q` lifts the toe-off {@link RISE_M} above the landing; the flight is the parabola between them.
 * - horizontal: ground force `−W·B·sin(2π·s)`: braking in the first half of the stance, the same impulse of propulsion in the second, so the step is periodic at {@link SPEED_MS}; the foot lands {@link LANDING_X_M} ahead of the centre of mass.
 * - stance keys: thigh, shank and foot of the stance leg are solved so that the centre of mass sits on the target above the planted ball and the sole has the scheduled pitch (flat while the heel span lasts, so the heel and the ball plant agree).
 * - trunk: planted root motion carries the pelvis through the flight at one speed, so the lean of the trunk (a few degrees about its design value) is solved in every key to keep the centre of mass at the scheduled distance ahead of the pelvis; that distance changes at the same rate at toe-off, in the flight and at landing, which makes the centre of mass keep its speed in the air.
 * - flight keys: the pose roots put the centre of mass on the parabola; four joint values (trailing thigh and shank after toe-off, landing thigh and shank before the landing) are solved with the product's own clip sampling so that the root velocity is continuous at toe-off and landing: the foot leaves and meets the ground with zero ground speed.
 * The toes stay bent for four keys after toe-off ({@link TOE_AFTER_DEG}) so that the tip of the trailing shoe clears the ground.
 * Arms, trunk and the swinging leg are the design tables below (the six poses per side of the first authoring), splined over the cycle. Stance and flight are whole multiples of the 1/240 s step of planted root motion, so both flights inherit the pelvis speed in the same way.
 *
 * @see ../fix-jogger-wheelchair.md — why and what was measured
 * @see ./scan.ts — the scan of the result on the 1/240 s grid */
import { readFileSync, writeFileSync } from "node:fs";
import { sampleOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { apply, solveChain } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const G = 9.80665;
const STANCE_S = 0.25;
const FLIGHT_S = 0.125;
const STEP_S = STANCE_S + FLIGHT_S;
const NODE_S = STEP_S / 6;
const EDGE_S = STEP_S / 12;
const TIMES = Array.from({ length: 12 }, (_, key) => key * EDGE_S);
const KEYS = TIMES.length;
const TOE_OFF = TIMES.indexOf(STANCE_S);
const F0 = Number(process.env.F0 ?? 0.6);
const RISE_M = Number(process.env.RISE_M ?? 0.01);
const SPEED_MS = 2.3377;
const LANDING_COM_M = Number(process.env.LANDING_COM_M ?? 0.97);
const BRAKE = Number(process.env.BRAKE ?? 0.2);
const LANDING_X_M = Number(process.env.LANDING_X_M ?? -0.25);
const PITCH: readonly (readonly [number, number])[] = [[0, 0], [0.09375, 0], [0.125, -5], [0.15625, -12], [0.1875, -21], [0.21875, -31], [STANCE_S, -40]];
const HEEL_END_S = 0.0625;
const PITCH_AFTER_DEG = Number(process.env.PITCH_AFTER_DEG ?? -48);
const TOE_AFTER_DEG = [34, 28, 18, 9] as const;
const SHOE_CONTACT = [0.2, 0.24, 0.26, 0.26, 0.26, 0.2, 0.12, 0.09, 0.06] as const;
const CLIP = "run-cycle";

const DESIGN: Readonly<Record<string, readonly number[]>> = {
  thigh: [29.06, 28.9, 20.53, 9.62, -10.79, -13.25, 1.35, 20.93, 45.99, 63.87, 60.62, 44.89],
  shank: [-34.18, -55.35, -56.47947, -57.18, -31.29, -40.67, -74.44, -104.65, -120.42, -112.01, -85.03, -52.57],
  foot: [5.12, 26.45, 35.95, 24.11, 16.09, 15.37, 13.54, 11.11, 8.59, 6.46, 5.24, 5.12],
  "upper-arm": [-16.7, -16.6, -9.5, -0.2, 17.2, 19.3, 6.9, -9.8, -31.1, -46.3, -43.5, -30.2],
  clavicle: [5.01, 4.98, 2.85, 0.06, -5.16, -5.79, -2.07, 2.94, 6, 6, 6, 6],
};
const FIXED: Readonly<Record<string, number>> = { torso: -5.4, chest: -3.6, head: 3, neck: 6, forearm: 85, hand: 5 };

const NAMES: readonly { readonly id: string; readonly en: string; readonly de: string; readonly aboutEn: string; readonly aboutDe: string }[] = [
  { id: "initial-contact", en: "Initial contact, {side}", de: "Erstkontakt {seite}", aboutEn: "The {side} foot lands flat ahead of the centre of mass and with zero ground speed; the other leg trails with a flexed knee.", aboutDe: "Der Fuß {seite} setzt flach vor dem Schwerpunkt und ohne Geschwindigkeit gegenüber dem Boden auf; das andere Bein folgt mit gebeugtem Knie." },
  { id: "weight-acceptance", en: "Weight acceptance, {side}", de: "Lastübernahme {seite}", aboutEn: "The ground force rises above the body weight; the sole stays flat.", aboutDe: "Die Bodenkraft steigt über das Körpergewicht; die Sohle bleibt flach." },
  { id: "loading", en: "Loading, {side}", de: "Belastung {seite}", aboutEn: "The knee flexes while the fall of the body is braked; the other knee swings forward.", aboutDe: "Das Knie beugt sich, während das Sinken des Körpers gebremst wird; das andere Knie schwingt nach vorn." },
  { id: "absorption", en: "Absorption, {side}", de: "Abfedern {seite}", aboutEn: "Largest ground force of about twice the body weight; the heel is about to leave the ground.", aboutDe: "Größte Bodenkraft von etwa dem doppelten Körpergewicht; die Ferse löst sich gleich vom Boden." },
  { id: "midstance", en: "Midstance, {side}", de: "Mittlere Standphase {seite}", aboutEn: "The centre of mass passes the planted foot near its lowest height above a flexed leg.", aboutDe: "Der Schwerpunkt geht nahe seiner tiefsten Lage über den aufgesetzten Fuß; das Standbein ist gebeugt." },
  { id: "drive", en: "Drive, {side}", de: "Streckung {seite}", aboutEn: "Hip and knee start to extend and the heel lifts; the body rises again.", aboutDe: "Hüfte und Knie beginnen sich zu strecken und die Ferse hebt sich; der Körper steigt wieder." },
  { id: "propulsion", en: "Propulsion, {side}", de: "Abdruckphase {seite}", aboutEn: "The heel rises, the leg extends and pushes the body forward and up over the ball of the foot.", aboutDe: "Die Ferse hebt sich, das Bein streckt sich und schiebt den Körper über den Fußballen nach vorn und oben." },
  { id: "push-off", en: "Push-off, {side}", de: "Abstoß {seite}", aboutEn: "The ankle extends over the toes while the ground force falls towards the body weight.", aboutDe: "Das Sprunggelenk streckt sich über die Zehen, während die Bodenkraft zum Körpergewicht hin abfällt." },
  { id: "toe-off", en: "Toe-off, {side}", de: "Abdruck {seite}", aboutEn: "Last instant of ground contact before the flight.", aboutDe: "Letzter Moment des Bodenkontakts vor der Flugphase." },
  { id: "flight-rise", en: "Rising flight after {side} toe-off", de: "Steigflug nach Abdruck {seite}", aboutEn: "Both feet are off the ground and the centre of mass still rises on its parabola.", aboutDe: "Beide Füße sind in der Luft und der Schwerpunkt steigt noch auf seiner Parabel." },
  { id: "flight", en: "Flight after {side} toe-off", de: "Flugphase nach Abdruck {seite}", aboutEn: "Both feet are off the ground: the body flies ballistically for an eighth of a second.", aboutDe: "Beide Füße sind in der Luft: der Körper fliegt eine Achtelsekunde ballistisch." },
  { id: "flight-fall", en: "Falling flight after {side} toe-off", de: "Sinkflug nach Abdruck {seite}", aboutEn: "The leading foot is drawn back until it has no speed over the ground; the centre of mass falls towards the landing.", aboutDe: "Der vordere Fuß wird zurückgezogen, bis er gegenüber dem Boden ruht; der Schwerpunkt sinkt dem Aufsetzen entgegen." },
];
const NAMED = new Set(["initial-contact-l", "midstance-l", "flight-l"]);

type Offset = { angleDeg?: number; abductionDeg?: number };
type Bones = Record<string, Offset>;
type Pose = { id: string; title: { en: string; de: string }; description: { en: string; de: string }; bones: Bones; ground: string; contacts: string[]; thermal: { regions: Record<string, { contact?: { fraction: number; tempC: number } }> }; loads: unknown[]; root?: { y: number }; key?: boolean };
type Clip = { id: string; description: { en: string; de: string }; durationS: number; advanceM?: number; keys: unknown[]; contacts: { anchor: string; fromS: number; toS: number }[] };
type Asset = { rig: { bones: { id: string; joint: { minDeg?: number; maxDeg?: number }; mass?: { kg: number; comX: number; comY: number } }[] }; anchors: { id: string; bone: string; x: number; y: number }[]; poses: Pose[]; clips: Clip[] };

const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
const source = JSON.parse(text).asset as Asset;

//#region 🎯️Target
const P = (Math.PI * (FLIGHT_S + (1 - F0) * STANCE_S)) / (2 * STANCE_S);
const TAKE_OFF_V = RISE_M / -FLIGHT_S + (G * FLIGHT_S) / 2;
const LANDING_V = TAKE_OFF_V - G * FLIGHT_S;
const Q = ((RISE_M - LANDING_V * STANCE_S - G * (((F0 - 1) * STANCE_S ** 2) / 2 + (P * STANCE_S ** 2) / Math.PI)) * 2 * Math.PI) / (G * STANCE_S ** 2);
const force = (t: number): number => F0 + P * Math.sin((Math.PI * t) / STANCE_S) + Q * Math.sin((2 * Math.PI * t) / STANCE_S);
const height = (t: number): number => {
  if (t > STANCE_S) return LANDING_COM_M + RISE_M + TAKE_OFF_V * (t - STANCE_S) - (G * (t - STANCE_S) ** 2) / 2;
  const w = Math.PI / STANCE_S;
  return LANDING_COM_M + LANDING_V * t + G * (((F0 - 1) * t * t) / 2 + (P / w) * (t - Math.sin(w * t) / w) + (Q / (2 * w)) * (t - Math.sin(2 * w * t) / (2 * w)));
};
const FLIGHT_SPEED = SPEED_MS + (BRAKE * G * STANCE_S ** 2) / (2 * Math.PI * STEP_S);
const forwardAt = (t: number): number => {
  const w = (2 * Math.PI) / STANCE_S;
  return LANDING_X_M + FLIGHT_SPEED * t - ((BRAKE * G) / w) * (t - Math.sin(w * t) / w);
};
const pitchAt = (t: number): number => {
  const i = PITCH.findIndex(([at]) => at >= t);
  if (i <= 0) return PITCH[Math.max(i, 0)]![1];
  const [[ta, a], [tb, b]] = [PITCH[i - 1]!, PITCH[i]!];
  return a + ((b - a) * (t - ta)) / (tb - ta);
};
//#endregion 🎯️Target

//#region 🦴️Kinematics
const other = (bone: string): string => (bone.endsWith("-l") ? `${bone.slice(0, -2)}-r` : bone.endsWith("-r") ? `${bone.slice(0, -2)}-l` : bone);
const mirror = (bones: Bones): Bones => Object.fromEntries(Object.entries(bones).map(([id, offset]) => [id, { ...bones[other(id)]!, ...(offset.abductionDeg === undefined ? {} : { abductionDeg: -bones[other(id)]!.abductionDeg! || 0 }) }]));
const frames = (bones: Bones) => solveChain(source.rig as never, bones as never, { x: 0, y: 0, angleDeg: 0 });
const anchorAt = (solved: ReturnType<typeof frames>, id: string): { x: number; y: number } => {
  const anchor = source.anchors.find((candidate) => candidate.id === id)!;
  return apply(solved[anchor.bone]!.affine, anchor.x, anchor.y);
};
function centre(solved: ReturnType<typeof frames>): { x: number; y: number } {
  let [m, x, y] = [0, 0, 0];
  for (const bone of source.rig.bones) {
    if (bone.mass === undefined || !(bone.mass.kg > 0)) continue;
    const f = solved[bone.id]!.frame;
    m += bone.mass.kg;
    x += bone.mass.kg * (f.o[0] + bone.mass.comX * f.u[0] + bone.mass.comY * f.v[0]);
    y += bone.mass.kg * (f.o[1] + bone.mass.comX * f.u[1] + bone.mass.comY * f.v[1]);
  }
  return { x: x / m, y: y / m };
}
const pitchOf = (solved: ReturnType<typeof frames>, side: string): number => {
  const heel = anchorAt(solved, `heel-${side}`);
  const ball = anchorAt(solved, `contact-${side}`);
  return (Math.atan2(ball.y - heel.y, ball.x - heel.x) * 180) / Math.PI;
};

function linear(a: number[][], b: number[]): number[] {
  const n = b.length;
  const m = a.map((row, i) => [...row, b[i]!]);
  for (let col = 0; col < n; col++) {
    let pivot = col;
    for (let row = col + 1; row < n; row++) if (Math.abs(m[row]![col]!) > Math.abs(m[pivot]![col]!)) pivot = row;
    [m[col], m[pivot]] = [m[pivot]!, m[col]!];
    for (let row = 0; row < n; row++) {
      if (row === col) continue;
      const factor = m[row]![col]! / m[col]![col]!;
      for (let k = col; k <= n; k++) m[row]![k]! -= factor * m[col]![k]!;
    }
  }
  return m.map((row, i) => row[n]! / row[i]!);
}

function newton(residual: (z: readonly number[]) => number[], start: readonly number[], step: number, label: string, rounds = 40): number[] {
  let z = [...start];
  let r = residual(z);
  const size = (v: readonly number[]): number => Math.sqrt(v.reduce((sum, value) => sum + value * value, 0));
  for (let round = 0; round < rounds && size(r) > 1e-11; round++) {
    const columns = z.map((_, k) => residual(z.map((value, i) => (i === k ? value + step : value))).map((value, i) => (value - r[i]!) / step));
    const delta = linear(r.map((_, i) => z.map((__, k) => columns[k]![i]!)), r.map((value) => -value));
    let scale = 1;
    for (let attempt = 0; attempt < 12; attempt++, scale /= 2) {
      const candidate = z.map((value, i) => value + scale * delta[i]!);
      const next = residual(candidate);
      if (size(next) < size(r)) {
        z = candidate;
        r = next;
        break;
      }
    }
    if (process.env.TRACE) console.log(`[DEBUG] ${label} round ${round} residual ${size(r).toExponential(3)}`);
  }
  if (size(r) > 1e-7) console.log(`[DEBUG] ${label} did not converge: ${size(r).toExponential(3)}`);
  return z;
}
//#endregion 🦴️Kinematics

//#region 🧱️Keys
const cycle = (name: string, timeS: number): number => {
  const s = DESIGN[name]!;
  const at = (i: number): number => s[((i % 12) + 12) % 12]!;
  const u = timeS / NODE_S;
  const i = Math.floor(u + 1e-9);
  const w = Math.max(0, u - i);
  const [p0, p1, p2, p3] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
  const value = p1 + 0.5 * w * (p2 - p0 + w * (2 * p0 - 5 * p1 + 4 * p2 - p3 + w * (3 * (p1 - p2) + p3 - p0)));
  return name === "clavicle" ? Math.max(-6, Math.min(6, value)) : value;
};
const round = (value: number, digits: number): number => Math.round(value * 10 ** digits) / 10 ** digits || 0;

type Key = { bones: Bones; rootY: number; flat: boolean };

/** 🫁️ The forward offset of the centre of mass from the pelvis the trunk is solved for: a quintic over the stance from `a` to `b` and a line over the flight back to `a`, with the slope of the line and no curvature at both ends, so that the pelvis keeps one speed through the flight as the planted root motion demands. */
let sway: ((t: number) => number) | null = null;
const swayBasis = (t: number): [number, number] => {
  if (t > STANCE_S) return [(t - STANCE_S) / FLIGHT_S, 1 - (t - STANCE_S) / FLIGHT_S];
  const s = t / STANCE_S;
  const [h00, h10, h01, h11] = [1 - 10 * s ** 3 + 15 * s ** 4 - 6 * s ** 5, s - 6 * s ** 3 + 8 * s ** 4 - 3 * s ** 5, 10 * s ** 3 - 15 * s ** 4 + 6 * s ** 5, -4 * s ** 3 + 7 * s ** 4 - 3 * s ** 5];
  const slope = ((h10 + h11) * STANCE_S) / FLIGHT_S;
  return [h00 + slope, h01 - slope];
};

/** 🧩️ The keys of the left step for the four flight unknowns `z`: trailing thigh and shank at the key just after toe-off and landing thigh and shank at the key just before the landing. */
function leftStep(z: readonly number[], exact = true): Key[] {
  const digits = exact ? 12 : 4;
  const keys: Key[] = [];
  const base = (t: number): Bones => ({
    "thigh-l": { angleDeg: cycle("thigh", t) }, "shank-l": { angleDeg: cycle("shank", t) }, "foot-l": { angleDeg: cycle("foot", t) },
    "thigh-r": { angleDeg: cycle("thigh", t + STEP_S) }, "shank-r": { angleDeg: cycle("shank", t + STEP_S) }, "foot-r": { angleDeg: cycle("foot", t + STEP_S) },
    torso: { angleDeg: FIXED.torso! }, chest: { angleDeg: FIXED.chest! }, head: { angleDeg: FIXED.head! },
    "upper-arm-l": { angleDeg: round(cycle("upper-arm", t), 4) }, "forearm-l": { angleDeg: FIXED.forearm! }, "hand-l": { angleDeg: FIXED.hand! },
    "upper-arm-r": { angleDeg: round(cycle("upper-arm", t + STEP_S), 4) }, "forearm-r": { angleDeg: FIXED.forearm! }, "hand-r": { angleDeg: FIXED.hand! },
    "toe-l": { angleDeg: 0 }, "toe-r": { angleDeg: 0 }, neck: { angleDeg: FIXED.neck! },
    "clavicle-l": { abductionDeg: round(cycle("clavicle", t), 4) }, "clavicle-r": { abductionDeg: round(-cycle("clavicle", t + STEP_S), 4) || 0 },
  });
  const set = (bones: Bones, side: string, leg: readonly [number, number, number]): void => {
    bones[`thigh-${side}`] = { angleDeg: round(leg[0], digits) };
    bones[`shank-${side}`] = { angleDeg: round(leg[1], digits) };
    bones[`foot-${side}`] = { angleDeg: round(leg[2], digits) };
  };
  const legOf = (bones: Bones, side: string): [number, number, number] => [bones[`thigh-${side}`]!.angleDeg!, bones[`shank-${side}`]!.angleDeg!, bones[`foot-${side}`]!.angleDeg!];
  const baseLeg = (t: number): [number, number, number] => [cycle("thigh", t), cycle("shank", t), cycle("foot", t)];
  const after = STANCE_S + EDGE_S;
  const before = STEP_S - EDGE_S;
  const trail: [number, number, number] = [z[0]! - cycle("thigh", after), z[1]! - cycle("shank", after), PITCH_AFTER_DEG - z[0]! - z[1]! - cycle("foot", after)];
  const lead: [number, number, number] = [z[2]! - cycle("thigh", before + STEP_S), z[3]! - cycle("shank", before + STEP_S), -(z[2]! + z[3]!) - cycle("foot", before + STEP_S)];
  const fade = (sinceToeOff: number): number => Math.max(0, Math.min(1, 1 - (sinceToeOff - EDGE_S) / 0.2));
  const ramp = (toLanding: number): number => (toLanding > FLIGHT_S - EDGE_S + 1e-9 ? 0 : Math.max(0, Math.min(1, 1 - (toLanding - EDGE_S) / 0.1)));
  const swingRight = (t: number): [number, number, number] => {
    const b = baseLeg(t + STEP_S);
    const [f, r] = [t < STANCE_S ? fade(t + FLIGHT_S) : 0, t > STANCE_S ? ramp(STEP_S - t) : 0];
    return [b[0] + f * trail[0] + r * lead[0], b[1] + f * trail[1] + r * lead[1], b[2] + f * trail[2] + r * lead[2]];
  };
  for (let key = 0; key <= TOE_OFF; key++) {
    const t = TIMES[key]!;
    const bones = base(t);
    set(bones, "r", swingRight(t));
    bones["toe-r"] = { angleDeg: TOE_AFTER_DEG[key + KEYS - TOE_OFF - 1] ?? 0 };
    const target = { x: forwardAt(t), y: height(t), pitch: pitchAt(t) };
    const guess = key === 0 ? baseLeg(0) : legOf(keys[key - 1]!.bones, "l");
    const lean = sway;
    const solved = newton((leg) => {
      const trial = { ...bones, "thigh-l": { angleDeg: leg[0]! }, "shank-l": { angleDeg: leg[1]! }, "foot-l": { angleDeg: target.pitch - leg[0]! - leg[1]! }, ...(lean === null ? {} : { torso: { angleDeg: leg[2]! } }) };
      const f = frames(trial);
      const c = centre(f);
      const ball = anchorAt(f, "contact-l");
      return [c.x - ball.x - target.x, c.y - ball.y - target.y, ...(lean === null ? [] : [c.x - lean(t)])];
    }, lean === null ? [guess[0], guess[1]] : [guess[0], guess[1], key === 0 ? FIXED.torso! : keys[key - 1]!.bones.torso!.angleDeg!], 1e-5, `stance ${key}`);
    const thigh = round(solved[0]!, digits);
    const shank = round(solved[1]!, digits);
    if (lean !== null) bones.torso = { angleDeg: round(solved[2]!, digits) };
    set(bones, "l", [thigh, shank, target.pitch - thigh - shank]);
    bones["toe-l"] = { angleDeg: round(-target.pitch, 4) };
    keys.push({ bones, rootY: -anchorAt(frames(bones), "contact-l").y, flat: target.pitch === 0 });
  }
  for (let key = TOE_OFF + 1; key < KEYS; key++) {
    const t = TIMES[key]!;
    const bones = base(t);
    set(bones, "r", swingRight(t));
    const b = baseLeg(t);
    const f = fade(t - STANCE_S);
    set(bones, "l", [b[0] + f * trail[0], b[1] + f * trail[1], b[2] + f * trail[2]]);
    bones["toe-l"] = { angleDeg: TOE_AFTER_DEG[key - TOE_OFF - 1]! };
    const lean = sway;
    if (lean !== null) bones.torso = { angleDeg: round(newton((angle) => [centre(frames({ ...bones, torso: { angleDeg: angle[0]! } })).x - lean(t)], [keys[key - 1]!.bones.torso!.angleDeg!], 1e-5, `flight ${key}`)[0]!, digits) };
    keys.push({ bones, rootY: height(t) - centre(frames(bones)).y, flat: false });
  }
  return keys;
}

const poseId = (key: number): string => `${NAMES[key % KEYS]!.id}-${key < KEYS ? "l" : "r"}`;
const OLD = ["touch-down", "release", "lift-off", "reach", "initial-contact", "weight-acceptance", "loading", "absorption", "midstance", "drive", "propulsion", "push-off", "toe-off", "flight-rise", "flight", "flight-fall"].flatMap((id) => [`${id}-l`, `${id}-r`]);
const thermalOf = (id: string): Pose["thermal"] => structuredClone(source.poses.find((pose) => pose.id === id)!.thermal);

function posesOf(keys: readonly Key[]): Pose[] {
  const fill = (template: string, left: boolean): string => template.replaceAll("{side}", left ? "left" : "right").replaceAll("{seite}", left ? "links" : "rechts");
  return [...keys.map((key, index) => ({ key, index, left: true })), ...keys.map((key, index) => ({ key: { ...key, bones: mirror(key.bones) }, index, left: false }))].map(({ key, index, left }) => {
    const side = left ? "l" : "r";
    const name = NAMES[index]!;
    const id = `${name.id}-${side}`;
    const thermal = thermalOf(index <= TOE_OFF ? "midstance-l" : "flight-l");
    if (index <= TOE_OFF) thermal.regions.shoes!.contact = { fraction: SHOE_CONTACT[index]!, tempC: thermal.regions.shoes!.contact?.tempC ?? 22 };
    const pose: Pose = {
      id,
      title: { en: fill(name.en, left), de: fill(name.de, left) },
      description: { en: fill(name.aboutEn, left), de: fill(name.aboutDe, left) },
      bones: key.bones,
      ground: "auto",
      contacts: index > TOE_OFF ? [] : key.flat ? [`heel-${side}`, `contact-${side}`] : [`contact-${side}`],
      thermal,
      loads: [],
      root: { y: key.rootY },
    };
    return NAMED.has(id) ? pose : { ...pose, key: true };
  });
}

function build(keys: readonly Key[]): Asset {
  const run = posesOf(keys);
  const ids = new Set(run.map((pose) => pose.id));
  const old = new Set(OLD);
  const clips = source.clips.map((clip) => {
    if (clip.id !== CLIP) return clip;
    const half = (side: string, from: number) => [{ anchor: `heel-${side}`, fromS: from, toS: round(from + HEEL_END_S, 6) }, { anchor: `contact-${side}`, fromS: from, toS: round(from + STANCE_S, 6) }];
    return {
      ...clip,
      description: {
        en: `Jogging at about ${SPEED_MS.toFixed(2)} m/s with a step rate of ${(1 / STEP_S).toFixed(1)} Hz: each foot is on the ground for a third of the cycle and the body flies for two phases of ${FLIGHT_S} s per cycle. The centre of mass follows a ground force of up to about twice the body weight in stance and a parabola in flight; a foot meets and leaves the ground without speed over it and is planted while in contact.`,
        de: `Joggen mit etwa ${SPEED_MS.toFixed(2).replace(".", ",")} m/s bei ${(1 / STEP_S).toFixed(1).replace(".", ",")} Schritten pro Sekunde: jeder Fuß hat ein Drittel des Zyklus Bodenkontakt, und der Körper fliegt in zwei Phasen von ${`${FLIGHT_S}`.replace(".", ",")} s pro Zyklus. Der Schwerpunkt folgt in der Standphase einer Bodenkraft bis etwa zum doppelten Körpergewicht und im Flug einer Parabel; ein Fuß trifft und verlässt den Boden ohne Geschwindigkeit ihm gegenüber und gleitet im Kontakt nicht.`,
      },
      durationS: 2 * STEP_S,
      keys: Array.from({ length: 2 * KEYS }, (_, key) => ({ t: round(TIMES[key % KEYS]! + (key < KEYS ? 0 : STEP_S), 6), pose: poseId(key), ease: "smooth" })),
      phases: (clip as never as { phases: { id: string; fromS: number; toS: number }[] }).phases.map((phase, index) => ({ ...phase, fromS: [0, STANCE_S, STEP_S, STEP_S + STANCE_S][index]!, toS: [STANCE_S, STEP_S, STEP_S + STANCE_S, 2 * STEP_S][index]! })),
      events: (clip as never as { events: { t: number }[] }).events.map((event) => ({ ...event, t: event.t < 0.1 ? 0 : event.t < 0.3 ? STANCE_S : event.t < 0.5 ? STEP_S : STEP_S + STANCE_S })),
      contacts: [...half("l", 0), ...half("r", STEP_S)],
    };
  });
  return { ...source, poses: [...run, ...source.poses.filter((pose) => !ids.has(pose.id) && !old.has(pose.id))], clips };
}
//#endregion 🧱️Keys

//#region 🔧️Solve
const H = 2e-3;
function measure(asset: Asset) {
  const at = (timeS: number) => {
    const sample = sampleOf(asset as never, { kind: "clip", clip: CLIP, timeS } as never);
    const solved = solveChain(asset.rig as never, sample.offsets, sample.root);
    return { root: sample.root, com: centre(solved) };
  };
  const jump = (timeS: number, axis: "x" | "y"): number => {
    const [a2, a1, b0, c1, c2] = [-2, -1, 0, 1, 2].map((k) => at(timeS + k * H).root[axis]) as [number, number, number, number, number];
    return (-3 * b0 + 4 * c1 - c2) / (2 * H) - (3 * b0 - 4 * a1 + a2) / (2 * H);
  };
  return { at, jump };
}

function residual(z: readonly number[], exact = true): number[] {
  const { at, jump } = measure(build(leftStep(z, exact)));
  const start = at(STANCE_S).com.x;
  const drift = (timeS: number): number => (at(timeS).com.x - start - FLIGHT_SPEED * (timeS - STANCE_S)) / NODE_S;
  return [jump(STANCE_S, "y"), drift(STEP_S), jump(STEP_S, "x"), jump(STEP_S, "y")];
}

console.log(`target: P ${P.toFixed(4)} Q ${Q.toFixed(4)} peak ${Math.max(...Array.from({ length: 241 }, (_, i) => force((i * STANCE_S) / 240))).toFixed(3)} W, landing ${LANDING_V.toFixed(4)} m/s, take-off ${TAKE_OFF_V.toFixed(4)} m/s, lowest ${Math.min(...Array.from({ length: 241 }, (_, i) => height((i * STANCE_S) / 240))).toFixed(4)} m, landing offset ${forwardAt(0).toFixed(4)} m, toe-off offset ${forwardAt(STANCE_S).toFixed(4)} m, flight speed ${FLIGHT_SPEED.toFixed(4)} m/s`);
const SEED = [cycle("thigh", STANCE_S + EDGE_S), cycle("shank", STANCE_S + EDGE_S), cycle("thigh", 2 * STEP_S - EDGE_S), cycle("shank", 2 * STEP_S - EDGE_S)];
const plain = leftStep(SEED).map((key) => centre(frames(key.bones)).x);
const normal = [[0, 0], [0, 0]];
const right = [0, 0];
plain.forEach((value, key) => {
  const basis = swayBasis(TIMES[key]!);
  for (const i of [0, 1]) {
    right[i]! += basis[i]! * value;
    for (const j of [0, 1]) normal[i]![j]! += basis[i]! * basis[j]!;
  }
});
const [swayA, swayB] = linear(normal, right) as [number, number];
sway = (t) => {
  const basis = swayBasis(t);
  return swayA * basis[0] + swayB * basis[1];
};
console.log(`centre of mass ahead of the pelvis: ${swayA.toFixed(4)} m at landing, ${swayB.toFixed(4)} m at toe-off; unfitted ${plain.map((value) => value.toFixed(4)).join(" ")}`);
const seed = leftStep(SEED);
const leg = (key: Key, side: string): [number, number] => [key.bones[`thigh-${side}`]!.angleDeg!, key.bones[`shank-${side}`]!.angleDeg!];
const [first, second, last, late] = [leg(seed[0]!, "l"), leg(seed[1]!, "l"), leg(seed[TOE_OFF]!, "l"), leg(seed[TOE_OFF - 1]!, "l")];
const start = [2 * last[0] - late[0], 2 * last[1] - late[1], 2 * first[0] - second[0], 2 * first[1] - second[1]];
const solved = newton((z) => residual(z), start, 1e-4, "flight");
const keys = leftStep(solved, false);
const asset = build(keys);
const run = asset.clips.find((clip) => clip.id === CLIP)!;
const x = (timeS: number): number => sampleOf(asset as never, { kind: "clip", clip: CLIP, timeS } as never).root.x;
run.advanceM = round(x(run.durationS - 1e-9) - x(0), 8);
console.log("flight unknowns", solved.map((value) => value.toFixed(4)).join(" "), "| residual after rounding", residual(solved, false).map((value) => value.toExponential(2)).join(" "));
console.log("key  thigh-l  shank-l   foot-l  toe-l | thigh-r  shank-r   foot-r | root y   | pitch l/r | torso");
keys.forEach((key, index) => {
  const f = frames(key.bones);
  const n = (bone: string): string => (key.bones[bone]!.angleDeg ?? 0).toFixed(2).padStart(8);
  console.log(`${`${index}`.padStart(3)} ${n("thigh-l")} ${n("shank-l")} ${n("foot-l")} ${n("toe-l")} | ${n("thigh-r")} ${n("shank-r")} ${n("foot-r")} | ${key.rootY.toFixed(4)} | ${pitchOf(f, "l").toFixed(3)} ${pitchOf(f, "r").toFixed(3)} | ${n("torso")}`);
});
for (const pose of asset.poses.slice(0, 2 * KEYS)) {
  for (const [id, offset] of Object.entries(pose.bones)) {
    const joint = source.rig.bones.find((bone) => bone.id === id)!.joint;
    if (offset.angleDeg !== undefined && (offset.angleDeg < (joint.minDeg ?? -Infinity) || offset.angleDeg > (joint.maxDeg ?? Infinity))) console.log(`LIMIT ${pose.id} ${id} ${offset.angleDeg} outside ${joint.minDeg}..${joint.maxDeg}`);
  }
}
console.log(`summary: largest foot ${Math.max(...keys.map((key) => key.bones["foot-l"]!.angleDeg!)).toFixed(2)}, landing leg ${[KEYS - 2, KEYS - 1, 0, 1, 2].map((k) => (k > TOE_OFF ? leg(keys[k]!, "r") : leg(keys[k]!, "l")).map((v) => v.toFixed(1)).join("/")).join(" > ")}, toe-off leg ${[TOE_OFF - 2, TOE_OFF - 1, TOE_OFF, TOE_OFF + 1, TOE_OFF + 2].map((k) => leg(keys[k]!, "l").map((v) => v.toFixed(1)).join("/")).join(" > ")}, torso ${Math.min(...keys.map((key) => key.bones.torso!.angleDeg!)).toFixed(2)}..${Math.max(...keys.map((key) => key.bones.torso!.angleDeg!)).toFixed(2)}`);
console.log("advanceM", run.advanceM, "speed", (run.advanceM / run.durationS).toFixed(4), "m/s");
//#endregion 🔧️Solve

//#region 📝️Write
function arrayRange(key: string): [number, number] {
  const head = text.indexOf(`\n    "${key}": [`);
  if (head < 0) throw new Error(`no ${key}`);
  const open = text.indexOf("[", head);
  let depth = 0;
  let quoted = false;
  for (let i = open; i < text.length; i++) {
    const ch = text[i]!;
    if (quoted) {
      if (ch === "\\") i++;
      else if (ch === '"') quoted = false;
    } else if (ch === '"') quoted = true;
    else if (ch === "[" || ch === "{") depth++;
    else if ((ch === "]" || ch === "}") && --depth === 0) return [open, i + 1];
  }
  throw new Error(`unclosed ${key}`);
}

function elements(from: number, to: number): string[] {
  const out: string[] = [];
  let depth = 0;
  let quoted = false;
  let begin = -1;
  for (let i = from + 1; i < to - 1; i++) {
    const ch = text[i]!;
    if (quoted) {
      if (ch === "\\") i++;
      else if (ch === '"') quoted = false;
    } else if (ch === '"') quoted = true;
    else if (ch === "{" || ch === "[") {
      if (depth++ === 0) begin = i;
    } else if ((ch === "}" || ch === "]") && --depth === 0) out.push(text.slice(begin, i + 1));
  }
  return out;
}

if (process.argv.includes("--write")) {
  const indent = (value: unknown): string => JSON.stringify(value, null, 2).split("\n").map((line, i) => (i === 0 ? line : `      ${line}`)).join("\n");
  const replace = (key: string, own: (id: string) => boolean, fresh: readonly { id: string }[], front: boolean, body: string): string => {
    const [from, to] = arrayRange(key);
    const kept = elements(from, to).filter((element) => !own(JSON.parse(element).id));
    const made = fresh.map(indent);
    const list = front ? [...made, ...kept] : kept.map((element) => element);
    return `${body.slice(0, from)}[\n      ${list.join(",\n      ")}\n    ]${body.slice(to)}`;
  };
  const runIds = new Set(asset.poses.slice(0, 2 * KEYS).map((pose) => pose.id));
  const old = new Set(OLD);
  const [clipFrom, clipTo] = arrayRange("clips");
  const clipTexts = elements(clipFrom, clipTo).map((element) => (JSON.parse(element).id === CLIP ? indent(run) : element));
  let next = `${text.slice(0, clipFrom)}[\n      ${clipTexts.join(",\n      ")}\n    ]${text.slice(clipTo)}`;
  next = replace("poses", (id) => runIds.has(id) || old.has(id), asset.poses.slice(0, 2 * KEYS), true, next);
  const written = JSON.parse(next).asset as Asset;
  if (JSON.stringify(written.poses) !== JSON.stringify(asset.poses) || JSON.stringify(written.clips) !== JSON.stringify(asset.clips)) throw new Error("the text edit does not equal the solved asset; nothing written");
  for (const key of Object.keys(source)) if (key !== "poses" && key !== "clips" && JSON.stringify((written as never)[key]) !== JSON.stringify((source as never)[key])) throw new Error(`the text edit changed "${key}"; nothing written`);
  writeFileSync(file, next);
  console.log("written", file);
}
//#endregion 📝️Write
