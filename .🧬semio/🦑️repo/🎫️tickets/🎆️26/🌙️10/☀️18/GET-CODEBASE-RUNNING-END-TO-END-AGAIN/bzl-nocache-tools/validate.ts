/** ✅ Validates the manifests of an overlay (or the live tree) that `staged/ops.json` touches against the closed project schema with Ajv: `bun validate.ts [root]`. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
const ws = "/Users/ueli/Documents/semio", root = process.argv[2] ?? ws;
const require = createRequire(join(ws, "package.json"));
const Ajv = require("ajv").default ?? require("ajv");
const schema = JSON.parse(readFileSync(join(ws, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true }), validate = ajv.compile(schema);
const ops = JSON.parse(readFileSync(join(import.meta.dir, "staged/ops.json"), "utf8")) as { manifest: string }[];
let bad = 0;
for (const manifest of new Set(ops.map((o) => o.manifest))) { const ok = validate(JSON.parse(readFileSync(join(root, manifest), "utf8"))); if (!ok) { bad++; console.log("INVALID", manifest, JSON.stringify(validate.errors?.slice(0, 3))); } }
console.log(`${new Set(ops.map((o) => o.manifest)).size} manifests, ${bad} invalid`);
