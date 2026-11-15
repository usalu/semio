import { readFileSync } from "node:fs";
import { checkVolumes, VOLUME_RULES } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const scale = process.argv[3] ? Number(process.argv[3]) : undefined;
for (const r of VOLUME_RULES) { const t = Date.now(); checkVolumes(a, { rules: [r], ...(scale ? { scale: scale as never } : {}) }); console.log(r, Date.now() - t); }
