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
for (const line of t.split("\n")) {
  if (/export.*EXTENSION_TARGETS|export \{[^}]*EXTENSION/.test(line) || line.includes("EXTENSION_TARGETS")) {
    console.log(line.trim().slice(0, 200));
  }
}

const pkg = find((p, n) => p.includes("framework-core") && n === "package.json");
console.log("pkg", pkg);

// encodeActionWire location
const i = t.indexOf("export function encodeActionWire");
console.log("encodeActionWire", i >= 0);
