/** 🪤️ Author 62: trap scan of three vehicle sources (revolve closure, taper, loft folds) without touching kernel-traps.md. Usage: bun author-62-traps.ts vehicles/city-bus ... */
import { readFileSync } from "node:fs";
import { loftFault, revolveClosure, taperFault } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
let total = 0;
for (const a of process.argv.slice(2)) {
  const src = JSON.parse(readFileSync(`./volumes/${a}.json`, "utf8"));
  for (const [sc, v] of Object.entries<any>(src.volumes)) {
    for (const list of ["body", "thermal", "structure"]) {
      for (const s of v[list] ?? []) {
        if (s.kind === "revolve") { const r: any = revolveClosure(s.profile); if (r && r.kind === "shell") { console.log(a, sc, list, s.id, "revolve", JSON.stringify(r)); total++; } }
        if (s.kind === "extrude" && s.taperTo) { const f: any = taperFault(s.taperTo); if (f) { console.log(a, sc, list, s.id, "taper", JSON.stringify(f)); total++; } }
        if (s.kind === "loft") { const f: any = loftFault(s.sections); if (f) { console.log(a, sc, list, s.id, "loft", JSON.stringify(f).slice(0, 200)); total++; } }
      }
    }
  }
}
console.log("findings", total);
