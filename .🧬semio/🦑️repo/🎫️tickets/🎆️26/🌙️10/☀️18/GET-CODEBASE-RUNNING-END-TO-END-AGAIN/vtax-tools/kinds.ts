// resolves every discovery child directory (old and new) through the live semantic directory engine and checks folder shape: one RGI emoji + VS16, kebab slug, docstring emoji, uniqueness, path length
import { readdirSync, readFileSync } from "node:fs";
import { loadCatalogTaxonomy, semanticDirectoryKindId } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const root = "/Users/ueli/Documents/semio/", discovery = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery";
const taxonomy = loadCatalogTaxonomy();
const kindOf = new Map<string, string | null>();
const resolve = (path: string): string | null => {
  if (kindOf.has(path)) return kindOf.get(path)!;
  const parts = path.split("/"), chain: string[] = [];
  for (let i = parts.length - 1; i >= 1; i -= 1) { const k = resolve(parts.slice(0, i).join("/")); if (k) chain.push(k); }
  const kind = semanticDirectoryKindId(parts[parts.length - 1]!, taxonomy, { parentKindId: chain[0], ancestorKindIds: chain.slice(1) });
  kindOf.set(path, kind);
  return kind;
};
const names = readdirSync(root + discovery, { withFileTypes: true }).filter((e) => e.isDirectory()).map((e) => e.name).sort();
const authorityKind = resolve(`${discovery}/🔐️taxonomy-authority`);
const problems: string[] = [];
const seenEmoji = new Map<string, string>();
let checked = 0, maxPath = 0;
for (const name of names) {
  const kind = resolve(`${discovery}/${name}`);
  const first = [...name][0]!, rest = name.slice(first.length);
  if (kind !== authorityKind && !["📋️policy", "🟦️typescript", "🧪️tests", "🧬️schema", "🗿️artifact", "🗺️surface", "🕸️runtime", "🚶️file-walk", "🔗️schema-reads", "🛠️"].some((n) => name.startsWith(n))) problems.push(`${name}: kind ${kind} differs from authority kind ${authorityKind}`);
  const emoji = [...name.matchAll(/^\p{RGI_Emoji}/gv)][0]?.[0];
  if (!emoji) problems.push(`${name}: no leading RGI emoji`);
  if (seenEmoji.has(emoji ?? "")) problems.push(`${name}: emoji shared with ${seenEmoji.get(emoji ?? "")}`);
  seenEmoji.set(emoji ?? "", name);
  if (/audit|taxonomy-audit-context/.test(name)) {
    checked++;
    const file = `${discovery}/${name}/🟦️.ts`, text = readFileSync(root + file, "utf8");
    maxPath = Math.max(maxPath, Buffer.byteLength(file));
    const slug = name.replace(/^\p{RGI_Emoji}\uFE0F?/v, "");
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(slug)) problems.push(`${name}: slug`);
    const docs = [...text.matchAll(/\/\*\*\s*(\p{RGI_Emoji})/gv)].map((m) => m[1]);
    if (docs.length === 0 || docs.some((d) => d !== emoji)) problems.push(`${name}: docstring emoji ${docs.join(",")} !== folder emoji ${emoji}`);
    if (!name.startsWith(emoji + "️") && emoji && !emoji.includes("️")) problems.push(`${name}: missing VS16`);
  }
}
console.log(JSON.stringify({ authorityKind, siblings: names.length, newFolders: checked, maxPathBytes: maxPath, problems }, null, 1));
