import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of readdirSync(CAT)) {
  if (["agave", "date-palm", "hosta-clump"].includes(d)) continue;
  const file = `${CAT}/${d}/🔣️.json`; const t = readFileSync(file, "utf8"); const indent = t.split("\n")[1]!.match(/^ */)![0].length; const doc = JSON.parse(t); let n = 0;
  for (const clip of doc.asset.clips) { if (!/sway|flutter/.test(clip.id)) continue; for (const k of clip.keys) for (const b of Object.keys(k.bones ?? {})) if (k.bones[b].abductionDeg !== undefined) { delete k.bones[b].abductionDeg; n++; if (Object.keys(k.bones[b]).length === 0) delete k.bones[b]; } }
  console.log(d, "removed", n); if (n) writeFileSync(file, JSON.stringify(doc, null, indent) + "\n");
}
