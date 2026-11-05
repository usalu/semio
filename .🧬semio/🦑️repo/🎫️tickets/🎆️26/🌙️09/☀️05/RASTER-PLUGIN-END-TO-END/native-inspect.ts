import { readFileSync, readdirSync } from "node:fs";
const [root, pattern = "", before = "0", after = "0"] = process.argv.slice(2);
const expression = new RegExp(pattern);
function inspect(path: string) {
  const lines = readFileSync(path, "utf8").split("\n");
  const indices = new Set<number>();
  lines.forEach((line, index) => { if (expression.test(line)) for (let i = Math.max(0, index - Number(before)); i <= Math.min(lines.length - 1, index + Number(after)); i++) indices.add(i); });
  if (indices.size) console.log(path + "\n" + [...indices].sort((a, b) => a - b).map(i => `${i + 1}:${lines[i]}`).join("\n"));
}
function walk(path: string) {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(entry.name)) continue;
    const next = path + "/" + entry.name;
    if (entry.isDirectory()) walk(next);
    else if (/\.(rs|json|ts)$/.test(entry.name)) inspect(next);
  }
}
try { inspect(root); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "EISDIR") throw error; walk(root); }
