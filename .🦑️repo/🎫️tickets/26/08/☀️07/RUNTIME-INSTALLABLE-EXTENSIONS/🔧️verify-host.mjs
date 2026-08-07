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
for (const n of ["extensionsHostRef", "frameworkExtensionsTabs", "installExtension", "extensionLedger", "createFrameworkExtensionsPanelTabs", "extensionIdSet"]) {
  let c = 0;
  for (const line of text.split("\n")) {
    if (line.includes(n)) {
      if (c < 3) console.log(n, ":", line.trim().slice(0, 140));
      c++;
    }
  }
  console.log(n, "count", c);
}

// Check pluginsHost still unfiltered
const idx = text.indexOf("const pluginsHost:");
console.log("\npluginsHost snippet:\n", text.slice(idx, idx + 800));
