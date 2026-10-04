/** 🔍️ Ticket tool of work package F1: why a stage-trace digest moved — for every script whose committed trace differs between two versions of the vectors, the first checkpoint whose digest differs and, at that checkpoint (or the first one where the actors differ), every field of every actor that differs, so the change of the stage that moved it can be named.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_trace_diff.ts" <before.json> <after.json>
 */
import { readFileSync } from "node:fs";

type Actor = Readonly<Record<string, unknown>> & { readonly species: string };
type Checkpoint = { readonly tick: number; readonly digest: number; readonly actors: readonly Actor[]; readonly drawn: Readonly<Record<string, unknown>> };
type Vectors = { readonly scripts: readonly { readonly id: string; readonly expected: { readonly digest: number; readonly checkpoints: readonly Checkpoint[] } }[] };

/** 🧾️ The fields of two actors of one species that differ, as `field before→after`. */
function differences(before: Actor | undefined, after: Actor | undefined): string[] {
  if (before === undefined || after === undefined) return [before === undefined ? "arrives only after" : "gone only after"];
  const found: string[] = [];
  for (const field of Object.keys(after)) {
    const was = JSON.stringify(before[field]);
    const now = JSON.stringify(after[field]);
    if (was !== now) found.push(`${field} ${was.slice(0, 60)}→${now.slice(0, 60)}`);
  }
  return found;
}

const [beforePath, afterPath] = process.argv.slice(2);
if (beforePath === undefined || afterPath === undefined) throw new Error("usage: f1_trace_diff.ts <before.json> <after.json>");
const before = JSON.parse(readFileSync(beforePath, "utf8").replace(/^﻿/u, "")) as Vectors;
const after = JSON.parse(readFileSync(afterPath, "utf8").replace(/^﻿/u, "")) as Vectors;
const lines: string[] = [];
for (const script of after.scripts) {
  const was = before.scripts.find((entry) => entry.id === script.id);
  if (was === undefined || was.expected.digest === script.expected.digest) continue;
  const first = script.expected.checkpoints.findIndex((checkpoint, index) => was.expected.checkpoints[index]?.digest !== checkpoint.digest);
  let at = first;
  let found: string[] = [];
  for (; at >= 0 && at < script.expected.checkpoints.length && found.length === 0; at++) {
    const now = script.expected.checkpoints[at]!;
    const then = was.expected.checkpoints[at];
    if (then === undefined) break;
    for (const actor of now.actors) for (const change of differences(then.actors.find((entry) => entry.species === actor.species), actor)) found.push(`${actor.species}: ${change}`);
    for (const field of Object.keys(now.drawn)) if (JSON.stringify(now.drawn[field]) !== JSON.stringify(then.drawn[field])) found.push(`drawn ${field} ${JSON.stringify(then.drawn[field])}→${JSON.stringify(now.drawn[field])}`);
    if (found.length > 0) found.unshift(`checkpoint ${at} (tick ${now.tick}, digests first differ at checkpoint ${first}, tick ${script.expected.checkpoints[first]?.tick})`);
  }
  lines.push(`${script.id}: ${found.slice(0, 6).join("; ") || `digest differs from checkpoint ${first} on, no field of a checkpoint differs`}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
