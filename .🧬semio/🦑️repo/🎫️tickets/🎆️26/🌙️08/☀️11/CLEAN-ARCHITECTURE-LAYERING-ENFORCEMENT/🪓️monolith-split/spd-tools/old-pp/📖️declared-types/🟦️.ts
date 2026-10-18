/** 📖️ Readers of the TypeScript already on disk: the declared interface and alias types a parser must satisfy, and the parsers a file already declares, imports or promises.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts
 */
import { escapeRegExp } from "../🔡️json-text/🟦️.ts";

//#region 🔖️Declarations
const GRAPHQL_KEYWORDS = "type|input|enum|interface|union|scalar";

/** 🔎️ True when an export list (`export { a as b }`, `export type { A as B } from "…"`) of the file exports `name` under that name, whichever name it carries inside. */
export function reexports(source: string, name: string): boolean {
  for (const list of source.matchAll(/^[ \t]*export\s*(?:type\s*)?\{([^}]*)\}/gmu)) {
    for (const element of list[1]!.split(",")) if (element.trim().replace(/^type\s+/u, "").split(/\s+as\s+/u).at(-1)!.trim() === name) return true;
  }
  return false;
}

/** 🔎️ The local binding a file exports as `name` through an export list without `from` (`export { local as name }`), or `undefined`: a re-export under an alias creates no binding of that name inside the file, so a call must use the local one. */
export function reexportedBinding(source: string, name: string): string | undefined {
  for (const list of source.matchAll(/^[ \t]*export\s*(?:type\s*)?\{([^}]*)\}(?!\s*from)/gmu)) {
    for (const element of list[1]!.split(",")) {
      const [local, exported = local] = element.trim().replace(/^type\s+/u, "").split(/\s+as\s+/u).map((part) => part.trim());
      if (exported === name && local !== undefined && local !== "") return local;
    }
  }
  return undefined;
}

/** 🔎️ True when `source` already declares `name` in the leaf language of `format`. */
export function declares(format: string, source: string, name: string): boolean {
  const escaped = escapeRegExp(name);
  if (format === "🛰️protobuf") return new RegExp(`^\\s*message\\s+${escaped}\\b`, "m").test(source);
  if (format === "🔗️graphql") return new RegExp(`^\\s*(?:${GRAPHQL_KEYWORDS})\\s+${escaped}\\b`, "m").test(source);
  if (format === "🦀️rust") return new RegExp(`^\\s*pub\\s+(?:struct|enum|type)\\s+${escaped}\\b`, "m").test(source);
  return new RegExp(`^\\s*(?:export\\s+)?(?:declare\\s+)?(?:interface|type|const|class)\\s+${escaped}\\b`, "m").test(source) || reexports(source, name);
}

/** 🔎️ True when the file exports `parse<name>`, declared there or re-exported under that name. */
export const declaresParser = (source: string, name: string): boolean => new RegExp(`^\\s*export\\s+(?:(?:async\\s+)?function|const|let|declare\\s+function)\\s+parse${escapeRegExp(name)}\\b`, "m").test(source) || reexports(source, `parse${name}`);

/** 🔎️ True when the file has `parse<name>` in scope: exported or private, hand-written or projected earlier, or re-exported under that name. */
export const definesParser = (source: string, name: string): boolean => new RegExp(`^\\s*(?:export\\s+)?(?:(?:async\\s+)?function|const|let)\\s+parse${escapeRegExp(name)}\\b`, "m").test(source) || reexports(source, `parse${name}`);

/** 🔎️ True when the file imports `parse<name>`. */
export const importsParser = (source: string, name: string): boolean => new RegExp(`import\\s*(?:type\\s*)?\\{[^}]*\\bparse${escapeRegExp(name)}\\b[^}]*\\}\\s*from`, "u").test(source);

