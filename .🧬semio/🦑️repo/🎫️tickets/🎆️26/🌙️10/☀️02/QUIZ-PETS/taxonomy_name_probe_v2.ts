/** 🔎️ Read-only probe for the second round: resolves every new directory name of design §14–§22 against an in-memory copy of the taxonomy. */
import { loadTaxonomy, semanticDirectoryKindId, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const V = "️";
const name = (emoji: string, slug: string) => `${emoji}${V}${slug}`;
const taxonomy: any = structuredClone(loadTaxonomy());
const baseline = validateTaxonomy(loadTaxonomy());

const plan: Record<string, { owner: string; list: string; names: string[] }> = {
  modules: {
    owner: "modules",
    list: "members-of-modules",
    names: [
      name("📝", "draft"),
      name("🚧", "clearance"),
      name("🗓", "schedule"),
      name("👀", "attention"),
      name("🚶", "locomotion"),
      name("💞", "sociability"),
      name("🎯", "choice"),
      name("👥", "population"),
      name("🕰", "clock"),
      name("🎥", "projection"),
      name("🪢", "swing"),
      name("🧗", "climbing"),
      name("👆", "gesture"),
      name("💗", "feeling"),
      name("✨", "effects"),
      name("🪄", "mischief"),
      name("🤏", "grasp"),
      name("🪞", "lifting"),
      name("🧰", "gear"),
    ],
  },
  tests: {
    owner: "tests",
    list: "members-of-tests",
    names: [
      name("🪢", "swing-dynamics"),
      name("🪂", "parachute-descent"),
      name("🧗", "wall-climbing"),
      name("🪜", "ladder-geometry"),
      name("🎣", "grapple-reach"),
      name("🚧", "clearance-proof"),
      name("👆", "gesture-recognition"),
      name("💗", "feeling-dynamics"),
      name("⚗", "chemistry-rules"),
      name("✨", "particle-motion"),
      name("🪄", "mischief-choice"),
      name("🤏", "pet-handling"),
      name("🪞", "fixture-lifting"),
      name("🎆", "effect-painting"),
      name("🧰", "gear-depiction"),
      name("🎮", "pet-play"),
    ],
  },
  fixtures: {
    owner: "fixtures",
    list: "members-of-fixtures",
    names: [
      name("🪢", "swing-dynamics"),
      name("🪂", "parachute-descent"),
      name("🧗", "wall-climbing"),
      name("🪜", "ladder-geometry"),
      name("🎣", "grapple-reach"),
      name("🚧", "clearance-proof"),
      name("👆", "gesture-recognition"),
      name("💗", "feeling-dynamics"),
      name("⚗", "chemistry-rules"),
      name("✨", "particle-motion"),
      name("🪄", "mischief-choice"),
    ],
  },
};

const pairs = new Set<string>();
for (const spec of Object.values<any>(taxonomy.semanticDirectoryMemberKinds)) for (const owner of spec.ownerKindIds) for (const member of spec.memberNames) pairs.add(`${owner}\0${member}`);

console.log("== before registration");
for (const group of Object.values(plan)) for (const entry of group.names) console.log(group.owner.padEnd(10), entry.padEnd(26), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: group.owner })));
for (const group of Object.values(plan)) {
  const fresh = group.names.filter((entry) => !pairs.has(`${group.owner}\0${entry}`));
  taxonomy.semanticDirectoryMemberKinds[group.list].memberNames.push(...fresh);
  console.log("register", group.list, fresh.length, "fresh,", group.names.length - fresh.length, "already present");
}
console.log("== after registration");
for (const group of Object.values(plan)) for (const entry of group.names) console.log(group.owner.padEnd(10), entry.padEnd(26), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: group.owner })));
const after = validateTaxonomy(taxonomy);
console.log("validateTaxonomy problems baseline:", baseline.length, "after:", after.length);
for (const problem of after.filter((entry: string) => !baseline.includes(entry))) console.log("NEW PROBLEM:", problem);
