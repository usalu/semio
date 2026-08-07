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

const shellPath = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx") && n.includes("component"));
const text = fs.readFileSync(shellPath, "utf8");
const idx = text.indexOf("const extensionsHost");
console.log(text.slice(idx, idx + 2200));
