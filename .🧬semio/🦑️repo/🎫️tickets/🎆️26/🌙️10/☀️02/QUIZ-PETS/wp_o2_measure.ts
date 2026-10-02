/** 📏️ Ticket tool (work package O2): numbers that the eye cannot judge on a sheet — per species the pupil travel, the clips that play once and do not start or end on rest values, the length of every clip per activity, and, on the real stage, how far a planted foot slides while the species walks.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_measure.ts" <species.json>...
 */
import { readFileSync } from "node:fs";
import { MENAGERIE_SCHEMA, type Actor, type Menagerie, type Species, type Stage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const ENGINE = process.env.WP_O2_ENGINE ? resolve(process.env.WP_O2_ENGINE) : join(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const engine = (path: string): string => pathToFileURL(join(ENGINE, path)).href;
const { clipTicks } = (await import(engine("🔨️modules/🎞️animation/🟦️.ts"))) as typeof import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts");
const { advance, frameOf, openStage } = (await import(engine("🔨️modules/🎪️stage/🟦️.ts"))) as typeof import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts");

const WIDTH = 4000;
const FLOOR = 300;
const HOME = 400;

for (const file of process.argv.slice(2)) {
  const kind = JSON.parse(readFileSync(file, "utf8")) as Species;
  const menagerie: Menagerie = { schema: MENAGERIE_SCHEMA, id: "probe", title: { en: "Probe", de: "Probe" }, species: [kind], bonds: [], casts: [] };
  const hover = kind.locomotion.hover ?? 0;
  console.log(`\n== ${kind.id} · ${kind.name.en} / ${kind.name.de} · thing ${kind.thing.en} / ${kind.thing.de} · ${kind.size.width}×${kind.size.height} · ${kind.locomotion.gait} ${kind.locomotion.speed}px/s${hover > 0 ? ` hover ${hover}` : ""}`);
  for (const eye of kind.face.eyes) {
    const travel = eye.radius - eye.pupil - 0.25;
    console.log(`   eye ${eye.id}: radius ${eye.radius} pupil ${eye.pupil} (${Math.round((eye.pupil / eye.radius) * 100)} %) travel ${travel.toFixed(2)}px${travel < 1.5 ? "  << under 1.5" : ""}`);
  }
  const uses = new Map<string, string[]>();
  for (const [activity, ids] of Object.entries(kind.repertoire)) for (const id of ids ?? []) uses.set(id, [...(uses.get(id) ?? []), activity]);
  for (const clip of kind.clips) {
    const loose: string[] = [];
    if (!clip.loop) {
      for (const track of clip.tracks) {
        const rest = track.channel === "scaleX" || track.channel === "scaleY" ? 1 : 0;
        const first = track.keys[0]!.value;
        const last = track.keys[track.keys.length - 1]!.value;
        const closed = track.channel === "rotation" ? (((last - rest) % 360) + 360) % 360 === 0 : last === rest;
        if (first !== rest || !closed) loose.push(`${track.bone}.${track.channel} ${first}→${last}`);
      }
    }
    console.log(`   clip ${clip.id.padEnd(14)} ${String(clip.seconds).padStart(5)}s ${clip.loop ? "loop" : "once"} ${String(clipTicks(clip)).padStart(4)} ticks · ${(uses.get(clip.id) ?? ["UNUSED"]).join(", ")}${loose.length > 0 ? `  << not at rest: ${loose.join("; ")}` : ""}`);
  }
  const gaitId = kind.repertoire.walk?.[0] ?? (kind.locomotion.gait === "hop" ? kind.repertoire.hop?.[0] : undefined);
  const gait = kind.clips.find((clip) => clip.id === gaitId);
  if (gait === undefined) continue;
  let stage: Stage = advance(menagerie, openStage(7), [
    { kind: "surveyed", width: WIDTH, height: FLOOR, surfaces: [{ id: "floor", x0: 0, x1: WIDTH, y: FLOOR }], keepouts: [] },
    { kind: "tuned", mode: "lively" },
    { kind: "summoned", species: [kind.id] },
    { kind: "ticked", ticks: 64 },
  ]);
  const length = clipTicks(gait);
  const patch: Partial<Actor> = { x: HOME, y: FLOOR - hover, facing: 1, opacity: 1, activity: "walk", clip: gait.id, goal: HOME + (kind.locomotion.speed * length * 6) / 64, since: stage.tick, until: stage.tick + 1920, blink: stage.tick + 100000, partner: null, perch: "floor", vx: 0, vy: 0 };
  stage = { ...stage, actors: [{ ...stage.actors[0]!, ...patch }] };
  const feet = kind.parts.filter((part) => /^(leg|foot|caster|wheel|pipe|blade|skate)/u.test(part.bone) && (part.shape.kind === "line" || part.shape.kind === "ellipse"));
  const paths = feet.map(() => [] as { x: number; y: number }[]);
  const places: number[] = [];
  for (let tick = 0; tick < length * 3; tick++) {
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    if (tick < length) continue;
    const frame = frameOf(menagerie, stage).actors[0]!;
    places.push(frame.x);
    feet.forEach((part, index) => {
      const bone = kind.bones.findIndex((candidate) => candidate.id === part.bone) * 6;
      const shape = part.shape;
      const tipX = shape.kind === "line" ? shape.x2 : shape.kind === "ellipse" ? shape.cx : 0;
      const tipY = shape.kind === "line" ? shape.y2 : shape.kind === "ellipse" ? shape.cy : 0;
      const sole = (part.stroke === "none" ? 0 : (part.strokeWidth ?? 2) / 2) + (shape.kind === "ellipse" ? shape.ry : 0);
      const [a, b, c, d, e, f] = frame.bones.slice(bone, bone + 6) as [number, number, number, number, number, number];
      paths[index]!.push({ x: frame.x + a * tipX + c * tipY + e, y: b * tipX + d * tipY + f + sole });
    });
  }
  console.log(`   walk ${gait.id}: ${length} ticks, body travels ${((kind.locomotion.speed * length) / 64).toFixed(2)}px per cycle (measured ${((places[places.length - 1]! - places[0]!) / 2 + kind.locomotion.speed / 128).toFixed(2)})`);
  feet.forEach((part, index) => {
    const path = paths[index]!;
    const lowest = Math.max(...path.map((point) => point.y));
    const highest = Math.min(...path.map((point) => point.y));
    const local = path.map((point, tick) => point.x - places[tick]!);
    let stance = 0;
    let drift = 0;
    let worst = 0;
    let stanceHigh = Infinity;
    let stanceLow = -Infinity;
    let swingHigh = Infinity;
    for (let tick = 1; tick <= length; tick++) {
      const back = kind.locomotion.gait === "hop" ? path[tick]!.y >= lowest - 0.3 && path[tick - 1]!.y >= lowest - 0.3 : local[tick]! < local[tick - 1]!;
      if (back) {
        stance++;
        drift += path[tick]!.x - path[tick - 1]!.x;
        worst = Math.max(worst, Math.abs(path[tick]!.x - path[tick - 1]!.x));
        stanceHigh = Math.min(stanceHigh, path[tick]!.y);
        stanceLow = Math.max(stanceLow, path[tick]!.y);
      } else swingHigh = Math.min(swingHigh, path[tick]!.y);
    }
    console.log(`     ${part.id.padEnd(14)} sole between ${highest.toFixed(2)} and ${lowest.toFixed(2)} (ground = ${hover}) · swings ${(Math.max(...local) - Math.min(...local)).toFixed(2)}px under the body · stance ${stance}/${length} ticks: drifts ${drift.toFixed(2)}px over the ground (worst ${worst.toFixed(2)}/tick), sole at ${stanceHigh.toFixed(2)}…${stanceLow.toFixed(2)} · lifted to ${swingHigh.toFixed(2)} in the swing`);
  });
}
