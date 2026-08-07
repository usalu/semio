import fs from "fs";
import path from "path";

const engine = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine";
const elements = fs.readdirSync(engine).find((n) => n.includes("elements"));
const shell = path.join(engine, elements, "ShellHost", "🟦️component.tsx");
const lines = fs.readFileSync(shell, "utf8").split("\n");

for (const n of ["contributorInstances", "createApp(", "UPSERT_LOADED", "pluginRuntime", "SET_LOADED", "extensionStore", "createExtensionSource", "EXTENSION_TARGETS", "role ===", 'role === "extension"']) {
  let c = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(n)) {
      if (c < 12) console.log(String(i + 1).padStart(5), n, lines[i].trim().slice(0, 140));
      c++;
    }
  }
  if (c > 12) console.log(" +" + (c - 12) + " more " + n);
}

console.log("\n=== props / FrameworkOsShell signature ===");
for (let i = 420; i < 700; i++) {
  if (/export (function|type|const)|props|hostConfig|pluginFilter|extension/.test(lines[i])) {
    console.log(`${i + 1}|${lines[i]}`);
  }
}

// ChromePanels exports / imports at top
const chrome = path.join(engine, elements, "ChromePanels", "🟦️component.tsx");
const cl = fs.readFileSync(chrome, "utf8").split("\n");
console.log("\n=== ChromePanels top imports ===");
console.log(cl.slice(0, 60).join("\n"));

// shellLabel catalog location via uiDataLabel / shell labels map
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
const labelMaps = walk("🧰️framework", (p, n) => {
  if (!/\.(ts|tsx)$/.test(n)) return false;
  if (p.includes("/node_modules/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return t.includes('"ui.plugins.action.install"') || t.includes("'ui.plugins.action.install'");
  } catch {
    return false;
  }
});
console.log("\nLABEL MAP FILES", labelMaps);
for (const f of labelMaps) {
  const t = fs.readFileSync(f, "utf8");
  const idx = t.indexOf("ui.plugins.action.install");
  console.log(f, "\n", t.slice(Math.max(0, idx - 400), idx + 600));
}
