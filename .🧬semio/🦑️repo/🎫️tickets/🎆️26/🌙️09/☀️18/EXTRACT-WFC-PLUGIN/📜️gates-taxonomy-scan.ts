/** 🔎️ Reports every child directory of a semantic-collection owner whose name is neither a registered
 * `memberNames` entry of that owner nor itself a `semanticDirectoryKinds` name admissible under it.
 *
 * Positional rather than recursive-classifying: a directory is an owner when its own basename matches
 * an owner kind's `emoji + slugPattern`. That is exactly where `members-of-*` registration is read, and
 * a name missing there makes discovery return zero rows for that owner while the gate still reads green.
 * A basename can match several kinds, so every match contributes its member registries.
 *
 * usage: bun 📜️gates-taxonomy-scan.ts <dir> [<dir> …]
 */
import { readdirSync, statSync } from "node:fs";
import { basename, join, relative } from "node:path";
import { loadTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const taxonomy = loadTaxonomy();
const kinds = taxonomy.semanticDirectoryKinds as Record<string, { emoji: string; slugPattern: string; allowEmojiOnly?: boolean; parentKindIds?: readonly string[] }>;
const ownerMembers = new Map<string, Set<string>>();
for (const spec of Object.values(taxonomy.semanticDirectoryMemberKinds)) {
  for (const owner of spec.ownerKindIds) {
    const set = ownerMembers.get(owner) ?? new Set<string>();
    for (const name of spec.memberNames) set.add(name.normalize("NFC"));
    ownerMembers.set(owner, set);
  }
}

/** 🏷️ Every `semanticDirectoryKinds` id whose emoji and slug pattern accept this bare directory name. */
function kindsOf(name: string): string[] {
  const hits: string[] = [];
  for (const [id, spec] of Object.entries(kinds)) {
    if (!spec.emoji || !name.startsWith(spec.emoji)) continue;
    const slug = name.slice(spec.emoji.length);
    if ((slug.length === 0 && spec.allowEmojiOnly) || (slug.length > 0 && new RegExp(`^(?:${spec.slugPattern.replace(/^\^|\$$/gu, "")})$`, "u").test(slug))) hits.push(id);
  }
  return hits;
}

let total = 0;
for (const target of process.argv.slice(2)) {
  const walk = (abs: string): void => {
    const children = readdirSync(abs).filter((entry) => !entry.startsWith(".") && statSync(join(abs, entry)).isDirectory()).sort();
    const ownerKinds = kindsOf(basename(abs).normalize("NFC"));
    const registries = ownerKinds.filter((id) => ownerMembers.has(id));
    if (registries.length > 0) {
      const admitted = new Set<string>(registries.flatMap((id) => [...ownerMembers.get(id)!]));
      for (const child of children) {
        const name = child.normalize("NFC");
        if (admitted.has(name)) continue;
        if (kindsOf(name).some((id) => !kinds[id]!.parentKindIds || kinds[id]!.parentKindIds!.some((parent) => ownerKinds.includes(parent)))) continue;
        total += 1;
        console.log(JSON.stringify({ path: relative(process.cwd(), join(abs, child)), owners: registries, name }));
      }
    }
    for (const child of children) walk(join(abs, child));
  };
  walk(target);
}
console.log(`[member scan] ${total} unregistered member names`);
