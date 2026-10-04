/** 📋️ Ticket tool of work package C4: prints what the twenty species of the architecture menagerie can become and do — gait, gear, resting mood, grip, reach, canopy, states (with `lasts`/`then`, overlay clip, emitter, tint), tricks (cues, `from`, `to`, mood, clip, emitter), purr and the activities the repertoire covers — in the order of the ensemble.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_species_digest.ts" [--markdown]
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type State = { id: string; lasts?: number; then?: string; clip?: string; emitter?: string; tint?: Record<string, string> };
type Trick = { id: string; cues: string[]; from?: string[]; to?: string; mood?: string; clip: string; emitter?: string };
type Species = { id: string; locomotion: { gait: string }; gear: string[]; mood: string; grip: number; reach: number; canopy?: unknown; states: State[]; tricks: Trick[]; purr: { clip: string; emitter?: string }; repertoire: Record<string, string[]>; emitters: { id: string }[]; clips: { id: string }[] };

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets");
const ensemble = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8")) as { species: string[] };
const markdown = process.argv.includes("--markdown");
const lines: string[] = [];
for (const path of ensemble.species) {
  const species = JSON.parse(readFileSync(resolve(root, path), "utf8")) as Species;
  if (markdown) {
    const states = species.states.map((state, index) => `\`${state.id}\`${index === 0 ? " (rest)" : ""}${state.lasts === undefined ? "" : ` ${state.lasts} s → \`${state.then ?? species.states[0]!.id}\``}`).join(", ");
    const tricks = species.tricks.map((trick) => `\`${trick.id}\` [${trick.cues.join(", ") || "—"}]${trick.from === undefined ? "" : ` from ${trick.from.join("/")}`}${trick.to === undefined ? "" : ` → \`${trick.to}\``}`).join(", ");
    lines.push(`| ${species.id} | ${species.gear.join(", ") || "—"} | ${states} | ${tricks} |`);
    continue;
  }
  lines.push(`## ${species.id} gait=${species.locomotion.gait} gear=[${species.gear.join(",")}] mood=${species.mood} grip=${species.grip} reach=${species.reach} canopy=${species.canopy === undefined ? "plain" : "own"}`);
  for (const [index, state] of species.states.entries()) lines.push(`  state ${index === 0 ? "*" : " "}${state.id} lasts=${state.lasts ?? "-"} then=${state.then ?? "-"} clip=${state.clip ?? "-"} emitter=${state.emitter ?? "-"} tint=${state.tint === undefined ? "-" : JSON.stringify(state.tint)}`);
  for (const trick of species.tricks) lines.push(`  trick ${trick.id} cues=[${trick.cues.join(",")}] from=[${(trick.from ?? []).join(",")}] to=${trick.to ?? "-"} mood=${trick.mood ?? "-"} clip=${trick.clip} emitter=${trick.emitter ?? "-"}`);
  lines.push(`  purr clip=${species.purr.clip} emitter=${species.purr.emitter ?? "-"}`);
  lines.push(`  repertoire ${Object.keys(species.repertoire).join(",")}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
