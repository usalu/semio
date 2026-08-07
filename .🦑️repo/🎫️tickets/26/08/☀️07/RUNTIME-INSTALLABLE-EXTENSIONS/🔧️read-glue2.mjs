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

const glues = walk(".", (p, n) => n.includes("glue") && p.includes("💻️os") && p.includes("packages"));
console.log(glues);
for (const glue of glues) {
  console.log("\n====", glue);
  const t = fs.readFileSync(glue, "utf8");
  for (const line of t.split("\n").slice(0, 80)) console.log(line);
}
