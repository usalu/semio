/**
 * 🎲️ Picks a deterministic sample of the glue files listed in `repoapi-tools/exports-final.json` (existing, TypeScript, not wrapping JSON) and prints them one per line.
 *
 *   bun glue-tools/sample-glue.ts [--count=20]
 */
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { ROOT } from "./check-exports.ts";

const count = Number(process.argv.find(a => a.startsWith("--count="))?.slice(8) ?? 20);
const spec = JSON.parse(readFileSync(resolve(import.meta.dir, "../repoapi-tools/exports-final.json"), "utf8"));
const glue = Object.values<any>(spec.packages).flatMap(p => Object.values<any>(p.entries)).filter(e => e.glueFile && !e.existing && e.kind === "module" && /\.tsx?$/u.test(e.glueFile) && existsSync(resolve(ROOT, e.glueFile))).map(e => e.glueFile as string).sort();
const step = Math.max(1, Math.floor(glue.length / count));
for (let i = 0; i < glue.length && i / step < count; i += step) console.log(glue[i]);
