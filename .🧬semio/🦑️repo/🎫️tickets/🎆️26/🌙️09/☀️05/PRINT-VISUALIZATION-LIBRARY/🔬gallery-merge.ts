#!/usr/bin/env bun
/** 🖼️ Assembles the committed exhaustive gallery fixture from the lane evidence files.
 *
 * `bun 🔬gallery-merge.ts <lane.json…>` writes
 * `🧪️tests/🖼️gallery-render/🧫️fixtures/🖼️gallery-render.json` in exactly the shape
 * `regeneratePrintGalleryFixtures` writes, so `bun ./📜️script.ts test viz fixtures <section>`
 * reproduces any one section of it and `test exhaustive` verifies all of them.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const PRODUCT = join(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/📓️print");
const FIXTURE = join(PRODUCT, "🧪️tests", "🖼️gallery-render", "🧫️fixtures", "🖼️gallery-render.json");

const variants: Record<string, unknown> = {};
for (const lane of process.argv.slice(2)) for (const [id, evidence] of Object.entries(JSON.parse(readFileSync(lane, "utf8")) as Record<string, unknown>)) variants[id] = evidence;

const fixture = {
  schemaVersion: 1,
  generatedBy: "bun ./📜️script.ts test viz fixtures [section…]",
  variants: Object.fromEntries(Object.keys(variants).sort().map((id) => [id, variants[id]])),
};
writeFileSync(FIXTURE, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
console.log(`[DEBUG] gallery merge: ${Object.keys(variants).length} variants written to ${FIXTURE}`);
