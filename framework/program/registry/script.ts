#!/usr/bin/env bun
/** 📜 `@semio-tech/program-registry` — single-source program registry codegen from workspace crates. */
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/index.ts";

//#region 🔖ProgramRegistryEntry
export type ProgramHostMetadata = {
  readonly landingAppId: string;
  readonly hostAppId: string;
};

export type ProgramRegistryEntry = {
  readonly programId: string;
  readonly cratePath: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  readonly host?: ProgramHostMetadata;
};

function findProgramCargoFiles(root: string): string[] {
  const out: string[] = [];
  function walk(dir: string) {
    for (const name of readdirSync(dir)) {
      // 🚫 Skip build/vendor noise and any dot-directory (e.g. `.claude/worktrees/…`), which used to
      // leak duplicate registry rows for every crate that also exists inside a worktree checkout.
      if (name.startsWith(".") || name === "node_modules" || name === "generated" || name === "target") continue;
      const path = join(dir, name);
      let st: ReturnType<typeof statSync>;
      try {
        st = statSync(path);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        walk(path);
      } else if (name === "Cargo.toml" && !path.includes("/framework/program/rs/")) {
        const isProgramCrate = path.endsWith("/program/rs/Cargo.toml");
        const isModuleCrate = /\/module\/[^/]+\/rs\/Cargo\.toml$/.test(path);
        if (isProgramCrate || isModuleCrate) {
          out.push(path);
        }
      }
    }
  }
  walk(root);
  return out.sort();
}

function parseProgramCargo(manifestPath: string, repoRoot: string): ProgramRegistryEntry {
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
  const hostBlock = semioBlock?.join("\n").match(/^host\s*=\s*\{([^}]*)\}/m)?.[1];
  const landingAppId = hostBlock?.match(/landing\s*=\s*"([^"]+)"/)?.[1];
  const hostAppId = hostBlock?.match(/studio\s*=\s*"([^"]+)"/)?.[1];
  const host = landingAppId && hostAppId ? { landingAppId, hostAppId } : undefined;
  return { programId: componentPackage, cratePath, packageName, wasmOut, contributes, consumes, ...(host ? { host } : {}) };
}

//#region 🔖PlaygroundEntry
/** @emoji 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
 * program crate. `app` optionally scopes the row to one playground variant of a multi-app crate (unset
 * ⇒ every variant of the crate). Mirrors the TS discriminated union emitted for consumers as
 * `PlaygroundAssetSpec` (see `emitPlaygroundsTypeScript`). */
export type AssetSpecRow = {
  readonly kind: "tile-proxy" | "static-dir" | "mesh-collection";
  readonly route: string;
  readonly app?: string;
  readonly upstream?: string;
  readonly cache?: string;
  readonly root?: string;
  readonly roots?: readonly string[];
  readonly placeholder?: string;
  readonly filterFromExamples?: boolean;
};

/** @emoji 🎮 One `[[package.metadata.semio.playground]]` row scoped to its owning program crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly programId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** @emoji 🏷️ Shell brand id (see `framework/product/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  readonly examples: readonly string[];
  /** @emoji 🔌 Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** @emoji 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
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

function parseTomlBoolField(block: string, key: string): boolean {
  return new RegExp(`^${key}\\s*=\\s*true\\s*$`, "m").test(block);
}

function parsePlaygroundBlock(block: string, programId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  const engines = parseTomlStringArray(block, "engines");
  return { variant, programId, cratePath, app, brand, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, examples: [], engines, assets: [] };
}

/** @emoji 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate manifest. */
function parseAssetsForCrate(manifestPath: string): AssetSpecRow[] {
  if (!existsSync(manifestPath)) return [];
  const text = readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.assets]]");
  const rows: AssetSpecRow[] = [];
  for (const blockLines of blocks) {
    const block = blockLines.join("\n");
    const kind = block.match(/^kind\s*=\s*"([^"]+)"/m)?.[1] as AssetSpecRow["kind"] | undefined;
    const route = block.match(/^route\s*=\s*"([^"]+)"/m)?.[1];
    if (!kind || !route) {
      continue;
    }
    const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
    const upstream = block.match(/^upstream\s*=\s*"([^"]+)"/m)?.[1];
    const cache = block.match(/^cache\s*=\s*"([^"]+)"/m)?.[1];
    const root = block.match(/^root\s*=\s*"([^"]+)"/m)?.[1];
    const roots = parseTomlStringArray(block, "roots");
    const placeholder = block.match(/^placeholder\s*=\s*"([^"]+)"/m)?.[1];
    const filterFromExamples = parseTomlBoolField(block, "filter_from_examples");
    rows.push({
      kind,
      route,
      ...(app ? { app } : {}),
      ...(upstream ? { upstream } : {}),
      ...(cache ? { cache } : {}),
      ...(root ? { root } : {}),
      ...(roots.length ? { roots } : {}),
      ...(placeholder ? { placeholder } : {}),
      ...(filterFromExamples ? { filterFromExamples: true } : {}),
    });
  }
  return rows;
}

