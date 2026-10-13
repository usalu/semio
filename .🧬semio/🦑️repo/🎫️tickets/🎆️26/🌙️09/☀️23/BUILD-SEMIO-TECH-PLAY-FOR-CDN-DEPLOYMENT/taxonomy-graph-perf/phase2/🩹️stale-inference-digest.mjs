/** 🩹️ Forces a persisted inference record to miss while keeping its observations (the state after any one observed file changes):
 * rewrites only the header digest. argv: <cacheFile>... */
import { readFileSync, writeFileSync } from "node:fs";
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8"), split = text.indexOf("\n"), header = JSON.parse(text.slice(0, split));
  header.digest = "0".repeat(64);
  writeFileSync(file, `${JSON.stringify(header)}${text.slice(split)}`);
  console.log(`[DEBUG] staled ${file} observations=${header.observations.length}`);
}
