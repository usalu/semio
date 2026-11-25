// 🏷️ Sets the titles of the tags of the asset documents of the catalog and the fixtures from `vocabulary.json`, touching nothing else: only the first `tags` array of a file is read, the text of the file is re-read immediately before the write and the write is skipped when it changed meanwhile. Usage: node retitle.mjs <staffage root> [--dry]
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const here = fileURLToPath(new URL(".", import.meta.url));
const vocabulary = JSON.parse(fs.readFileSync(here + "vocabulary.json", "utf8"));
const root = process.argv[2];
const dry = process.argv.includes("--dry");
const START = /"tags":\s*\[/;
const ITEM = /("id":\s*"([a-z0-9-]+)",\s*"title":\s*\{\s*"en":\s*)"(?:[^"\\]|\\.)*"(,\s*"de":\s*)"(?:[^"\\]|\\.)*"/g;

function transform(text) {
  const start = START.exec(text);
  if (start === null) return null;
  const from = start.index + start[0].length;
  const to = text.indexOf("]", from);
  const region = text.slice(from, to);
  const next = text.slice(0, from) + region.replace(ITEM, (_, head, id, middle) => {
    const title = vocabulary[id];
    if (title === undefined) throw new Error(`no title for ${id}`);
    return `${head}${JSON.stringify(title.en)}${middle}${JSON.stringify(title.de)}`;
  }) + text.slice(to);
  if (next === text) return text;
  const before = JSON.parse(text), after = JSON.parse(next);
  const holder = (document) => document.asset ?? document;
  const expected = structuredClone(before);
  holder(expected).tags = holder(expected).tags.map((tag) => ({ id: tag.id, title: vocabulary[tag.id] }));
  if (JSON.stringify(after) !== JSON.stringify(expected)) throw new Error("rewrite changed more than the tag titles");
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
let changed = 0, same = 0, retried = 0;
for (const file of files) {
  for (let attempt = 0; attempt < 5; attempt++) {
    const text = fs.readFileSync(file, "utf8");
    const next = transform(text);
    if (next === text) { same++; break; }
    if (dry) { changed++; break; }
    if (fs.readFileSync(file, "utf8") !== text) { retried++; continue; }
    fs.writeFileSync(file, next);
    changed++;
    break;
  }
}
console.log({ files: files.length, changed, same, retried });
