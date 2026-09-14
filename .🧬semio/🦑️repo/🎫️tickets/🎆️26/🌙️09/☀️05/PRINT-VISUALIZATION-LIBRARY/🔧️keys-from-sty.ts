/** 🔧️ Ticket-local extractor: the l3keys every viz family actually implements.
 *
 * Reads `🖋️latex/*.sty`, resolves the `\keys_define:nn` blocks — including the ones a
 * `…_keys_add:n` helper installs under a path it receives as `#1` — and the `\keys_set:nn`
 * delegation chain a family body walks, and prints one JSON object
 * `{ "<family>": { keys: […], docs: { key: "doc line" } } }`.
 *
 * Usage: `bun "<ticket>/🔧️keys-from-sty.ts" [--diff]`
 *   plain   — writes the extraction to stdout as JSON
 *   --diff  — compares it with `🧬️schema/🔣️.json` `x-semio-family-options` and reports both sides
 */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const LATEX = "C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex";
const SCHEMA = "C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json";

//#region 🔖️Reading
type Source = { file: string; text: string };

function sources(): Source[] {
  return readdirSync(LATEX)
    .filter((name) => name.endsWith(".sty"))
    .map((name) => ({ file: name, text: readFileSync(join(LATEX, name), "utf8") }));
}

/** ✂️ The balanced `{…}` group that starts at `open` (the index of the `{`); returns its body. */
function group(text: string, open: number): { body: string; end: number } {
  let depth = 0;
  for (let index = open; index < text.length; index += 1) {
    const char = text[index]!;
    if (char === "\\") {
      index += 1;
      continue;
    }
    if (char === "%" && text[index - 1] !== "\\") {
      while (index < text.length && text[index] !== "\n") index += 1;
      continue;
    }
    if (char === "{") depth += 1;
    else if (char === "}") {
      depth -= 1;
      if (depth === 0) return { body: text.slice(open + 1, index), end: index + 1 };
    }
  }
  throw new Error("unbalanced group");
}

/** ✂️ Index of the next non-space character at or after `from`. */
function skipSpace(text: string, from: number): number {
  let index = from;
  while (index < text.length && /\s/.test(text[index]!)) index += 1;
  return index;
}
//#endregion 🔖️Reading

//#region 🔖️KeyBlocks
/** 🔑 The key names an l3keys body declares (`name .prop:… = …`), `unknown` excluded. */
export function keysOfBody(body: string): string[] {
  const names = new Set<string>();
  const code = body.replace(/(^|[^\\])%[^\n]*/g, "$1");
  for (const match of code.matchAll(/(^|,)\s*([A-Za-z][A-Za-z0-9-]*)\s*\./gm)) names.add(match[2]!);
  names.delete("unknown");
  return [...names];
}

type Path = string;
type Template = { template: string; keys: string[] };
type Unit = { keys: Set<string>; templates: Template[]; setPaths: Set<Path>; definePaths: Set<Path>; calls: { macro: string; argument: string }[] };

function unit(): Unit {
  return { keys: new Set(), templates: [], setPaths: new Set(), definePaths: new Set(), calls: [] };
}

/** 🔍️ Every `\keys_define:nn`, `\keys_set:nn` and `\<macro>:n{…}` occurrence of one code body,
 * with `#1` kept verbatim so a caller can substitute the path it passes in. */
