
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
if (text.includes("const installExtension = useCallback")) {
  console.log("already patched");
  process.exit(0);
}

// Import ChromePanels extensions
{
  const oldImp = 'import { createFrameworkDisplayPanelTabs, createFrameworkPluginsPanelTabs, createFrameworkSettingsPanelTabs, type DisplayHostApi, PluginRecoveryPanel, type PluginsHostApi, type PluginsPanelEntry, type SettingsHostApi, ShellRouteNotFoundPage, useNamedLayoutHost } from "../ChromePanels/\u{1F7E2}\uFE0Fcomponent.tsx";';
  // find actual import line
  const m = text.match(/import \{ createFrameworkDisplayPanelTabs[\s\S]*?from "\.\.\/ChromePanels\/[^"]+";/);
  if (!m) throw new Error("chrome import not found");
  const newImp = m[0]
    .replace("createFrameworkPluginsPanelTabs", "createFrameworkExtensionsPanelTabs, createFrameworkPluginsPanelTabs")
    .replace("type DisplayHostApi,", "type DisplayHostApi, type ExtensionsHostApi, type ExtensionsPanelEntry,");
  text = text.replace(m[0], newImp);
  console.log("patched chrome import");
}

// Import EXTENSION_TARGETS
{
  const pluginRuntimeImp = text.match(/import \{ type PluginWasmHandle \} from "\.\.\/PluginRuntime\/[^"]+";/);
  if (!pluginRuntimeImp) throw new Error("plugin runtime import not found");
  const extImp = 'import { EXTENSION_TARGETS } from "../../../../\u{1F50C}\uFE0Fplugin/\u{1F4E6}\uFE0Fpackages/\u{1F7E2}\uFE0Ftypescript/\u{1F4D6}\uFE0Fregistry/\u{1F916}\uFE0Fgenerated/\u{1F7E2}\uFE0Fplugins.ts";\n';
  // Use relative path discovered at runtime
  const gen = find((p, n) => p.includes("registry") && p.includes("generated") && n.includes("plugins") && n.endsWith(".ts") && !n.includes("json"));
  const rel = path.relative(path.dirname(shellPath), gen).split(path.sep).join("/");
  const line = 'import { EXTENSION_TARGETS } from "' + rel + '";\n';
  if (!text.includes("EXTENSION_TARGETS")) {
    text = text.replace(pluginRuntimeImp[0], pluginRuntimeImp[0] + "\n" + line);
    console.log("added EXTENSION_TARGETS import", rel);
  }
}

fs.writeFileSync(shellPath, text);
console.log("phase1 done", shellPath);
