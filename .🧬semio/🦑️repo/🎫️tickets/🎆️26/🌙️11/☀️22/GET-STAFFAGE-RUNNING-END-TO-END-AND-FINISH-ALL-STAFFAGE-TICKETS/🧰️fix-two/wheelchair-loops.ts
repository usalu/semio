/** ♿️ Generator of the wheelchair user's `push-cycle` and `ramp-roll` as closed loops of steady rolling, written into the asset file as text.
 *
 * `bun <this file> <asset json> [--write]`: prints the loops; `--write` replaces the two clips in the file (everything else stays byte for byte). Run it on the file of the first authoring (one-shot clips of four and two strokes); a file that already holds the loops is refused.
 *
 * Both clips were strokes of a steady roll authored as one-shots, so the spline started every joint from rest and the trunk needed 27 m/s² in the first key interval. A loop needs a closing key that equals the first key up to whole turns of the wheels (rule `loop-closing`). The rear wheel (radius 0.305 m) and the castor (radius 0.0678 m) roll on the same ground, 4.4985 castor turns per wheel turn, so the shortest loop that closes both is two wheel turns and nine castor turns (the castor then rolls 0.03 % further than the wheel, 1.2 mm in 3.83 m):
 * - `push-cycle`: three strokes of 1 s with 240° of wheel per stroke (the stroke the clip already repeated from its second stroke on; the first stroke began from another grip and is dropped). The release of the rims is eased over six keys.
 * - `ramp-roll`: four strokes with 180° of wheel per stroke at the authored wheel speed of 192°/s, so a stroke lasts 0.9375 s: the push (hands on the rims) keeps its keys and its timing, the recovery is sampled from the authored spline 8 % faster.
 * Tracks, phases and events repeat per stroke; `advanceM` is the rolled distance of the rear wheel.
 *
 * @see ../fix-jogger-wheelchair.md — why and what was measured */
import { readFileSync, writeFileSync } from "node:fs";
import { sampleClip } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";

const WHEEL_RADIUS_M = 0.305;
const STEP_S = 1 / 32;
const PUSH_KEYS = 6;
const WHEELS = ["rear-wheel", "castor"] as const;

type Offset = { angleDeg?: number; abductionDeg?: number; twistDeg?: number };
type Key = { t: number; pose: string; ease: string; bones: Record<string, Offset> };
type Text = { en: string; de: string };
type Clip = { id: string; title: Text; description: Text; durationS: number; fps: number; loop: boolean; rootMotion: string; advanceM: number; keys: Key[]; contacts: { anchor: string; fromS: number; toS: number }[]; phases: { id: string; fromS: number; toS: number; title: Text }[]; events: { t: number; kind: string; title: Text }[]; tracks: { target: string; ease: string; keys: { t: number; v: number }[] }[] };

const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
const asset = JSON.parse(text).asset as { clips: Clip[] };
const round = (value: number, digits: number): number => Math.round(value * 10 ** digits) / 10 ** digits || 0;
const clipOf = (id: string): Clip => {
  const clip = asset.clips.find((candidate) => candidate.id === id)!;
  if (clip.loop) throw new Error(`"${id}" already loops: run this on the file of the first authoring`);
  return clip;
};

/** 🔁️ A loop of `strokes` strokes of `keysPerStroke` keys: the body keys of one stroke repeated, both wheels turning at a constant rate, closed by the first key two wheel turns and nine castor turns on. */
function loopOf(clip: Clip, stroke: readonly Record<string, Offset>[], strokes: number): Key[] {
  const count = stroke.length * strokes;
  const key = (index: number): Key => ({
    t: round(index * STEP_S, 5),
    pose: clip.keys[0]!.pose,
    ease: "smooth",
    bones: { ...stroke[index % stroke.length]!, "rear-wheel": { angleDeg: round((-720 * index) / count, 4) }, castor: { angleDeg: round((-3240 * index) / count, 4) } },
  });
  return Array.from({ length: count + 1 }, (_, index) => key(index));
}

const push = clipOf("push-cycle");
const RELEASE = [6, 12] as const;
const ARMS = ["arm-r", "forearm-r", "hand-r", "arm-l", "forearm-l", "hand-l"] as const;