/** 🔎️ The span `[start, end)` of `export function parse<name>` in `source`, from its first line through the line that closes it at column 0 (or its first line when it closes there). `undefined` when the function is missing or another declaration starts before it closes, so a splice can never swallow a neighbour. */
export function parserRange(source: string, name: string): [number, number] | undefined {
  const head = new RegExp(`^export function parse${escapeRegExp(name)}\\(`, "m").exec(source);
  if (head === null) return undefined;
  const lineEnd = source.indexOf("\n", head.index);
  if (lineEnd < 0) return undefined;
  const first = source.slice(head.index, lineEnd);
  if (first.trimEnd().endsWith("}") && first.split("{").length === first.split("}").length) return [head.index, lineEnd + 1];
  let at = lineEnd + 1;
  while (at < source.length) {
    const next = source.indexOf("\n", at);
    const line = source.slice(at, next < 0 ? source.length : next);
    if (line === "}") return [head.index, next < 0 ? source.length : next + 1];
    if (/^(?:export |function |const |let |class |interface |type |import |\/\/#)/u.test(line)) return undefined;
    if (next < 0) return undefined;
    at = next + 1;
  }
  return undefined;
}

/** 🔎️ The guard prefix a file already states (`export class <prefix>Refusal`), which new parsers must call. */
export function existingHelper(source: string): string | undefined {
  for (const found of source.matchAll(/^export class (\w+)Refusal extends Error/gm)) {
    if (new RegExp(`^export const ${escapeRegExp(found[1]!)}Object\\b`, "m").test(source)) return found[1];
  }
  return undefined;
}

//#endregion 🔖️Declarations

//#region 🔖️TypeText
/** ✂️ Removes block and line comments. */
export const scrub = (text: string): string => text.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/\/\/[^\n]*/gu, "");

/** 🧽️ A field type as plain text: no `import(...)` qualifier, no trailing `| undefined`, no leading `readonly`. */
export function normalizeType(text: string): string {
  const unqualified = text.trim().replace(/import\([^)]*\)\./gu, "");
  return unqualified.replace(/\s*\|\s*undefined\s*$/u, "").replace(/^readonly\s+/u, "").trim();
}

/** ✂️ The members of a union type at its top level (`A | B<C | D>` has two). */
export function unionMembers(text: string): string[] {
  const parts: string[] = [];
  let current = "";
  let nesting = 0;
  for (const char of text) {
    nesting += "[({<".includes(char) ? 1 : "])}>".includes(char) ? -1 : 0;
    if (char === "|" && nesting === 0) {
      parts.push(current.trim());
      current = "";
    } else current += char;
  }
  parts.push(current.trim());
  return parts.filter((part) => part !== "");
}

/** ✂️ Splits at the commas that sit outside every bracket pair. */
export function splitTop(text: string): string[] {
  const parts: string[] = [];
  let current = "";
  let nesting = 0;
  for (const char of text) {
    nesting += "[({<".includes(char) ? 1 : "])}>".includes(char) ? -1 : 0;
    if (char === "," && nesting === 0) {
      parts.push(current.trim());
      current = "";
    } else current += char;
  }
  parts.push(current.trim());
  return parts.filter((part) => part !== "");
}

/** ✂️ The `name: type` members of an interface body or an inline object type, split at the separators outside every bracket pair. */
function memberParts(body: string): string[] {
  const parts: string[] = [];
  let current = "";
  let nesting = 0;
  for (const char of scrub(body)) {
    nesting += "{[(<".includes(char) ? 1 : "}])>".includes(char) ? -1 : 0;
    if (";,\n".includes(char) && nesting === 0 && current.includes(":")) {
      parts.push(current);
      current = "";
    } else if (";,\n".includes(char) && nesting === 0) current += char === "\n" ? " " : "";
    else current += char;
  }
  parts.push(current);
  return parts;
}

const MEMBER = /^\s*(?:readonly\s+)?([A-Za-z_$][A-Za-z0-9_$]*)(\??)\s*:\s*([\s\S]+?)\s*$/u;

/** 📋️ `field: type` members of an interface body or an inline object type. */
export function memberTypes(body: string): Map<string, string> {
  const found = new Map<string, string>();
  for (const part of memberParts(body)) {
    const field = MEMBER.exec(part);
    if (field) found.set(field[1]!, normalizeType(field[3]!));
  }
  return found;
}

