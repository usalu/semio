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

const chromePath = find((p, n) => p.includes("ChromePanels") && n.endsWith(".tsx") && n.includes("component"));
let text = fs.readFileSync(chromePath, "utf8");

if (text.includes("createFrameworkExtensionsPanelTabs")) {
  console.log("ChromePanels already patched");
  process.exit(0);
}

const headerOld = ` * chrome: the Display panel (window-kind palette + named-layout tree), the Settings panel (general/
 * driver/theme/keybindings trees), and the Plugins panel (install/reload/uninstall tree), plus the
 * small standalone route-not-found and plugin-recovery affordances they share the chrome namespace
 * with. Each panel's \`*HostApi\` type is the read/write surface \`ShellHost\` implements to drive it.
 */`;

const headerNew = ` * chrome: the Display panel (window-kind palette + named-layout tree), the Settings panel (general/
 * driver/theme/keybindings trees), the Plugins panel (install/reload/uninstall tree), and the
 * Extensions panel (install-from-URL / enable / uninstall, grouped by host), plus the small
 * standalone route-not-found and plugin-recovery affordances they share the chrome namespace with.
 * Each panel's \`*HostApi\` type is the read/write surface \`ShellHost\` implements to drive it.
 */`;

if (!text.includes(headerOld)) throw new Error("header not found");
text = text.replace(headerOld, headerNew);

const extensionsPanel = `
//#region ExtensionsPanel
/** 🧩️ One extension as the extensions settings panel wants to render it — grouped by the host
 * plugin id it \`extends\` (or \`"unscoped"\` when the catalog row has no host). */
export type ExtensionsPanelEntry = {
  readonly extensionId: string;
  readonly label: string;
  readonly version?: string;
  readonly extendsHost: string;
  readonly enabled: boolean;
  readonly status: PluginPanelStatus;
};

export type ExtensionsHostApi = {
  readonly extensions: readonly ExtensionsPanelEntry[];
  readonly installFromUrl: (sourceUri: string) => void;
  readonly uninstall: (extensionId: string) => void;
  readonly setEnabled: (extensionId: string, enabled: boolean) => void;
};

const FRAMEWORK_SETTINGS_EXTENSIONS_TAB_ID = "framework.settings.extensions";

function buildExtensionsTree(host: ExtensionsHostApi): TreePanelConfig {
  const byHost = new Map<string, ExtensionsPanelEntry[]>();
  for (const entry of host.extensions) {
    const list = byHost.get(entry.extendsHost) ?? [];
    list.push(entry);
    byHost.set(entry.extendsHost, list);
  }
  const sections: TreeDataSection[] = [
    {
      id: "framework.settings.extensions.install",
      label: uiDataLabel("Install"),
      defaultOpen: true,
      items: [
        {
          id: "framework.settings.extensions.install.url",
          label: uiDataLabel("From URL"),
          control: (
            <Button
              id="framework.settings.extensions.install.url"
              size="sm"
              text={uiDataLabel("Install from URL")}
              onClick={() => {
                const sourceUri = typeof window !== "undefined" ? window.prompt("Extension package URL") : null;
                if (sourceUri?.trim()) host.installFromUrl(sourceUri.trim());
              }}
            />
          ),
        },
      ],
    },
    ...[...byHost.entries()]
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([hostId, entries]) => ({
        id: \`framework.settings.extensions.host.\${hostId}\`,
        label: \`\${uiDataLabel("Extends")}: \${hostId}\`,
        defaultOpen: true,
        items: [...entries]
          .sort((a, b) => a.extensionId.localeCompare(b.extensionId))
          .map((entry) => ({
            id: \`framework.settings.extensions.\${entry.extensionId}\`,
            label: \`\${entry.label}\${entry.version ? \` · \${entry.version}\` : ""} · \${entry.enabled ? uiDataLabel("enabled") : uiDataLabel("disabled")}\`,
            loading: entry.status === "installing" || entry.status === "reloading",
            control: (
              <div className="flex items-center gap-1">
                <Button
                  id={\`framework.settings.extensions.\${entry.extensionId}.enable\`}
                  size="sm"
                  text={entry.enabled ? uiDataLabel("Disable") : uiDataLabel("Enable")}
                  disabled={entry.status !== "loaded" && entry.status !== "available"}
                  onClick={() => host.setEnabled(entry.extensionId, !entry.enabled)}
                />
                <Button
                  id={\`framework.settings.extensions.\${entry.extensionId}.uninstall\`}
                  size="sm"
                  text={uiDataLabel("Uninstall")}
                  disabled={entry.status === "installing" || entry.status === "reloading"}
                  onClick={() => host.uninstall(entry.extensionId)}
                />
              </div>
            ),
          })),
      })),
  ];
  return { sections };
}

export function createFrameworkExtensionsPanelTabs(getHost: () => ExtensionsHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_SETTINGS_EXTENSIONS_TAB_ID,
      icon: shellTabIcon("plug"),
      name: uiDataLabel("Extensions"),
      order: -95,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildExtensionsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: uiDataLabel("Extensions unavailable") }] }] };
        },
      },
    }),
  ];
}
//#endregion ExtensionsPanel
`;

const endMarker = `//#endregion PluginsPanel
//#endregion 🔖️os-chrome-panels
`;
if (!text.includes(endMarker)) throw new Error("end marker not found");
text = text.replace(endMarker, `//#endregion PluginsPanel
${extensionsPanel}//#endregion 🔖️os-chrome-panels
`);

fs.writeFileSync(chromePath, text);
console.log("patched ChromePanels", chromePath);
