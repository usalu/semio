import { readFileSync, existsSync } from "node:fs";
import { execSync } from "node:child_process";
import { dirname, resolve } from "node:path";
const files = execSync("find 🌎️hub -name '*.ts' -not -path '*/node_modules/*' -not -path '*/📤️dist/*'", { encoding: "utf8", maxBuffer: 1 << 28 }).trim().split("\n");
let bad = 0, checked = 0;
for (const f of files) {
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(/(?:from|import)\s+"(\.[^"]+)"/g)) {
    checked++;
    const target = resolve(dirname(f), m[1]);
    if (!existsSync(target)) { bad++; console.log(`MISSING ${f}\n  -> ${m[1]}`); }
  }
}
console.log(`hub ts files=${files.length} relative-imports=${checked} missing=${bad}`);
