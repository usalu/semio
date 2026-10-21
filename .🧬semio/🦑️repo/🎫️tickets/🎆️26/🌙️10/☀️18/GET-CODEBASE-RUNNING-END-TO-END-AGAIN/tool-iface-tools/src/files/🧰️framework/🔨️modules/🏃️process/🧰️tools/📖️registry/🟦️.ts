import registryDocument from "../🔣️.json" with { type: "json" };
import registrySchema from "../🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { resolveHostCapacityLanguageV1 } from "../../🏗️build/🌡️host-capacity/🟦️.ts";

/** 🗣️ A text in every declared language. */
export type ToolTextV1 = Readonly<{ en: string; de: string }>;
/** 🌐️ A language the registry texts exist in. */
export type ToolLanguageV1 = keyof ToolTextV1;
/** 🧭️ A platform a tool may be provisioned for. */
export type ToolPlatformV1 = "win32" | "darwin" | "linux";
/** 🔢️ A place a version or a row table is read from: a JSON pointer, a TOML key, a text file or the rows of a JSON array. */
export type ToolSourceV1 =
  | Readonly<{ file: string; format: "json"; pointer: string }>
  | Readonly<{ file: string; format: "toml"; table: string; key: string }>
  | Readonly<{ file: string; format: "text" }>
  | Readonly<{ file: string; format: "rows"; pointer: string; where: Readonly<Record<string, string | number | boolean>>; field: string }>;
/** 🔐️ Where the per-platform digests of a pinned tool are read from, and which row fields carry them: `platform` and `architecture`, or one `host` field such as `darwin-arm64`. */
export type ToolDigestSourceV1 = Readonly<{ file: string; pointer: string; where: Readonly<Record<string, string | number | boolean>>; platform?: string; architecture?: string; host?: string; sha256: string; name: string; bytes?: string; extras?: readonly string[] }>;
/** 📌️ How a tool is pinned. */
export type ToolPinSpecV1 = Readonly<{ state: "pinned" | "constrained" | "floating" | "system" | "internal"; version?: ToolSourceV1; minimum?: ToolSourceV1; digests?: ToolDigestSourceV1; gap?: ToolTextV1 }>;
/** 🧰️ How a tool is provisioned. */
export type ToolProvisionV1 = Readonly<{ kind: "internal" | "bootstrap-asset" | "release-archive" | "package" | "toolchain-file" | "system"; command: string; installer?: string; hints?: Readonly<Partial<Record<ToolPlatformV1, string>>> }>;
/** 🔎️ One place a tool executable may be found, tried in declared order. */
export type ToolLocatorV1 =
  | Readonly<{ kind: "environment"; name: string }>
  | Readonly<{ kind: "runtime" }>
  | Readonly<{ kind: "provisioned"; partition: string; segments: readonly string[]; executable: string }>
  | Readonly<{ kind: "package-script"; package: string; script: string; via: string; prefix?: readonly string[] }>
  | Readonly<{ kind: "script"; path: string; via: string; prefix?: readonly string[] }>
  | Readonly<{ kind: "path"; directories: readonly string[]; names?: readonly string[] }>;
/** 🌿️ The environment policy of one tool. */
export type ToolEnvironmentSpecV1 = Readonly<{ mode: "inherit" | "allowlist"; pass: readonly string[]; deny: readonly string[]; set: Readonly<Record<string, string>>; cacheDirectories?: Readonly<Record<string, readonly string[]>> }>;
/** 🎟️ How a tool takes part in machine capacity admission. */
export type ToolCapacitySpecV1 = Readonly<{ mode: "none" | "inherit" | "admit"; operation?: string; kind?: "test" | "check" | "generate" | "build" | "script" | "service" }>;
/** 🧩️ One registered external tool. */
export type ToolEntryV1 = Readonly<{
  id: string; kind: string; summary: ToolTextV1; capabilities: readonly string[]; commands: readonly string[]; platforms: readonly ToolPlatformV1[];
  pin: ToolPinSpecV1; provision: ToolProvisionV1; locate: readonly ToolLocatorV1[]; probe?: Readonly<{ arguments: readonly string[]; pattern: string }>;
  adapter: string; adapterScope: readonly string[]; allowedCallers: readonly string[]; environment: ToolEnvironmentSpecV1; capacity: ToolCapacitySpecV1;
}>;
/** 🗂️ The registry document. */
export type ToolRegistryV1 = Readonly<{
  version: 1; languages: readonly ToolLanguageV1[]; partitions: Readonly<Record<string, string>>; packageRoots: readonly string[]; testCallers?: readonly string[]; funnels?: readonly Readonly<{ path: string; marker: string }>[];
  environment: Readonly<{ base: readonly string[]; deny: readonly string[] }>;
  scan: Readonly<{ callees: readonly Readonly<{ name: string; language: "ts" | "rust" | "go" | "python"; shape: "command" | "array" | "shell-string" | "template" | "optionsCmd" }>[]; shellWrappers: readonly string[]; selfExpressions: readonly string[]; testSegments: readonly string[]; ignoredSegments: readonly string[]; extensions: Readonly<Record<"ts" | "rust" | "go" | "python" | "shell", readonly string[]>>; declaredNames: readonly string[] }>;
  messages: Readonly<Record<ToolMessageKeyV1, ToolTextV1>>;
  tools: readonly ToolEntryV1[];
}>;
/** 🧱️ The names of every registry message. */
export type ToolMessageKeyV1 = "unknownTool" | "toolMissing" | "callerRefused" | "versionMismatch" | "cancelled" | "failed" | "execUsage" | "listHeader" | "verifyOk" | "verifyProblem" | "admitting" | "running" | "finished" | "guardRefused" | "guardWarned";

