import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { existsSync } from "node:fs";
for (const root of process.argv.slice(2)) {
  const files = listModuleTree(root, { tests: true });
  const missing = files.filter((file) => !existsSync(file));
  console.log(JSON.stringify({ root: root.split("/").slice(-3).join("/"), files: files.length, missing: missing.length }));
}
