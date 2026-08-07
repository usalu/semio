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
const idx = text.indexOf("[DEBUG] setContributions push skipped");
console.log(text.slice(idx - 400, idx + 500));
console.log("\n--- count contributionsJson ---");
let c = 0;
let i = 0;
while ((i = text.indexOf("buildContributionsJson", i)) >= 0) {
  console.log(text.slice(i, i + 200).replace(/\n/g, " "));
  i += 1;
  c++;
}
console.log("count", c);
