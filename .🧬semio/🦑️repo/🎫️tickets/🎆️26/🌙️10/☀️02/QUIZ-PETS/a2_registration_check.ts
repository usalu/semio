/** 🧾️ Read-only check of work package A2: every name of the second round resolves in the committed taxonomy, and the taxonomy still validates. */
import { loadTaxonomy, semanticDirectoryKindId, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const V = "️";
const name = (emoji: string, slug: string) => `${emoji}${V}${slug}`;
const taxonomy: any = loadTaxonomy();
const cases = ["🪢 swing-dynamics", "🪂 parachute-descent", "🧗 wall-climbing", "🪜 ladder-geometry", "🎣 grapple-reach", "🚧 clearance-proof", "👆 gesture-recognition", "💗 feeling-dynamics", "⚗ chemistry-rules", "✨ particle-motion", "🪄 mischief-choice"];
const plan: Record<string, string[]> = {
  modules: ["📝 draft", "🚧 clearance", "📏 spacing", "🗓 schedule", "👀 attention", "🚶 locomotion", "💞 sociability", "🎯 choice", "👥 population", "🕰 clock", "🎥 projection", "🪢 swing", "🧗 climbing", "👆 gesture", "💗 feeling", "✨ effects", "🪄 mischief", "🤏 grasp", "🪞 lifting", "🧰 gear"],
  tests: [...cases, "🤏 pet-handling", "🪞 fixture-lifting", "🎆 effect-painting", "🧰 gear-depiction", "🎮 pet-play"],
  fixtures: cases,
};

let missing = 0;
for (const [owner, entries] of Object.entries(plan)) {
  for (const entry of entries) {
    const [emoji, slug] = entry.split(" ");
    const kind = semanticDirectoryKindId(name(emoji, slug), taxonomy, { parentKindId: owner });
    if (kind === null || kind === undefined) missing += 1;
    console.log(owner.padEnd(10), name(emoji, slug).padEnd(26), String(kind));
  }
}
const counts = new Map<string, number>();
for (const list of ["members-of-modules", "members-of-tests", "members-of-fixtures"]) for (const member of taxonomy.semanticDirectoryMemberKinds[list].memberNames) counts.set(`${list} ${member}`, (counts.get(`${list} ${member}`) ?? 0) + 1);
const repeated = [...counts].filter(([, count]) => count > 1).map(([key]) => key);
console.log("missing:", missing, "repeated:", repeated.length, repeated.join(", "));
console.log("validateTaxonomy problems:", validateTaxonomy(taxonomy).length);
