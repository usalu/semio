#!/usr/bin/env bun
/** 📜 `@semio-tech/plugin-registry` — single-source plugin registry codegen from workspace crates. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
import { contributorPluginIdsFor, resolvePluginRegistryId } from "../../core/js/index.ts";

//#region 🔖PluginRegistryEntry
export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly cratePath: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
};

function findPluginCargoFiles(root: string): string[] {
  const out: string[] = [];
  function walk(dir: string) {
    for (const name of readdirSync(dir)) {
      if (name === "node_modules" || name === "generated" || name === "target" || name === ".git") continue;
      const path = join(dir, name);
      let st: ReturnType<typeof statSync>;
      try {
        st = statSync(path);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        walk(path);
      } else if (name === "Cargo.toml" && !path.includes("/framework/plugin/rs/")) {
        const isPluginCrate = path.endsWith("/plugin/rs/Cargo.toml");
        const isModuleCrate = /\/module\/[^/]+\/rs\/Cargo\.toml$/.test(path);
        if (isPluginCrate || isModuleCrate) {
          out.push(path);
        }
      }
    }
  }
  walk(root);
  return out.sort();
}

function parsePluginCargo(manifestPath: string, repoRoot: string): PluginRegistryEntry {
  const text = readFileSync(manifestPath, "utf8");
  const packageName = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`missing package name in ${manifestPath}`);
  const componentPackage = text.match(/\[package\.metadata\.component\][\s\S]*?^package = "semio:([^"]+)"/m)?.[1];
  if (!componentPackage) throw new Error(`missing [package.metadata.component].package in ${manifestPath}`);
  const cratePath = relative(repoRoot, dirname(manifestPath));
  const wasmOut = `${packageName.replace(/-/g, "_")}.wasm`;
  const semioBlock = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[package.metadata.semio]")[0];
  const contributes = semioBlock ? parseTomlStringArray(semioBlock.join("\n"), "contributes") : [];
  const consumes = semioBlock ? parseTomlStringArray(semioBlock.join("\n"), "consumes") : [];
  return { pluginId: componentPackage, cratePath, packageName, wasmOut, contributes, consumes };
}

//#region 🔖PlaygroundEntry
/** @emoji 🎮 One `[[package.metadata.semio.playground]]` row scoped to its owning plugin crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
};

function tomlBlocksAfterHeader(lines: readonly string[], headerTest: (line: string) => boolean): string[][] {
  const blocks: string[][] = [];
  for (let i = 0; i < lines.length; i++) {
    if (!headerTest(lines[i].trim())) continue;
    const body: string[] = [];
    for (let j = i + 1; j < lines.length; j++) {
      if (lines[j].trim().startsWith("[")) break;
      body.push(lines[j]);
    }
    blocks.push(body);
  }
  return blocks;
}

function parseTomlStringArray(block: string, key: string): string[] {
  const match = block.match(new RegExp(`^${key}\\s*=\\s*\\[([^\\]]*)\\]`, "m"));
  if (!match) return [];
  return [...match[1].matchAll(/"([^"]*)"/g)].map((m) => m[1]);
}

function parsePlaygroundBlock(block: string, pluginId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  return { variant, pluginId, cratePath, app, aliases, ports: { react: Number(react), wgpu: Number(wgpu) } };
}

function parsePlaygroundsForCrate(manifestPath: string, pluginId: string, cratePath: string): PlaygroundEntry[] {
  const text = readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]");
  const entries: PlaygroundEntry[] = [];
  for (const block of blocks) {
    const entry = parsePlaygroundBlock(block.join("\n"), pluginId, cratePath);
    if (entry) entries.push(entry);
    else console.warn(`[DEBUG] plugin registry catalog: skipping malformed playground entry in ${manifestPath}`);
  }
  return entries;
}

/** @emoji 🕹️ Scans every plugin/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PlaygroundEntry[] {
  const entries = generatePluginRegistry(repoRoot, options);
  const playgrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    playgrounds.push(...parsePlaygroundsForCrate(manifestPath, entry.pluginId, entry.cratePath));
  }
  playgrounds.sort((a, b) => a.variant.localeCompare(b.variant));
  return playgrounds;
}
//#endregion

export type GeneratePluginRegistryOptions = {
  readonly filterPlaygroundPlugin?: string;
};

export function isStudioPluginFilter(pluginFilter?: string): boolean {
  return !pluginFilter || pluginFilter === "s";
}

/** @emoji 🎯 Resolves wasm component ids to catalog for one playground dev session. */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string): readonly string[] {
  const registryId = resolvePluginRegistryId(filterPlaygroundPlugin);
  const ids = new Set<string>([registryId]);
  for (const extra of contributorPluginIdsFor(registryId)) ids.add(extra);
  return [...ids];
}

