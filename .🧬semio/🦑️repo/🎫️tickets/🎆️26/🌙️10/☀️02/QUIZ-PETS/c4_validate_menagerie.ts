/** ⚗️ Ticket tool of work package C4: assembles the architecture menagerie from the documents on disk and prints what the product's own validators find (`ensembleIssues`, `menagerieIssues`), then per reaction of its chemistry the scenes in which its two sides can stand on one stage — a quiz's cast shows six of its seven, the home cast five of the owner's nine and one visitor, so two visitors of home never meet there. Exit code 1 when a validator finds anything.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_validate_menagerie.ts" [--scenes]
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { assembleMenagerie, ensembleIssues, menagerieIssues } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🟦️.ts";
import type { Cast, Ensemble, Species, Trait } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets");
const ensemble = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8")) as Ensemble;
const species = ensemble.species.map((path) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Species);
const menagerie = assembleMenagerie(ensemble, species);
const ensembleFound = ensembleIssues(ensemble);
const menagerieFound = menagerieIssues(menagerie);
process.stdout.write(`ensembleIssues: ${ensembleFound.length} ${JSON.stringify(ensembleFound.slice(0, 20))}\n`);
process.stdout.write(`menagerieIssues: ${menagerieFound.length} ${JSON.stringify(menagerieFound.slice(0, 20))}\n`);
process.stdout.write(`reactions: ${menagerie.chemistry.length}\n`);

/** 🎟️ Whether two species can stand on one stage in a cast: both in it, and at home not both visitors. */
function together(cast: Cast, one: string, other: string): boolean {
  const members = [...cast.core, ...cast.rotation];
  if (!members.includes(one) || !members.includes(other)) return false;
  return cast.scene !== "home" || cast.core.includes(one) || cast.core.includes(other);
}

/** 🔎️ The species a trait can be. */
function kinds(trait: Trait): readonly string[] {
  return trait.species === undefined ? menagerie.species.map((kind) => kind.id) : [trait.species];
}

if (process.argv.includes("--scenes")) {
  for (const reaction of menagerie.chemistry) {
    const scenes = menagerie.casts.filter((cast) => kinds(reaction.when).some((one) => kinds(reaction.near).some((other) => one !== other && together(cast, one, other)))).map((cast) => cast.scene);
    process.stdout.write(`${reaction.id}: ${scenes.join(" ") || "NEVER"}\n`);
  }
}
process.exit(ensembleFound.length + menagerieFound.length > 0 ? 1 : 0);
