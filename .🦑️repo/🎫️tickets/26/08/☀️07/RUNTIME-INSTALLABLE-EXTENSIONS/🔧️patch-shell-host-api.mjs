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

if (text.includes("extensionsHostRef")) {
  console.log("extensionsHost already present");
  process.exit(0);
}

// Filter pluginsHost to exclude extension ids
const pluginsHostOld = `  const pluginsHost: PluginsHostApi = useMemo(
    () => ({
      plugins: registry.map((entry): PluginsPanelEntry => {
        const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === entry.pluginId);
        return {
          pluginId: entry.pluginId,
          label: loadedEntry?.manifest.label ?? entry.pluginId,
          version: loadedEntry?.manifest.version,
          status: pluginStatusById[entry.pluginId] ?? "available",
          sourceId: pluginSource.id,
          canUninstall: entry.pluginId !== primaryPluginId && session?.pluginId !== entry.pluginId,
        };
      }),
      install: (pluginId) => void installPlugin(pluginId),
      uninstall: (pluginId) => void uninstallPlugin(pluginId),
      reload: (pluginId) => void reloadPlugin(pluginId),
    }),
    [registry, loadedPlugins, pluginStatusById, pluginSource, primaryPluginId, session?.pluginId, installPlugin, uninstallPlugin, reloadPlugin],
  );
  pluginsHostRef.current = pluginsHost;
  const frameworkPluginsTabs = useMemo(() => createFrameworkPluginsPanelTabs(() => pluginsHostRef.current), [pluginsHost]);`;

const pluginsHostNew = `  const pluginsHost: PluginsHostApi = useMemo(
    () => ({
      plugins: registry
        .filter((entry) => !extensionIdSet.has(entry.pluginId))
        .map((entry): PluginsPanelEntry => {
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === entry.pluginId);
          return {
            pluginId: entry.pluginId,
            label: loadedEntry?.manifest.label ?? entry.pluginId,
            version: loadedEntry?.manifest.version,
            status: pluginStatusById[entry.pluginId] ?? "available",
            sourceId: pluginSource.id,
            canUninstall: entry.pluginId !== primaryPluginId && session?.pluginId !== entry.pluginId,
          };
        }),
      install: (pluginId) => void installPlugin(pluginId),
      uninstall: (pluginId) => void uninstallPlugin(pluginId),
      reload: (pluginId) => void reloadPlugin(pluginId),
    }),
    [registry, loadedPlugins, pluginStatusById, pluginSource, primaryPluginId, session?.pluginId, installPlugin, uninstallPlugin, reloadPlugin, extensionIdSet],
  );
  pluginsHostRef.current = pluginsHost;
  const frameworkPluginsTabs = useMemo(() => createFrameworkPluginsPanelTabs(() => pluginsHostRef.current), [pluginsHost]);

  const extensionsHostRef = useRef<ExtensionsHostApi | null>(null);
  const extensionsHost: ExtensionsHostApi = useMemo(() => {
    const ledgerById = new Map(extensionLedger.map((entry) => [entry.extensionId, entry] as const));
    const catalogEntries: ExtensionsPanelEntry[] = EXTENSION_TARGETS.map((target) => {
      const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === target.pluginId);
      const ledger = ledgerById.get(target.pluginId);
      return {
        extensionId: target.pluginId,
        label: loadedEntry?.manifest.label ?? target.pluginId,
        version: loadedEntry?.manifest.version ?? ledger?.version,
        extendsHost: target.extends ?? ledger?.extendsHost ?? "unscoped",
        enabled: ledger?.enabled ?? true,
        status: pluginStatusById[target.pluginId] ?? (loadedEntry ? "loaded" : "available"),
      };
    });
    for (const ledger of extensionLedger) {
      if (catalogEntries.some((entry) => entry.extensionId === ledger.extensionId)) continue;
      const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === ledger.extensionId);
      catalogEntries.push({
        extensionId: ledger.extensionId,
        label: loadedEntry?.manifest.label ?? ledger.extensionId,
        version: loadedEntry?.manifest.version ?? ledger.version,
        extendsHost: ledger.extendsHost,
        enabled: ledger.enabled,
        status: pluginStatusById[ledger.extensionId] ?? (loadedEntry ? "loaded" : "available"),
      });
    }
    return {
      extensions: catalogEntries,
      installFromUrl: (sourceUri) => void installExtension(sourceUri),
      uninstall: (extensionId) => void uninstallExtension(extensionId),
      setEnabled: (extensionId, enabled) => void setExtensionEnabled(extensionId, enabled),
    };
  }, [extensionLedger, loadedPlugins, pluginStatusById, installExtension, uninstallExtension, setExtensionEnabled]);
  extensionsHostRef.current = extensionsHost;
  const frameworkExtensionsTabs = useMemo(() => createFrameworkExtensionsPanelTabs(() => extensionsHostRef.current), [extensionsHost]);`;

if (!text.includes(pluginsHostOld)) throw new Error("pluginsHost block not found");
text = text.replace(pluginsHostOld, pluginsHostNew);

// Merge extensions tabs into bottomRight
const bottomOld = `const bottomRight: PanelTabNode[] = [...settingsRightTabs, ...frameworkPluginsTabs];`;
const bottomNew = `const bottomRight: PanelTabNode[] = [...settingsRightTabs, ...frameworkPluginsTabs, ...frameworkExtensionsTabs];`;
if (!text.includes(bottomOld)) throw new Error("bottomRight merge not found");
text = text.replace(bottomOld, bottomNew);

const depsOld = `[commandCategoryTabs, detailsRightTabs, frameworkDisplayTabs, frameworkPluginsTabs, frameworkSyncTab, frameworkUtilitiesHistoryTab, settingsRightTabs, toolTa`;
// find the useMemo deps line
const depsIdx = text.indexOf("frameworkPluginsTabs, frameworkSyncTab");
if (depsIdx < 0) throw new Error("panel deps not found");
text = text.replace("frameworkPluginsTabs, frameworkSyncTab", "frameworkPluginsTabs, frameworkExtensionsTabs, frameworkSyncTab");

fs.writeFileSync(shellPath, text);
console.log("patched extensionsHost + tabs");
