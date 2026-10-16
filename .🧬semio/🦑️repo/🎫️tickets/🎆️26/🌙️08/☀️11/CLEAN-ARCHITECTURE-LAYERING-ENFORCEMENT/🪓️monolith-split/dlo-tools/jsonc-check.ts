/** 🔎️ Parses JSONC launch files and prints the configurations whose name contains a needle. Usage: bun jsonc-check.ts <needle> <file>... */
import { parse } from "/Users/ueli/Documents/semio/node_modules/jsonc-parser/lib/esm/main.js";
import { readFileSync } from "node:fs";
const [needle, ...files] = process.argv.slice(2);
for (const file of files) {
  const errors: unknown[] = [];
  const json = parse(readFileSync(file, "utf8"), errors as never);
  const rows = (json.configurations as { name: string; presentation: { order: number } }[]).filter((c) => c.name?.includes(needle!));
  console.log(file, "errors", errors.length, rows.map((c) => `${c.name}|${c.presentation?.order}`).join(" ; "));
}
