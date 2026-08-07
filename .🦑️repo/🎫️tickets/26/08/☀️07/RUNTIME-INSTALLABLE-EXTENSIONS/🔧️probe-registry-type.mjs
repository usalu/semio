import fs from "fs";

function findFile(pred) {
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

const shell = findFile((p, n) => p.includes("ShellHost") && n.endsWith(".tsx") && n.includes("component"));
const lines = fs.readFileSync(shell, "utf8").split("\n");
for (let i = 820; i < 845; i++) console.log(`${i + 1}|${lines[i]}`);

const core = findFile((p, n) => p.includes("🧩core") && n === "🟦️component.ts" && !p.includes("node_modules"));
console.log("core", core);
const t = fs.readFileSync(core, "utf8");
const i = t.indexOf("export type PluginRegistryEntry");
console.log(t.slice(i, i + 500));
const j = t.indexOf("export function expandPluginRegistry");
console.log("---expand---");
console.log(t.slice(j, j + 800));
