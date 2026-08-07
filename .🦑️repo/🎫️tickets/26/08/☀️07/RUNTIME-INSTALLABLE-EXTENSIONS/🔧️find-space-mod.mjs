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

for (const p of walk("🧰️framework/🛍️products/💻️os", (p, n) => n === "lib.rs" || n === "Cargo.toml" || n === "mod.rs")) {
  const t = fs.readFileSync(p, "utf8");
  if (/🪐️space|modules\/.*space|path = ".*space/.test(t) && (p.includes("packages") || p.endsWith("lib.rs"))) {
    console.log(p);
    for (const line of t.split("\n")) {
      if (/space/i.test(line)) console.log(" ", line.trim().slice(0, 160));
    }
  }
}