/**
 * @emoji 🖼️ Example ids for one playground row: tries the crate's own `example/` dir (stripping a
 * trailing `/rs` and `/plugin`), then — for multi-app crates where the playground `variant` diverges
 * from the shared crate path (e.g. `puzzle/program/rs` hosting `puzzle2d`/`puzzle3d`) — the sibling
 * module directory named after the variant's `programId`-stripped suffix (`puzzle2d` - `puzzle` = `2d`
 * → `puzzle/2d/example`). Mirrors `ui/tui/rs`'s `discover_examples_for_playground` byte-for-byte.
 */
function discoverExamplesForPlayground(repoRoot: string, cratePath: string, programId: string, variant: string): string[] {
  const idsIn = (dir: string): string[] => {
    if (!existsSync(dir)) return [];
    const ids = readdirSync(dir)
      .filter((name) => name.endsWith(".json"))
      .map((name) => name.split(".")[0]);
    return [...new Set(ids)].sort();
  };
  const trimmed = cratePath.endsWith("/rs") ? cratePath.slice(0, -"/rs".length) : cratePath;
  for (const base of [trimmed, trimmed.endsWith("/plugin") ? trimmed.slice(0, -"/plugin".length) : trimmed]) {
    const dir = join(repoRoot, base, "example");
    if (existsSync(dir)) return idsIn(dir);
  }
  if (variant.startsWith(programId) && variant.length > programId.length) {
    const suffix = variant.slice(programId.length);
    const techRoot = trimmed.split("/")[0];
    const dir = join(repoRoot, techRoot, suffix, "example");
    if (existsSync(dir)) return idsIn(dir);
  }
  return [];
}

function parsePlaygroundsForCrate(manifestPath: string, programId: string, cratePath: string): PlaygroundEntry[] {
  const text = readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]");
  const entries: PlaygroundEntry[] = [];
  for (const block of blocks) {
    const entry = parsePlaygroundBlock(block.join("\n"), programId, cratePath);
    if (entry) entries.push(entry);
  }
  return entries;
}

/** @emoji 🕹️ Scans every program/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GenerateProgramRegistryOptions = {}): PlaygroundEntry[] {
  const entries = generateProgramRegistry(repoRoot, options);
  const playgrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath);
    for (const playground of parsePlaygroundsForCrate(manifestPath, entry.programId, entry.cratePath)) {
      const assets = crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app);
      playgrounds.push({ ...playground, examples: discoverExamplesForPlayground(repoRoot, entry.cratePath, entry.programId, playground.variant), assets });
    }
  }
  for (let i = 0; i < playgrounds.length; i++) {
    const row = playgrounds[i];
    if (!row.brand || row.examples.length > 0) continue;
    const donor = playgrounds.find((other) => other !== row && other.cratePath === row.cratePath && other.app === row.app && other.examples.length > 0);
    if (donor) playgrounds[i] = { ...row, examples: donor.examples, engines: row.engines.length > 0 ? row.engines : donor.engines };
  }
  playgrounds.sort((a, b) => a.variant.localeCompare(b.variant));
  return playgrounds;
}
//#endregion

export type GenerateProgramRegistryOptions = {
  readonly filterPlaygroundPlugin?: string;
};

/** @emoji 🎯 Resolves a playground variant/alias or bare program id to its wasm registry program id. */
export function resolveRegistryPluginIdForFilter(pluginFilter: string, repoRoot = getWorkspaceRoot()): string {
  for (const manifestPath of findProgramCargoFiles(repoRoot)) {
    const text = readFileSync(manifestPath, "utf8");
    const componentPackage = text.match(/\[package\.metadata\.component\][\s\S]*?^package = "semio:([^"]+)"/m)?.[1];
    if (!componentPackage) continue;
    for (const block of tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]")) {
      const body = block.join("\n");
      const variant = body.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
      if (!variant) continue;
      const aliases = parseTomlStringArray(body, "aliases");
      if (variant === pluginFilter || aliases.includes(pluginFilter)) return componentPackage;
    }
  }
  return pluginFilter;
}

