/** 🖨️ Prints the print-suitability violations of the city bus drawings at one scale: `bun r3-city-bus-suit.ts 500 [asset document json]`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? "500");
const asset = process.argv[3] ? JSON.parse(readFileSync(process.argv[3], "utf8")).asset : entryOf("vehicles/city-bus").asset;
const r = (resolveRepresentation as any)(asset, { denominator: scale }, DEFAULT_PRINT_PROFILE);
console.log(r.kind, JSON.stringify(r.suitability?.violations ?? r.violations ?? r, null, 0).slice(0, 3000));
