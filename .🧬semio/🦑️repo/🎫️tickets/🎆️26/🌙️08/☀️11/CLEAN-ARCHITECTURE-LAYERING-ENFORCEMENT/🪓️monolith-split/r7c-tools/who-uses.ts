// 🔎️ Lists which of the top-level item names of one Rust file are used by each of the given other files: usage: bun who-uses.ts <defining file> <file>...
import { readFileSync } from "node:fs";
import { parseRustSourceTree, usedNames } from "./lib-imports.ts";
const [def, ...files] = process.argv.slice(2);
const text = readFileSync(def!, "utf8");
const names = parseRustSourceTree(text).items.filter((i: any) => i.name && i.kind !== "use" && i.kind !== "impl").map((i: any) => i.name as string);
for (const f of files) {
  const used = [...usedNames(readFileSync(f, "utf8"), names)];
  console.log(`${f.split("/").slice(-6).join("/")}: ${used.join(", ") || "-"}`);
}
