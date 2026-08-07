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

const shell = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx") && n.includes("component"));
const lines = fs.readFileSync(shell, "utf8").split("\n");
for (const n of ["encodeActionWire", "loadPluginModuleResilient", "loadPluginModule", "EXTENSION_INSTALL", "createFrameworkPluginsPanelTabs"]) {
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(n)) console.log(i + 1, n, lines[i].trim().slice(0, 140));
  }
}

// ChromePanels export section end
const chrome = find((p, n) => p.includes("ChromePanels") && n.endsWith(".tsx") && n.includes("component"));
console.log("chrome", chrome);
const ct = fs.readFileSync(chrome, "utf8");
console.log("ends with", ct.slice(-200));
