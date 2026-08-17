import fs from "fs";
import path from "path";

function findFile() {
  const fw = fs.readdirSync(".").find((n) => n.endsWith("framework"));
  const products = path.join(fw, "🛍️products");
  const repo = fs.readdirSync(products).find((n) => n.endsWith("repo"));
  // walk to packages/typescript index
  function walk(d) {
    for (const e of fs.readdirSync(d, { withFileTypes: true })) {
      const p = path.join(d, e.name);
      if (e.isDirectory()) {
        if (["node_modules", "target", "dist"].includes(e.name)) continue;
        const hit = walk(p);
        if (hit) return hit;
      } else if (e.name.endsWith("index.ts") && p.includes("packages") && p.includes("typescript") && p.includes("lib")) {
        const t = fs.readFileSync(p, "utf8");
        if (t.includes("cachedCrateIndex")) return p;
      }
    }
  }
  return walk(path.join(products, repo));
}

const file = findFile();
const text = fs.readFileSync(file, "utf8");
const start = text.indexOf("const cachedCrateIndex = ephemeralBox");
if (start < 0) {
  console.log("not found");
  process.exit(1);
}
// show surrounding
console.log(text.slice(start, start + 500));
