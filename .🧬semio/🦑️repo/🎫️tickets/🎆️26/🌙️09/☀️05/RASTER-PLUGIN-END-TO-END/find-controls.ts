import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const root = process.argv[2];
const pattern = new RegExp(process.argv[3] ?? "editPixels|filter|algorithm");
function visit(path: string): void {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(entry.name)) continue;
    if (process.argv[4] === "production" && ["🧪️tests", "🧪️testing"].includes(entry.name)) continue;
    const file = join(path, entry.name);
    if (entry.isDirectory()) visit(file);
    else if (/\.(rs|tsx|ts)$/.test(file)) {
      const matches = readFileSync(file, "utf8").split("\n").flatMap((line, index) => pattern.test(line) ? [`${index + 1}: ${line}`] : []);
      if (matches.length) console.log(`${file}\n${matches.join("\n")}`);
    }
  }
}
visit(root);
