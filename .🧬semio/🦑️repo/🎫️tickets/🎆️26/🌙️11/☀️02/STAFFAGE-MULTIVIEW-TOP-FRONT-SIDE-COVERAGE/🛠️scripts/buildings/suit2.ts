import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { resolveRepresentation, DEFAULT_PRINT_PROFILE } = await import(`${R}/🔨️modules/🖨️representation/🟦️.ts`);
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset;
const r:any = resolveRepresentation(a,{denominator:Number(process.argv[3])},DEFAULT_PRINT_PROFILE);
console.log(r.kind); for (const v of r.suitability?.violations ?? []) console.log(JSON.stringify(v));
