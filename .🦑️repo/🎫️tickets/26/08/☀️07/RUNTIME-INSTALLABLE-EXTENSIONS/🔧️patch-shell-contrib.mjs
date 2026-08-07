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

const contribJsonOld =
  "      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));";
const contribJsonNew = `      const disabledExtensionIds = new Set(extensionLedgerRef.current.filter((entry) => !entry.enabled).map((entry) => entry.extensionId));
      const contributionsJson = buildContributionsJson(
        loadedPlugins
          .filter((entry) => !disabledExtensionIds.has(entry.handle.pluginId))
          .map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })),
      );`;

if (!text.includes(contribJsonOld)) throw new Error("contributionsJson builder not found");
// Only replace the first occurrence (refreshUi path)
text = text.replace(contribJsonOld, contribJsonNew);

const idx = text.indexOf('[DEBUG] setContributions push skipped');
if (idx < 0) throw new Error("setContributions marker missing");
const blockStart = text.lastIndexOf("const pluginEntry = loadedPlugins.find", idx);
const blockEnd = text.indexOf("}\n", idx) + 2;
if (blockStart < 0 || blockEnd < 0) throw new Error("setContributions block bounds missing");
console.log("block", blockStart, blockEnd);
console.log(text.slice(blockStart, blockEnd));

const pushNew = `for (const pluginEntry of loadedPlugins) {
            if (!pluginEntry.manifest.apps?.length) continue;
            const isActive = pluginEntry.handle.pluginId === nextSession.pluginId;
            const instanceId = isActive ? nextSession.instanceId : contributorInstancesRef.current.get(pluginEntry.handle.pluginId);
            if (instanceId == null) continue;
            const controllerId = isActive
              ? nextSession.app.controllerId
              : ((pluginEntry.manifest.apps[0] as { controllerId?: string } | undefined)?.controllerId ?? nextSession.app.controllerId);
            try {
              const wire = encodeActionWire({ controllerId, action: "setContributions", args: { json: contributionsJson } });
              await pluginEntry.handle.handleAction(instanceId, wire, nextSession.viewState);
            } catch (error) {
              console.warn("[DEBUG] setContributions push skipped", pluginEntry.handle.pluginId, error instanceof Error ? error.message : String(error));
            }
          }
`;

text = text.slice(0, blockStart) + pushNew + text.slice(blockEnd);
fs.writeFileSync(shellPath, text);
console.log("patched contributions push");
