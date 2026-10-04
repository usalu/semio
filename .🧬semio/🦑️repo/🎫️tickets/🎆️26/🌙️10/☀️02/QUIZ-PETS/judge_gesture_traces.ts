/** ⚖️ Ticket tool: lets the TypeScript subject judge a corpus of pointer traces.
 *
 * Usage (called by `generate_gesture_vectors.py`; from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/judge_gesture_traces.ts" <corpus.json> <verdicts.json>
 *
 * `corpus.json` holds `{ quantum, hold, page, hovers, helds, presses, warmths }` in the encoding of the committed
 * vectors; a hover trace carries the bodies it passes and, optionally, in how many of its variants it is judged
 * (`variants`, 1 when absent). `verdicts.json` receives, per trace id, what the adapter of the case answers: for a
 * hover `[variant, body, tick, cue]` rows, for a held path and a press `[tick, cue]` rows, for a warmth its answers.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { type Box, type PressVector, type WarmthVector, boxVariant, held, hovered, pressed, trailOf, variantOf, warmed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧪️tests/👆️gesture-recognition/🟦️.ts";
import { type Guards, UNGUARDED } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/👆️gesture/🟦️.ts";

/** 🚧️ The guards with one of them up, or none. */
function guarded(guard: keyof Guards | null | undefined): Guards {
  return guard === undefined || guard === null ? UNGUARDED : { ...UNGUARDED, [guard]: true };
}

type Corpus = {
  readonly quantum: number;
  readonly hold: number;
  readonly page: readonly [number, number];
  readonly hovers?: readonly { readonly id: string; readonly bodies: readonly Box[]; readonly path: readonly number[]; readonly variants?: number; readonly guard?: keyof Guards | null }[];
  readonly helds?: readonly { readonly id: string; readonly height: number; readonly path: readonly number[]; readonly guard?: keyof Guards | null }[];
  readonly presses?: readonly Pick<PressVector, "id" | "ticks" | "events">[];
  readonly warmths?: readonly Pick<WarmthVector, "id" | "caresses">[];
};

const [inputs, outputs] = process.argv.slice(2);
if (inputs === undefined || outputs === undefined) throw new Error("usage: judge_gesture_traces.ts <corpus.json> <verdicts.json>");
const corpus = JSON.parse(readFileSync(inputs, "utf8")) as Corpus;
const hovers: Record<string, [number, number, number, string][]> = {};
for (const trace of corpus.hovers ?? []) {
  const trail = trailOf(trace.path, corpus.hold);
  const rows: [number, number, number, string][] = [];
  for (let variant = 0; variant < (trace.variants ?? 1); variant++) {
    const turned = variant === 0 ? trail : variantOf(trail, corpus.page, variant);
    trace.bodies.forEach((box, body) => {
      for (const [tick, cue] of hovered(turned, corpus.quantum, variant === 0 ? box : boxVariant(box, corpus.page, variant), guarded(trace.guard))) rows.push([variant, body, tick, cue]);
    });
  }
  hovers[trace.id] = rows;
}
const helds = Object.fromEntries((corpus.helds ?? []).map((trace) => [trace.id, held(trailOf(trace.path, corpus.hold), corpus.quantum, trace.height, guarded(trace.guard))]));
const presses = Object.fromEntries((corpus.presses ?? []).map((vector) => [vector.id, pressed(vector, corpus.quantum)]));
const warmths = Object.fromEntries((corpus.warmths ?? []).map((vector) => [vector.id, warmed(vector.caresses)]));
writeFileSync(outputs, `${JSON.stringify({ hovers, helds, presses, warmths })}\n`);
