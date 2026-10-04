/** 🔬️ Ticket tool of work package B2: plays the mind of the stage on the sample menagerie and the trace troupe — clicks, circles, strokes, contagion, chemistry — and prints what happened, so the unit suite's expectations can be read off real runs.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b2_probe.ts" [clicks|circles|strokes|contagion|chemistry|whims]
 * The troupe is read from 🗑️generated/wp-e/menagerie.json (`generate_behavior_vectors.py --troupe` writes it).
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { TICKS_PER_SECOND, type Menagerie, type Stage, type StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { settled } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/💗️feeling/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const SAMPLE = (JSON.parse(readFileSync(resolve(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie;
const TROUPE = JSON.parse(readFileSync(resolve(import.meta.dir, "🗑️generated/wp-e/menagerie.json"), "utf8")) as Menagerie;
const LEDGE = { id: "ledge", x0: 600, x1: 644, y: 300 };
const CARD = { id: "card", x0: 300, x1: 800, y: 400 };

/** 🎬️ A stage with the species on the surfaces, play permitted. */
function staged(menagerie: Menagerie, species: string[], surfaces = [LEDGE], mode: "calm" | "lively" = "calm", seed = 1): Stage {
  return advance(menagerie, openStage(seed), [{ kind: "tuned", mode }, { kind: "surveyed", width: 1280, height: 720, surfaces, keepouts: [], walls: [], fixtures: [] }, { kind: "summoned", species }, { kind: "permitted", play: true, mischief: false }, { kind: "ticked", ticks: 64 }]);
}

/** 🧸️ One line about an actor. */
function line(menagerie: Menagerie, stage: Stage, species: string): string {
  const actor = stage.actors.find((entry) => entry.species === species)!;
  const kind = menagerie.species.find((entry) => entry.id === species)!;
  const feeling = settled(actor.feeling, kind.mood, stage.tick);
  return `${stage.tick} ${species} ${actor.activity}${actor.trick === null ? "" : `:${actor.trick}`} until ${actor.until} state ${actor.state}@${actor.stateSince} ${feeling.mood} ${feeling.intensity.toFixed(3)} heat ${actor.warmth.heat.toFixed(3)} ${actor.warmth.tier}/${actor.warmth.run}/${actor.warmth.tricks} plumes ${JSON.stringify(actor.emitters)}`;
}

/** 👆️ Clicks every half second on the walker. */
function clicks(menagerie: Menagerie, species: string): void {
  let stage = staged(menagerie, [species]);
  const actor = stage.actors[0]!;
  const kind = menagerie.species.find((entry) => entry.id === species)!;
  for (let click = 0; click < 11; click++) {
    stage = advance(menagerie, stage, [{ kind: "pressed", x: actor.x, y: actor.y - kind.size.height / 2, pointer: "mouse" }, { kind: "released", x: actor.x, y: actor.y - kind.size.height / 2 }]);
    console.log(line(menagerie, stage, species));
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 32 }]);
  }
  stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 600 }]);
  console.log(line(menagerie, stage, species));
}

/** 🌀️ The pointer circling the walker. */
function circles(menagerie: Menagerie, species: string, way: number): void {
  let stage = staged(menagerie, [species]);
  const actor = stage.actors[0]!;
  const kind = menagerie.species.find((entry) => entry.id === species)!;
  const cx = actor.x;
  const cy = actor.y - kind.size.height / 2;
  const radius = 1.8 * Math.max(kind.size.width, kind.size.height);
  for (let lap = 0; lap < 3; lap++) {
    for (let tick = 0; tick <= 160; tick++) {
      const turn = (way * 2 * Math.PI * tick) / TICKS_PER_SECOND;
      stage = advance(menagerie, stage, [{ kind: "pointed", x: Math.round(4 * (cx + radius * Math.cos(turn))) / 4, y: Math.round(4 * (cy + radius * Math.sin(turn))) / 4, over: "free" }, { kind: "ticked", ticks: 1 }]);
      const now = stage.actors[0]!;
      if (now.activity === "trick" && now.since === stage.tick) console.log(`cue ${line(menagerie, stage, species)}`);
    }
    stage = advance(menagerie, stage, [{ kind: "unpointed" }, { kind: "ticked", ticks: 200 }]);
    console.log(line(menagerie, stage, species));
  }
}

