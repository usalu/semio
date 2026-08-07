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
let text = fs.readFileSync(shellPath, "utf8");

if (text.includes(".filter((entry) => !extensionIdSet.has(entry.pluginId))")) {
  console.log("plugins already filtered");
  process.exit(0);
}

const old = `      plugins: registry.map((entry): PluginsPanelEntry => {`;
const neu = `      plugins: registry
        .filter((entry) => !extensionIdSet.has(entry.pluginId))
        .map((entry): PluginsPanelEntry => {`;

if (!text.includes(old)) throw new Error("plugins map not found");
text = text.replace(old, neu);

const depsOld = `[registry, loadedPlugins, pluginStatusById, pluginSource, primaryPluginId, session?.pluginId, installPlugin, uninstallPlugin, reloadPlugin],
  );
  pluginsHostRef.current = pluginsHost;`;

const depsNew = `[registry, loadedPlugins, pluginStatusById, pluginSource, primaryPluginId, session?.pluginId, installPlugin, uninstallPlugin, reloadPlugin, extensionIdSet],
  );
  pluginsHostRef.current = pluginsHost;`;

if (!text.includes(depsOld)) throw new Error("pluginsHost deps not found");
text = text.replace(depsOld, depsNew);

fs.writeFileSync(shellPath, text);
console.log("filtered pluginsHost");
