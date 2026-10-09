#!/usr/bin/env bun
/** 🔲️ Writes the authored input of the committed spaces case `🧫️fixtures/💡️inferences/🛋️spaces/🔲️hung-edit`: the committed rooms model after four ceiling edits (the living ceiling hangs 0.4 m lower, a bulkhead hangs lower still over the living seed, a hole opens the kitchen ceiling over its seed, the sloped kitchen ceiling loses its slope). The shapely oracle of `🧪️tests/🏠️infer-bim-1-spaces/🐍️.py` writes the expectation (`python 🐍️.py write`). Usage: `bun r11-w06-ceilings-spaces-case.ts`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../..");
const spaces = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/🛋️spaces");
const snapshot = JSON.parse(readFileSync(join(spaces, "🏡️rooms/📸️snapshot/🔣️.json"), "utf8"));
const square = (x0: number, y0: number, x1: number, y1: number) => [[x0, y0], [x1, y0], [x1, y1], [x0, y1]].map(([x, y]) => ({ point: { x, y }, bulge: 0 }));

snapshot.ceilings["ce-living"].offset = 0.8;
snapshot.ceilings["ce-bulkhead"] = { storey: "st-ground", ceiling_type: "cet-deep", boundary: square(2, 2, 3, 3), holes: [], offset: 1.2, name: "Living bulkhead" };
snapshot.ceilings["ce-kitchen"].holes = [square(6.2, 2.7, 6.8, 3.3)];
delete snapshot.ceilings["ce-kitchen"].slope;

const target = join(spaces, "🔲️hung-edit/📸️snapshot/🔣️.json");
mkdirSync(join(target, ".."), { recursive: true });
writeFileSync(target, JSON.stringify(snapshot, null, 2) + "\n", "utf8");
console.log("wrote", target);
