/** 📒️ Round 3 heat pump: the product's manifest validation for this asset's entry (`bun r3-heat-pump-outdoor-unit-manifest.ts`). */
import { readFileSync } from "node:fs";
import { Glob } from "bun";
import { validateManifest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const root = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/";
const manifest = JSON.parse(readFileSync(root + "🔣️.json", "utf8"));
const assets = [...new Glob("*/*/🔣️.json").scanSync(root)].map((f) => JSON.parse(readFileSync(root + f, "utf8")).asset);
const issues = validateManifest(manifest, assets).filter((i) => JSON.stringify(i).includes("heat-pump"));
console.log(issues.length === 0 ? "manifest entry heat-pump-outdoor-unit: 0 issues" : issues.map((i) => `${i.code} ${i.path}: ${i.message.en}`).join("\n"));
