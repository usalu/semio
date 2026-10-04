/** 🔬️ Ticket tool of work package C1a: feeds the core the events the layer sends at mount and during a calm minute (with and without `permitted`, with the hand's events) and prints where it throws, to tell faults of the layer from faults of the core in flight.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c1a_probe_stage.ts"
 */
import { advance, frameOf, openStage, type Menagerie, type StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts";
import sample from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";

const menagerie = sample.menagerie as unknown as Menagerie;
const surveyed: StageEvent = { kind: "surveyed", width: 1200, height: 800, surfaces: [{ id: "s1", x0: 100, x1: 500, y: 300 }, { id: "floor", x0: 0, x1: 1200, y: 800 }], keepouts: [], walls: [{ id: "s1:left", surface: "s1", side: -1, x: 100, y0: 300, y1: 500 }], fixtures: [] };
const species = menagerie.species.map((each) => each.id);

for (const [name, opening] of [
  ["without permitted", [{ kind: "tuned", mode: "calm" }, { kind: "hushed", quiet: false }]],
  ["with permitted", [{ kind: "tuned", mode: "calm" }, { kind: "hushed", quiet: false }, { kind: "permitted", play: true, mischief: true }]],
] as const) {
  let step = "open";
  try {
    let stage = advance(menagerie, openStage(7), opening as readonly StageEvent[]);
    step = "first";
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }, surveyed, { kind: "summoned", species }]);
    let frame = frameOf(menagerie, stage);
    for (let index = 0; index < 64 * 30; index += 8) {
      step = `tick ${index}`;
      const events: StageEvent[] = [{ kind: "ticked", ticks: 8 }];
      if (index === 640) {
        const body = frame.actors[0]?.body;
        if (body !== undefined) events.push({ kind: "stirred" }, { kind: "pressed", x: body.x + body.width / 2, y: body.y + body.height / 2, pointer: "mouse" });
      }
      if (index === 648 || index === 656) events.push({ kind: "dragged", x: 400 + index / 8, y: 200 });
      if (index === 664) events.push({ kind: "released", x: 420, y: 200 }, { kind: "scrolled" }, { kind: "played", species: species[0]!, deed: "hello" });
      if (index === 700) events.push({ kind: "pointed", x: 300, y: 250, over: "free" }, { kind: "cancelled" });
      stage = advance(menagerie, stage, events);
      frame = frameOf(menagerie, stage);
    }
    console.log(name, "ok", frame.actors.map((actor) => `${actor.species} ${actor.footing} ${actor.activity} body ${JSON.stringify(actor.body)}`), "held", frame.held);
  } catch (error) {
    console.log(name, "throws at", step, String(error), (error as Error).stack?.split("\n").slice(0, 6).join(" | "));
  }
}
