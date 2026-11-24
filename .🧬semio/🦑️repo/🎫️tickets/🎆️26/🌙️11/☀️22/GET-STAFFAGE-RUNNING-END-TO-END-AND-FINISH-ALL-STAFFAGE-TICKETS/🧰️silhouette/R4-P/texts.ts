/** 📝️ Writes the drawing decisions of a plant that the redraw changes: `texts/<id>.json` maps a scale to the members of `representations[scale].decisions` it replaces (each `{ en, de }`); only these values are rewritten in the text of the file. Idempotent.
 *  bun texts.ts <id> [--in file] [--out file]
 * @see ./redraw.ts — the redraw the texts describe */
import { parseArgs } from "node:util";
import { fileOf } from "./plants.ts";
import { rewrite } from "./preserve.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" } } });
const id = positionals[0]!;
const source = values.in ?? fileOf(id);
const text = await Bun.file(source).text();
const doc = JSON.parse(text);
if (doc.asset.id !== id) throw new Error(`${source} holds ${doc.asset.id}, not ${id}`);
const next = structuredClone(doc);
const texts = (await Bun.file(`${import.meta.dir}/texts/${id}.json`).json()) as Record<string, Record<string, { en: string; de: string }>>;
let count = 0;
for (const [scale, members] of Object.entries(texts)) for (const [key, value] of Object.entries(members)) {
  if (!(key in next.asset.representations[scale].decisions)) throw new Error(`${id} 1:${scale} has no decision "${key}"`);
  next.asset.representations[scale].decisions[key] = value;
  count++;
}
const written = rewrite(text, doc, next);
if (written === text) console.log(`${id}: the file already has these ${count} decision texts`);
else { await Bun.write(values.out ?? source, written); console.log(`${id}: ${count} decision texts -> ${values.out ?? source}`); }
