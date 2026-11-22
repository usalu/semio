/** 🔬 Prints what `measureSuitability` reports for one scale of an asset document: `bun finisher-print-probe.ts <asset document json> <scale>`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const result = measureSuitability(asset as never, asset.representations[scale] as never, Number(scale), DEFAULT_PRINT_PROFILE) as Record<string, unknown>;
console.log(JSON.stringify(result, (key, value) => (Array.isArray(value) && value.length > 12 ? value.slice(0, 12) : value), 1).slice(0, 4000));
