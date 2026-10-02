/** 🧾️ Ticket tool of work package P: prints what a run of `wp_n_drive.mjs` found, one block per stop — how many pets
 * every sample saw, how many of them stood on the footer line, whether a pet covered text or a control, stood in
 * another one or the page overflowed, where everybody stood at the end, and (with `--probe 1`) every surveyed surface
 * with the perches the core cuts from it and what blocks it.
 *
 * Usage (from the repository root): node ".../wp_p_summary.mjs" <drive.json>
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const findings = JSON.parse(readFileSync(resolve(process.argv[2]), "utf8"));
const floor = findings.viewport.height;
for (const [label, stop] of Object.entries(findings.stops)) {
  const samples = stop.samples;
  if (samples.length === 0) {
    process.stdout.write(`${label}: no sample ${JSON.stringify(stop.notes)}\n`);
    continue;
  }
  const low = (sample) => sample.pets.filter((pet) => pet.y !== null && pet.y > floor - 60).length;
  process.stdout.write(`${label}: pets ${samples.map((sample) => sample.pets.length).join(",")} · on the footer line ${samples.map(low).join(",")} · covered ${samples.reduce((sum, sample) => sum + sample.covered.length, 0)} · stacked ${samples.reduce((sum, sample) => sum + sample.stacked.length, 0)} · overflow ${Math.max(...samples.map((sample) => sample.overflow))}\n`);
  const last = samples.at(-1);
  process.stdout.write(`  at the end: ${last.pets.map((pet) => `${pet.id}@${pet.x},${pet.y}${pet.edge === 0 ? "" : ` (edge ${pet.edge})`}`).join(" | ")}\n`);
  for (const sample of samples) for (const cover of sample.covered) process.stdout.write(`  covered: ${JSON.stringify(cover)}\n`);
  if (stop.probe !== undefined) for (const surface of stop.probe.surfaces) process.stdout.write(`  ${surface.id} [${surface.x0}..${surface.x1} @${surface.y}] perches ${surface.perches.join("; ") || "none"}${surface.blockers > 0 ? ` · blocked by ${surface.blocked.join(", ")}` : ""}\n`);
  if (stop.notes.length > 0) process.stdout.write(`  notes: ${JSON.stringify(stop.notes)}\n`);
}
process.stdout.write(`console ${JSON.stringify(findings.console)} · errors ${JSON.stringify(findings.errors)} · failed ${JSON.stringify(findings.failed)}\n`);
