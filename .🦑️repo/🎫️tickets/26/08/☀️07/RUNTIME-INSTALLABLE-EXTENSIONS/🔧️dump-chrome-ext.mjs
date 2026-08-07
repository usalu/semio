import fs from "fs";
import path from "path";

function find(pred) {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = path.join(dir, e.name);
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

const chrome = find((p, n) => p.includes("ChromePanels") && n.endsWith(".tsx"));
const t = fs.readFileSync(chrome, "utf8");
const idx = t.indexOf("ExtensionsPanel");
console.log(t.slice(idx, idx + 2500));
