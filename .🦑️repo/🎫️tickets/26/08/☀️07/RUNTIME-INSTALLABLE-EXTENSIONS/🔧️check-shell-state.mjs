import fs from "fs";
import path from "path";

function findShell() {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = path.join(dir, e.name);
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (p.includes("ShellHost") && e.name.endsWith(".tsx") && e.name.includes("component")) {
          return p;
        }
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const shellPath = findShell();
let text = fs.readFileSync(shellPath, "utf8");
const markers = {
  hasInstallExtension: text.includes("installExtension"),
  hasExtensionsHost: text.includes("ExtensionsHostApi"),
  hasLoadedExtensions: text.includes("loadedExtensions") || text.includes("extensionLedger"),
  hasCreateFrameworkExtensions: text.includes("createFrameworkExtensionsPanelTabs"),
  setContributionsPush: text.includes("setContributions push skipped"),
};
console.log(JSON.stringify(markers, null, 2));
console.log("shell", shellPath);

// Show uninstallPlugin end of region for insertion point
const endIdx = text.indexOf("//#endregion 🔌️PluginRuntime");
console.log("--- before endregion ---");
console.log(text.slice(endIdx - 400, endIdx + 40));
