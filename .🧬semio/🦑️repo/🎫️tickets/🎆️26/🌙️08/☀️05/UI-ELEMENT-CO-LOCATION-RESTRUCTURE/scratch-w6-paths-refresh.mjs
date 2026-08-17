import { readdirSync, statSync, writeFileSync, readFileSync } from "fs";
import { join } from "path";
const repo = process.cwd();
function findChild(p, sub) {
  for (const n of readdirSync(p)) {
    const x = join(p, n);
    try {
      if (statSync(x).isDirectory() && n.includes(sub)) return x;
    } catch {}
  }
  return null;
}
function findNamed(root, name, depth = 0, out = []) {
  if (depth > 8) return out;
  for (const n of readdirSync(root)) {
    const p = join(root, n);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (!st.isDirectory()) continue;
    if (n === name) out.push(p);
    else findNamed(p, name, depth + 1, out);
  }
  return out;
}
const ui = findChild(findChild(findChild(repo, "framework"), "modules"), "ui");
const el = findChild(ui, "elements");
const react = findChild(findChild(findChild(findChild(ui, "packages"), "typescript"), "targets"), "react");
const barrel = join(react, readdirSync(react).find((n) => n.includes("index") && n.endsWith(".tsx")));
const tickets = findNamed(join(repo, ".🦑️repo"), "UI-ELEMENT-CO-LOCATION-RESTRUCTURE");
const ticket = tickets.map((p) => ({ p, n: readdirSync(p).length })).sort((a, b) => b.n - a.n)[0].p;
writeFileSync("/tmp/semio-w6-paths.txt", [ticket, el, barrel].join("\n") + "\n");
console.log("TICKET", ticket);
console.log("EL", el);
console.log("BARREL", barrel);
const t = readFileSync(barrel, "utf8");
for (const m of t.matchAll(/from "([^"]*elements\/[^"]*(?:Button|ButtonGroup|ContextMenu|Icons)[^"]*)"/g)) {
  console.log("import", m[1]);
}
console.log(
  "dirs",
  readdirSync(el).filter((n) => /Button|Context|Icon|Drag|PanelTab|Action|Select/.test(n)),
);
// verify leaf exports still present
for (const name of ["Button", "ButtonGroup", "ContextMenu"]) {
  const dir = readdirSync(el).find((n) => n.endsWith(name) || n === name);
  const comp = join(el, dir, readdirSync(join(el, dir)).find((n) => n.endsWith("component.tsx")));
  const exports = readFileSync(comp, "utf8")
    .split("\n")
    .filter((l) => l.startsWith("export"))
    .slice(0, 5);
  console.log(dir, exports);
}
