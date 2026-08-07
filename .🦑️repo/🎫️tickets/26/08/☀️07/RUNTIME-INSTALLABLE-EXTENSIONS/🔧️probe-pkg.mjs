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

const pkg = find((p, n) => p.includes("📈️registry") && n === "package.json");
console.log("pkg", pkg);
if (pkg) console.log(fs.readFileSync(pkg, "utf8"));

const idx = find((p, n) => p.includes("registry") && n === "index.ts");
console.log("idx", idx);
