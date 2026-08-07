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

const glue = walk(".", (p, n) => p.includes("os/📦️packages/🦀️rust") && n.includes("glue.rs"))[0];
const t = fs.readFileSync(glue, "utf8");
console.log("len", t.length);
for (const line of t.split("\n")) {
  if (/space|workflow|path =/i.test(line)) console.log(line);
}
