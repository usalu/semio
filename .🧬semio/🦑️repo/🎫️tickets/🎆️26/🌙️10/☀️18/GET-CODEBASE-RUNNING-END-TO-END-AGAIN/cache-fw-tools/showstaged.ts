import { readFileSync } from "node:fs";
import { join } from "node:path";
const dir = join(import.meta.dir, "../🗑️generated/cache-fw-stage");
const idx = readFileSync(join(dir, "index.txt"), "utf8").split("\n");
const [pn, ...tn] = process.argv.slice(2);
for (const line of idx) { const [n] = line.split(" "); const j = JSON.parse(readFileSync(join(dir, `${n}.new.json`), "utf8")); if (j.name !== pn) continue; console.log("namedInputs:", JSON.stringify(j.namedInputs).slice(0, 900)); for (const t of tn) { const x = j.targets[t]; if (!x) continue; const { options, metadata, ...rest } = x; console.log(t, JSON.stringify(rest).slice(0, 500)); } }
