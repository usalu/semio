/** 🔎️ Ticket tool of work package F1: prints what `f1_survey.mjs` measured — per sample of the home overview how far every surface, wall and keep-out moved since the first sample, and the quiz page's surfaces, walls, fixtures and the cast with its sizes — so the panning of the panorama and the real page's boxes can be read without opening the JSON.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_survey_look.ts" [--dir <survey dir>] [--quiz heating]
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { Surveyed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

type Sample = { readonly pointer: readonly number[]; readonly surveyed: Surveyed; readonly pets: readonly { readonly id: string; readonly footing: string; readonly activity: string; readonly transform: string }[] };
type Sizes = Record<string, { readonly size: { readonly width: number; readonly height: number }; readonly gear: readonly string[]; readonly gait: string; readonly grounds: readonly string[] }>;

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const dir = option("--dir", join(import.meta.dir, "🗑️generated", "f1", "survey"));
const quiz = option("--quiz", "heating");
const home = JSON.parse(readFileSync(join(dir, "home.json"), "utf8")) as { sizes: Sizes; samples: Sample[] };
const first = home.samples[0]!.surveyed;
const lines: string[] = [];
lines.push(`home: ${first.surfaces.length} surfaces, ${first.walls.length} walls, ${first.keepouts.length} keep-outs`);
for (const surface of first.surfaces) lines.push(`  surface ${surface.id} ${surface.x0.toFixed(1)}…${surface.x1.toFixed(1)} @${surface.y.toFixed(1)}`);
for (const wall of first.walls) lines.push(`  wall ${wall.id} x ${wall.x.toFixed(1)} ${wall.y0.toFixed(1)}…${wall.y1.toFixed(1)}`);
for (const [index, sample] of home.samples.entries()) {
  const moved = sample.surveyed.surfaces.map((surface) => {
    const was = first.surfaces.find((entry) => entry.id === surface.id);
    return was === undefined ? `${surface.id}:new` : `${(surface.x0 - was.x0).toFixed(1)},${(surface.y - was.y).toFixed(1)}`;
  });
  const walls = sample.surveyed.walls.map((wall) => {
    const was = first.walls.find((entry) => entry.id === wall.id);
    return was === undefined ? "new" : `${(wall.x - was.x).toFixed(1)},${(wall.y0 - was.y0).toFixed(1)}`;
  });
  lines.push(`sample ${index} pointer ${sample.pointer.join(",")}: surfaces ${sample.surveyed.surfaces.length} moved [${moved.join(" ")}] walls [${walls.join(" ")}] pets ${sample.pets.map((pet) => `${pet.id}:${pet.footing}/${pet.activity}`).join(" ")}`);
}
const quizPage = JSON.parse(readFileSync(join(dir, `${quiz}.json`), "utf8")) as { sizes: Sizes; sample: Sample };
const seen = quizPage.sample.surveyed;
lines.push(`${quiz}: ${seen.width}×${seen.height}`);
for (const surface of seen.surfaces) lines.push(`  surface ${surface.id} ${surface.x0.toFixed(1)}…${surface.x1.toFixed(1)} @${surface.y.toFixed(1)}`);
for (const wall of seen.walls) lines.push(`  wall ${wall.id} (${wall.surface}) side ${wall.side} x ${wall.x.toFixed(1)} ${wall.y0.toFixed(1)}…${wall.y1.toFixed(1)}`);
for (const keepout of seen.keepouts) lines.push(`  keepout ${keepout.x.toFixed(1)},${keepout.y.toFixed(1)} ${keepout.width.toFixed(1)}×${keepout.height.toFixed(1)}`);
for (const fixture of seen.fixtures) lines.push(`  fixture ${fixture.id} ${fixture.key} ${fixture.x.toFixed(1)},${fixture.y.toFixed(1)} ${fixture.width.toFixed(1)}×${fixture.height.toFixed(1)}`);
for (const pet of quizPage.sample.pets) {
  const kind = quizPage.sizes[pet.id]!;
  lines.push(`  pet ${pet.id} ${pet.footing}/${pet.activity} ${pet.transform} size ${kind.size.width}×${kind.size.height} gear ${kind.gear.join("+")} gait ${kind.gait} grounds ${kind.grounds.join(",")}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