/** 🤲️ The stroke with the release of the rims eased: the first authoring switched from the gripping to the swinging arm between two keys (the hand turned 66° in 1/32 s, an inertia of 0.59 body weights); the arm keys between the release and the sixth key after it now blend from the one to the other by a smoothstep. */
function eased(stroke: readonly Record<string, Offset>[]): Record<string, Offset>[] {
  const [from, to] = RELEASE;
  return stroke.map((bones, index) => {
    if (index <= from || index >= to) return bones;
    const u = (index - from) / (to - from);
    const w = u * u * (3 - 2 * u);
    const blended = Object.fromEntries(ARMS.map((id) => [id, Object.fromEntries((Object.keys(bones[id]!) as (keyof Offset)[]).map((field) => [field, round(stroke[from]![id]![field]! + (stroke[to]![id]![field]! - stroke[from]![id]![field]!) * w, 3)]))]));
    return { ...bones, ...blended };
  });
}
const pushStroke = eased(push.keys.slice(32, 64).map((key) => key.bones));
const pushLoop: Clip = {
  ...push,
  description: {
    en: "Steady rolling at 1.28 m/s as a loop of three push strokes: the gloved hands grip the rims behind the top, push through a 45 degree arc with the trunk leaning forward, release and swing back over the rims. The rear wheels turn 240 degrees and the castors three full turns per stroke, so the ground advances 1.278 m per stroke (treadmill frame, the chair stays in place); after three strokes the wheels have made two whole turns and the castors nine, and the loop closes.",
    de: "Gleichmäßiges Rollen mit 1,28 m/s als Schleife aus drei Antriebsschüben: Die behandschuhten Hände greifen die Reifen hinter dem Scheitel, schieben durch einen 45-Grad-Bogen mit nach vorn geneigtem Rumpf, lösen und schwingen über den Reifen zurück. Die Hinterräder drehen sich pro Schub um 240 Grad und die Lenkrollen um drei volle Umdrehungen, der Boden läuft also 1,278 m pro Schub vorbei (Laufbandsystem, der Stuhl bleibt stehen); nach drei Schüben haben die Räder zwei ganze und die Lenkrollen neun Umdrehungen gemacht, und die Schleife schließt sich.",
  },
  durationS: 3,
  loop: true,
  advanceM: round(4 * Math.PI * WHEEL_RADIUS_M, 4),
  keys: loopOf(push, pushStroke, 3),
  contacts: push.contacts.map((span) => ({ ...span, toS: 3 })),
  phases: push.phases.slice(0, 6),
  events: push.events.slice(0, 6),
  tracks: [
    { target: "activityMet", ease: "smooth", keys: [0, 1, 2].flatMap((stroke) => [{ t: stroke + 0.1, v: 3 }, { t: stroke + 0.6, v: 2.4 }]) },
    { target: "breathing", ease: "step", keys: [{ t: 0, v: 2 }, { t: 3, v: 2 }] },
  ],
};

