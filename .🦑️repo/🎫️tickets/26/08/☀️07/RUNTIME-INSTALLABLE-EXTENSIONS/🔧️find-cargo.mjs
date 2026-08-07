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

const spaceRs = find((p, n) => p.includes("🪐️space") && n.endsWith("component.rs") && p.includes("modules") && !p.includes("apps"));
const dir = path.dirname(spaceRs);
console.log("dir", dir);
for (const n of fs.readdirSync(dir)) console.log(" ", n);
// find Cargo.toml nearby
let cur = dir;
for (let i = 0; i < 6; i++) {
  const cargo = path.join(cur, "Cargo.toml");
  if (fs.existsSync(cargo)) {
    console.log("cargo", cargo);
    console.log(fs.readFileSync(cargo, "utf8").slice(0, 400));
    break;
  }
  cur = path.dirname(cur);
}

const shell = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx"));
const t = fs.readFileSync(shell, "utf8");
const idx = t.indexOf(".filter((entry) => !extensionIdSet.has");
console.log("\nfilter snippet:\n", t.slice(idx - 80, idx + 700));
