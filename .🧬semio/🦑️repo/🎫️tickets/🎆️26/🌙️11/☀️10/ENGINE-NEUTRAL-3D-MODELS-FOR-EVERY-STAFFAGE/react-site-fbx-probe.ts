import { readFileSync } from "node:fs";
import { exportSpatial } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧊️spatial/📤️exports/🟦️.ts";
const asset = JSON.parse(readFileSync("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-adult/🔣️.json", "utf8")).asset;
console.log("[DEBUG] clips", asset.clips.map((c: { id: string }) => c.id), asset.presentation);
for (const view of ["dark-line", "light-solid"] as const) {
  try {
    const made = await exportSpatial(asset, "fbx", { scale: 100, view, clip: asset.clips[0].id });
    console.log("[DEBUG] ok", view, made.filename, made.bytes.length);
  } catch (e) { console.log("[DEBUG] fail", view, String(e).slice(0, 200)); }
}
