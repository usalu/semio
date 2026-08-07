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
  if (p.includes("/.🦑️repo/") || p.includes("/target/")) return false;
  try {
    return fs.readFileSync(p, "utf8").includes("modules/🪐️space/🦀️component.rs");
  } catch {
    return false;
  }
});
console.log(hits.join("\n") || "(none)");
