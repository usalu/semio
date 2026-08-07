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
  try {
    const t = fs.readFileSync(p, "utf8");
    return /#\[path[^\]]*space/.test(t) || /mod space\s*\{/.test(t) || t.includes('pub mod space');
  } catch {
    return false;
  }
});
for (const p of hits) {
  console.log(p);
  const lines = fs.readFileSync(p, "utf8").split("\n");
  for (let i = 0; i < lines.length; i++) {
    if (/space/i.test(lines[i]) && /mod |path|use /.test(lines[i])) console.log(" ", i + 1, lines[i].trim().slice(0, 160));
  }
}
