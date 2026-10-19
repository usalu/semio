import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const glue = resolve("🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts");
for (const file of process.argv.slice(2)) {
  const spec = relative(dirname(resolve(file)), glue).replaceAll("\\", "/");
  const text = readFileSync(file, "utf8");
  const next = text.split('"@semio-tech/infinite-world-r3f"').join(`"${spec}"`);
  writeFileSync(file, next);
  console.log(text.split('"@semio-tech/infinite-world-r3f"').length - 1, file.slice(-60));
}
