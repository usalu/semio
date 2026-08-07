import fs from "fs";

function find(pred) {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = `${dir}/${e.name}`;
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (pred(p, e.name)) return p;
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const store = find((p, n) => p.includes("🏪️store") && n.includes("store.ts"));
const t = fs.readFileSync(store, "utf8");
const i = t.indexOf("async installFromUrl");
console.log(t.slice(i, i + 1200));
console.log("--- uninstall ---");
const j = t.indexOf("async uninstall");
console.log(t.slice(j, j + 600));
console.log("--- list ---");
const k = t.indexOf("async listInstalled");
console.log(t.slice(k, k + 800));

const gen = find((p, n) => p.includes("generated") && n.includes("plugins.ts") && p.includes("registry"));
const g = fs.readFileSync(gen, "utf8");
console.log("--- PluginBuildTarget ---");
const m = g.match(/export type PluginBuildTarget[\s\S]{0,500}/);
console.log(m && m[0]);
console.log("--- first extensions with extends ---");
for (const line of g.split("\n")) {
  if (line.includes('role: "extension"') && line.includes("extends")) console.log(line.trim().slice(0, 200));
  else if (line.includes('role: "extension"')) console.log(line.trim().slice(0, 200));
}
