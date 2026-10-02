/** 🔎️ Read-only check (work package A): every directory name of design §11 resolves against the taxonomy on disk, with its exact code points.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/taxonomy_registration_check.ts"
 */
import { loadTaxonomy, semanticDirectoryKindId, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const V = "\u{fe0f}";
const name = (emoji: string, slug: string) => `${emoji}${V}${slug}`;
const taxonomy: any = loadTaxonomy();

const expected: [string, string, string][] = [
  ["products", name("🐾", "pets"), "members-of-products"],
  ...["📐 trigonometry", "🎲 randomness", "🦴 rig", "🎞 animation", "🏞 terrain", "🧠 behavior", "🎪 stage", "✅ validation", "🖌 depiction", "📡 survey", "⏲ pacing", "🫧 layer", "🐾 pets"].map((entry): [string, string, string] => ["modules", name(...(entry.split(" ") as [string, string])), "members-of-modules"]),
  ...[
    "🧬 schema-conformance",
    "📐 turn-trigonometry",
    "🎲 counter-randomness",
    "🦴 rig-solving",
    "👀 gaze-tracking",
    "🎞 animation-sampling",
    "🪀 spring-settling",
    "🏞 terrain-walking",
    "🦘 hop-ballistics",
    "🧠 behavior-choice",
    "🤝 bond-dynamics",
    "🎪 stage-trace",
    "🖌 pet-depiction",
    "📡 surface-survey",
    "⏲ frame-pacing",
    "🫥 decorative-layer",
    "🐾 pet-companions",
    "🐾 pet-cast",
    "🐕 pet-walk",
    "🔬 unit",
  ].map((entry): [string, string, string] => ["tests", name(...(entry.split(" ") as [string, string])), "members-of-tests"]),
  ["tests", name("🎚", "config"), "configuration"],
  ...[
    "🧬 schema-conformance",
    "📐 turn-trigonometry",
    "🎲 counter-randomness",
    "🦴 rig-solving",
    "👀 gaze-tracking",
    "🎞 animation-sampling",
    "🪀 spring-settling",
    "🏞 terrain-walking",
    "🦘 hop-ballistics",
    "🧠 behavior-choice",
    "🤝 bond-dynamics",
    "🎪 stage-trace",
    "🖌 pet-depiction",
    "📡 surface-survey",
    "🐾 pet-companions",
  ].map((entry): [string, string, string] => ["fixtures", name(...(entry.split(" ") as [string, string])), "members-of-fixtures"]),
  ["teaching-architecture", name("🐾", "pets"), "teaching-pets"],
  ...["☀ sunny", "☁ cloudy", "🏠 housy", "🔆 solary", "♨ radiatory", "🌀 pumpy", "🪟 windowy", "🧱 waly", "🔋 battery", "💨 windy", "🔥 boily", "🛖 roofy", "🧶 insuly", "😎 shady", "🌬 venty", "❄ chilly", "🫖 kettly", "🕯 flamy", "🌡 thermy", "🖥 servy"].map(
    (entry): [string, string, string] => ["teaching-pets", name(...(entry.split(" ") as [string, string])), "members-of-teaching-pets"],
  ),
];

let failed = 0;
for (const [parent, entry, kind] of expected) {
  const resolved = String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: parent }));
  if (resolved !== kind) {
    failed += 1;
    console.log("MISMATCH", parent, entry, "expected", kind, "resolved", resolved);
  }
}
const lists = ["members-of-products", "members-of-modules", "members-of-tests", "members-of-fixtures", "members-of-teaching-pets"];
for (const list of lists) {
  const names: string[] = taxonomy.semanticDirectoryMemberKinds[list].memberNames;
  const repeated = names.filter((entry, index) => names.indexOf(entry) !== index);
  if (repeated.length > 0) {
    failed += 1;
    console.log("DUPLICATE in", list, repeated.join(" "));
  }
}
const problems = validateTaxonomy(taxonomy);
for (const problem of problems) console.log("PROBLEM", problem);
console.log(`checked=${expected.length} mismatches=${failed} taxonomyProblems=${problems.length}`);
process.exit(failed > 0 || problems.length > 0 ? 1 : 0);
