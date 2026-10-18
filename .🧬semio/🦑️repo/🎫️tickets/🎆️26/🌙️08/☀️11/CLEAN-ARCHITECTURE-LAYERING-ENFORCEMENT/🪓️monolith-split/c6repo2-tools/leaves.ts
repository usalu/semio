import { assign, type Plan } from "./split.ts";
import { readFileSync } from "node:fs";
import { dirname, relative } from "node:path";
const [planFile, backupParent] = process.argv.slice(2);
const plan: Plan = JSON.parse(readFileSync(planFile, "utf8"));
const a = assign({ ...plan, file: backupParent, dir: plan.dir ?? dirname(plan.file) });
console.log(JSON.stringify(a.chunkFiles.map((f) => relative("/Users/ueli/Documents/semio", f))));
