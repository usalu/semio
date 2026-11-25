// 🏷️ Rewrites the `tags` of the asset documents of the catalog and the fixtures from slugs to `{ id, title }`, touching nothing else: the text of each file is re-read immediately before the write and the write is skipped when it changed meanwhile (the file is then transformed again from its new text).
// Usage: node apply.mjs <staffage root> [--dry]
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const here = fileURLToPath(new URL(".", import.meta.url));
const vocabulary = JSON.parse(fs.readFileSync(here + "vocabulary.json", "utf8"));
const root = process.argv[2];
const dry = process.argv.includes("--dry");
const PATTERN = /"tags":(\s*)\[(\s*(?:"[a-z0-9-]+"\s*,?\s*)*)\]/;

function titled(slug) {
  const title = vocabulary[slug];
  if (title === undefined) throw new Error(`no title for ${slug}`);
  return { id: slug, title };
}

function transform(text) {
  const match = PATTERN.exec(text);
  if (match === null) return null;
  const slugs = [...match[2].matchAll(/"([a-z0-9-]+)"/g)].map((m) => m[1]);
  const lineStart = text.lastIndexOf("\n", match.index) + 1;
  const base = match.index - lineStart;
  const multi = match[2].includes("\n");
  let block;
  if (!multi) block = JSON.stringify(slugs.map(titled));
  else {
    const first = /\n( *)"/.exec(match[2]);
    const unit = first[1].length - base;
    const pad = (levels) => " ".repeat(base + unit * levels);
    const items = slugs.map((slug) => {
      const { id, title } = titled(slug);
      return [`${pad(1)}{`, `${pad(2)}"id": ${JSON.stringify(id)},`, `${pad(2)}"title": {`, `${pad(3)}"en": ${JSON.stringify(title.en)},`, `${pad(3)}"de": ${JSON.stringify(title.de)}`, `${pad(2)}}`, `${pad(1)}}`].join("\n");
    });
    block = slugs.length === 0 ? "[]" : `[\n${items.join(",\n")}\n${pad(0)}]`;
  }
  const next = text.slice(0, match.index) + `"tags":${match[1]}${block}` + text.slice(match.index + match[0].length);
  const before = JSON.parse(text);
  const after = JSON.parse(next);
  const holder = (document) => (document.asset ?? document);
  const expected = structuredClone(before);
  holder(expected).tags = slugs.map(titled);
  if (JSON.stringify(after) !== JSON.stringify(expected)) throw new Error("rewrite changed more than the tags");
  return next;
}

const files = [];
const catalog = path.join(root, "🗂️catalog");
for (const category of fs.readdirSync(catalog)) {
  const directory = path.join(catalog, category);
  if (!fs.statSync(directory).isDirectory()) continue;
  for (const asset of fs.readdirSync(directory)) {
    const file = path.join(directory, asset, "🔣️.json");
    if (fs.existsSync(file)) files.push(file);
  }
}
for (const fixture of ["🦶️sparse-smooth-walk", "👤️sample-figure", "🔄️continuous-rotation", "🚶️reference-walker"]) files.push(path.join(root, "🧫️fixtures", fixture, "🔣️.json"));

let changed = 0, skipped = 0, retried = 0;
for (const file of files) {
  for (let attempt = 0; attempt < 5; attempt++) {
    const text = fs.readFileSync(file, "utf8");
    const next = transform(text);
    if (next === null) { skipped++; break; }
    if (dry) { changed++; break; }
    if (fs.readFileSync(file, "utf8") !== text) { retried++; continue; }
    fs.writeFileSync(file, next);
    changed++;
    break;
  }
}
console.log({ files: files.length, changed, skipped, retried });
