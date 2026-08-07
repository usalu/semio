import fs from "fs";
import path from "path";

function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (e.name === "node_modules" || e.name === "target" || e.name === ".git") continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, pred, acc);
    else if (pred(p, e.name)) acc.push(p);
  }
  return acc;
}

const root = process.cwd();
const hits = walk(root, (p, n) => {
  if (!/\.(ts|tsx|mjs|js)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/") || p.includes("/target/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /createExtensionSource|ExtensionStore|installFromUrl|ExtensionsHostApi|installExtension\b/.test(t);
  } catch {
    return false;
  }
});
console.log(hits.join("\n"));
console.log("COUNT", hits.length);

const regHits = walk(path.join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"), (p, n) => n.endsWith(".ts") || n.endsWith(".tsx"));
console.log("\nPLUGIN TS FILES:");
for (const p of regHits) console.log(p);