function pluginEntryHasHost(programId: string, repoRoot: string): boolean {
  for (const manifestPath of findProgramCargoFiles(repoRoot)) {
    const entry = tryParsePluginCargo(manifestPath, repoRoot);
    if (entry?.programId === programId) return entry.host !== undefined;
  }
  return false;
}

/** @emoji 🏠 True when the filter resolves to a program crate that declares `[package.metadata.semio].host`. */
export function isSpaceProgramFilter(pluginFilter?: string, repoRoot = getWorkspaceRoot()): boolean {
  if (!pluginFilter) return true;
  return pluginEntryHasHost(resolveRegistryPluginIdForFilter(pluginFilter, repoRoot), repoRoot);
}

/**
 * 🎯 Resolves a raw playground filter (a variant id like "puzzle5d", or an already-bare crate
 * programId like "note") to the set of crate programIds that must be built for one dev session: the
 * target crate itself plus every crate whose declared `contributes` intersects the target crate
 * `consumes` (per `[package.metadata.semio]` in each crate Cargo.toml — no more registry-id
 * indirection through framework/core/js).
 */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string): readonly string[] {
  const repoRoot = getWorkspaceRoot();
  const allEntries = generateProgramRegistry(repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const variantRow = playgrounds.find((p) => p.variant === filterPlaygroundPlugin);
  const targetPluginId = variantRow?.programId ?? filterPlaygroundPlugin;
  const targetEntry = allEntries.find((e) => e.programId === targetPluginId);
  const ids = new Set<string>([targetPluginId]);
  if (targetEntry) {
    for (const entry of allEntries) {
      if (entry.programId === targetPluginId) continue;
      if (entry.contributes.some((topic) => targetEntry.consumes.includes(topic))) ids.add(entry.programId);
    }
  }
  return [...ids];
}

function findPluginCargoPathsForIds(repoRoot: string, programIds: readonly string[]): string[] {
  const idSet = new Set(programIds);
  return findProgramCargoFiles(repoRoot).filter((path) => {
    const entry = tryParsePluginCargo(path, repoRoot);
    return entry !== undefined && idSet.has(entry.programId);
  });
}

function tryParsePluginCargo(manifestPath: string, repoRoot: string): ProgramRegistryEntry | undefined {
  try {
    return parseProgramCargo(manifestPath, repoRoot);
  } catch {
    return undefined;
  }
}

export function generateProgramRegistry(repoRoot = getWorkspaceRoot(), options: GenerateProgramRegistryOptions = {}): ProgramRegistryEntry[] {
  const filterPlaygroundPlugin = options.filterPlaygroundPlugin;
  const filterIds = filterPlaygroundPlugin && !isSpaceProgramFilter(filterPlaygroundPlugin) ? resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin) : undefined;
  const manifestPaths = filterIds ? findPluginCargoPathsForIds(repoRoot, filterIds) : findProgramCargoFiles(repoRoot);
  const entries: ProgramRegistryEntry[] = [];
  for (const path of manifestPaths) {
    const entry = tryParsePluginCargo(path, repoRoot);
    if (entry) entries.push(entry);
  }
  entries.sort((a, b) => a.programId.localeCompare(b.programId));
  return entries;
}

function emitTypeScript(entries: ProgramRegistryEntry[]): string {
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `\t{ programId: ${JSON.stringify(entry.programId)}, landingAppId: ${JSON.stringify(entry.host!.landingAppId)}, hostAppId: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const rows = entries
    .map((entry) => {
      const host = entry.host ? `, host: { landingAppId: ${JSON.stringify(entry.host.landingAppId)}, hostAppId: ${JSON.stringify(entry.host.hostAppId)} }` : "";
      return `\t{ programId: ${JSON.stringify(entry.programId)}, cratePath: ${JSON.stringify(entry.cratePath)}, wasmOut: ${JSON.stringify(entry.wasmOut)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)}${host} },`;
    })
    .join("\n");
  return `/** @generated by framework/program/registry/script.ts — do not edit. */
