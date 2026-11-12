import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
let poses = 0, clips = 0, n = 0, solids = 0; const rows: string[] = [];
for (const c of readdirSync(root, { withFileTypes: true })) { if (!c.isDirectory()) continue;
 for (const s of readdirSync(join(root, c.name), { withFileTypes: true })) { if (!s.isDirectory()) continue;
  let t: string; try { t = readFileSync(join(root, c.name, s.name, "🔣️.json"), "utf8"); } catch { continue; }
  const a = JSON.parse(t).asset; n++; const p = a.poses.filter((x: any) => x.key !== true).length; poses += p; clips += a.clips.length; solids += a.volume.solids.length;
  rows.push(`${c.name}/${s.name} bytes=${t.length} poses=${p} clips=${a.clips.length} solids=${a.volume.solids.length} dur=${a.clips.map((x: any) => x.durationS + "@" + x.fps).join(",")}`); } }
console.log(n, poses, clips, solids); console.log(rows.join("\n"));
