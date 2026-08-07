import fs from "fs";
import path from "path";

function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, pred, acc);
    else if (pred(p, e.name)) acc.push(p);
  }
  return acc;
}

const hits = walk(".", (p, n) => {
  if (!/\.(rs|ts|tsx)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/") || p.includes("/target/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /enum SpaceCommand|SpaceCommand::|install.?extension|InstallProgram/.test(t);
  } catch {
    return false;
  }
});
console.log(hits.join("\n"));

for (const f of hits) {
  if (f.includes("space") && f.endsWith(".rs")) {
    const lines = fs.readFileSync(f, "utf8").split("\n");
    for (let i = 0; i < lines.length; i++) {
      if (/SpaceCommand|InstallProgram|InstallExtension|programs/.test(lines[i])) {
        console.log(`${f}:${i + 1}:${lines[i].trim().slice(0, 140)}`);
      }
    }
  }
}

// shellLabel catalog
const helpersDir = walk("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer", (p, n) => n.endsWith(".tsx") || n.endsWith(".ts"));
for (const f of helpersDir) {
  const t = fs.readFileSync(f, "utf8");
  if (t.includes('ui.plugins.action.install') || (t.includes("SHELL_LABELS") && t.includes("plugins"))) {
    console.log("FOUND LABELS IN", f);
    const idx = t.indexOf("ui.plugins");
    if (idx >= 0) console.log(t.slice(Math.max(0, idx - 100), idx + 900));
  }
  if (/export function shellLabel/.test(t)) {
    console.log("shellLabel fn in", f);
    const m = t.match(/export function shellLabel[\s\S]{0,800}/);
    if (m) console.log(m[0].slice(0, 800));
  }
}
