/** 🔬️ Scans one clip of an asset on a fine time grid with the product's own evaluation and structure solver and prints, per instant, what the check CLI would judge: contacts, root, centre of mass, its acceleration, the support forces and the force residual.
 *
 * `bun <this file> --file <asset json> --clip <id> [--step 0.004166] [--from 0] [--to <duration>] [--samples 9] [--quiet]`
 *
 * Columns: time, phase, contacts, root x and y (m), centre of mass x and y (m), its acceleration over g (x, y), the sum of the normal reactions over the weight, the sum of the horizontal reactions over the weight, the residual (N) and flags (`S` a contact slides, `L` a contact lifted off, `U` unsupported outside a ballistic phase, `R` residual above the check tolerance). The summary counts the flagged instants; `--samples` also lists the instants the check CLI samples.
 *
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts — the check this mirrors */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const { values } = parseArgs({ options: { file: { type: "string" }, clip: { type: "string" }, step: { type: "string", default: `${1 / 240}` }, from: { type: "string", default: "0" }, to: { type: "string" }, samples: { type: "string", default: "9" }, quiet: { type: "boolean", default: false } } });
const asset = JSON.parse(readFileSync(resolve(values.file!), "utf8")).asset;
const clip = asset.clips.find((candidate: { id: string }) => candidate.id === values.clip);
if (clip === undefined) throw new Error(`no clip "${values.clip}"`);
const to = values.to === undefined ? clip.durationS : Number(values.to);
const step = Number(values.step);

function row(timeS: number): { line: string; flags: string } {
  const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
  const structure = solveStructure(state);
  const weight = structure.totalMassKg * structure.gravityMS2;
  let [ax, ay, m] = [0, 0, 0];
  for (const bone of asset.rig.bones) {
    if (!(bone.mass?.kg > 0)) continue;
    const a = state.kinematics.segmentAccelerations[bone.id]!;
    ax += bone.mass.kg * a.x;
    ay += bone.mass.kg * a.y;
    m += bone.mass.kg;
  }
  const active = structure.reactions.filter((reaction) => reaction.active);
  const fx = active.reduce((sum, reaction) => sum + reaction.fxN, 0);
  const fy = active.reduce((sum, reaction) => sum + reaction.fyN, 0);
  const notes = structure.notes.map((note) => note.en).join(" ");
  const ballistic = clip.phases.some((phase: { id: string; ballistic?: boolean }) => phase.id === state.phase && phase.ballistic === true);
  const tolerance = Math.max(1e-6, weight * 1e-6);
  const bad = active.length > 0 && (Math.abs(structure.residual.fxN) > tolerance || Math.abs(structure.residual.fyN) > tolerance);
  const flags = `${/slides/.test(notes) ? "S" : ""}${/lifted off/.test(notes) ? "L" : ""}${active.length === 0 && !ballistic ? "U" : ""}${bad ? "R" : ""}`;
  const n = (value: number, digits = 4): string => value.toFixed(digits).padStart(digits + 4);
  const line = `${timeS.toFixed(4)} ${(state.phase ?? "-").padEnd(10)} ${state.contacts.join("+").padEnd(20)} root ${n(state.root.x)} ${n(state.root.y)} com ${n(structure.centreOfMass.x)} ${n(structure.centreOfMass.y)} a/g ${n(ax / m / structure.gravityMS2, 3)} ${n(ay / m / structure.gravityMS2, 3)} N/W ${n(fy / weight, 3)} Fx/W ${n(fx / weight, 3)} res ${structure.residual.fxN.toExponential(2)} ${structure.residual.fyN.toExponential(2)} ${flags}`;
  return { line, flags };
}

const counts: Record<string, number> = {};
let total = 0;
for (let i = 0; ; i++) {
  const t = Number(values.from) + i * step;
  if (t > to + 1e-12) break;
  const { line, flags } = row(Math.min(t, to));
  total++;
  for (const flag of flags) counts[flag] = (counts[flag] ?? 0) + 1;
  if (!values.quiet || flags !== "") console.log(line);
}
console.log(`instants ${total}, flagged ${JSON.stringify(counts)}`);
const samples = Number(values.samples);
console.log("check samples:");
for (let i = 0; i < samples; i++) console.log(row((clip.durationS * i) / Math.max(1, samples - 1)).line);
