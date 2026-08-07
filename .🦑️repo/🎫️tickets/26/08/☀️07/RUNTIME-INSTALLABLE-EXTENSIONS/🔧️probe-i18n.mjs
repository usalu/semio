import fs from "fs";
import path from "path";
function walk(dir, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, acc);
    else acc.push(p);
  }
  return acc;
}
const hits = [];
for (const p of walk(".")) {
  if (!/\.(ts|tsx|json)$/.test(p)) continue;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/") || p.includes("/target/")) continue;
  try {
    const t = fs.readFileSync(p, "utf8");
    if (t.includes("ui.plugins.status.available") || t.includes("ui.plugins.action.install")) hits.push(p);
  } catch {}
}
console.log(hits.join("\n"));
for (const p of hits) {
  if (p.includes("ChromePanels")) continue;
  const t = fs.readFileSync(p, "utf8");
  const i = t.indexOf("ui.plugins");
  console.log("\n====", p, "====");
  console.log(t.slice(Math.max(0, i - 100), i + 1200));
}
