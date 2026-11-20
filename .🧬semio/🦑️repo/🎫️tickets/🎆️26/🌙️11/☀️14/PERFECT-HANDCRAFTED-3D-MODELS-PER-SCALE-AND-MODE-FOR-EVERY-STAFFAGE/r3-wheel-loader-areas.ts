/** 📏 Outer areas per thermal region of the wheel loader candidate at one scale, as the product measures them: `bun r3-wheel-loader-areas.ts <scale>`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const scale = Number(process.argv[2] ?? 50);
const asset = JSON.parse(readFileSync("🗑️generated/r3-wheel-loader/🔣️.json", "utf8")).asset;
const m = measureVolumes(asset, { scale: scale as never, rules: ["volumes-thermal-coverage" as never] });
for (const x of m) if (/outerAreaM2|areaDeviation/.test(x.name)) console.log(`[DEBUG] ${x.name}=${x.value.toFixed(3)}`);
