// Replaces literal no-break spaces (U+00A0) inside the given source files with the visible escape sequence, so a
// reader sees the deliberate no-break space. The escape is assembled from char codes to survive shell and tool quoting.
// Usage: node quiz_react_nbsp_escapes.mjs <file> [<file> …]
import { readFileSync, writeFileSync } from "node:fs";

const escape = String.fromCharCode(92) + "u00a0";
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8");
  const count = text.split(" ").length - 1;
  writeFileSync(file, text.replaceAll(" ", escape));
  console.log(`${count} ${file}`);
}
