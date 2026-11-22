/** ⚡️ Total power (W) of HW-13 assets with the library's `totalPowerW`, as JSON {slug: watts}.
 *  bun power.ts <slug> ... */
import { totalPowerW } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const out: Record<string, number> = {};
for (const slug of process.argv.slice(2)) out[slug] = totalPowerW((await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset);
console.log(JSON.stringify(out));
