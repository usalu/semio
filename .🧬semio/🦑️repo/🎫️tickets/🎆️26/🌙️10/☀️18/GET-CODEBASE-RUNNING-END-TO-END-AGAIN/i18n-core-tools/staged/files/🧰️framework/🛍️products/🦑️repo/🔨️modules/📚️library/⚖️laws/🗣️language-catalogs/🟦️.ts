import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";

/** 🗣️ One source or catalog the law reads: a path relative to the workspace root and its text. */
export type LanguageLawSourceV1 = Readonly<{ path: string; text: string }>;

/** 🔒️ The ratchet of English prose literals per source file; the shape is `🔒️allowlist/🧬️schema/🔣️.json`. */
export type LanguageLawAllowlistV1 = Readonly<{ version: 1; ceiling: number; files: Readonly<Record<string, number>>; defaultsCeiling: number; defaults: Readonly<Record<string, number>> }>;

/** 🚦️ The closed set of rules; every finding names exactly one. */
export const LANGUAGE_LAW_RULES_V1 = ["languages-not-en-de", "leaf-missing-language", "placeholder-mismatch", "table-key-mismatch", "default-language", "prose-literal", "allowance-stale", "ceiling-grown"] as const;

/** 🏷️ One rule of the language law. */
export type LanguageLawRuleV1 = (typeof LANGUAGE_LAW_RULES_V1)[number];

/** 🚩️ One violated rule on one file. */
export type LanguageLawFindingV1 = Readonly<{ rule: LanguageLawRuleV1; path: string; detail: string }>;

