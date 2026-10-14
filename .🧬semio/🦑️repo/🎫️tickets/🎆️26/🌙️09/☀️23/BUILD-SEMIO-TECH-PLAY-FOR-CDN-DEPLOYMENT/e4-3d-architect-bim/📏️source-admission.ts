/** 📏️ Measures crate roots against the global source admission ceiling (4,000 active lines / 262,144 full bytes / 32 files)
 * with the canonical cfg-selected closure reader of the native build (`📥️source/🟦️.ts`), for the native production cfg
 * and, with `--test`, the same cfg plus `test`. usage: bun 📏️source-admission.ts [--test] <name>=<repo-relative root.rs>... */
import { readFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = "/Users/ueli/Documents/semio", args = process.argv.slice(2), test = args.includes("--test");
const cfg = execFileSync("rustc", ["--print", "cfg"], { cwd: root, encoding: "utf8" }).trim().split("\n").concat(test ? ["test"] : []);
const read = async (path: string) => { try { return new Uint8Array(await readFile(path)); } catch { return null; } };
console.log(`| unit | files | active lines | full bytes | verdict |\n|---|---:|---:|---:|---|`);
for (const arg of args.filter(a => a !== "--test")) {
  const [name, entry] = arg.split("=");
  try {
    const files = await readRustSourceClosureV1([entry!], root, cfg, read);
    const lines = files.reduce((s, f) => s + f.activeLines, 0), bytes = files.reduce((s, f) => s + f.bytes, 0);
    const ok = files.length <= 32 && lines <= 4000 && bytes <= 262144;
    console.log(`| ${name}${test ? " (test)" : ""} | ${files.length} | ${lines} | ${bytes} | ${ok ? "admitted" : "REFUSED"} |`);
  } catch (error) { console.log(`| ${name} | – | – | – | ERROR ${(error as Error).message.slice(0, 400)} |`); }
}
