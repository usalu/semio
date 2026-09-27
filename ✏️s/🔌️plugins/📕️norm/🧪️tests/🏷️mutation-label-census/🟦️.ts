/**
 * 🏷️ Census law over every norm mutation leaf's `label()` — the text the undo/redo history and the MCP transcript show a
 * user. Every leaf of all fifteen families must speak both locales in words: no locale may equal the leaf's semantic
 * kind, no locale may carry a field identifier (a verb-led or three-part kebab id, camelCase, or snake_case other than
 * a short subscripted symbol such as `q_k` or `f_u`), and the German label must not reuse an English literal (the same
 * literal passed to both locales) or be word-for-word the English one.
 *
 * @see ../../🗿️artifacts — the fifteen families whose `🧬️schema/🧬️mutations/<leaf>/🦀️.rs` this law reads.
 */
import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const artifacts = join(dirname(fileURLToPath(import.meta.url)), "../../🗿️artifacts");
const VERB = "(?:change|insert|remove|add|update|replace|reorder|set|specify|retire|introduce|rename|resize)";
const IDENTIFIER = new RegExp(String.raw`(?<![\w-])(?:${VERB}-[a-z0-9-]+|[a-z][a-z0-9]*(?:-[a-z0-9]+){2,}|[a-z]+[A-Z][A-Za-z0-9]*|[a-z][a-z0-9]*(?:_[a-z0-9]+)*_[a-z0-9]{4,}|[a-z][a-z0-9]{3,}(?:_[a-z0-9]+)+)(?![\w-])`, "u");

type Leaf = { family: string; kind: string; en: string[]; de: string[]; opening: { en: string; de: string } };

/** 🔎️ The string literals one locale argument of `LocalizedLabel::native(en, de)` is built from. */
function literals(argument: string): string[] {
  return [...argument.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((match) => match[1]!.replace(/\{[^}]*\}/gu, "").trim()).filter((literal) => literal.length > 0);
}

/** 🔠️ The first character of a locale argument's first string literal. */
function opening(argument: string): string {
  return argument.match(/"((?:[^"\\]|\\.)*)"/u)?.[1]?.charAt(0) ?? "";
}

/** ✂️ Splits the two top-level arguments of the `native(…)` call that follows `start`. */
function nativeArguments(source: string, start: number): [string, string] {
  const open = source.indexOf("native(", start) + "native(".length;
  let depth = 0, inString = false, split = -1, index = open;
  for (; index < source.length; index++) {
    const character = source[index]!;
    if (inString) {
      if (character === "\\") index++;
      else if (character === '"') inString = false;
      continue;
    }
    if (character === '"') inString = true;
    else if ("([{".includes(character)) depth++;
    else if (")]}".includes(character)) {
      if (depth === 0) break;
      depth--;
    } else if (character === "," && depth === 0 && split < 0) split = index;
  }
  return [source.slice(open, split), source.slice(split + 1, index)];
}

/** 🧬️ Every leaf of every family that declares a `label()`. */
function census(): Leaf[] {
  const leaves: Leaf[] = [];
  for (const family of readdirSync(artifacts)) {
    const root = join(artifacts, family, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    if (!existsSync(root)) continue;
    for (const leaf of readdirSync(root)) {
      const source = [join(root, leaf, "🦀️.rs"), join(root, leaf, "🦠️mutation", "🦀️.rs")].find((path) => existsSync(path) && readFileSync(path, "utf8").includes("fn label(&self)")), descriptor = join(root, leaf, "🔣️.json");
      if (source === undefined || !existsSync(descriptor)) continue;
      const text = readFileSync(source, "utf8");
      const start = text.indexOf("fn label(&self) -> protocol::LocalizedLabel");
      if (start < 0) continue;
      const [en, de] = nativeArguments(text, start);
      leaves.push({ family, kind: (JSON.parse(readFileSync(descriptor, "utf8")) as { semanticKind: string }).semanticKind, en: literals(en), de: literals(de), opening: { en: opening(en), de: opening(de) } });
    }
  }
  return leaves;
}

/** 🚨️ Every way one leaf's label fails the law. */
function breaches(leaf: Leaf): string[] {
  const found: string[] = [];
  for (const [locale, parts] of [["en", leaf.en], ["de", leaf.de]] as const) {
    if (parts.length === 0) found.push(`${locale}: empty label`);
    if (/\p{Ll}/u.test(leaf.opening[locale])) found.push(`${locale}: starts with a lowercase word — an untranslated field name, not a sentence`);
    for (const part of parts) {
      if (part === leaf.kind) found.push(`${locale}: equals its kind ${JSON.stringify(part)}`);
      const identifier = part.match(IDENTIFIER);
      if (identifier) found.push(`${locale}: untranslated identifier ${JSON.stringify(identifier[0])} in ${JSON.stringify(part)}`);
    }
  }
  const shared = leaf.de.filter((part) => leaf.en.includes(part) && /[a-z]{3,}/u.test(part) && !/^[A-ZÄÖÜ0-9 ./()-]+$/u.test(part));
  for (const part of shared) found.push(`de reuses the English literal ${JSON.stringify(part)}`);
  if (leaf.en.join(" ") === leaf.de.join(" ")) found.push("de is word-for-word the English label");
  return found;
}

describe("norm mutation labels", () => {
  const leaves = census();

  test("the census reaches every family's mutation vocabulary", () => {
    expect(new Set(leaves.map((leaf) => leaf.family)).size).toBe(15);
    expect(leaves.length).toBeGreaterThan(500);
  });

  test("every leaf label is written in words in English and German — never its kind, never a field identifier", () => {
    const report = leaves.flatMap((leaf) => breaches(leaf).map((breach) => `${leaf.family} ${leaf.kind}: ${breach}`));
    expect(report).toEqual([]);
  });
});
