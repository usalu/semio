/** 🏷️ N1 one-off codemod: renames every norm test-case directory the contract flags `test-case-name` to one canonical emoji + kebab slug, and rewrites the `#[path]` mounts naming it. */
import { readFileSync, renameSync, readdirSync, statSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname, basename } from "node:path";
import { leadingEmojiIdentity, pathEmojiStatuteFindings, loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const write = process.argv.includes("--write");
const breaches = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { id: string; scope: string; priority: string }[];
const taxonomy = loadCatalogTaxonomy(root) as { pathEmojiPolicy: { genericEmojiIdentities: string[] } };
const kebab = (s: string) => s.replace(/([a-z0-9])([A-Z])/g, "$1-$2").replace(/([A-Z])([A-Z][a-z])/g, "$1-$2").toLowerCase();
const canonical = (name: string) => { const id = leadingEmojiIdentity(name); return !!id.first && /^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id.rest) && pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length === 0; };
const cases = [...new Set(breaches.filter((b) => b.id === "test-case-name" && b.scope.includes("📕️norm")).map((b) => dirname(b.scope)))];
const renames: [string, string][] = [];
for (const rel of cases.filter((c) => existsSync(join(root, c)))) {
  const old = basename(rel), leaf = basename(dirname(dirname(rel)));
  const id = leadingEmojiIdentity(old);
  const leafEmoji = leadingEmojiIdentity(leaf).first ?? "";
  let emoji = id.first ?? "";
  if (!emoji || /^\p{Script=Greek}/u.test(old)) emoji = leafEmoji;
  let rest = id.first ? id.rest : old.replace(/^\p{Script=Greek}+/u, "");
  const candidates = [emoji, `${emoji}️`, `${emoji.replace(/️/g, "")}️`].map((e) => `${e}${kebab(rest)}`);
  const next = candidates.find(canonical);
  if (!next) { console.log(`NO CANONICAL NAME for ${rel} (tried ${candidates.join(" ")})`); continue; }
  renames.push([rel, join(dirname(rel), next)]);
}
const crateFiles = (dir: string): string[] => readdirSync(dir).flatMap((e) => { const p = join(dir, e); return statSync(p).isDirectory() ? crateFiles(p) : p.endsWith(".rs") ? [p] : []; });
const families = [...new Set(renames.map(([rel]) => rel.split("/").slice(0, 5).join("/")))];
const sources = families.flatMap((f) => crateFiles(join(root, f)));
let edited = 0;
for (const [from, to] of renames) {
  const oldSeg = `🧪️tests/${basename(from)}/`, newSeg = `🧪️tests/${basename(to)}/`;
  const users = sources.filter((p) => existsSync(p) && readFileSync(p, "utf8").includes(oldSeg) && (p.startsWith(join(root, dirname(dirname(dirname(from))))) || true));
  console.log(`${from.split("/").slice(-4).join("/")} -> ${basename(to)}  mounts: ${users.length}`);
  if (!write) continue;
  renameSync(join(root, from), join(root, to));
  for (const p of users) {
    const text = readFileSync(p, "utf8");
    const leafSeg = `${basename(dirname(dirname(from)))}/${oldSeg}`;
    if (!text.includes(leafSeg)) continue;
    writeFileSync(p, text.split(leafSeg).join(`${basename(dirname(dirname(to)))}/${newSeg}`)); edited++;
  }
  const impl = join(root, to, "🦀️.rs");
  if (existsSync(impl)) { const t = readFileSync(impl, "utf8"); const o = leadingEmojiIdentity(basename(from)).rest || basename(from); writeFileSync(impl, t.split(`\`${o}\``).join(`\`${leadingEmojiIdentity(basename(to)).rest}\``)); }
}
console.log(`${renames.length} renames, ${edited} mount edits${write ? "" : " (dry run)"}`);
