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
  if (!p.endsWith(".rs")) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/target/")) return false;
  if (p.includes("🪐️space/🦀️component.rs")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /empty_space_projection|pub mod space|mod space /.test(t) && /#\[path/.test(t);
  } catch {
    return false;
  }
});
console.log("path mods", hits.join("\n"));

const hits2 = walk(".", (p, n) => {
  if (!p.endsWith(".rs")) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/target/")) return false;
  try {
    return fs.readFileSync(p, "utf8").includes("empty_space_projection");
  } catch {
    return false;
  }
});
console.log("empty_space refs", hits2.join("\n"));