const ramp = clipOf("ramp-roll");
const RAMP_KEYS = 30;
const rampStroke = Array.from({ length: RAMP_KEYS }, (_, index): Record<string, Offset> => {
  if (index <= PUSH_KEYS) return ramp.keys[index]!.bones;
  const timeS = PUSH_KEYS * STEP_S + ((index - PUSH_KEYS) / (RAMP_KEYS - PUSH_KEYS)) * (1 - PUSH_KEYS * STEP_S);
  const sample = sampleClip(asset as never, ramp as never, timeS);
  return Object.fromEntries(Object.entries(ramp.keys[0]!.bones).map(([id, authored]) => {
    const offset = sample.offsets[id]!;
    return [id, Object.fromEntries(Object.keys(authored).map((field) => [field, round(offset[field as "angleDeg"], 3)]))];
  }));
});
const RAMP_STROKE_S = RAMP_KEYS * STEP_S;
const ordinal: readonly Text[] = [{ en: "First", de: "Erster" }, { en: "Second", de: "Zweiter" }, { en: "Third", de: "Dritter" }, { en: "Fourth", de: "Vierter" }];
const rampLoop: Clip = {
  ...ramp,
  description: {
    en: "Steady rolling up a 1:12 ramp at 1.02 m/s as a loop of four push strokes of 0.94 s: the chair is tilted 4.8 degrees about the rear axle, the trunk leans forward up to 30 degrees and the activity rises to 3.2 met; the drawn ground line follows the ramp. The rear wheels turn 180 degrees per stroke, so after four strokes they have made two whole turns and the castors nine, and the loop closes.",
    de: "Gleichmäßiges Rollen mit 1,02 m/s eine Rampe 1:12 hinauf als Schleife aus vier Antriebsschüben von 0,94 s: Der Stuhl ist um die Hinterachse 4,8 Grad geneigt, der Rumpf neigt sich bis zu 30 Grad nach vorn und die Aktivität steigt auf 3,2 met; die gezeichnete Bodenlinie folgt der Rampe. Die Hinterräder drehen sich pro Schub um 180 Grad; nach vier Schüben haben sie zwei ganze und die Lenkrollen neun Umdrehungen gemacht, und die Schleife schließt sich.",
  },
  durationS: 4 * RAMP_STROKE_S,
  loop: true,
  advanceM: round(4 * Math.PI * WHEEL_RADIUS_M, 4),
  keys: loopOf(ramp, rampStroke, 4),
  contacts: ramp.contacts.map((span) => ({ ...span, toS: 4 * RAMP_STROKE_S })),
  phases: ordinal.map((name, index) => ({ id: `${name.en.toLowerCase()}-push`, fromS: index * RAMP_STROKE_S, toS: (index + 1) * RAMP_STROKE_S, title: { en: `${name.en} push on the slope`, de: `${name.de} Schub an der Steigung` } })),
  events: ordinal.map((name, index) => ({ t: index * RAMP_STROKE_S, kind: "load-on", title: { en: `${name.en} grip on the slope`, de: `${name.de} Griff an der Steigung` } })),
  tracks: [
    { target: "activityMet", ease: "smooth", keys: [0, 1, 2, 3].map((stroke) => ({ t: stroke * RAMP_STROKE_S, v: stroke % 2 === 0 ? 3 : 3.2 })) },
    { target: "breathing", ease: "step", keys: [{ t: 0, v: 3 }, { t: 4 * RAMP_STROKE_S, v: 3 }] },
  ],
};

for (const clip of [pushLoop, rampLoop]) {
  const [first, last] = [clip.keys[0]!, clip.keys[clip.keys.length - 1]!];
  const open = Object.keys(first.bones).filter((id) => !WHEELS.includes(id as never) && JSON.stringify(first.bones[id]) !== JSON.stringify(last.bones[id]));
  console.log(`${clip.id}: ${clip.durationS} s, ${clip.keys.length} keys, closing key at ${last.t} s, wheel ${last.bones["rear-wheel"]!.angleDeg}°, castor ${last.bones.castor!.angleDeg}°, advanceM ${clip.advanceM}, body bones that differ between first and closing key: ${open.length}`);
  console.log(`  lumbar of the first stroke: ${clip.keys.slice(0, clip.id === "push-cycle" ? 33 : RAMP_KEYS + 1).map((key) => key.bones.lumbar!.angleDeg).join(" ")}`);
}

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
  const [from, to] = arrayRange("clips");
  const fresh = new Map([[pushLoop.id, pushLoop], [rampLoop.id, rampLoop]]);
  const texts = elements(from, to).map((element) => {
    const made = fresh.get(JSON.parse(element).id);
    return made === undefined ? element : indent(made);
  });
  const next = `${text.slice(0, from)}[\n      ${texts.join(",\n      ")}\n    ]${text.slice(to)}`;
  const written = JSON.parse(next).asset as Record<string, unknown> & { clips: Clip[] };
  const expected = asset.clips.map((clip) => fresh.get(clip.id) ?? clip);
  if (JSON.stringify(written.clips) !== JSON.stringify(expected)) throw new Error("the text edit does not equal the built clips; nothing written");
  for (const key of Object.keys(asset)) if (key !== "clips" && JSON.stringify(written[key]) !== JSON.stringify((asset as never)[key])) throw new Error(`the text edit changed "${key}"; nothing written`);
  writeFileSync(file, next);
  console.log("written", file);
}
