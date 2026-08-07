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

const hits = walk(".", (p, n) => {
  if (!/\.(rs|toml)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/target/") || p.includes("/node_modules/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return t.includes("modules/🪐️space") || t.includes('name = "semio-framework-os-space"') || /mod space;|pub mod space/.test(t);
  } catch {
    return false;
  }
});
console.log(hits.join("\n"));
