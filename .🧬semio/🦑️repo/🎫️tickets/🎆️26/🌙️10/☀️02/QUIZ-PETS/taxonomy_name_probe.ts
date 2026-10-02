/** 🔎️ Read-only probe: resolves every directory name the pets design plans against an in-memory copy of the taxonomy. */
import { loadTaxonomy, semanticDirectoryKindId, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const V = "️";
const name = (emoji: string, slug: string) => `${emoji}${V}${slug}`;
const taxonomy: any = structuredClone(loadTaxonomy());
const baseline = validateTaxonomy(loadTaxonomy());

const plan: Record<string, { owner: string; list?: string; names: string[] }> = {
  products: { owner: "products", list: "members-of-products", names: [name("🐾", "pets")] },
  modules: {
    owner: "modules",
    list: "members-of-modules",
    names: [
      name("📐", "trigonometry"),
      name("🎲", "randomness"),
      name("🦴", "rig"),
      name("🎞", "animation"),
      name("🏞", "terrain"),
      name("🧠", "behavior"),
      name("🎪", "stage"),
      name("✅", "validation"),
      name("🖌", "depiction"),
      name("📡", "survey"),
      name("⏲", "pacing"),
      name("🫧", "layer"),
      name("🐾", "pets"),
    ],
  },
  tests: {
    owner: "tests",
    list: "members-of-tests",
    names: [
      name("🧬", "schema-conformance"),
      name("📐", "turn-trigonometry"),
      name("🎲", "counter-randomness"),
      name("🦴", "rig-solving"),
      name("👀", "gaze-tracking"),
      name("🎞", "animation-sampling"),
      name("🪀", "spring-settling"),
      name("🏞", "terrain-walking"),
      name("🦘", "hop-ballistics"),
      name("🧠", "behavior-choice"),
      name("🤝", "bond-dynamics"),
      name("🎪", "stage-trace"),
      name("🩺", "document-validation"),
      name("🖌", "pet-depiction"),
      name("📡", "surface-survey"),
      name("⏲", "frame-pacing"),
      name("🫥", "decorative-layer"),
      name("🐾", "pet-companions"),
      name("🐾", "pet-cast"),
      name("🐕", "pet-walk"),
      name("🔬", "unit"),
      name("🎚", "config"),
    ],
  },
  fixtures: {
    owner: "fixtures",
    list: "members-of-fixtures",
    names: [
      name("🧬", "schema-conformance"),
      name("📐", "turn-trigonometry"),
      name("🎲", "counter-randomness"),
      name("🦴", "rig-solving"),
      name("👀", "gaze-tracking"),
      name("🎞", "animation-sampling"),
      name("🪀", "spring-settling"),
      name("🏞", "terrain-walking"),
      name("🦘", "hop-ballistics"),
      name("🧠", "behavior-choice"),
      name("🤝", "bond-dynamics"),
      name("🎪", "stage-trace"),
      name("🖌", "pet-depiction"),
      name("📡", "surface-survey"),
      name("🐾", "pet-companions"),
    ],
  },
};

const pairs = new Set<string>();
for (const spec of Object.values<any>(taxonomy.semanticDirectoryMemberKinds)) for (const owner of spec.ownerKindIds) for (const member of spec.memberNames) pairs.add(`${owner}\0${member}`);

console.log("== before registration");
for (const group of Object.values(plan)) for (const entry of group.names) console.log(group.owner.padEnd(10), entry.padEnd(28), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: group.owner })));

for (const group of Object.values(plan)) {
  if (!group.list) continue;
  const fresh = group.names.filter((entry) => !pairs.has(`${group.owner}\0${entry}`));
  taxonomy.semanticDirectoryMemberKinds[group.list].memberNames.push(...fresh);
  console.log("register", group.list, fresh.length, "fresh,", group.names.length - fresh.length, "already present");
}

taxonomy.semanticDirectoryKinds["teaching-pets"] = { emoji: `🐾${V}`, slugPattern: "^pets$", allowEmojiOnly: false, parentKindIds: ["teaching-architecture"] };
const species = [
  name("☀", "sunny"),
  name("☁", "cloudy"),
  name("🏠", "housy"),
  name("🔆", "solary"),
  name("♨", "radiatory"),
  name("🌀", "pumpy"),
  name("🪟", "windowy"),
  name("🧱", "waly"),
  name("🔋", "battery"),
  name("💨", "windy"),
  name("🔥", "boily"),
  name("🛖", "roofy"),
  name("🧶", "insuly"),
  name("😎", "shady"),
  name("🌬", "venty"),
  name("❄", "chilly"),
  name("🫖", "kettly"),
  name("🕯", "flamy"),
  name("🌡", "thermy"),
  name("🖥", "servy"),
];
taxonomy.semanticDirectoryMemberKinds["members-of-teaching-pets"] = { ownerKindIds: ["teaching-pets"], memberNames: species, source: "registry" };

console.log("== after registration");
for (const group of Object.values(plan)) for (const entry of group.names) console.log(group.owner.padEnd(10), entry.padEnd(28), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: group.owner })));
console.log("teaching-architecture", name("🐾", "pets"), String(semanticDirectoryKindId(name("🐾", "pets"), taxonomy, { parentKindId: "teaching-architecture" })));
for (const entry of species) console.log("teaching-pets".padEnd(14), entry.padEnd(16), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: "teaching-pets" })));

for (const parent of ["react-target", "product", "products", "pets", "teaching-pets", "teaching-quiz"]) {
  for (const entry of [name("🛝", "playground"), name("🏗", "builder"), name("🔨", "modules"), name("🧪", "tests"), name("🧫", "fixtures"), name("🔮", "oracles"), name("🧬", "schema"), name("📦", "packages"), name("🎯", "targets")]) {
    console.log("context".padEnd(8), parent.padEnd(16), entry.padEnd(16), String(semanticDirectoryKindId(entry, taxonomy, { parentKindId: parent })));
  }
}
const related = Object.entries<any>(taxonomy.semanticDirectoryKinds).filter(([id, kind]) => /playground|gallery|stories|demo|showcase/.test(id) || /playground|gallery|stories|demo|showcase/.test(String(kind.slugPattern)));
for (const [id, kind] of related) console.log("kind", id, kind.emoji, kind.slugPattern, JSON.stringify(kind.parentKindIds ?? null));

const after = validateTaxonomy(taxonomy);
console.log("validateTaxonomy problems baseline:", baseline.length, "after:", after.length);
for (const problem of after.filter((entry: string) => !baseline.includes(entry))) console.log("NEW PROBLEM:", problem);
