import { assign, exportTable, type Plan } from "./split.ts";
import { rewriteFile, lookupFrom } from "./rewrite.ts";
import { readFileSync, copyFileSync, mkdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
const [planFile, backupParent, mode, ...files] = process.argv.slice(2);
const plan: Plan = JSON.parse(readFileSync(planFile, "utf8"));
const a = assign({ ...plan, file: backupParent, dir: plan.dir ?? dirname(plan.file) });
const parents = new Map([[plan.file, lookupFrom(exportTable(a), plan.file, a.chunkFiles)]]);
const write = mode === "write";
for (const f of files) {
  if (write) { const d = join("/Users/ueli/.cache/c6repo2/backup/fix", relative("/Users/ueli/Documents/semio", f)); mkdirSync(dirname(d), { recursive: true }); copyFileSync(f, d); }
  const r = rewriteFile(f, parents, write);
  console.log(`${relative("/Users/ueli/Documents/semio", f)}: ${r.edits} edits`);
  r.manual.forEach((m) => console.log("  MANUAL " + m));
}