export type ProgramHostMetadata = {
\treadonly landingAppId: string;
\treadonly hostAppId: string;
};

export type ProgramHostConfig = ProgramHostMetadata & {
\treadonly programId: string;
};

export type ProgramBuildTarget = {
\treadonly programId: string;
\treadonly cratePath: string;
\treadonly wasmOut: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
\treadonly host?: ProgramHostMetadata;
};

export const PLUGIN_HOST_CONFIGS: readonly ProgramHostConfig[] = [
${hostRows}
];

export const PROGRAM_BUILD_TARGETS: readonly ProgramBuildTarget[] = [
${rows}
];

export const PROGRAM_TARGETS = PROGRAM_BUILD_TARGETS.map((target) => ({
\tprogramId: target.programId,
\tmoduleUrl: \`/program-modules/\${target.programId}/\${target.wasmOut.replace(/\\.wasm$/, ".js")}\`,
}));

export const programModuleUrl = (programId: string, fileName: string) =>
\t\`/program-modules/\${programId}/\${fileName.replace(/\\.wasm$/, ".js")}\`;
`;
}

function emitAssetSpecTypeScript(asset: AssetSpecRow): string {
  const fields = [`kind: ${JSON.stringify(asset.kind)}`, `route: ${JSON.stringify(asset.route)}`];
  if (asset.upstream !== undefined) fields.push(`upstream: ${JSON.stringify(asset.upstream)}`);
  if (asset.cache !== undefined) fields.push(`cache: ${JSON.stringify(asset.cache)}`);
  if (asset.root !== undefined) fields.push(`root: ${JSON.stringify(asset.root)}`);
  if (asset.roots !== undefined) fields.push(`roots: ${JSON.stringify(asset.roots)}`);
  if (asset.placeholder !== undefined) fields.push(`placeholder: ${JSON.stringify(asset.placeholder)}`);
  if (asset.filterFromExamples) fields.push(`filterFromExamples: true`);
  return `{ ${fields.join(", ")} }`;
}

function emitPlaygroundsTypeScript(playgrounds: PlaygroundEntry[]): string {
  const rows = playgrounds
    .map((entry) => {
      const app = entry.app !== undefined ? `, app: ${JSON.stringify(entry.app)}` : "";
      const brand = entry.brand !== undefined ? `, brand: ${JSON.stringify(entry.brand)}` : "";
      const assets = entry.assets.map(emitAssetSpecTypeScript).join(", ");
      return `\t{ variant: ${JSON.stringify(entry.variant)}, programId: ${JSON.stringify(entry.programId)}, cratePath: ${JSON.stringify(entry.cratePath)}${app}${brand}, aliases: ${JSON.stringify(entry.aliases)}, ports: { react: ${entry.ports.react}, wgpu: ${entry.ports.wgpu} }, examples: ${JSON.stringify(entry.examples)}, engines: ${JSON.stringify(entry.engines)}, assets: [${assets}] },`;
    })
    .join("\n");
  return `/** @generated by framework/program/registry/script.ts — do not edit. */
export type PlaygroundAssetSpec =
\t| { readonly kind: "tile-proxy"; readonly route: string; readonly upstream: string; readonly cache: string }
\t| { readonly kind: "static-dir"; readonly route: string; readonly root: string }
\t| { readonly kind: "mesh-collection"; readonly route: string; readonly roots: readonly string[]; readonly placeholder: string; readonly filterFromExamples?: boolean };

export type PlaygroundBuildTarget = {
\treadonly variant: string;
\treadonly programId: string;
\treadonly cratePath: string;
\treadonly app?: string;
\treadonly brand?: string;
\treadonly aliases: readonly string[];
\treadonly ports: { readonly react: number; readonly wgpu: number };
\treadonly examples: readonly string[];
\treadonly engines: readonly string[];
\treadonly assets: readonly PlaygroundAssetSpec[];
};

export const PLAYGROUND_BUILD_TARGETS: readonly PlaygroundBuildTarget[] = [
${rows}
];
`;
}

//#region 🎮PlaygroundSession
export type PlaygroundSessionPlugin = {
  readonly programId: string;
  readonly moduleUrl: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
};

export type PlaygroundSession = {
  readonly variant: string;
  readonly registryPluginId: string;
  readonly defaultAppId?: string;
  readonly studioMode: boolean;
  readonly host?: ProgramHostMetadata;
  readonly plugins: readonly PlaygroundSessionPlugin[];
};

/** @emoji 🎮 Builds the pre-expanded program list and host metadata for one playground launch. */
export function buildPlaygroundSession(variant: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const studioMode = isSpaceProgramFilter(variant, repoRoot);
  const registryPluginId = resolveRegistryPluginIdForFilter(variant, repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const playground = playgrounds.find((entry) => entry.variant === variant || entry.aliases.includes(variant));
  const entries = generateProgramRegistry(repoRoot, studioMode ? {} : { filterPlaygroundPlugin: registryPluginId });
  const host = entries.find((entry) => entry.programId === registryPluginId)?.host;
  return {
    variant,
    registryPluginId,
    defaultAppId: playground?.app,
    studioMode,
    ...(host ? { host } : {}),
    plugins: entries.map((entry) => ({
      programId: entry.programId,
      moduleUrl: `/program-modules/${entry.programId}/${entry.wasmOut.replace(/\.wasm$/, ".js")}`,
      contributes: entry.contributes,
      consumes: entry.consumes,
    })),
  };
}

function emitSessionTypeScript(session: PlaygroundSession): string {
  const host = session.host ? `{ landingAppId: ${JSON.stringify(session.host.landingAppId)}, hostAppId: ${JSON.stringify(session.host.hostAppId)} }` : "undefined";
  const defaultAppId = session.defaultAppId !== undefined ? JSON.stringify(session.defaultAppId) : "undefined";
  const pluginRows = session.plugins
    .map((entry) => `\t{ programId: ${JSON.stringify(entry.programId)}, moduleUrl: ${JSON.stringify(entry.moduleUrl)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)} },`)
    .join("\n");
  return `/** @generated by framework/program/registry/script.ts — do not edit. */
export type PlaygroundSessionPlugin = {
\treadonly programId: string;
\treadonly moduleUrl: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
};

export type PlaygroundSession = {
\treadonly variant: string;
\treadonly registryPluginId: string;
\treadonly defaultAppId?: string;
\treadonly studioMode: boolean;
\treadonly host?: { readonly landingAppId: string; readonly hostAppId: string };
\treadonly plugins: readonly PlaygroundSessionPlugin[];
};

export const PLAYGROUND_SESSION: PlaygroundSession = {
\tvariant: ${JSON.stringify(session.variant)},
\tregistryPluginId: ${JSON.stringify(session.registryPluginId)},
\tdefaultAppId: ${defaultAppId},
\tstudioMode: ${session.studioMode},
\thost: ${host},
\tplugins: [
${pluginRows}
\t],
};
`;
}

function emitRustHosts(entries: ProgramRegistryEntry[], playgrounds: PlaygroundEntry[]): string {
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `    ProgramHostConfig { program_id: ${JSON.stringify(entry.programId)}, landing_app_id: ${JSON.stringify(entry.host!.landingAppId)}, host_app_id: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const variantRows = playgrounds.map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.programId)}),`).join("\n");
  const aliasRows = playgrounds.flatMap((entry) => entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.programId)}),`)).join("\n");
  const variantAppRows = playgrounds.filter((entry) => entry.app).map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.app)}),`).join("\n");
  const aliasAppRows = playgrounds.flatMap((entry) => entry.app ? entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.app)}),`) : []).join("\n");
  return `// @generated by framework/program/registry/script.ts — do not edit.

