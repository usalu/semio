/** 🎙️ Ticket tool: records the trace of every script of the stage-trace vectors from the TypeScript subject.
 *
 * Usage (called by `generate_behavior_vectors.py`; from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/record_stage_trace.ts" <inputs.json> <traces.json>
 *
 * `inputs.json` holds `{ menagerie, scripts }` without expectations; `traces.json` receives `{ <script id>: trace }`
 * exactly as the adapter of the case projects it.
 */
import { readFileSync, writeFileSync } from "node:fs";
import type { Menagerie } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { type Script, traceOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧪️tests/🎪️stage-trace/🟦️.ts";

const [inputs, outputs] = process.argv.slice(2);
if (inputs === undefined || outputs === undefined) throw new Error("usage: record_stage_trace.ts <inputs.json> <traces.json>");
const document = JSON.parse(readFileSync(inputs, "utf8")) as { menagerie: Menagerie; scripts: Script[] };
const traces: Record<string, unknown> = {};
for (const script of document.scripts) {
  const started = Date.now();
  traces[script.id] = traceOf(document.menagerie, script).trace;
  console.log(`recorded ${script.id}: ${script.ticks} ticks in ${Date.now() - started} ms`);
}
writeFileSync(outputs, `${JSON.stringify(traces)}\n`);