function findPluginCargoPathsForIds(repoRoot: string, pluginIds: readonly string[]): string[] {
  const paths: string[] = [];
  for (const pluginId of pluginIds) {
    const result = spawnSync("rg", ["-l", `package = "semio:${pluginId}"`, "--glob", "**/Cargo.toml", "-g", "!**/target/**", "-g", "!**/node_modules/**", "-g", "!**/framework/plugin/rs/**"], { cwd: repoRoot, encoding: "utf8" });
    const hit = (result.stdout ?? "").trim().split("\n").filter(Boolean)[0];
    if (hit) {
      paths.push(join(repoRoot, hit));
      continue;
    }
    console.warn(`[DEBUG] plugin registry catalog: no crate found for semio:${pluginId}`);
  }
  return paths;
}

function tryParsePluginCargo(manifestPath: string, repoRoot: string): PluginRegistryEntry | undefined {
  try {
    return parsePluginCargo(manifestPath, repoRoot);
  } catch (error) {
    console.warn(`[DEBUG] plugin registry catalog: skipping ${manifestPath}: ${error instanceof Error ? error.message : String(error)}`);
    return undefined;
  }
}

export function generatePluginRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PluginRegistryEntry[] {
  const filterPlaygroundPlugin = options.filterPlaygroundPlugin;
  const filterIds = filterPlaygroundPlugin && !isStudioPluginFilter(filterPlaygroundPlugin) ? resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin) : undefined;
  const manifestPaths = filterIds ? findPluginCargoPathsForIds(repoRoot, filterIds) : findPluginCargoFiles(repoRoot);
  const entries: PluginRegistryEntry[] = [];
  for (const path of manifestPaths) {
    const entry = tryParsePluginCargo(path, repoRoot);
    if (entry) entries.push(entry);
  }
  entries.sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  return entries;
}

function emitTypeScript(entries: PluginRegistryEntry[]): string {
  const rows = entries
    .map(
      (entry) =>
        `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, cratePath: ${JSON.stringify(entry.cratePath)}, wasmOut: ${JSON.stringify(entry.wasmOut)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)} },`,
    )
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PluginBuildTarget = {
\treadonly pluginId: string;
\treadonly cratePath: string;
\treadonly wasmOut: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
};

export const PLUGIN_BUILD_TARGETS: readonly PluginBuildTarget[] = [
${rows}
];

export const PLUGIN_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
\tpluginId: target.pluginId,
\tmoduleUrl: \`/plugin-modules/\${target.pluginId}/\${target.wasmOut.replace(/\\.wasm$/, ".js")}\`,
}));

export const pluginModuleUrl = (pluginId: string, fileName: string) =>
\t\`/plugin-modules/\${pluginId}/\${fileName.replace(/\\.wasm$/, ".js")}\`;
`;
}

