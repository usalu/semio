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

const kernel = walk(".", (p, n) => p.includes("framework") && p.includes("os") && p.includes("packages") && (n === "lib.rs" || n === "Cargo.toml") && p.includes("💻️os/📦️packages")).slice(0, 20);
console.log(kernel);
for (const p of kernel) {
  const t = fs.readFileSync(p, "utf8");
  console.log("\n====", p);
  for (const line of t.split("\n")) {
    if (/space|path =/i.test(line)) console.log(line.trim().slice(0, 180));
  }
}
