/** 🔎️ Compares the semantic kind of every existing `🧩️component` directory, and of every new component facet, with and without the wave L1
 * taxonomy admission (`component-crate`, `hub-compositions`, `members-of-hub-compositions`). */
import { execSync } from "node:child_process";
import { readdirSync } from "node:fs";
import { semanticDirectoryKindId, loadTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const repo = "/Users/ueli/Documents/semio";
const base = loadTaxonomy();
const COMPONENT_PARENTS = ["members-of-hub-compositions", "members-of-plan", "members-of-modules", "members-of-operating-system", "members-of-elements", "wgpu-target", "distribution-puzzle", "stdio", "text", "windows"];
const compositions = readdirSync(`${repo}/🌎️hub/🧩️compositions`, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !["🧪️tests", "🧫️fixtures", "🧬️schema"].includes(entry.name)).map((entry) => entry.name.normalize("NFC")).sort();
const candidate = process.argv.includes("--current") ? base : { ...base,
  semanticDirectoryKinds: { ...base.semanticDirectoryKinds, "hub-compositions": { emoji: "🧩️", slugPattern: "^compositions$", allowEmojiOnly: false, parentKindIds: ["hub"] }, "component-crate": { emoji: "🧩️", slugPattern: "^component$", allowEmojiOnly: false, parentKindIds: COMPONENT_PARENTS } },
  semanticDirectoryMemberKinds: { ...base.semanticDirectoryMemberKinds, "members-of-hub-compositions": { ownerKindIds: ["hub-compositions"], memberNames: compositions, source: "registry" } } } as typeof base;
const kinds = (taxonomy: typeof base, path: string): string => { const ancestors: string[] = []; let last = "?"; for (const segment of path.split("/")) { const kind = semanticDirectoryKindId(segment, taxonomy, ancestors.length ? { parentKindId: ancestors[ancestors.length - 1], ancestorKindIds: [...ancestors].reverse() } : {}); last = kind ?? "?"; if (kind) ancestors.push(kind); } return last; };
const tracked = execSync("git ls-files", { cwd: repo, encoding: "utf8", maxBuffer: 1 << 30 }).split("\n").filter((path) => !path.startsWith(".🧬semio"));
const existing = [...new Set(tracked.flatMap((path) => { const index = path.lastIndexOf("/🧩️component/"); return index >= 0 ? [path.slice(0, index + "/🧩️component".length)] : []; }))];
let changed = 0;
for (const dir of existing) { const before = kinds(base, dir), after = kinds(candidate, dir); if (before !== after) { changed++; console.log(`existing ${before} -> ${after}  ${dir}`); } }
const facets = execSync("git ls-files --cached --others --exclude-standard -- '*/🧩️component/📦️packages/🦀️rust/Cargo.toml'", { cwd: repo, encoding: "utf8" }).trim().split("\n").filter(Boolean).map((path) => path.replace("/📦️packages/🦀️rust/Cargo.toml", ""));
const unadmitted = facets.filter((dir) => kinds(candidate, dir) !== "component-crate");
for (const dir of unadmitted) console.log(`facet ${kinds(candidate, dir)}  ${dir}`);
const hubOwners = compositions.map((name) => `🌎️hub/🧩️compositions/${name}`).filter((dir) => kinds(candidate, dir) === "?");
console.log(`[DEBUG] existing=${existing.length} changed=${changed} facets=${facets.length} unadmitted=${unadmitted.length} hub-owners-unresolved=${hubOwners.length} compositions=${compositions.length}`);