pub struct ProgramHostConfig {
    pub program_id: &'static str,
    pub landing_app_id: &'static str,
    pub host_app_id: &'static str,
}

pub const PLUGIN_HOST_CONFIGS: &[ProgramHostConfig] = &[
${hostRows}
];

const PLAYGROUND_VARIANT_REGISTRY_IDS: &[(&str, &str)] = &[
${variantRows}
${aliasRows}
];

const PLAYGROUND_VARIANT_APP_IDS: &[(&str, &str)] = &[
${variantAppRows}
${aliasAppRows}
];

pub fn resolve_registry_program_id(plugin_filter: &str) -> &str {
    for (variant, program_id) in PLAYGROUND_VARIANT_REGISTRY_IDS {
        if *variant == plugin_filter {
            return program_id;
        }
    }
    plugin_filter
}

pub fn resolve_playground_app_id(plugin_filter: &str) -> Option<&'static str> {
    PLAYGROUND_VARIANT_APP_IDS.iter().find_map(|(variant, app_id)| (*variant == plugin_filter).then_some(*app_id))
}

pub fn resolve_program_host_config(plugin_filter: &str) -> Option<&'static ProgramHostConfig> {
    let registry_id = resolve_registry_program_id(plugin_filter);
    PLUGIN_HOST_CONFIGS.iter().find(|entry| entry.program_id == registry_id)
}

