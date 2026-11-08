import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = process.cwd();
for (const entry of process.argv.slice(2)) {
  const out = await readRustSourceClosureV1([resolve(root, entry)], root, [], async (p: string) => { try { return readFileSync(p); } catch { return null; } });
  console.log(out.length, "files", out.reduce((s: number, i: any) => s + i.bytes, 0), "bytes", out.reduce((s: number, i: any) => s + i.activeLines, 0), "lines", entry.slice(-60));
}
