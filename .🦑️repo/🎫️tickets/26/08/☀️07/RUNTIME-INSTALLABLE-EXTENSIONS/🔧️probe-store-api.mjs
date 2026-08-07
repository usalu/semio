import fs from "fs";

function findStore() {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = `${dir}/${e.name}`;
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (p.includes("store") && p.endsWith("store.ts") && p.includes("plugin")) {
          return p;
        }
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const store = findStore();
console.log(store);
console.log(fs.readFileSync(store, "utf8").slice(0, 4000));