pub fn is_space_mode(plugin_filter: &str) -> bool {
    resolve_program_host_config(plugin_filter).is_some()
}
`;
}

/** @emoji 💾 Writes the per-launch playground session artifact consumed by os/dev and wgpu boot. */
export function writePlaygroundSession(variant: string, outPath: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const session = buildPlaygroundSession(variant, repoRoot);
  mkdirSync(dirname(outPath), { recursive: true });
  writeFileSync(outPath, emitSessionTypeScript(session));
  return session;
}
//#endregion 🎮PlaygroundSession

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

/** @emoji 🧪 Verifies that representative standalone and studio launches expand to complete sessions. */
function validatePlaygroundSessions(repoRoot: string): string[] {
  const errors: string[] = [];
  const standalone = buildPlaygroundSession("playbook", repoRoot);
  const standalonePluginIds = standalone.plugins.map((entry) => entry.programId).sort();
  if (standalone.registryPluginId !== "playbook" || standalone.studioMode || standalonePluginIds.join(",") !== "playbook,playbook-module-procedural") {
    errors.push(`playbook session resolved unexpectedly (${JSON.stringify({ registryPluginId: standalone.registryPluginId, studioMode: standalone.studioMode, programIds: standalonePluginIds })})`);
  }
  const studio = buildPlaygroundSession("s", repoRoot);
  if (!studio.studioMode || studio.host?.landingAppId !== "home" || studio.host.hostAppId !== "studio" || studio.plugins.length <= 10) {
    errors.push(`studio session resolved unexpectedly (${JSON.stringify({ studioMode: studio.studioMode, host: studio.host, pluginCount: studio.plugins.length })})`);
  }
  if (!isSpaceProgramFilter("s", repoRoot) || isSpaceProgramFilter("puzzle3d", repoRoot)) {
    errors.push("studio filter metadata does not distinguish host and standalone playgrounds");
  }
  return errors;
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generateProgramRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "generated");
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, "programs.json"), `${JSON.stringify(entries, null, 2)}\n`);
    writeFileSync(join(outDir, "programs.ts"), emitTypeScript(entries));
    writeFileSync(join(outDir, "playgrounds.json"), `${JSON.stringify(playgrounds, null, 2)}\n`);
    writeFileSync(join(outDir, "playgrounds.ts"), emitPlaygroundsTypeScript(playgrounds));
    writeFileSync(join(outDir, "hosts.rs"), emitRustHosts(entries, playgrounds));
    console.log(`program registry catalog refreshed (${entries.length} program crates, ${playgrounds.length} playgrounds) -> ${outDir}`);
  }
}

/** @emoji 🔎 Renders the catalog in memory and byte-compares it against `generated/*` — never writes (a lint/verify step must never let the auto-commit daemon land regenerated files). */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generateProgramRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "generated");
    const expected: Record<string, string> = {
      "programs.json": `${JSON.stringify(entries, null, 2)}\n`,
      "programs.ts": emitTypeScript(entries),
      "playgrounds.json": `${JSON.stringify(playgrounds, null, 2)}\n`,
      "playgrounds.ts": emitPlaygroundsTypeScript(playgrounds),
      "hosts.rs": emitRustHosts(entries, playgrounds),
    };
    const stale = Object.entries(expected)
      .filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content)
      .map(([name]) => name);
    if (stale.length > 0) {
      console.error(`program registry catalog is stale: ${stale.map((name) => `generated/${name}`).join(", ")}`);
      console.error("run `bun nx run @semio-tech/program-registry:generate` to refresh.");
      process.exit(1);
    }
    const violations = [...validatePlaygroundRegistry(playgrounds), ...validatePlaygroundSessions(repoRoot)];
    if (violations.length > 0) {
      console.error("program registry catalog has playground validation errors:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    console.log(`program registry catalog is fresh (${entries.length} program crates, ${playgrounds.length} playgrounds).`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("check", CheckScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
}
