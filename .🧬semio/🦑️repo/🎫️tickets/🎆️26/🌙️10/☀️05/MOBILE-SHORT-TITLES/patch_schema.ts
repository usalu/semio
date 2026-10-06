#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../..");
const path = join(root, "🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json");
const data = JSON.parse(readFileSync(path, "utf8")) as { $defs: Record<string, { properties: Record<string, unknown> }> };
data.$defs.Quiz.properties.short = { $ref: "#/$defs/ShortText" };
data.$defs.CatalogView.properties.short = { $ref: "#/$defs/ShortText" };
writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`);
console.log("patched", path);
