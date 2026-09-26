/** 🔎️ LA: prints the `default` / `production` named inputs the 🟨️.mjs plugin derives for one project (argv[2] = project root). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { cacheInternals } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";

const root = "/Users/ueli/Documents/semio";
const projectRoot = process.argv[2]!;
const json = JSON.parse(readFileSync(join(root, projectRoot, "📋️project.json"), "utf8"));
const named = cacheInternals.projectInputs(json, projectRoot, root, new Map());
console.log(JSON.stringify({ default: named.default, production: named.production }, null, 1));