/** 🖐️ The pointer stroking across the walker. */
function strokes(menagerie: Menagerie, species: string): void {
  let stage = staged(menagerie, [species]);
  const actor = stage.actors[0]!;
  const kind = menagerie.species.find((entry) => entry.id === species)!;
  for (let tick = 0; tick <= 256; tick++) {
    const x = Math.round(4 * (actor.x + 0.5 * kind.size.width * Math.sin((2 * Math.PI * 2 * tick) / TICKS_PER_SECOND))) / 4;
    stage = advance(menagerie, stage, [{ kind: "pointed", x, y: actor.y - kind.size.height / 2, over: "free" }, { kind: "ticked", ticks: 1 }]);
    if (tick % 32 === 0) console.log(line(menagerie, stage, species));
  }
}

/** 🎪️ A lively company on a card: what the chemistry and the moods do over a few minutes. */
function lively(menagerie: Menagerie): void {
  let stage = staged(menagerie, menagerie.species.map((kind) => kind.id), [CARD, { id: "floor", x0: 0, x1: 1280, y: 720 }], "lively", 7);
  const seen = new Map<string, number>();
  for (let tick = 0; tick < 5 * 60 * TICKS_PER_SECOND; tick++) {
    const before = stage;
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    for (const actor of stage.actors) {
      const earlier = before.actors.find((entry) => entry.species === actor.species);
      if (earlier !== undefined && (earlier.activity !== actor.activity || earlier.since !== actor.since)) seen.set(actor.activity, (seen.get(actor.activity) ?? 0) + 1);
      if (earlier !== undefined && earlier.state !== actor.state) console.log(`state ${line(menagerie, stage, actor.species)}`);
    }
    if (stage.coolings.length !== before.coolings.length) console.log(`chemistry ${stage.tick} ${JSON.stringify(stage.coolings.map((cooling) => cooling.reaction))} pledges ${JSON.stringify(stage.pledges)}`);
  }
  console.log(JSON.stringify([...seen.entries()]), "rate", frameOf(menagerie, stage).rate);
}

/** ⚖️ Replays one script of the stage-trace inputs and prints the first ticks on which a grounded actor is not on its perch. */
function laws(id: string): void {
  const inputs = JSON.parse(readFileSync(resolve(import.meta.dir, "🗑️generated/wp-e/stage-trace-inputs.json"), "utf8")) as { menagerie: Menagerie; scripts: { id: string; seed: number; ticks: number; steps: { at: number; events: StageEvent[] }[] }[] };
  const menagerie = inputs.menagerie;
  const script = inputs.scripts.find((entry) => entry.id === id)!;
  let stage = openStage(script.seed);
  let shown = 0;
  for (let tick = 1; tick <= script.ticks && shown < 6; tick++) {
    const events = script.steps.filter((step) => step.at === tick - 1).flatMap((step) => step.events);
    const before = stage;
    stage = advance(menagerie, advance(menagerie, stage, events), [{ kind: "ticked", ticks: 1 }]);
    const frame = frameOf(menagerie, stage);
    const sorted = frame.actors.every((drawn, at) => at === 0 || frame.actors[at - 1]!.y < drawn.y || (frame.actors[at - 1]!.y === drawn.y && frame.actors[at - 1]!.species < drawn.species));
    const wake = frame.wake === null || (frame.rate === 0 && frame.wake > frame.tick);
    const inside = stage.actors.every((actor) => actor.x >= 0 && actor.x <= stage.width && actor.opacity >= 0 && actor.opacity <= 1);
    if (!sorted || !wake || !inside || ![0, 16, 32, 64].includes(frame.rate)) {
      console.log(`whole ${tick} rate ${frame.rate} wake ${frame.wake} sorted ${sorted} inside ${inside} ${JSON.stringify(stage.actors.map((actor) => [actor.species, actor.activity, actor.footing, actor.x, actor.y, actor.opacity]))} events ${JSON.stringify(events.map((event) => event.kind))}`);
      shown++;
    }
    for (const actor of stage.actors) {
      if (actor.perch === null) continue;
      const kind = menagerie.species.find((entry) => entry.id === actor.species)!;
      const hover = kind.locomotion.gait === "float" && kind.locomotion.hover !== undefined ? kind.locomotion.hover : 0;
      if (stage.perches.some((perch) => perch.surface === actor.perch && perch.x0 <= actor.x && actor.x <= perch.x1 && actor.y === perch.y - hover)) continue;
      const earlier = before.actors.find((entry) => entry.species === actor.species);
      console.log(`${tick} ${actor.species} ${earlier?.activity}→${actor.activity} footing ${actor.footing} perch ${actor.perch} x ${actor.x} y ${actor.y} perches ${JSON.stringify(stage.perches.filter((perch) => perch.surface === actor.perch))} events ${JSON.stringify(events.map((event) => event.kind))}`);
      shown++;
    }
  }
}

