/** 🔧️ Ticket-local worksheet builder for the schema key documentation.
 *
 * Every family key that the schema does not yet document is attributed to the `\keys_define` block
 * that declares it. A block that another, already documented family shares gives its description
 * away for free; the rest is the authoring surface, printed here with the l3keys property, the
 * declaring package and the `%region 🔖️Keys` comment line that stands above the declaration.
 *
 * `bun 🔧️gaps-worksheet.ts todo`   — the (block, key) pairs that still need an authored description
 * `bun 🔧️gaps-worksheet.ts docs`   — writes `🔧️gaps-key-docs.json`, the per-family option map
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { implementedKeySites } from "./🔧️keys-from-sty.ts";

const LATEX = "C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex";
const SCHEMA = "C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json";
const AUTHORED = `${import.meta.dir}/🔧️gaps-authored.json`;

type Localized = { en: string; de: string };
type Option = { type?: string; default?: unknown; enum?: string[]; description: Localized };

function familyOptions(): Record<string, { owner: string; options: Record<string, Option> }> {
  return JSON.parse(readFileSync(SCHEMA, "utf8"))["x-semio-family-options"];
}

//#region 🔖️Blocks
/** 📖️ block id → the source text of that `\keys_define` body, and the package it lives in. */
export function blockSources(): Map<string, { file: string; body: string; region: string }> {
  const blocks = new Map<string, { file: string; body: string; region: string }>();
  for (const name of readdirSync(LATEX).filter((file) => file.endsWith(".sty"))) {
    const text = readFileSync(join(LATEX, name), "utf8");
    for (const match of text.matchAll(/\\keys_define:nn\s*\{[^}]*\}\s*\n?\s*\{/g)) {
      const open = match.index + match[0].length - 1;
      const body = balanced(text, open);
      const before = text.slice(Math.max(0, open - 2600), open);
      blocks.set(`${name}#${open}`, { file: name, body, region: before });
    }
  }
  return blocks;
}

function balanced(text: string, open: number): string {
  let depth = 0;
  for (let index = open; index < text.length; index += 1) {
    const char = text[index]!;
    if (char === "\\") {
      index += 1;
      continue;
    }
    if (char === "{") depth += 1;
    else if (char === "}") {
      depth -= 1;
      if (depth === 0) return text.slice(open + 1, index);
    }
  }
  return "";
}

/** 🔤 The l3keys property a key is declared with, as the schema `type` it implies. */
export function schemaType(body: string, key: string): string {
  const declaration = new RegExp(`(?:^|,)\\s*${key.replace(/[-]/g, "\\-")}\\s*\\.([a-z_]+):`, "m").exec(body);
  const property = declaration?.[1] ?? "";
  if (property.startsWith("bool")) return "boolean";
  if (property.startsWith("int")) return "integer";
  if (property.startsWith("fp") || property.startsWith("dim")) return "number";
  return "string";
}

/** 💬 The `% key  meaning` line of the `%region 🔖️Keys` comment that stands above a declaration. */
export function commentFor(region: string, key: string): string {
  const lines = region.split("\n").filter((line) => line.trimStart().startsWith("%"));
  const escaped = key.replace(/[-]/g, "\\-");
  for (const line of lines.reverse()) {
    const match = new RegExp(`^%\\s*(?:🔑\\s*)?${escaped}\\b[\\s,/]*(?:\\(([^)]*)\\))?\\s*[:\\-]?\\s*(.*)$`).exec(line.trim());
    if (match !== null && (match[2] ?? "").length > 0) return `${match[1] ? `(${match[1]}) ` : ""}${match[2]}`;
  }
  return "";
}
//#endregion 🔖️Blocks

//#region 🔖️Fill
type Authored = Record<string, Option>;

/** 🧩 The per-family option map the schema should carry: what it already documents, what a
 * same-block sibling documents, and what `🔧️gaps-authored.json` supplies for the rest. */
