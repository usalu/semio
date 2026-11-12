import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
let longest = ""; 
for (const c of readdirSync(root, { withFileTypes: true })) { if (!c.isDirectory()) continue;
 for (const s of readdirSync(join(root, c.name), { withFileTypes: true })) { if (!s.isDirectory()) continue;
  let t: string; try { t = readFileSync(join(root, c.name, s.name, "🔣️.json"), "utf8"); } catch { continue; }
  const a = JSON.parse(t).asset; const ids = new Set(a.poses.filter((x: any) => x.key !== true).map((x: any) => x.id));
  for (const cl of a.clips) if (ids.has(cl.id)) console.log("COLLISION", a.id, cl.id);
  for (const p of a.poses) if (p.key === true) { }
  for (const w of [...a.poses.map((x:any)=>x.id), ...a.clips.map((x:any)=>x.id)]) { const n = `${a.id}-${w}-light-solid-model-1to50.json`; if (n.length > longest.length) longest = n; }
  if (s.name !== a.id) console.log("folder != id", s.name, a.id);
 } }
console.log(longest, longest.length);