const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [
  new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu"),
];
const CODEISH = new RegExp(`[\\\\{}()\\[\\]<>=;]|^${WS}*/|\\bimport\\b|\\bexport\\b|\\bfunction\\b|\\breturn\\b`, "u");
const CODE_WORDS = new RegExp(`[a-z]{3,}${WS}+[a-z]{2,}`, "u");
const PROSE_WORD = /[A-Za-z]{3,}/u;
const PROSE_PAIR = new RegExp(`[A-Za-z]{3,}${WS}+[A-Za-z]{2,}`, "u");
const DEFAULTS: readonly (readonly [string, RegExp])[] = [
  ["en-fallback", /\?\?\s*["']en["']/u],
  ["en-fallback", /\|\|\s*["']en["']/u],
  ["en-return", /\breturn\s*["']en["']/u],
  ["first-language", /\blanguages\s*\[\s*0\s*\]/u],
  ["de-or-en", /\?\s*["']de["']\s*:\s*["']en["']/u],
  ["dashboard-locale", /SEMIO_DASHBOARD_LOCALE/u],
];
const PLACEHOLDER = /\{([A-Za-z_][A-Za-z0-9_]*)\}/gu;
const TAGS = new Set(["en", "de", "fr", "es", "it", "pt", "nl", "pl", "ja", "zh"]);
const SKIP_DIRECTORIES = new Set(["node_modules", "🧪️tests", "🧫️fixtures", ".git", "target", "dist", ".cache", "🗑️generated"]);
const OWN_DIRECTORY = "⚖️laws/🗣️language-catalogs/";
const LAW_ROOTS = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🔨️modules/🏃️process"] as const;
const SOURCE_FILE = /\.(?:ts|mts|mjs|cjs)$/u;

const placeholders = (text: string): string => [...new Set([...text.matchAll(PLACEHOLDER)].map((match) => match[1]!))].sort().join(",");
const compare = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);
const isComment = (line: string): boolean => /^(?:\/\/|\/\*|\*)/u.test(line.trim());

/** 🔎️ Is this text English prose the way a person reads it: two words of letters, not code. */
export function isProseV1(text: string): boolean {
  if (CODEISH.test(text) && !CODE_WORDS.test(text)) return false;
  if (text.split(new RegExp(`${WS}+`, "u")).filter((token) => token !== "").length < 2) return false;
  return PROSE_WORD.test(text) && PROSE_PAIR.test(text);
}

/** 🧮️ How many English prose literals a source passes to `Error(`, `console.*(` and `stdout|stderr.write(`. */
export function proseSitesV1(text: string): number {
  let count = 0;
  for (const line of text.split("\n")) {
    if (isComment(line)) continue;
    for (const site of SITES) {
      for (const match of line.matchAll(site)) {
        const literal = match[1]!;
        if (literal.startsWith("`") && literal.includes("\n")) continue;
        if (isProseV1(literal.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) count++;
      }
    }
  }
  return count;
}

function walkCatalog(value: unknown, path: string, file: string, out: LanguageLawFindingV1[]): void {
  if (Array.isArray(value)) return value.forEach((entry, index) => walkCatalog(entry, `${path}/${index}`, file, out));
  if (typeof value !== "object" || value === null) return;
  const row = value as Record<string, unknown>, keys = Object.keys(row);
  const names = Array.isArray(row.languages) && row.languages.every((entry) => typeof entry === "string") ? (row.languages as string[]) : undefined;
  if (names !== undefined && names.some((name) => name.toLowerCase() === "en" || name.toLowerCase() === "de") && JSON.stringify(names) !== '["en","de"]') out.push({ rule: "languages-not-en-de", path: file, detail: path });
  const pair = keys.length === 2 && keys.includes("en") && keys.includes("de");
  if (pair && Object.values(row).every((entry) => typeof entry === "string")) {
    if (placeholders(row.en as string) !== placeholders(row.de as string)) out.push({ rule: "placeholder-mismatch", path: file, detail: path });
    return;
  }
  if (pair && Object.values(row).every((entry) => typeof entry === "object" && entry !== null && !Array.isArray(entry))) {
    const flat = (table: unknown, base: string, into: Map<string, string>): void => {
      for (const [name, entry] of Object.entries(table as Record<string, unknown>)) {
        if (typeof entry === "string") into.set(`${base}/${name}`, entry);
        else if (typeof entry === "object" && entry !== null && !Array.isArray(entry)) flat(entry, `${base}/${name}`, into);
      }
    };
    const english = new Map<string, string>(), german = new Map<string, string>();
    flat(row.en, "", english);
    flat(row.de, "", german);
    for (const key of [...english.keys(), ...german.keys()].filter((name) => english.has(name) !== german.has(name))) out.push({ rule: "table-key-mismatch", path: file, detail: `${path}${key}` });
    for (const [key, text] of english) if (german.has(key) && placeholders(text) !== placeholders(german.get(key)!)) out.push({ rule: "placeholder-mismatch", path: file, detail: `${path}${key}` });
    return;
  }
  const tagged = keys.length > 0 && keys.every((key) => TAGS.has(key)) && Object.values(row).every((entry) => typeof entry === "string" || (typeof entry === "object" && entry !== null));
  if (tagged && (keys.includes("en") !== keys.includes("de") || keys.some((key) => key !== "en" && key !== "de"))) out.push({ rule: "leaf-missing-language", path: file, detail: path });
  for (const [key, entry] of Object.entries(row)) walkCatalog(entry, `${path}/${key}`, file, out);
}

/** 🧾️ The findings of one JSON document: the languages arrays, the bilingual leaves and the bilingual tables. */
export function catalogFindingsV1(source: LanguageLawSourceV1): readonly LanguageLawFindingV1[] {
  const out: LanguageLawFindingV1[] = [];
  let document: unknown;
  try { document = JSON.parse(source.text); } catch { return out; }
  walkCatalog(document, "", source.path, out);
  return out;
}

/** 🚫️ The findings of one source that assumes a language: an English fallback, the first offered language, a German-or-English ternary or a side variable of the language order. */
export function defaultLanguageFindingsV1(source: LanguageLawSourceV1): readonly LanguageLawFindingV1[] {
  const out: LanguageLawFindingV1[] = [];
  source.text.split("\n").forEach((line, index) => {
    if (isComment(line)) return;
    for (const [name, pattern] of DEFAULTS) if (pattern.test(line)) out.push({ rule: "default-language", path: source.path, detail: `${index + 1}:${name}` });
  });
  return out;
}

/** 🔬️ Every finding of every rule over the sources and the allowlist, sorted by file, rule and detail. */
export function languageLawFindingsV1(sources: readonly LanguageLawSourceV1[], allowlist: LanguageLawAllowlistV1): readonly LanguageLawFindingV1[] {
  const out: LanguageLawFindingV1[] = [], seen = new Set<string>(), defaultsSeen = new Set<string>();
  for (const source of sources) {
    if (source.path.endsWith(".json")) out.push(...catalogFindingsV1(source));
    else if (SOURCE_FILE.test(source.path)) {
      if (!source.path.includes(OWN_DIRECTORY)) {
        const assumed = defaultLanguageFindingsV1(source), tolerated = allowlist.defaults[source.path] ?? 0;
        if (assumed.length > tolerated) out.push(...assumed);
        else if (assumed.length < tolerated) out.push({ rule: "allowance-stale", path: source.path, detail: `defaults allowed=${tolerated} actual=${assumed.length}` });
        defaultsSeen.add(source.path);
      }
      const actual = proseSitesV1(source.text), allowed = allowlist.files[source.path] ?? 0;
      seen.add(source.path);
      if (actual > allowed) out.push({ rule: "prose-literal", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
      else if (actual < allowed) out.push({ rule: "allowance-stale", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
    }
  }
  for (const path of Object.keys(allowlist.files)) if (!seen.has(path)) out.push({ rule: "allowance-stale", path, detail: `allowed=${allowlist.files[path]} actual=0` });
  for (const path of Object.keys(allowlist.defaults)) if (!defaultsSeen.has(path)) out.push({ rule: "allowance-stale", path, detail: `defaults allowed=${allowlist.defaults[path]} actual=0` });
  const total = Object.values(allowlist.files).reduce((sum, count) => sum + count, 0), assumedTotal = Object.values(allowlist.defaults).reduce((sum, count) => sum + count, 0);
  if (total > allowlist.ceiling) out.push({ rule: "ceiling-grown", path: "allowlist", detail: `ceiling=${allowlist.ceiling} total=${total}` });
  if (assumedTotal > allowlist.defaultsCeiling) out.push({ rule: "ceiling-grown", path: "allowlist", detail: `defaults ceiling=${allowlist.defaultsCeiling} total=${assumedTotal}` });
  return out.sort((a, b) => compare(a.path, b.path) || compare(a.rule, b.rule) || compare(a.detail, b.detail));
}

/** 📚️ Reads the JSON catalogs and the TypeScript and JavaScript sources of the repo build library and of the process module, tests and fixtures excluded. */
export function readLanguageLawSourcesV1(workspace: string): readonly LanguageLawSourceV1[] {
  const out: LanguageLawSourceV1[] = [];
  const visit = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!SKIP_DIRECTORIES.has(entry.name)) visit(path);
      } else if (entry.isFile() && (entry.name.endsWith(".json") || SOURCE_FILE.test(entry.name))) out.push({ path: relative(workspace, path).split("\\").join("/"), text: readFileSync(path, "utf8") });
    }
  };
  for (const root of LAW_ROOTS) if (existsSync(join(workspace, root))) visit(join(workspace, root));
  return out.sort((a, b) => compare(a.path, b.path));
}

/** 🪜️ Cuts the allowlist from the sources: every file with its literal count and its count of assumed languages; a ceiling may only be passed lower than the total it replaces. */
export function languageLawAllowlistV1(sources: readonly LanguageLawSourceV1[], ceilings?: Readonly<{ ceiling: number; defaultsCeiling: number }>): LanguageLawAllowlistV1 {
  const files: Record<string, number> = {}, defaults: Record<string, number> = {};
  for (const source of sources) {
    if (!SOURCE_FILE.test(source.path)) continue;
    const count = proseSitesV1(source.text), assumed = source.path.includes(OWN_DIRECTORY) ? 0 : defaultLanguageFindingsV1(source).length;
    if (count > 0) files[source.path] = count;
    if (assumed > 0) defaults[source.path] = assumed;
  }
  const sum = (row: Record<string, number>): number => Object.values(row).reduce((total, count) => total + count, 0);
  return Object.freeze({ version: 1, ceiling: ceilings?.ceiling ?? sum(files), files: Object.freeze(files), defaultsCeiling: ceilings?.defaultsCeiling ?? sum(defaults), defaults: Object.freeze(defaults) });
}

/** 📋️ The findings of the language law as policy breaches. */
export function policyLanguageLawBreaches(findings: readonly LanguageLawFindingV1[]): BreachRecord[] {
  return findings.map((finding) => ({ id: `language-law:${finding.rule}:${finding.path}:${finding.detail}`, summary: `${finding.rule} ${finding.path} ${finding.detail}`, kind: `language-law/${finding.rule}`, scope: finding.path, priority: "medium" as const }));
}