export function keyDocs(): { docs: Record<string, Record<string, Option>>; todo: { id: string; key: string; file: string; type: string; comment: string; families: number }[] } {
  const rawSites = implementedKeySites();
  const schema = familyOptions();
  const blocks = blockSources();
  const authored: Authored = JSON.parse(readFileSync(AUTHORED, "utf8"));

  const offsets = new Map<string, number[]>();
  for (const keys of Object.values(rawSites)) {
    for (const block of Object.values(keys)) {
      const [file, offset] = block.split("#");
      if (offset === undefined) continue;
      const bag = offsets.get(file!) ?? [];
      if (!bag.includes(Number(offset))) bag.push(Number(offset));
      offsets.set(file!, bag);
    }
  }
  for (const bag of offsets.values()) bag.sort((left, right) => left - right);
  const canonical = new Map<string, string>();
  const source = new Map<string, string>();
  for (const [file, bag] of offsets) {
    for (const [rank, offset] of bag.entries()) {
      canonical.set(`${file}#${offset}`, `${file}#${rank}`);
      source.set(`${file}#${rank}`, `${file}#${offset}`);
    }
  }
  const sites: Record<string, Record<string, string>> = Object.fromEntries(
    Object.entries(rawSites).map(([family, keys]) => [family, Object.fromEntries(Object.entries(keys).map(([key, block]) => [key, canonical.get(block) ?? block]))]),
  );

  const sibling = new Map<string, Map<string, number>>();
  for (const [family, keys] of Object.entries(sites)) {
    const documented = schema[family]?.options;
    if (documented === undefined) continue;
    for (const [key, block] of Object.entries(keys)) {
      const option = documented[key];
      if (option === undefined) continue;
      const id = `${block}|${key}`;
      const bag = sibling.get(id) ?? new Map<string, number>();
      const text = JSON.stringify(option);
      bag.set(text, (bag.get(text) ?? 0) + 1);
      sibling.set(id, bag);
    }
  }

  const docs: Record<string, Record<string, Option>> = {};
  const todo = new Map<string, { id: string; key: string; file: string; type: string; comment: string; families: number }>();
  for (const [family, keys] of Object.entries(sites)) {
    const documented = schema[family]?.options ?? {};
    const options: Record<string, Option> = {};
    for (const [key, block] of Object.entries(keys)) {
      const id = `${block}|${key}`;
      const own = documented[key];
      if (own !== undefined) {
        options[key] = own;
        continue;
      }
      const shared = sibling.get(id);
      if (shared !== undefined) {
        options[key] = JSON.parse([...shared].sort((left, right) => right[1] - left[1])[0]![0]) as Option;
        continue;
      }
      const hand = authored[id];
      if (hand !== undefined) {
        options[key] = hand;
        continue;
      }
      const declared = blocks.get(source.get(block) ?? "");
      const entry = todo.get(id) ?? {
        id,
        key,
        file: declared?.file ?? block,
        type: declared === undefined ? "string" : schemaType(declared.body, key),
        comment: declared === undefined ? "" : commentFor(declared.region, key),
        families: 0,
      };
      entry.families += 1;
      todo.set(id, entry);
    }
    docs[family] = options;
  }
  return { docs, todo: [...todo.values()].sort((left, right) => right.families - left.families) };
}
//#endregion 🔖️Fill

const action = process.argv[2];
if (action === "todo") {
  const { todo } = keyDocs();
  for (const entry of todo) console.log([entry.families, entry.id, entry.type, entry.comment].join("\t"));
  console.log(`\n${todo.length} (block, key) pairs to author, covering ${todo.reduce((sum, entry) => sum + entry.families, 0)} family slots`);
} else if (action === "docs") {
  const { docs, todo } = keyDocs();
  if (todo.length > 0) throw new Error(`${todo.length} (block, key) pairs still unauthored`);
  writeFileSync(`${import.meta.dir}/🔧️gaps-key-docs.json`, `${JSON.stringify(docs, null, 1)}\n`, "utf8");
  console.log(`[DEBUG] wrote key docs for ${Object.keys(docs).length} families`);
} else throw new Error(`unknown action ${String(action)}`);
