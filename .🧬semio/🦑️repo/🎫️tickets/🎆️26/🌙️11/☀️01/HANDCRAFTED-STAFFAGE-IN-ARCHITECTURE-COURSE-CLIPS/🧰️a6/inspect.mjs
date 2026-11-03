import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const root = process.argv[2];
const mode = process.argv[3];
if (mode === "plans") {
  for (const cat of readdirSync(root)) {
    let ents;
    try { ents = readdirSync(join(root, cat)); } catch { continue; }
    for (const id of ents) {
      let d;
      try { d = JSON.parse(readFileSync(join(root, cat, id, "🔣️.json"), "utf8")); } catch { continue; }
      const a = d.asset;
      const pl = Object.entries(a.representations).filter(([, r]) => r.geometry && r.geometry.plan).map(([k]) => k);
      if (pl.length) console.log(cat, id, pl.join(","), JSON.stringify(a.presentation.viewpoints));
    }
  }
} else {
  const d = JSON.parse(readFileSync(mode, "utf8"));
  const a = d.asset;
  const keys = process.argv.slice(4);
  let v = a;
  for (const k of keys) v = v[k];
  console.log(JSON.stringify(v, null, 1).slice(0, 12000));
}