const freeze = <T>(value: T): T => {
  if (typeof value === "object" && value !== null && !Object.isFrozen(value)) { Object.freeze(value); for (const child of Object.values(value)) freeze(child); }
  return value;
};

/** 📖️ Validates a registry document against its schema and its cross references (unique ids, one owner per command, `via` tools exist) and freezes it. */
export function readToolRegistryV1(document: unknown): ToolRegistryV1 {
  const errors = validateJsonSchemaSubset(registrySchema, document);
  if (errors.length) throw new Error(`Tool registry invalid: ${errors.slice(0, 5).join("; ")}`);
  const registry = document as ToolRegistryV1, ids = new Set<string>(), owners = new Map<string, string>();
  for (const tool of registry.tools) {
    if (ids.has(tool.id)) throw new Error(`Tool registry invalid: duplicate tool ${tool.id}`);
    ids.add(tool.id);
    for (const command of tool.commands) {
      const owner = owners.get(command);
      if (owner !== undefined) throw new Error(`Tool registry invalid: command ${command} belongs to ${owner} and ${tool.id}`);
      owners.set(command, tool.id);
    }
  }
  for (const tool of registry.tools) for (const locator of tool.locate) if ("via" in locator && !ids.has(locator.via)) throw new Error(`Tool registry invalid: ${tool.id} locates through unknown tool ${locator.via}`);
  return freeze(registry);
}

/** 🧵️ The repository registry, read once. */
export const TOOL_REGISTRY_V1: ToolRegistryV1 = readToolRegistryV1(registryDocument);

/** 🧮️ Answers the entry of one tool id, or undefined. */
export function findToolV1(id: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): ToolEntryV1 | undefined {
  return registry.tools.find(tool => tool.id === id);
}

/** 🧯️ Answers the tool that owns an executable name (`bunx`, `cargo`, `python3`); directories and `.exe`/`.cmd` suffixes are ignored. */
export function toolForCommandV1(command: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): ToolEntryV1 | undefined {
  const name = command.replace(/\\/gu, "/").split("/").pop()!.replace(/\.(?:exe|cmd|bat)$/iu, "");
  return registry.tools.find(tool => tool.commands.includes(name));
}

/** 🪪️ Picks the message language from the environment (SEMIO_LANGUAGE, then the locale), restricted to the registry languages. */
export function toolLanguageV1(environment: Readonly<Record<string, string | undefined>> = process.env): ToolLanguageV1 {
  return resolveHostCapacityLanguageV1(environment);
}

/** 🔖️ Renders one registry message in a language, filling `{placeholder}` values. */
export function toolMessageV1(key: ToolMessageKeyV1, values: Readonly<Record<string, string | number>> = {}, language: ToolLanguageV1 = toolLanguageV1(), registry: ToolRegistryV1 = TOOL_REGISTRY_V1): string {
  return registry.messages[key][language].replace(/\{(\w+)\}/gu, (whole, name: string) => name in values ? String(values[name]) : whole);
}

/** 🧷️ Matches a repository-relative path against a glob where `**` crosses directories and `*` stays inside one segment. */
export function toolGlobMatchesV1(pattern: string, path: string): boolean {
  const source = pattern.replace(/[.+^$()|\\[\]{}?]/gu, "\\$&").replace(/\*\*\/|\*\*|\*/gu, token => token === "**/" ? "(?:.*/)?" : token === "**" ? ".*" : "[^/]*");
  return new RegExp(`^${source}$`, "u").test(path);
}

/** 🧾️ Tells whether a repository-relative path may call a tool through the registry (its callers, or a test path). */
export function toolCallerAllowedV1(entry: ToolEntryV1, path: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): boolean {
  return [...entry.allowedCallers, ...registry.testCallers ?? []].some(pattern => toolGlobMatchesV1(pattern, path));
}

/** 🛡️ Tells whether a repository-relative path may spawn a tool's executables directly: the tool adapter scope and the tools module itself. */
export function toolRawSpawnAllowedV1(entry: ToolEntryV1, path: string): boolean {
  return path === entry.adapter || path === entry.provision.installer || entry.adapterScope.some(pattern => toolGlobMatchesV1(pattern, path)) || toolGlobMatchesV1("🧰️framework/🔨️modules/🏃️process/🧰️tools/**", path);
}
