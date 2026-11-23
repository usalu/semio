import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🗣️ The languages every catalog offers, English first and German second; no language is a default, every caller names one. */
export const CATALOG_LANGUAGES_V1: readonly ["en", "de"] = Object.freeze(schema.properties.languages.const) as readonly ["en", "de"];

/** 🏷️ One language a catalog offers. */
export type CatalogLanguageV1 = (typeof CATALOG_LANGUAGES_V1)[number];

/** 🧩️ A value that fills one named placeholder. */
export type TextValuesV1 = Readonly<Record<string, string | number>>;

/** 🍃️ One message in every offered language. */
export type CatalogLeafV1 = Readonly<Record<CatalogLanguageV1, string>>;

/**
 * 📖️ The texts of one namespace.
 * @see ./🧬️schema/🔣️.json the schema this shape is written first in
 */
export type LanguageCatalogV1 = Readonly<{ version: 1; id?: string; languages: readonly ["en", "de"]; messages: Readonly<Record<string, CatalogLeafV1>> }>;

/** 🔖️ Why a text could not be produced; the reasons are machine codes, never prose. */
export type LanguageTextReasonV1 = "language-required" | "language-unavailable" | "unknown-key" | "unknown-catalog" | "missing-placeholder" | "unknown-placeholder" | "invalid-template" | "invalid-catalog" | "catalog-conflict";

/** 🚫️ Refuses a text request; `message` is `<reason>:<detail>`, so no English prose can reach a console from here. */
export class LanguageTextErrorV1 extends Error {
  readonly reason: LanguageTextReasonV1;
  readonly detail: string;
  constructor(reason: LanguageTextReasonV1, detail: string) {
    super(`${reason}:${detail}`);
    this.name = "LanguageTextError";
    this.reason = reason;
    this.detail = detail;
  }
}

/** 🔤️ One piece of a parsed template: literal text or a named placeholder. */
export type TemplatePartV1 = string | Readonly<{ name: string }>;

/** 🪪️ A catalog together with the namespace its messages are addressed by. */
export type CatalogHandleV1 = Readonly<{ id: string; catalog: LanguageCatalogV1 }>;

const TOKEN = /\{\{|\}\}|\{([A-Za-z_][A-Za-z0-9_]*)\}|[{}]/gu;
const ID = new RegExp(schema.properties.id.pattern, "u");
const KEY = new RegExp(schema.properties.messages.propertyNames.pattern, "u");
const registry = new Map<string, LanguageCatalogV1>();

/** ✂️ Splits a template into literal text and `{name}` placeholders; `{{` and `}}` are literal braces and a lone brace is refused. */
export function parseTemplateV1(template: string): readonly TemplatePartV1[] {
  const parts: TemplatePartV1[] = [];
  let last = 0, text = "";
  for (const match of template.matchAll(TOKEN)) {
    text += template.slice(last, match.index);
    last = match.index + match[0].length;
    if (match[0] === "{{") text += "{";
    else if (match[0] === "}}") text += "}";
    else if (match[1] !== undefined) {
      if (text !== "") parts.push(text);
      text = "";
      parts.push(Object.freeze({ name: match[1] }));
    } else throw new LanguageTextErrorV1("invalid-template", template);
  }
  text += template.slice(last);
  if (text !== "") parts.push(text);
  return Object.freeze(parts);
}

/** 🔠️ The sorted placeholder names of a template. */
export function placeholdersV1(template: string): readonly string[] {
  return Object.freeze([...new Set(parseTemplateV1(template).flatMap((part) => (typeof part === "string" ? [] : [part.name])))].sort());
}

