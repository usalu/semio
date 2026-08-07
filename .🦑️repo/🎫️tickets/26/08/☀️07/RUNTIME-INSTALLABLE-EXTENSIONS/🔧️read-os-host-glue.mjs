import fs from "fs";
import path from "path";

function walk(dir, pred, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    try {
      if (e.isDirectory()) walk(p, pred, acc);
      else if (pred(p, e.name)) acc.push(p);
    } catch {}
  }
  return acc;
}

const glues = walk(".", (p, n) => p.includes("💻️os/🖥️host") && n.includes("glue"));
console.log(glues);
for (const g of glues) {
  console.log("\n====", g);
  console.log(fs.readFileSync(g, "utf8"));
}
