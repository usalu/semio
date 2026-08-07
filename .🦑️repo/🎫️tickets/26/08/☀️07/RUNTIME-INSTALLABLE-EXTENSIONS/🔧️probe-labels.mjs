import fs from "fs";
import path from "path";

const engine = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine";
const elements = fs.readdirSync(engine).find((n) => n.includes("elements"));
const helpersDir = path.join(engine, elements, "ShellHelpers");
for (const n of fs.readdirSync(helpersDir)) {
  const p = path.join(helpersDir, n);
  if (!/\.(ts|tsx)$/.test(n)) continue;
  const t = fs.readFileSync(p, "utf8");
  if (!t.includes("shellLabel")) continue;
  console.log("FILE", p);
  const idx = t.indexOf("function shellLabel");
  if (idx >= 0) console.log(t.slice(idx, idx + 900));
  const idx2 = t.indexOf("ui.plugins");
  if (idx2 >= 0) console.log("---\n", t.slice(Math.max(0, idx2 - 300), idx2 + 900));
}

// also search broader for SHELL_LABELS / translation map
function walk(dir, acc = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, acc);
    else if (/\.(ts|tsx)$/.test(e.name)) acc.push(p);
  }
  return acc;
}
for (const p of walk(path.join(engine, elements))) {
  const t = fs.readFileSync(p, "utf8");
  if (t.includes('"ui.plugins.status') || t.includes("ui.plugins.status.available")) {
    console.log("STATUS LABELS", p);
    const i = t.indexOf("ui.plugins.status");
    console.log(t.slice(Math.max(0, i - 200), i + 1000));
  }
}
