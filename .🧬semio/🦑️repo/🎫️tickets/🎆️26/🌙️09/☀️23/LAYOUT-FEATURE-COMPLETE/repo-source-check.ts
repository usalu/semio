import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const symbol = process.env.REPO_SOURCE_CHECK_SYMBOL ?? "portableSourceAnalysisOwner";
const roots = [
  "🧰️framework/🔨️modules/🏃️process",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules",
];

function search(directory: string): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "target") continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) search(path);
    if (entry.isFile() && /\.(?:ts|mjs|json)$/.test(path)) {
      const lines = readFileSync(path, "utf8").split("\n");
      for (const [index, line] of lines.entries()) {
        if (line.includes(symbol)) console.log(`[DEBUG] ${path}:${index + 1} ${line.slice(0, 400)}`);
      }
    }
  }
}

const [file, start, end] = process.argv.slice(2);
if (file) {
  const first = Number(start ?? 1);
  const last = Number(end ?? 200);
  const lines = readFileSync(file, "utf8").split("\n");
  for (let index = first - 1; index < Math.min(last, lines.length); index++) {
    console.log(`[DEBUG] ${index + 1} ${lines[index]}`);
  }
} else {
  for (const root of process.env.REPO_SOURCE_CHECK_ROOT ? [process.env.REPO_SOURCE_CHECK_ROOT] : roots) search(root);
}
