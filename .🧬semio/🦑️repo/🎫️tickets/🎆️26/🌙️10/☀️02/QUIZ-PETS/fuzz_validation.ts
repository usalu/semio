#!/usr/bin/env bun
/** 🧨️ Ticket tool of work package K: breaks the sample documents of the schema-conformance fixture in a few thousand random ways and records what the TypeScript validator finds, so the Rust validator can be held to the same findings.
 *
 * Every mutant applies one to three mutations to a sample document (a species, the menagerie or the ensemble):
 * a number replaced, a string replaced (ids, references, colours, literals), a boolean flipped, a list entry removed,
 * repeated or swapped, an object member removed. The choice is a pure function of the mutant's index (the module's
 * own counter-based randomness). From the repository root:
 *
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/fuzz_validation.ts
 *
 * writes `🗑️generated/wp-k/validation-mutants.json` beside this file; `compare_validation_findings.rs` reads it in the
 * scratch crate (`bash rust_scratch.sh wp-k test --offline findings -- --nocapture`).
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { randomWords } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎲️randomness/🟦️.ts";
import { ensembleIssues, menagerieIssues, speciesIssues } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "../../../../../../..");
const SEED = 20261004;
const MUTANTS = 6000;
const NUMBERS = [0, -1, 1, 1.5, 2, 0.5, 0.25, 360, -360, 720, 180, 1e-9, 100, -0.5, 0.999, 1.001];
const STRINGS = ["", "x", "Bad Id", "root", "body", "blobby", "hoppy", "floaty", "nobody", "#12345g", "#abcdef", "#ABCDEF", "walk", "hop", "float", "idle", "rotation", "x-", "a--b", "breathe", "stroll", "semio.pets.menagerie/v1", "semio.pets.ensemble/v1", "path", "ink"];

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type Place = { readonly holder: Json[] | { [key: string]: Json }; readonly key: string | number };

/** 🧭️ Every place of a document: each list entry and each object member, in document order. */
function places(value: Json, found: Place[] = []): Place[] {
  if (Array.isArray(value)) value.forEach((entry, index) => (found.push({ holder: value, key: index }), places(entry, found)));
  else if (value !== null && typeof value === "object") for (const [key, entry] of Object.entries(value)) (found.push({ holder: value, key }), places(entry, found));
  return found;
}

/** 🎲️ A draw source for one mutant: successive words of its key. */
function draws(index: number): (bound: number) => number {
  const words = randomWords([SEED, index], 64);
  let next = 0;
  return (bound) => words[next++ % words.length]! % bound;
}

/** 🔧️ One mutation at a drawn place; returns its name. */
function mutate(document: Json, draw: (bound: number) => number): string {
  const all = places(document);
  if (all.length === 0) return "nothing";
  const place = all[draw(all.length)]!;
  const holder = place.holder as Record<string | number, Json>;
  const value = holder[place.key]!;
  const action = draw(8);
  if (Array.isArray(place.holder) && action < 3) {
    const list = place.holder;
    const at = place.key as number;
    if (action === 0) return (list.splice(at, 1), "entry-removed");
    if (action === 1) return (list.splice(at, 0, structuredClone(list[at]!)), "entry-repeated");
    const other = draw(list.length);
    [list[at], list[other]] = [list[other]!, list[at]!];
    return "entries-swapped";
  }
  if (!Array.isArray(place.holder) && action === 0) return (delete (place.holder as Record<string, Json>)[place.key as string], "member-removed");
  if (typeof value === "number") return ((holder[place.key] = NUMBERS[draw(NUMBERS.length)]!), "number-replaced");
  if (typeof value === "string") return ((holder[place.key] = STRINGS[draw(STRINGS.length)]!), "string-replaced");
  if (typeof value === "boolean") return ((holder[place.key] = !value), "boolean-flipped");
  if (Array.isArray(value) && value.length > 0) return (value.splice(draw(value.length), 1), "entry-removed");
  return "untouched";
}

const fixture = JSON.parse(readFileSync(resolve(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as { menagerie: Json; ensemble: Json; species: { document: Json }[]; bases: { species: Json; menagerie: Json; ensemble: Json } };
const samples: { readonly definition: "Species" | "Menagerie" | "Ensemble"; readonly document: Json }[] = [
  ...fixture.species.map((entry) => ({ definition: "Species" as const, document: entry.document })),
  { definition: "Species", document: fixture.bases.species },
  { definition: "Menagerie", document: fixture.menagerie },
  { definition: "Menagerie", document: fixture.bases.menagerie },
  { definition: "Ensemble", document: fixture.ensemble },
  { definition: "Ensemble", document: fixture.bases.ensemble },
];
const judges = { Species: speciesIssues, Menagerie: menagerieIssues, Ensemble: ensembleIssues };
const tally = new Map<string, number>();
const mutants = Array.from({ length: MUTANTS }, (_, index) => {
  const draw = draws(index);
  const sample = samples[index % samples.length]!;
  const document = structuredClone(sample.document);
  const applied = Array.from({ length: 1 + draw(3) }, () => mutate(document, draw));
  const issues = judges[sample.definition](document);
  for (const issue of issues) tally.set(issue.code, (tally.get(issue.code) ?? 0) + 1);
  return { index, definition: sample.definition, applied, document, issues };
});
const target = resolve(HERE, "🗑️generated/wp-k/validation-mutants.json");
mkdirSync(dirname(target), { recursive: true });
writeFileSync(target, `${JSON.stringify({ $comment: "Generated by fuzz_validation.ts of ticket QUIZ-PETS (work package K) from the TypeScript validator — a proof run, not a fixture.", mutants })}\n`);
process.stdout.write(`mutants=${mutants.length} valid=${mutants.filter((mutant) => mutant.issues.length === 0).length} findings=${mutants.reduce((sum, mutant) => sum + mutant.issues.length, 0)}\n`);
for (const [code, count] of [...tally].sort()) process.stdout.write(`  ${code}: ${count}\n`);
