/** 🩹️ The last step of a TypeScript generator of an object: applies `ops/<id>.json` of the silhouette batch SO to the document the generator wrote, in its own indentation.
 * @see ./ops.ts — the ops */
import { readFileSync, writeFileSync } from "node:fs";
import { applyOps, type Op } from "./ops.ts";

/** 🩹️ Applies the repair ops of `slug` to the asset document at `path`. */
export function repairFile(path: string, slug: string): void {
  const text = readFileSync(path, "utf8");
  const second = text.split("\n", 2)[1] ?? "";
  const doc = JSON.parse(text);
  const ops = JSON.parse(readFileSync(`${import.meta.dir}/ops/${slug}.json`, "utf8")) as Op[];
  applyOps(doc.asset, ops);
  writeFileSync(path, JSON.stringify(doc, null, second.length - second.trimStart().length) + (text.endsWith("\n") ? "\n" : ""));
  console.log(`[DEBUG] ${slug}: ${ops.length} silhouette repair ops of batch SO applied`);
}