/** 📋️ Which members of an interface body or an inline object type are marked optional (`name?: …`). */
export function memberOptionality(body: string): Map<string, boolean> {
  const found = new Map<string, boolean>();
  for (const part of memberParts(body)) {
    const field = MEMBER.exec(part);
    if (field) found.set(field[1]!, field[2] === "?");
  }
  return found;
}

/** 🔎️ The body between the braces of an already-written `export interface <name>`, or `undefined`. */
function interfaceBody(source: string, name: string): string | undefined {
  const match = new RegExp(`^export interface ${escapeRegExp(name)}\\b[^{]*\\{`, "m").exec(source);
  if (match === null) return undefined;
  const start = match.index + match[0].length;
  let depth = 1;
  let at = start;
  while (at < source.length && depth) {
    depth += source[at] === "{" ? 1 : source[at] === "}" ? -1 : 0;
    at += 1;
  }
  return source.slice(start, at - 1);
}

/** 🔎️ Which fields an already-written `export interface <name>` marks optional, or `undefined` when it declares none (one-line interfaces included). */
export function declaredOptionalFields(source: string, name: string): Map<string, boolean> | undefined {
  const body = interfaceBody(source, name);
  if (body === undefined) return undefined;
  const found = memberOptionality(body);
  return found.size === 0 ? undefined : found;
}

/** 📋️ The type text every field of an already-written `export interface <name>` declares, so a parser for a field the schema leaves opaque follows the type on disk. */
export function declaredTypes(source: string, name: string): Map<string, string> {
  const body = interfaceBody(source, name);
  return body === undefined ? new Map() : memberTypes(body);
}

/** 📋️ The right-hand side of a one-line `type <name> = …;` alias, normalised. */
export function aliasText(source: string, name: string): string | undefined {
  const match = new RegExp(`^\\s*(?:export\\s+)?type\\s+${escapeRegExp(name)}\\s*=\\s*([^;\\n]+);?\\s*$`, "m").exec(source);
  return match ? normalizeType(match[1]!) : undefined;
}

/** 📋️ The scrubbed right-hand side of `type <name> = …`, up to its closing semicolon or the next unindented line. */
function aliasBody(source: string, pattern: RegExp): string | undefined {
  const match = pattern.exec(source);
  if (match === null) return undefined;
  const from = match.index + match[0].length;
  const end = /;\s*$|\n(?=\S)/m.exec(source.slice(from));
  return scrub(source.slice(from, from + (end ? end.index : source.length)));
}

/** 📋️ The top-level `{ … }` bodies of a scrubbed alias text. */
function objectBodies(text: string): string[] {
  const found: string[] = [];
  let nesting = 0;
  let start: number | undefined;
  for (let index = 0; index < text.length; index += 1) {
    if (text[index] === "{") {
      if (nesting === 0) start = index + 1;
      nesting += 1;
    } else if (text[index] === "}") {
      nesting -= 1;
      if (nesting === 0 && start !== undefined) found.push(text.slice(start, index));
    }
  }
  return found;
}

/** 📋️ For `export type <name> = | { … } | …`, the member types of every object variant, in order. */
export function aliasMembers(source: string, name: string): Map<string, string>[] {
  const text = aliasBody(source, new RegExp(`^\\s*(?:export\\s+)?type\\s+${escapeRegExp(name)}\\s*=\\s*`, "m"));
  return text === undefined ? [] : objectBodies(text).map(memberTypes);
}

/** 📋️ For `export type <name> = | { key: Type } | …`, the declared type of each single-field variant, by key. */
export function aliasVariants(source: string, name: string): Map<string, string> {
  const found = new Map<string, string>();
  const text = aliasBody(source, new RegExp(`^export type ${escapeRegExp(name)}\\s*=\\s*`, "m"));
  if (text === undefined) return found;
  for (const body of objectBodies(text)) {
    const members = memberTypes(body);
    if (members.size === 1) for (const [key, type] of members) found.set(key, type);
  }
  return found;
}
//#endregion 🔖️TypeText