function emitPlaygroundsTypeScript(playgrounds: PlaygroundEntry[]): string {
  const rows = playgrounds
    .map((entry) => {
      const app = entry.app !== undefined ? `, app: ${JSON.stringify(entry.app)}` : "";
      return `\t{ variant: ${JSON.stringify(entry.variant)}, pluginId: ${JSON.stringify(entry.pluginId)}, cratePath: ${JSON.stringify(entry.cratePath)}${app}, aliases: ${JSON.stringify(entry.aliases)}, ports: { react: ${entry.ports.react}, wgpu: ${entry.ports.wgpu} } },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PlaygroundBuildTarget = {
\treadonly variant: string;
\treadonly pluginId: string;
\treadonly cratePath: string;
\treadonly app?: string;
\treadonly aliases: readonly string[];
\treadonly ports: { readonly react: number; readonly wgpu: number };
};

export const PLAYGROUND_BUILD_TARGETS: readonly PlaygroundBuildTarget[] = [
${rows}
];
`;
}

/** @emoji 🚦 Cross-checks the flattened playground catalog for global uniqueness and multi-app crate discipline; returns human-readable violations. */
function validatePlaygroundRegistry(playgrounds: PlaygroundEntry[]): string[] {
  const errors: string[] = [];
  const variantOwners = new Map<string, string>();
  const aliasOwners = new Map<string, string>();
  const portOwners = new Map<string, string>();
  const entriesByCrate = new Map<string, PlaygroundEntry[]>();
  for (const entry of playgrounds) {
    if (variantOwners.has(entry.variant)) {
      errors.push(`duplicate playground variant "${entry.variant}" (${variantOwners.get(entry.variant)} and ${entry.cratePath})`);
    } else {
      variantOwners.set(entry.variant, entry.cratePath);
    }
    for (const alias of entry.aliases) {
      if (aliasOwners.has(alias)) {
        errors.push(`duplicate playground alias "${alias}" (variants "${aliasOwners.get(alias)}" and "${entry.variant}")`);
      } else {
        aliasOwners.set(alias, entry.variant);
      }
    }
    const portKey = `${entry.ports.react}:${entry.ports.wgpu}`;
    if (portOwners.has(portKey)) {
      errors.push(`duplicate playground ports react=${entry.ports.react}/wgpu=${entry.ports.wgpu} (variants "${portOwners.get(portKey)}" and "${entry.variant}")`);
    } else {
      portOwners.set(portKey, entry.variant);
    }
    entriesByCrate.set(entry.cratePath, [...(entriesByCrate.get(entry.cratePath) ?? []), entry]);
  }
  for (const group of entriesByCrate.values()) {
    if (group.length <= 1) continue;
    for (const entry of group) {
      if (!entry.app) errors.push(`playground variant "${entry.variant}" in ${entry.cratePath} must set "app" (crate declares ${group.length} playground entries)`);
    }
  }
  return errors;
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generatePluginRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "generated");
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, "plugins.json"), `${JSON.stringify(entries, null, 2)}\n`);
    writeFileSync(join(outDir, "plugins.ts"), emitTypeScript(entries));
    writeFileSync(join(outDir, "playgrounds.json"), `${JSON.stringify(playgrounds, null, 2)}\n`);
    writeFileSync(join(outDir, "playgrounds.ts"), emitPlaygroundsTypeScript(playgrounds));
    console.log(`plugin registry catalog refreshed (${entries.length} plugin crates, ${playgrounds.length} playgrounds) -> ${outDir}`);
  }
}

/** @emoji 🔎 Renders the catalog in memory and byte-compares it against `generated/*` — never writes (a lint/verify step must never let the auto-commit daemon land regenerated files). */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generatePluginRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "generated");
    const expected: Record<string, string> = {
      "plugins.json": `${JSON.stringify(entries, null, 2)}\n`,
      "plugins.ts": emitTypeScript(entries),
      "playgrounds.json": `${JSON.stringify(playgrounds, null, 2)}\n`,
      "playgrounds.ts": emitPlaygroundsTypeScript(playgrounds),
    };
    const stale = Object.entries(expected)
      .filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content)
      .map(([name]) => name);
    if (stale.length > 0) {
      console.error(`plugin registry catalog is stale: ${stale.map((name) => `generated/${name}`).join(", ")}`);
      console.error("run `bun nx run @semio-tech/plugin-registry:generate` to refresh.");
      process.exit(1);
    }
    const violations = validatePlaygroundRegistry(playgrounds);
    if (violations.length > 0) {
      console.error("plugin registry catalog has playground validation errors:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    console.log(`plugin registry catalog is fresh (${entries.length} plugin crates, ${playgrounds.length} playgrounds).`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("check", CheckScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
}
