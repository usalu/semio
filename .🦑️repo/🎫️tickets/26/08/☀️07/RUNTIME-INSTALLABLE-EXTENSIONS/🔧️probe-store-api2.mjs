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
        } else if (p.includes("🏪️store") && nEnds(e.name)) return p;
      } catch {}
    }
    return null;
  }
  function nEnds(n) {
    return n.endsWith("store.ts") || n === "📜️store.ts";
  }
  return walk(".");
}

const store = findStore();
const t = fs.readFileSync(store, "utf8");
console.log("len", t.length);
for (const key of ["installFromUrl", "EXTENSION_INSTALL", "handleExtension", "fetch(", "export function", "uninstall", "createExtensionStore"]) {
  let idx = 0;
  let n = 0;
  while ((idx = t.indexOf(key, idx)) >= 0 && n < 5) {
    const lineStart = t.lastIndexOf("\n", idx) + 1;
    const lineEnd = t.indexOf("\n", idx);
    console.log(key, "->", t.slice(lineStart, lineEnd).trim().slice(0, 160));
    idx = lineEnd + 1;
    n++;
  }
}
console.log("--- tail exports ---");
console.log(t.slice(-2500));
