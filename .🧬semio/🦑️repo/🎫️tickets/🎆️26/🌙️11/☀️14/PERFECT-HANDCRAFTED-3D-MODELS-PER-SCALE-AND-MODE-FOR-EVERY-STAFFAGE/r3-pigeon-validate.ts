/** 🩺️ Round 3 pigeon: runs the catalog `validateAsset` on an asset file (default: the catalog file with the author's source merged over it) and prints every issue. Usage: bun r3-pigeon-validate.ts [file] */
import { readFileSync } from "node:fs";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const file = process.argv[2];
const asset = file === undefined ? (() => { const { entry, asset } = entryOf("animals/pigeon"); const s = loadSource(entry); return s === null ? asset : merged(asset, s); })() : JSON.parse(readFileSync(file, "utf8")).asset;
const issues = validateAsset(asset as never);
console.log(`validateAsset: ${issues.length} issue(s)`);
for (const i of issues) console.log("  ", i.code, i.path, i.message.en.slice(0, 300));
