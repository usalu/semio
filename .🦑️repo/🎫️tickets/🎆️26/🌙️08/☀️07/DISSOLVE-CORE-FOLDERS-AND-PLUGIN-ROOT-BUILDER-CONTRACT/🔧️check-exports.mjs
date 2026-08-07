import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const platformDir = readdirSync(MODULES).find((n) => n.includes("platform"));
const t = readFileSync(join(MODULES, platformDir, "🟦️component.ts"), "utf8");
for (const name of [
  "createMemoryStoragePort",
  "emptyPaneState",
  "emptySkeleton",
  "emptyUiState",
  "DockLayoutStore",
  "DockUiStateStore",
  "WindowPaneStateStore",
  "DockSkeleton",
]) {
  const re = new RegExp(`(export )?(function|const|class|type|interface) ${name}\\b|export \\{[^}]*\\b${name}\\b`);
  const idx = t.split("\n").findIndex((l) => l.includes(name) && /export |function |interface |type |class /.test(l));
  console.log(name, idx >= 0 ? `line ${idx + 1}: ${t.split("\n")[idx].slice(0, 120)}` : "NOT FOUND as export-ish");
}

// Check kernel exports used by tests
const kernelDir = readdirSync(MODULES).find((n) => n.includes("kernel"));
const k = readFileSync(join(MODULES, kernelDir, "🟦️component.ts"), "utf8");
for (const name of [
  "createDevPluginSource",
  "createExtensionSource",
  "multiplexPluginSources",
  "pluginWorkerUrl",
  "resolvePlaygroundBoot",
  "resolvePluginHostConfig",
  "resolvePluginRegistryId",
  "acquirePluginModule",
  "evictPluginModule",
  "createLeasePool",
]) {
  const idx = k.split("\n").findIndex((l) => l.includes(`export function ${name}`) || l.includes(`export const ${name}`) || l.includes(`export async function ${name}`));
  console.log("kernel", name, idx >= 0 ? idx + 1 : "MISSING");
}
