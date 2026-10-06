#!/usr/bin/env bun
/**
 * 🔤️ S5-CHANNEL (design §22.19): seals `🖨️describe/🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json` from its `authored`
 * values with the TypeScript side only — `canonical` = the members of every object in UTF-8 key-byte order, `expectedPackHex` =
 * `encodePackValue(authored)` — and then runs the fixture's own oracle. The Rust emitter never sees this script: its law has to
 * reproduce the sealed bytes on its own.
 * Usage: bun 🧪️s5-channel-seal-canonical-descriptor-pack.ts [--write]
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { canonicalDescriptorPackOracle, canonicalDescriptorValue, descriptorPackHex } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🧪️canonical-descriptor-pack/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const PATH = join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json");
const write = process.argv.includes("--write");
const fixture = JSON.parse(readFileSync(PATH, "utf8"));
if (!Array.isArray(fixture.cases) || fixture.cases.length === 0) throw new Error("the fixture carries no cases");
const row = (value: unknown) => JSON.stringify(value).replace(/([{,])"/g, '$1 "').replace(/":/g, '": ').replace(/}/g, " }").replace(/\{ \}/g, "{}").replace(/\},\{/g, "}, {");
const blocks: string[] = [];
for (const entry of fixture.cases) {
  entry.canonical = canonicalDescriptorValue(entry.authored);
  entry.expectedPackHex = descriptorPackHex(entry.authored);
  console.log(`[DEBUG] ${entry.id}: ${entry.expectedPackHex.length / 2} bytes ${entry.expectedPackHex.slice(0, 48)}…`);
  blocks.push(`    {\n      "id": ${JSON.stringify(entry.id)},\n      "authored": ${row(entry.authored)},\n      "canonical": ${row(entry.canonical)},\n      "expectedPackHex": ${JSON.stringify(entry.expectedPackHex)}\n    }`);
}
const text = `{\n  "$schema": ${JSON.stringify(fixture.$schema)},\n  "note": ${JSON.stringify(fixture.note)},\n  "cases": [\n${blocks.join(",\n")}\n  ]\n}\n`;
if (JSON.stringify(JSON.parse(text)) !== JSON.stringify(fixture)) throw new Error("the sealed text does not parse back to the sealed value");
if (write) {
  writeFileSync(PATH, text);
  console.log(`[DEBUG] sealed ${fixture.cases.length} cases; oracle cases=${canonicalDescriptorPackOracle(ROOT)}`);
} else console.log(`[DEBUG] dry run: ${fixture.cases.length} cases would be sealed`);
