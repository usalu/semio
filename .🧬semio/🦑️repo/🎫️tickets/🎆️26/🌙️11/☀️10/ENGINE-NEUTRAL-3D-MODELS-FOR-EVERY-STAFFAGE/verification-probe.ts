import { readFileSync } from "node:fs";
import { writeGlb } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { writeFbx } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeModel } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🗿️model/🟦️.ts";
const [cat, slug] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
for (const p of a.poses.filter((x: any) => x.key !== true)) { const t = Date.now(); const g = await writeGlb(a, { pose: p.id, scale: 50 }); const t1 = Date.now(); const m = await writeModel(a, { pose: p.id, scale: 50 }); console.log("glb", p.id, g.length, t1 - t, "ms; model", m.length, Date.now() - t1, "ms"); }
for (const c of a.clips) { const t = Date.now(); const g = await writeFbx(a, { clip: c.id, scale: 50 }); const t1 = Date.now(); const m = await writeModel(a, { clip: c.id, scale: 50 }); console.log("fbx", c.id, g.length, t1 - t, "ms; model", m.length, Date.now() - t1, "ms"); }
console.log("rss MB", Math.round(process.resourceUsage().maxRSS / 1024));
