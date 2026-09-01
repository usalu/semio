import { readFileSync, readdirSync } from "node:fs";
import { dirname } from "node:path";

const taxonomyPath = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const raw = JSON.parse(readFileSync(taxonomyPath, "utf8"));
const directoryKinds = Object.entries(raw.semanticDirectoryKinds as Record<string, any>)
  .map(([id, spec]: [string, any]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") }))
  .sort((a, b) => a.id.localeCompare(b.id));
const memberKinds: Record<string, any> = raw.semanticDirectoryMemberKinds ?? {};
function emojiFold(v: string) { return v.normalize("NFC").replaceAll("️", ""); }
const SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });
function isEmojiGrapheme(v: string) { return /[\p{Extended_Pictographic}\p{Emoji_Presentation}️⃣]/u.test(v); }
function splitLeadingEmoji(v: string) { const first = SEGMENTER.segment(v)[Symbol.iterator]().next().value?.segment; if (!first || !isEmojiGrapheme(first)) return { emoji: "", rest: v }; return { emoji: first, rest: v.slice(first.length) }; }
function matchDirectoryKind(name: string, parentKindId?: string, ancestorKindIds: readonly string[] = []) {
  const normalized = name.normalize("NFC"); const leading = splitLeadingEmoji(normalized);
  const contextAllows = (kind: any) => (kind.parentKindIds?.length ?? 0) === 0 || (parentKindId !== undefined && kind.parentKindIds?.includes(parentKindId) === true);
  if (leading.emoji) {
    const global = directoryKinds.filter((kind: any) => emojiFold(kind.emoji) === emojiFold(leading.emoji) && ((leading.rest.length === 0 && kind.allowEmojiOnly) || kind.slugRegex.test(leading.rest)));
    const exact = global.filter((kind: any) => contextAllows(kind) && kind.id.normalize("NFC").toLocaleLowerCase("und") === leading.rest.toLocaleLowerCase("und"));
    if (exact.length === 1) return { kind: exact[0], ambiguous: [] };
    if (exact.length > 1) return { kind: null, ambiguous: exact.map((e: any) => e.id) };
    const contextual = parentKindId === undefined ? [] : global.filter((kind: any) => kind.parentKindIds?.includes(parentKindId) === true);
    const ordinary = contextual.length > 0 ? contextual : global.filter((kind: any) => (kind.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1) return { kind: ordinary[0], ambiguous: [] };
    const contexts = [parentKindId, ...ancestorKindIds].filter((k, i, r): k is string => Boolean(k) && r.indexOf(k) === i);
    const overlays = Object.entries(memberKinds).filter(([, spec]: [string, any]) => spec.memberNames.some((m: string) => emojiFold(m) === emojiFold(normalized))).map(([id, spec]: [string, any]) => ({ id, distance: contexts.findIndex((k) => spec.ownerKindIds.includes(k)) })).filter((e) => e.distance >= 0).sort((a, b) => a.distance - b.distance || a.id.localeCompare(b.id));
    if (overlays.length > 0) { const nearest = overlays.filter((e) => e.distance === overlays[0].distance); if (nearest.length === 1) return { kind: { id: nearest[0].id }, ambiguous: [] }; return { kind: null, ambiguous: nearest.map((e) => e.id) }; }
    return { kind: null, ambiguous: ordinary.length > 0 ? ordinary.map((e: any) => e.id) : global.map((e: any) => e.id) };
  }
  const exact = directoryKinds.filter((kind: any) => contextAllows(kind) && kind.inferWithoutEmoji !== false && kind.id.normalize("NFC").toLocaleLowerCase("und") === normalized.toLocaleLowerCase("und"));
  if (exact.length === 1) return { kind: exact[0], ambiguous: [] };
  if (exact.length > 1) return { kind: null, ambiguous: exact.map((e: any) => e.id) };
  const matching = directoryKinds.filter((kind: any) => kind.inferWithoutEmoji !== false && kind.slugRegex.test(normalized));
  const contextual = parentKindId === undefined ? [] : matching.filter((kind: any) => kind.parentKindIds?.includes(parentKindId) === true);
  const matches = contextual.length > 0 ? contextual : matching.filter((kind: any) => (kind.parentKindIds?.length ?? 0) === 0);
  return { kind: matches.length === 1 ? matches[0] : null, ambiguous: matches.map((e: any) => e.id) };
}

const pluginsRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins";
const found: string[] = [];
for (const d of readdirSync(pluginsRoot, { withFileTypes: true })) {
  if (!d.isDirectory()) continue;
  for (const sub of readdirSync(`${pluginsRoot}/${d.name}`, { withFileTypes: true })) {
    if (sub.isDirectory() && sub.name.startsWith("🧪️")) found.push(`${d.name}/${sub.name}`);
  }
}
for (const rel of found) {
  const parts = rel.split("/");
  const result = matchDirectoryKind(parts[1], "members-of-plugins", ["members-of-plugins", "plugins"]);
  console.log(rel, "->", result.kind?.id ?? "AMBIGUOUS:" + result.ambiguous.join(","));
}
