/** 🖨️ Which shapes of a band violate a print profile: `bun suit.ts <file.json> <scale> [profile]` prints the violations of `measureSuitability` (check, shapes, measured and required millimetres) for the authored representation of the scale, default profile `office-laser`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const rep = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
const scale = process.argv[3]!;
const name = process.argv[4] ?? "office-laser";
const profile = rep.printProfile(name);
const result = rep.measureSuitability(asset, asset.representations[scale], Number(scale), profile);
for (const violation of result.violations) console.log(JSON.stringify(violation));
console.log(`${asset.id} 1:${scale} ${name}: ${result.violations.length} violations`);
