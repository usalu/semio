/** 🕳️ The catalog case of `🧪️tests/🕳️fill-rule` as a list: every filled path whose subpaths overlap without nesting, with the oracle of the suite (shapely). Usage: overlaps.ts [<file.json> …] (default: every catalog document, one at a time). */
import { readdirSync, readFileSync, writeFileSync, mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { flattenPath } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { ringOverlaps } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️.ts";

const root = process.env.S + "/🗂️catalog";
const files = process.argv.length > 2 ? process.argv.slice(2) : readdirSync(root, { withFileTypes: true }).filter((e) => e.isDirectory()).flatMap((c) => readdirSync(join(root, c.name), { withFileTypes: true }).filter((e) => e.isDirectory()).map((e) => join(root, c.name, e.name, "🔣️.json")));
const shapes: { asset: string; scale: string; view: string; id: string; rings: number[][][] }[] = [];
for (const file of files) {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  for (const [scale, representation] of Object.entries<any>(asset.representations ?? {})) for (const [view, list] of Object.entries<any>(representation.geometry ?? {})) for (const shape of list as any[]) {
    if (shape.kind !== "path" || shape.filled === false) continue;
    const rings = flattenPath(shape.d).rings;
    if (rings.length > 1) shapes.push({ asset: asset.id, scale, view, id: shape.id, rings: rings.map((ring) => ring.map((p) => [p.x, p.y])) });
  }
}
const file = join(mkdtempSync(join(tmpdir(), "ml4-")), "rings.json");
writeFileSync(file, JSON.stringify(shapes));
const found = ringOverlaps(file);
for (const f of found) console.log(JSON.stringify(f));
console.log(`overlaps: ${found.length} in ${files.length} documents, ${shapes.length} multi-ring filled paths`);
