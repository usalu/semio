import { readFileSync } from "node:fs";
import { castManifestIssues } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
for (const f of process.argv.slice(2)) { const d = JSON.parse(readFileSync(f,"utf8")); const m = d.manifest ?? d; console.log(f, castManifestIssues(m).slice(0,5)); }
