/** 🖨️ Prints the print-suitability violations of the operating table drawings at one scale: `bun r3-operating-table-suit.ts 50 [asset document json]`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? "50");
const asset = process.argv[3] ? JSON.parse(readFileSync(process.argv[3], "utf8")).asset : entryOf("furniture/operating-table").asset;
const r = (resolveRepresentation as any)(asset, { denominator: scale }, DEFAULT_PRINT_PROFILE);
console.log(r.kind, JSON.stringify(r.suitability?.violations ?? r.violations ?? r, null, 0).slice(0, 3000));