/** 📜️ Replays one script of the stage-trace inputs and prints what the mind did in it: tricks, states, purrs, shrugs, reactions and pledges. */
function script(id: string): void {
  const inputs = JSON.parse(readFileSync(resolve(import.meta.dir, "🗑️generated/wp-e/stage-trace-inputs.json"), "utf8")) as { menagerie: Menagerie; scripts: { id: string; seed: number; ticks: number; steps: { at: number; events: StageEvent[] }[] }[] };
  const menagerie = inputs.menagerie;
  const chosen = inputs.scripts.find((entry) => entry.id === id)!;
  let stage = openStage(chosen.seed);
  for (let tick = 1; tick <= chosen.ticks; tick++) {
    const events = chosen.steps.filter((step) => step.at === tick - 1).flatMap((step) => step.events);
    const before = stage;
    stage = advance(menagerie, advance(menagerie, stage, events), [{ kind: "ticked", ticks: 1 }]);
    for (const actor of stage.actors) {
      const earlier = before.actors.find((entry) => entry.species === actor.species);
      if (earlier === undefined) continue;
      if ((earlier.activity !== actor.activity || earlier.since !== actor.since) && ["trick", "purr", "shrug", "greet", "cuddle", "squabble", "sleep"].includes(actor.activity)) console.log(`${tick} ${actor.species} ${actor.activity}${actor.trick === null ? "" : `:${actor.trick}`}${actor.partner === null ? "" : ` with ${actor.partner}`} tier ${actor.warmth.tier}`);
      if (earlier.state !== actor.state) console.log(`${tick} ${actor.species} state ${earlier.state} → ${actor.state}`);
    }
    for (const cooling of stage.coolings) if (!before.coolings.some((entry) => entry.reaction === cooling.reaction && entry.until === cooling.until && entry.when === cooling.when)) console.log(`${tick} chemistry ${cooling.reaction}: ${cooling.when} near ${cooling.near}`);
    for (const pledge of stage.pledges) if (!before.pledges.some((entry) => entry.until === pledge.until)) console.log(`${tick} pledge ${pledge.between.join(" & ")} ${pledge.encounter}`);
    if (tick % (16 * TICKS_PER_SECOND) === 0) console.log(`${tick} places ${stage.actors.map((actor) => `${actor.species}@${actor.perch}:${actor.x.toFixed(0)}`).join(" ")} rapports ${JSON.stringify(stage.rapports)}`);
  }
}

/** 🦘️ The tick of the first hop of a lonely walker between two shelves, per seed, in lively minutes (the stage suite's hopping test). */
function hopping(menagerie: Menagerie, species: string): void {
  const low = { id: "low", x0: 300, x1: 500, y: 400 };
  const high = { id: "high", x0: 520, x1: 800, y: 380 };
  const kind = menagerie.species.find((entry) => entry.id === species)!;
  const firsts: string[] = [];
  for (let seed = 1; seed <= 12; seed++) {
    const arrived = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, { kind: "surveyed", width: 1280, height: 720, surfaces: [low, high], keepouts: [], walls: [], fixtures: [] }, { kind: "summoned", species: [species] }, { kind: "ticked", ticks: 64 }]);
    let stage: Stage = { ...arrived, actors: arrived.actors.map((actor) => ({ ...actor, perch: low.id, x: low.x1 - kind.size.width, y: low.y, goal: low.x1 - kind.size.width })) };
    let first = -1;
    let count = 0;
    for (let tick = 0; tick < 300 * TICKS_PER_SECOND; tick++) {
      const before = stage.actors[0]?.activity;
      stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
      if (stage.actors[0]?.activity === "hop" && before !== "hop") {
        count++;
        if (first < 0) first = tick;
      }
    }
    firsts.push(`seed ${seed}: first ${first < 0 ? "-" : (first / TICKS_PER_SECOND).toFixed(1)} s, ${count} hops`);
  }
  console.log(`${menagerie.id} ${species}: ${firsts.join(" · ")}`);
}

