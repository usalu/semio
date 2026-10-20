/**
 * 🔎 Importers of given package keys: every (file, specifier) in the scan graph whose resolved target is the key's glue file.
 *
 *   bun importers.ts <pkgName> <out.json>   (keys read from package.json exports whose target file matches /misplaced/)
 */
import { readFileSync, writeFileSync } from "node:fs";
import ts from "typescript";
const [listFile, out] = [process.argv[2]!, process.argv[3]!];
const wanted = new Map<string, string>(JSON.parse(readFileSync(listFile, "utf8")));
const files = readFileSync(process.argv[4]!, "utf8").split("\0").filter(f => /\.(ts|tsx|mts|js|mjs|cjs)$/u.test(f) && !/(^|\/)(node_modules|target|dist|📤️dist|🗑️generated|\.cache|\.🧬semio)\//u.test(f));
const result: Record<string, { file: string; spec: string }[]> = {};
for (const f of files) {
  let text: string;
  try { text = readFileSync("/Users/ueli/Documents/semio/" + f, "utf8"); } catch { continue; }
  if (!text.includes("@semio-tech/framework/")) continue;
  for (const imp of ts.preProcessFile(text, true, true).importedFiles) {
    const k = wanted.get(imp.fileName);
    if (k) (result[k] ??= []).push({ file: f, spec: imp.fileName });
  }
}
writeFileSync(out, JSON.stringify(result, null, 1));
for (const [k, v] of Object.entries(result)) console.log(String(v.length).padStart(4), k);
