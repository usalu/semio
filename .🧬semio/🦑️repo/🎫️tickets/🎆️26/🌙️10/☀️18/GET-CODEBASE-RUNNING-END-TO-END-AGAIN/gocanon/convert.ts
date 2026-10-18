import {existsSync, mkdirSync, readdirSync, readFileSync, renameSync, writeFileSync} from "node:fs";
import {basename, dirname, join} from "node:path";
import {createHash} from "node:crypto";
const root = "/Users/ueli/Documents/semio";
const C = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const L = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const {loadCatalogTaxonomy, semanticDirectoryKindId} = await import(`${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const {canonicalTaxonomyEmoji} = await import(`${L}/🔍️discovery/📜️taxonomy-contracts/🟦️.ts`);
const {leadingEmojiIdentity} = await import(`${root}/🧰️framework/🔨️modules/🪪️identity/🧩️grapheme/🟦️.ts`);
const {foldPathEmojiIdentity} = await import(`${root}/🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts`);

const [slug, mode = "plan"] = process.argv.slice(2);
if (!slug) throw new Error("usage: convert.ts <moduleSlug> [plan|apply]");
const entry = readFileSync(`${C}/gocanon/modules.txt`, "utf8").split("\n").map((line) => line.trim().split(/\s+/u)).find((row) => row[1] === slug);
if (!entry) throw new Error(`unknown module ${slug}`);
const pkg = entry[0]!;
const owner = dirname(dirname(pkg));
const casesDir = `${owner}/🧪️tests`;
const t = structuredClone(loadCatalogTaxonomy());
const MEMBERS = "members-of-members-of-members-of-modules";
const members = t.semanticDirectoryMemberKinds[MEMBERS]!;
if (!members.ownerKindIds.includes("go-language")) members.ownerKindIds.push("go-language");
const registered = new Set<string>(members.memberNames);
const fold = (value: string): string => foldPathEmojiIdentity(value);
const generic = new Set<string>((t.pathEmojiPolicy.genericEmojiIdentities as string[]).map(fold));
const slugPattern = new RegExp(t.testCaseSlugPattern, "u");

const chain = (relDir: string) => {
  let parent: string | undefined;
  let anc: string[] = [];
  let kind: string | null = null;
  for (const seg of relDir.split("/")) {
    kind = semanticDirectoryKindId(seg, t, { parentKindId: parent, ancestorKindIds: [...anc] });
    if (kind) { if (parent) anc = [parent, ...anc]; parent = kind; }
  }
  return kind;
};

const pool: string[] = [];
{
  const seen = new Set<string>();
  for (const spec of Object.values(t.semanticDirectoryMemberKinds) as { memberNames: string[] }[]) for (const name of spec.memberNames) {
    const first = leadingEmojiIdentity(name).first;
    if (!first || seen.has(fold(first))) continue;
    seen.add(fold(first));
    if (generic.has(fold(first))) continue;
    if (!canonicalTaxonomyEmoji(first)) continue;
    const point = first.codePointAt(0)!;
    if (point < 0x1f300 || point > 0x1faff || [...first.replace(/\uFE0F/gu, "")].length !== 1) continue;
    pool.push(first);
  }
  pool.sort();
}

type Item = { file: string; test: boolean; emoji: string; slug: string; target: string; note?: string };
const entries = readdirSync(join(root, pkg), { withFileTypes: true });
const goFiles = entries.filter((e) => e.isFile() && e.name.endsWith(".go") && e.name !== "🐹️.go").map((e) => e.name).sort();
const OVERRIDES: Record<string, string> = JSON.parse(readFileSync(`${C}/gocanon/overrides.json`, "utf8"))[slug] ?? {};
const items: Item[] = goFiles.map((file) => {
  const stem = file.slice(0, -3);
  const id = leadingEmojiIdentity(stem);
  let rest = id.rest;
  const test = rest.endsWith("_test");
  if (test) rest = rest.slice(0, -"_test".length);
  let name = rest.replaceAll("_", "-").replace(/^-+/u, "");
  if (!name) name = "unit";
  if (name === "base") name = "language-base";
  const override = OVERRIDES[file] ? leadingEmojiIdentity(OVERRIDES[file]!) : undefined;
  return { file, test, emoji: override?.first ?? id.first, slug: override?.rest ?? name, target: "" };
});

const reservedPkg = new Set<string>();
for (const e of entries) {
  if (items.some((item) => item.file === e.name)) continue;
  const first = leadingEmojiIdentity(e.name).first;
  if (first) reservedPkg.add(fold(first));
}
const reservedCases = new Set<string>();
const existingCases = new Map<string, boolean>();
if (existsSync(join(root, casesDir))) for (const e of readdirSync(join(root, casesDir), { withFileTypes: true })) {
  const first = leadingEmojiIdentity(e.name).first;
  if (first) reservedCases.add(fold(first));
  existingCases.set(e.name, existsSync(join(root, casesDir, e.name, "🐹️.go")));
}

const sourceViable = (name: string): boolean => {
  if (!registered.has(name)) members.memberNames.push(name);
  return chain(`${pkg}/${name}`) === MEMBERS;
};
const caseViable = (name: string): boolean => {
  const id = leadingEmojiIdentity(name);
  if (!id.first || !slugPattern.test(id.rest)) return false;
  const kind = chain(`${casesDir}/${name}`);
  return kind === "members-of-tests" || kind === MEMBERS;
};

const taken = { pkg: new Set(reservedPkg), cases: new Set(reservedCases) };
const used = new Set<string>();
for (const item of items) {
  const set = item.test ? taken.cases : taken.pkg;
  const viable = item.test ? caseViable : sourceViable;
  const merge = item.test && existingCases.get(`${item.emoji}${item.slug}`) === false;
  let emoji = item.emoji;
  let nameOf = (e: string): string => `${e}${item.slug}`;
  const ok = (e: string): boolean => (!set.has(fold(e)) || (merge && fold(e) === fold(item.emoji))) && !generic.has(fold(e)) && viable(nameOf(e));
  if (!ok(emoji)) {
    const ranked = [...pool].sort((left, right) => createHash("sha1").update(item.file + left).digest("hex").localeCompare(createHash("sha1").update(item.file + right).digest("hex")));
    const candidate = ranked.find((e) => !set.has(fold(e)) && !used.has(`${item.test}:${fold(e)}`) && ok(e));
    if (!candidate) throw new Error(`no emoji available for ${item.file}`);
    item.note = `emoji ${item.emoji} -> ${candidate}`;
    emoji = candidate;
  }
  set.add(fold(emoji));
  used.add(`${item.test}:${fold(emoji)}`);
  item.emoji = emoji;
  const dir = nameOf(emoji);
  item.target = item.test ? `${casesDir}/${dir}/🐹️.go` : `${pkg}/${dir}/🐹️.go`;
}

const additions = items.filter((item) => !item.test).map((item) => `${item.emoji}${item.slug}`).filter((name) => !registered.has(name));
const plan = { slug, pkg, owner, items: items.map((item) => ({ from: `${pkg}/${item.file}`, to: item.target, test: item.test, note: item.note ?? null })), additions };
writeFileSync(`${C}/🗑️generated/gocanon-logs/${slug}-convert-plan.json`, JSON.stringify(plan, null, 1));
console.log(`${slug}: ${items.filter((i) => !i.test).length} sources, ${items.filter((i) => i.test).length} tests, ${items.filter((i) => i.note).length} emoji changes, ${additions.length} registry additions`);
for (const item of items) if (item.note) console.log("  ", item.file, item.note);
if (mode === "apply") {
  for (const move of plan.items) {
    const from = join(root, move.from), to = join(root, move.to);
    if (existsSync(to)) throw new Error(`target exists: ${move.to}`);
    mkdirSync(dirname(to), { recursive: true });
    renameSync(from, to);
  }
  writeFileSync(`${C}/🗑️generated/gocanon-logs/${slug}-additions.json`, JSON.stringify(additions));
  console.log("applied", plan.items.length);
}