/** 💥️ The stage suite's drag-and-throw test of the troupe replayed: the first overlap of bodies per seed, with what the pair did on the ticks before it. */
async function overlapping(): Promise<void> {
  const { overlaps } = await import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts");
  const { bodiesOf } = await import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts");
  const menagerie = TROUPE;
  const everyone = menagerie.species.map((kind) => kind.id);
  const card = { id: "card", x0: 300, x1: 800, y: 400 };
  const floor = { id: "floor", x0: 0, x1: 1280, y: 720 };
  const surveyed = (surfaces: { id: string; x0: number; x1: number; y: number }[]): StageEvent => ({ kind: "surveyed", width: 1280, height: 720, surfaces, keepouts: [], walls: [], fixtures: [] });
  for (let seed = 1; seed <= 6; seed++) {
    let stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, surveyed([card, { id: "shelf", x0: 700, x1: 1000, y: 250 }, floor]), { kind: "summoned", species: everyone }]);
    for (let tick = 0; tick < 64; tick++) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    stage = advance(menagerie, stage, [{ kind: "permitted", play: true, mischief: false }]);
    let unit = seed * 7919;
    const next = (bound: number): number => {
      unit = (unit * 48271) % 2147483647;
      return ((unit % 1000) / 1000) * bound;
    };
    const history: string[] = [];
    for (let tick = 0; tick < 2400; tick++) {
      const events: StageEvent[] = [];
      if (stage.press.phase === "idle" && next(1) < 0.02 && stage.actors.length > 0) {
        const target = stage.actors[Math.floor(next(stage.actors.length))]!;
        events.push({ kind: "pressed", x: target.x, y: target.y - menagerie.species.find((kind) => kind.id === target.species)!.size.height / 2, pointer: "mouse" });
      } else if (stage.press.phase !== "idle") {
        const pointer = stage.pointer ?? { x: 640, y: 360 };
        if (next(1) < 0.03) events.push({ kind: "released", x: pointer.x, y: pointer.y });
        else if (next(1) < 0.005) events.push({ kind: "cancelled" });
        else {
          const other = stage.actors[Math.floor(next(stage.actors.length))];
          const goal = other === undefined ? { x: next(1280), y: next(720) } : { x: other.x, y: other.y - 20 };
          events.push({ kind: "dragged", x: pointer.x + Math.max(-14, Math.min(14, goal.x - pointer.x)), y: pointer.y + Math.max(-14, Math.min(14, goal.y - pointer.y)) });
        }
      }
      if (next(1) < 0.002) events.push(surveyed(next(1) < 0.5 ? [card, floor] : [{ ...card, x0: 300 + next(100), x1: 600 + next(200), y: 400 + next(60) }, { id: "shelf", x0: 700, x1: 1000, y: 250 }, floor]));
      stage = advance(menagerie, stage, [...events, { kind: "ticked", ticks: 1 }]);
      history.push(`${stage.tick} [${events.map((event) => event.kind).join(",")}] ${stage.actors.map((actor) => `${actor.species}:${actor.activity}/${actor.footing}@${actor.perch}(${actor.x.toFixed(1)},${actor.y.toFixed(1)})`).join(" ")}`);
      const pairs = overlaps(bodiesOf(stage.actors, stage.actors.map((actor) => menagerie.species.find((kind) => kind.id === actor.species)!)));
      if (pairs.length > 0) {
        console.log(`seed ${seed}: ${pairs.map((pair) => `${pair.first}+${pair.second}`).join(", ")} at ${stage.tick}`);
        for (const line of history.slice(-6)) console.log(`  ${line}`);
        break;
      }
    }
  }
}

const what = process.argv[2] ?? "clicks";
if (what === "overlapping") {
  await overlapping();
  process.exit(0);
}
if (what === "hopping") {
  hopping(SAMPLE, "blobby");
  hopping(TROUPE, "mossy");
  process.exit(0);
}
if (what === "script") {
  script(process.argv[3] ?? "clicks-and-purrs");
  process.exit(0);
}
if (what === "laws") {
  laws(process.argv[3] ?? "moving-card");
  process.exit(0);
}
for (const [menagerie, walker] of [
  [SAMPLE, "blobby"],
  [TROUPE, "mossy"],
] as const) {
  console.log(`== ${menagerie.id} ${what}`);
  if (what === "clicks") clicks(menagerie, walker);
  if (what === "circles") {
    circles(menagerie, walker, 1);
    circles(menagerie, walker, -1);
  }
  if (what === "strokes") strokes(menagerie, walker);
  if (what === "lively") lively(menagerie);
}
