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

const core = find((p, n) => p.includes("🧩core") && n === "🟦️component.ts" && !p.includes("node_modules"));
const t = fs.readFileSync(core, "utf8");
const i = t.indexOf("export function multiplexPluginSources");
console.log(t.slice(i, i + 1200));
console.log("--- createExtensionSource full ---");
const j = t.indexOf("export function createExtensionSource");
console.log(t.slice(j, j + 900));
