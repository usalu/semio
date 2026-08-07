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

const glue = find((p, n) => p.includes("os/📦️packages") && n.includes("glue"));
console.log("glue", glue);
const t = fs.readFileSync(glue, "utf8");
for (const line of t.split("\n")) {
  if (/space|mod |path/i.test(line)) console.log(line);
}