/** 🖨️ Renders one message in one language. It throws on a missing or unoffered language, an unknown key, a placeholder without a value and a value without a placeholder. */
export function textV1(catalog: LanguageCatalogV1, key: string, language: string, values: TextValuesV1 = {}): string {
  if (typeof language !== "string" || language === "") throw new LanguageTextErrorV1("language-required", key);
  if (!(catalog.languages as readonly string[]).includes(language)) throw new LanguageTextErrorV1("language-unavailable", language);
  const leaf = Object.hasOwn(catalog.messages, key) ? catalog.messages[key] : undefined;
  if (leaf === undefined) throw new LanguageTextErrorV1("unknown-key", key);
  const parts = parseTemplateV1(leaf[language as CatalogLanguageV1]), names = new Set(parts.flatMap((part) => (typeof part === "string" ? [] : [part.name])));
  for (const name of Object.keys(values)) if (!names.has(name)) throw new LanguageTextErrorV1("unknown-placeholder", `${key}:${name}`);
  return parts.map((part) => {
    if (typeof part === "string") return part;
    const value = Object.hasOwn(values, part.name) ? values[part.name] : undefined;
    if (value === undefined) throw new LanguageTextErrorV1("missing-placeholder", `${key}:${part.name}`);
    return String(value);
  }).join("");
}

function invalid(path: string): never {
  throw new LanguageTextErrorV1("invalid-catalog", path);
}

/** 🧪️ Admits a catalog document: exactly `["en","de"]`, a well-formed id and keys, both languages in every leaf, valid templates and the same placeholders in both languages. */
export function defineCatalogV1(document: unknown): LanguageCatalogV1 {
  if (typeof document !== "object" || document === null || Array.isArray(document)) return invalid("$");
  const row = document as Record<string, unknown>;
  for (const name of Object.keys(row)) if (!["version", "id", "languages", "messages"].includes(name)) invalid(`$.${name}`);
  if (row.version !== 1) invalid("$.version");
  if (row.id !== undefined && (typeof row.id !== "string" || !ID.test(row.id))) invalid("$.id");
  if (JSON.stringify(row.languages) !== JSON.stringify(CATALOG_LANGUAGES_V1)) invalid("$.languages");
  const messages = row.messages;
  if (typeof messages !== "object" || messages === null || Array.isArray(messages) || Object.keys(messages).length === 0) return invalid("$.messages");
  for (const [key, leaf] of Object.entries(messages)) {
    if (!KEY.test(key)) invalid(`$.messages.${key}`);
    if (typeof leaf !== "object" || leaf === null || Object.keys(leaf).sort().join() !== "de,en") invalid(`$.messages.${key}`);
    const pair = leaf as Record<CatalogLanguageV1, unknown>;
    if (typeof pair.en !== "string" || typeof pair.de !== "string") invalid(`$.messages.${key}`);
    if (placeholdersV1(pair.en).join() !== placeholdersV1(pair.de).join()) invalid(`$.messages.${key}`);
  }
  return Object.freeze(document as LanguageCatalogV1);
}

/** 🔗️ Admits a catalog, registers it under its namespace and returns the handle that code addresses its messages by; registering the same content twice is the same registration. */
export function bindCatalogV1(id: string, document: unknown): CatalogHandleV1 {
  if (!ID.test(id)) throw new LanguageTextErrorV1("invalid-catalog", id);
  const catalog = defineCatalogV1(document);
  if (catalog.id !== undefined && catalog.id !== id) throw new LanguageTextErrorV1("invalid-catalog", `${id}:${catalog.id}`);
  const known = registry.get(id);
  if (known !== undefined && JSON.stringify(known) !== JSON.stringify(catalog)) throw new LanguageTextErrorV1("catalog-conflict", id);
  if (known === undefined) registry.set(id, catalog);
  return Object.freeze({ id, catalog: known ?? catalog });
}

/** 🔎️ Finds a registered catalog by its namespace. */
export function catalogHandleV1(id: string): CatalogHandleV1 {
  const catalog = registry.get(id);
  if (catalog === undefined) throw new LanguageTextErrorV1("unknown-catalog", id);
  return Object.freeze({ id, catalog });
}

/** 🧭️ Splits a qualified key `<namespace>.<key>` into its registered catalog and local key. */
export function qualifiedKeyV1(qualified: string): Readonly<{ handle: CatalogHandleV1; key: string }> {
  const dot = qualified.indexOf(".");
  if (dot <= 0) throw new LanguageTextErrorV1("unknown-key", qualified);
  return Object.freeze({ handle: catalogHandleV1(qualified.slice(0, dot)), key: qualified.slice(dot + 1) });
}

/** 🧮️ Renders a message addressed by its handle. */
export function handleTextV1(handle: CatalogHandleV1, key: string, language: string, values: TextValuesV1 = {}): string {
  return textV1(handle.catalog, key, language, values);
}
