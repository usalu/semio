/** 🗂️ Idempotent taxonomy registration of this ticket's directories (site-infra). The 🎓️teaching tree gets contextual kinds and
 * member lists (leutwiler precedent, scoped by parentKindIds); every directory the fast inventory still reports as
 * `directory-kind-unresolved` under a 🔨️modules, 🧪️tests, 🧫️fixtures or 🛍️products parent joins that parent's member list.
 * Re-run after other work packages add directories: `bun site_infra_register_taxonomy.ts [--dry]`. */
import { readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

type Json = Record<string, any>;
const VS = "️";
const root = join(import.meta.dir, "../../../../../../..");
const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const text = readFileSync(path, "utf8");
const taxonomy = JSON.parse(text) as Json;
const dry = process.argv.includes("--dry");
const scopes = ["🎓️teaching", "🧰️framework/🛍️products/❓️quiz"];
const AREA = `🎓${VS}teaching`;

const insertAfter = (object: Json, after: string, entries: Json): Json => {
  const missing = Object.entries(entries).filter(([key]) => !(key in object));
  if (missing.length === 0) return object;
  if (!(after in object)) throw new Error(`anchor ${after} missing`);
  const out: Json = {};
  for (const [key, value] of Object.entries(object)) {
    out[key] = value;
    if (key === after) for (const [newKey, newValue] of missing) out[newKey] = newValue;
  }
  return out;
};
const kind = (emoji: string, slug: string, parents?: string[]): Json => ({ emoji: emoji + VS, slugPattern: `^${slug}$`, allowEmojiOnly: false, ...(parents ? { parentKindIds: parents } : {}) });
const members = (owners: string[], names: string[]): Json => ({ ownerKindIds: owners, memberNames: names.map((name) => name.normalize("NFC")), source: "registry" });
const topics = ["teaching-physics", "teaching-heating", "teaching-cooling", "teaching-demand"];

taxonomy.semanticDirectoryKinds = insertAfter(taxonomy.semanticDirectoryKinds, "leutwiler-lean", {
  teaching: kind("🎓", "teaching"),
  "teaching-proctor": kind("🛂", "proctor", ["teaching"]),
  "teaching-architecture": kind("🏛", "architecture", ["teaching"]),
  "teaching-energy": kind("⚡", "energy", ["teaching-architecture"]),
  "teaching-physics": kind("🧲", "physics", ["teaching-energy"]),
  "teaching-heating": kind("🔥", "heating", ["teaching-energy"]),
  "teaching-cooling": kind("❄", "cooling", ["teaching-energy"]),
  "teaching-demand": kind("📊", "demand", ["teaching-energy"]),
  "teaching-quiz": kind("❓", "quiz", ["teaching-architecture", ...topics]),
  deploy: kind("🚀", "deploy"),
});
taxonomy.semanticDirectoryMemberKinds = insertAfter(taxonomy.semanticDirectoryMemberKinds, "members-of-leutwiler-realparts-of-powers-z-n", {
  "members-of-teaching": members(["teaching"], [`🛂${VS}proctor`, `🏛${VS}architecture`]),
  "members-of-teaching-architecture": members(["teaching-architecture"], [`❓${VS}quiz`, `⚡${VS}energy`]),
  "members-of-teaching-energy": members(["teaching-energy"], [`🧲${VS}physics`, `🔥${VS}heating`, `❄${VS}cooling`, `📊${VS}demand`]),
  "members-of-teaching-topics": members(topics, [`❓${VS}quiz`]),
});
taxonomy.fixedFilenameContracts = insertAfter(taxonomy.fixedFilenameContracts, "caddyfile", {
  "docker-compose": { pathPattern: "**/compose.yaml", authority: "Docker Compose", reason: "Compose file discovery", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "docker compose config", expires: null },
});
taxonomy.areas = insertAfter(taxonomy.areas, `👴${VS}leutwiler`, { [AREA]: "clean" });
taxonomy.areaLayers = insertAfter(taxonomy.areaLayers, `👴${VS}leutwiler`, { [AREA]: "implementation" });

const staged = JSON.stringify(taxonomy, null, 2) + "\n";
if (!dry) writeFileSync(path, staged);
const byParent: Record<string, string> = { [`🔨${VS}modules`]: "members-of-modules", [`🧪${VS}tests`]: "members-of-tests", [`🧫${VS}fixtures`]: "members-of-fixtures", [`🛍${VS}products`]: "members-of-products" };
const additions: Record<string, Set<string>> = {};
const unhandled: string[] = [];
for (const scope of scopes) {
  for (const violation of inventoryTaxonomy({ repoRoot: root, scope }).violations) {
    if (violation.code !== "directory-kind-unresolved") continue;
    const owner = byParent[basename(dirname(violation.path)).normalize("NFC")];
    if (owner) (additions[owner] ??= new Set()).add(basename(violation.path).normalize("NFC"));
    else unhandled.push(violation.path);
  }
}
for (const [owner, names] of Object.entries(additions)) {
  const spec = taxonomy.semanticDirectoryMemberKinds[owner];
  const fresh = [...names].filter((name) => !spec.memberNames.includes(name));
  spec.memberNames.push(...fresh);
  console.log(`[DEBUG] ${owner} += ${fresh.join(" ") || "(nothing new)"}`);
}
const final = JSON.stringify(taxonomy, null, 2) + "\n";
if (!dry) writeFileSync(path, final);
console.log(`[DEBUG] ${dry ? "dry run" : "written"}; changed=${final !== text}; unhandled unresolved directories: ${unhandled.join(", ") || "none"}`);