function scan(text: string, origin = "", base = 0): Unit {
  const found = unit();
  const tag = (keys: string[], open: number): string[] => keys.map((key) => (key.startsWith("@") ? key : `${key}${origin}#${base + open}`));
  const define = /\\keys_define:nn\s*\{/g;
  for (let match = define.exec(text); match !== null; match = define.exec(text)) {
    const head = group(text, match.index + match[0].length - 1);
    const argument = head.body.trim();
    const bodyOpen = skipSpace(text, head.end);
    if (text[bodyOpen] !== "{") continue;
    const block = group(text, bodyOpen).body;
    const keys = keysOfBody(block);
    for (const forward of block.matchAll(/unknown\s*\.code:n[\s\S]{0,90}?\\keys_set:nn\s*\{([^}]*)\}/g)) keys.push(`@forward:${forward[1]!.trim()}`);
    if (argument.includes("#1")) found.templates.push({ template: argument, keys: tag(keys, bodyOpen) });
    else found.definePaths.add(`${argument}::${tag(keys, bodyOpen).join("\u0001")}`);
    define.lastIndex = bodyOpen;
  }
  const routes = /\\keys_set_known:n/.test(text);
  const apply = /\\keys_set(?:_known)?:n[nVx]N?\s*\{/g;
  for (let match = apply.exec(text); match !== null; match = apply.exec(text)) {
    const head = group(text, match.index + match[0].length - 1);
    const listOpen = skipSpace(text, head.end);
    const list = text[listOpen] === "{" ? group(text, listOpen).body : "\\";
    if (routes || /#\d/.test(list)) found.setPaths.add(head.body.trim());
    apply.lastIndex = head.end;
  }
  const indirect = /\\exp_args:NV\s*\\keys_define:nn\s*\\[A-Za-z_@]+\s*\{/g;
  for (let match = indirect.exec(text); match !== null; match = indirect.exec(text)) {
    const open = match.index + match[0].length - 1;
    found.templates.push({ template: "#1", keys: tag(keysOfBody(group(text, open).body), open) });
    indirect.lastIndex = open;
  }
  const set = /\\keys_set(?::nn|:nV|:nx|:Vn)\s*\{/g;
  for (let match = set.exec(text); match !== null; match = set.exec(text)) {
    const head = group(text, match.index + match[0].length - 1);
    found.setPaths.add(head.body.trim());
    set.lastIndex = head.end;
  }
  const call = /\\(semio_viz_[a-z0-9_]+):([a-zA-Z]*)/g;
  for (let match = call.exec(text); match !== null; match = call.exec(text)) {
    const after = skipSpace(text, match.index + match[0].length);
    const takesGroup = match[2] === "n" && text[after] === "{";
    found.calls.push({ macro: match[1]!, argument: takesGroup ? group(text, after).body.trim() : "" });
  }
  return found;
}
//#endregion 🔖️KeyBlocks

//#region 🔖️Resolution
type Model = {
  /** direct `\keys_define:nn { <literal path> }` keys */
  literal: Map<Path, Set<string>>;
  /** a key-installing helper: the path templates it defines keys on in terms of its own `#1`,
   * plus the helpers it forwards `#1` to */
  helpers: Map<string, { templates: Template[]; forwards: string[] }>;
  /** helper invocations, with the literal argument the caller passed */
  installs: { argument: string; macro: string }[];
  /** family name → the body of its `\SemioVizFamily` registration */
  families: Map<string, string>;
  /** macro name → its body, for the `keys_set` delegation walk */
  macros: Map<string, string>;
};

export function model(): Model {
  const literal = new Map<Path, Set<string>>();
  const helpers = new Map<string, { templates: Template[]; forwards: string[] }>();
  const installs: { argument: string; macro: string }[] = [];
  const families = new Map<string, string>();
  const macros = new Map<string, string>();

  for (const source of sources()) {
    const text = source.text;
    const definition = /\\cs_(?:new|set|new_protected|set_protected|generate_variant)[a-z_]*:Npn\s+\\([A-Za-z@_]+:[A-Za-z]*)\s*((?:#\d\s*)*)\{/g;
    const bodies: { name: string; body: string; start: number; bodyStart: number; end: number }[] = [];
    for (let match = definition.exec(text); match !== null; match = definition.exec(text)) {
      const open = match.index + match[0].length - 1;
      const found = group(text, open);
      bodies.push({ name: match[1]!, body: found.body, start: match.index, bodyStart: open + 1, end: found.end });
      definition.lastIndex = found.end;
    }
    for (const entry of bodies) macros.set(entry.name, entry.body);

    for (const entry of bodies) {
      const scanned = scan(entry.body, source.file, entry.bodyStart);
      const base = entry.name.replace(/:.*$/, "");
      const helper = helpers.get(base) ?? { templates: [], forwards: [] };
      helper.templates.push(...scanned.templates);
      for (const call of scanned.calls) if (call.argument === "#1" || call.argument === "") helper.forwards.push(call.macro);
      helpers.set(base, helper);
    }

    const masked = maskRanges(text, bodies);
    const scanned = scan(masked, source.file);
    for (const record of scanned.definePaths) {
      const [path, keys] = record.split("::");
      const bag = literal.get(path!) ?? new Set<string>();
      for (const key of (keys ?? "").split("\u0001").filter((value) => value.length > 0)) bag.add(key);
      literal.set(path!, bag);
    }
    for (const call of [...scanned.calls, ...bodies.flatMap((entry) => scan(entry.body, source.file, entry.bodyStart).calls)]) {
      if (call.argument.length > 0 && !call.argument.includes("#")) installs.push({ argument: call.argument, macro: call.macro });
    }

    const family = /\\(?:SemioVizFamily|semio_viz_family_define:nn)\s*\{\s*([a-z0-9-]+)\s*\}\s*\{/g;
    for (let match = family.exec(text); match !== null; match = family.exec(text)) {
      const open = match.index + match[0].length - 1;
      const found = group(text, open);
      family.lastIndex = found.end;
      if (text.slice(text.lastIndexOf("\n", match.index) + 1, match.index).includes("%")) continue;
      families.set(match[1]!, found.body);
    }
  }
  return { literal, helpers, installs, families, macros };
}

/** ✂️ Blanks out the given source ranges so a second scan does not see them twice. */
function maskRanges(text: string, ranges: { start: number; end: number }[]): string {
  let masked = text;
  for (const range of ranges) masked = `${masked.slice(0, range.start)}${" ".repeat(range.end - range.start)}${masked.slice(range.end)}`;
  return masked;
}

/** 🔑 The keys a helper installs when it is called with `argument`, its forwards included. */
function helperKeys(model: Model, macro: string, argument: string, path: Path, seen = new Set<string>()): Set<string> {
  if (seen.has(macro)) return new Set();
  seen.add(macro);
  const helper = model.helpers.get(macro);
  const keys = new Set<string>();
  if (helper === undefined) return keys;
  for (const entry of helper.templates) {
    if (entry.template.replaceAll("#1", argument).trim() !== path) continue;
    for (const key of entry.keys) keys.add(key);
  }
  for (const forward of helper.forwards) for (const key of helperKeys(model, forward, argument, path, seen)) keys.add(key);
  return keys;
}

/** 🔑 Every key an l3keys path carries, helper installations included. */
export function pathKeys(model: Model, path: Path, seen = new Set<Path>()): Set<string> {
  if (seen.has(path)) return new Set();
  seen.add(path);
  const keys = new Set(model.literal.get(path) ?? []);
  for (const install of model.installs) for (const key of helperKeys(model, install.macro, install.argument, path)) keys.add(key);
  for (const key of [...keys]) {
    if (!key.startsWith("@forward:")) continue;
    keys.delete(key);
    for (const forwarded of pathKeys(model, key.slice("@forward:".length), seen)) keys.add(forwarded);
  }
  return keys;
}

/** 🔗️ (exported) The l3keys paths a family body reaches, through the macros it calls. */
export function reachedPaths(model: Model, body: string, depth = 0, seen = new Set<string>()): Set<Path> {
  const paths = new Set<Path>();
  if (depth > 6) return paths;
  for (const path of scan(body).setPaths) paths.add(path);
  for (const match of body.matchAll(/\\(semio_viz_[a-z0-9_]+:[a-zA-Z]*)/g)) {
    const name = match[1]!;
    if (seen.has(name)) continue;
    seen.add(name);
    const macro = model.macros.get(name);
    if (macro === undefined) continue;
    for (const path of reachedPaths(model, macro, depth + 1, seen)) paths.add(path);
  }
  return paths;
}

export type FamilyKeys = Record<string, string[]>;
/** 🔑 family name -> key name -> id of the keys_define block that declares it. */
export type FamilyKeySites = Record<string, Record<string, string>>;

/** 🔑 family name → the keys its option list actually accepts. */
function taggedKeys(): FamilyKeys {
  const built = model();
  const result: FamilyKeys = {};
  for (const [name, body] of built.families) {
    const own = `semio / viz / family / ${name}`;
    const keys = new Set(pathKeys(built, own));
    for (const path of reachedPaths(built, body)) {
      if (path === own || path.includes("#")) continue;
      for (const key of pathKeys(built, path)) keys.add(key);
    }
    keys.add("variantregistry");
    result[name] = [...keys].sort();
  }
  return result;
}
/** 🔑 family name -> the keys its option list actually accepts. */
export function implementedKeys(): FamilyKeys {
  return Object.fromEntries(Object.entries(taggedKeys()).map(([family, keys]) => [family, [...new Set(keys.map((key) => key.split("")[0]!))].sort()]));
}

/** 🔑 family name -> key name -> the id of the block that declares it, so two families that take a
 * key from the same declaration can share one description. */
export function implementedKeySites(): FamilyKeySites {
  return Object.fromEntries(
    Object.entries(taggedKeys()).map(([family, keys]) => [
      family,
      Object.fromEntries(keys.map((key) => [key.split("")[0]!, key.split("")[1] ?? ""])),
    ]),
  );
}
//#endregion 🔖️Resolution

//#region 🔖️Docs
/** 📖️ The `%region 🔖️Keys…` comment lines of a package, as `key → documentation line`. */
export function keyDocs(): Record<string, Record<string, string>> {
  const docs: Record<string, Record<string, string>> = {};
  for (const source of sources()) {
    const regions = source.text.matchAll(/%region 🔖️Keys([^\n]*)\n([\s\S]*?)%endregion 🔖️Keys/g);
    for (const region of regions) {
      const label = (region[1] ?? "").trim().replace(/^-/, "");
      const bag: Record<string, string> = {};
      for (const line of (region[2] ?? "").split("\n")) {
        const match = /^%\s{1,3}([A-Za-z][A-Za-z0-9-]*)\s{2,}(.+)$/.exec(line.trim().startsWith("%") ? line.trim() : "");
        if (match !== null) bag[match[1]!] = match[2]!.trim();
      }
      docs[`${source.file}::${label}`] = bag;
    }
  }
  return docs;
}
//#endregion 🔖️Docs

//#region 🔖️Main
function schemaOptions(): Record<string, string[]> {
  const schema = JSON.parse(readFileSync(SCHEMA, "utf8")) as { "x-semio-family-options": Record<string, { options: Record<string, unknown> }> };
  return Object.fromEntries(Object.entries(schema["x-semio-family-options"]).map(([name, entry]) => [name, Object.keys(entry.options).sort()]));
}

if (import.meta.main) {
  const implemented = implementedKeys();
  if (process.argv.includes("--diff")) {
    const schema = schemaOptions();
    const rows: string[] = [];
    for (const name of [...new Set([...Object.keys(implemented), ...Object.keys(schema)])].sort()) {
      const mine = implemented[name] ?? [];
      const theirs = schema[name];
      if (theirs === undefined) {
        rows.push(`NO-SCHEMA ${name} implements ${mine.length}: ${mine.join(" ")}`);
        continue;
      }
      if (implemented[name] === undefined) {
        rows.push(`NO-FAMILY ${name} schema ${theirs.length}`);
        continue;
      }
      const missing = mine.filter((key) => !theirs.includes(key));
      const phantom = theirs.filter((key) => !mine.includes(key));
      if (missing.length > 0 || phantom.length > 0) rows.push(`${name} (${theirs.length}/${mine.length}) missing=[${missing.join(" ")}] phantom=[${phantom.join(" ")}]`);
    }
    console.log(rows.join("\n"));
    console.log(`\n${rows.length} families out of step`);
  } else {
    console.log(JSON.stringify(implemented, null, 1));
  }
}
//#endregion 🔖️Main
