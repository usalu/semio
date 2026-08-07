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

const chrome = find((p, n) => p.includes("ChromePanels") && n.endsWith(".tsx"));
const t = fs.readFileSync(chrome, "utf8");
console.log("has ExtensionsPanel", t.includes("ExtensionsPanel"));
console.log("has createFrameworkExtensionsPanelTabs", t.includes("createFrameworkExtensionsPanelTabs"));
console.log("has framework.settings.extensions", t.includes("framework.settings.extensions"));

const space = find((p, n) => p.includes("🪐️space") && n.endsWith(".rs") && p.includes("modules") && !p.includes("apps"));
const s = fs.readFileSync(space, "utf8");
console.log("space", space);
console.log("InstalledExtension", s.includes("struct InstalledExtension"));
console.log("InstallExtension op", s.includes("InstallExtension {"));
console.log("demo_extension", s.includes("demo_extension"));
