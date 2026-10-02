/** 🗒️ Ticket tool of work package J: prints the roster of the architecture menagerie as it is on disk — directory, id,
 * names, thing, size, gait, the quizzes its grounds reach and every ground that does not exist in the quiz files.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_j_roster.ts" [--json]
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const petsRoot = resolve(repoRoot, "🎓️teaching/🏛️architecture/🐾️pets");
const catalogPath = resolve(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz/🔣️.json");
const read = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const catalog = read(catalogPath);
const quizzes: any[] = catalog.quizzes.map((path: string) => read(resolve(dirname(catalogPath), path)));
const known = new Set<string>();
for (const quiz of quizzes) {
  known.add(quiz.id);
  for (const task of quiz.tasks) {
    known.add(`${quiz.id}/${task.id}`);
    for (const item of task.items ?? []) known.add(`${quiz.id}/${task.id}/${item.id}`);
  }
}
const rows = readdirSync(petsRoot)
  .filter((name) => statSync(resolve(petsRoot, name)).isDirectory())
  .map((directory) => {
    const species = read(resolve(petsRoot, directory, "🔣️.json"));
    return {
      directory,
      codePoints: [...directory].slice(0, 2).map((character) => character.codePointAt(0)!.toString(16)),
      id: species.id,
      name: species.name,
      thing: species.thing,
      size: species.size,
      gait: species.locomotion.gait,
      speed: species.locomotion.speed,
      hover: species.locomotion.hover,
      temperament: species.temperament,
      palette: species.palette,
      quizzes: [...new Set((species.grounds as string[]).map((ground) => ground.split("/")[0]))],
      grounds: species.grounds as string[],
      missing: (species.grounds as string[]).filter((ground) => !known.has(ground)),
      clips: species.clips.length,
      bones: species.bones.length,
      parts: species.parts.length,
    };
  });
if (process.argv.includes("--json")) console.log(JSON.stringify(rows, null, 2));
else for (const row of rows) console.log(`${row.directory} [${row.codePoints.join(" ")}] id=${row.id} ${row.name.en} | ${row.name.de} | thing=${row.thing.en}/${row.thing.de} ${row.size.width}x${row.size.height} ${row.gait} quizzes=${row.quizzes.join(",")} grounds=${row.grounds.length} missing=${JSON.stringify(row.missing)}`);
console.log(`quizzes=${quizzes.map((quiz) => `${quiz.id}(${quiz.tasks.map((task: any) => `${task.id}:${(task.items ?? []).length}`).join(" ")})`).join(" ")}`);
