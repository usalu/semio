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

const cargos = walk(".", (p, n) => n === "Cargo.toml" && p.includes("os") && !p.includes("node_modules"));
for (const c of cargos) {
  const t = fs.readFileSync(c, "utf8");
  if (t.includes("space") || t.includes("🪐️space")) {
    const name = t.match(/name\s*=\s*"([^"]+)"/);
    console.log(c, name && name[1]);
  }
}
