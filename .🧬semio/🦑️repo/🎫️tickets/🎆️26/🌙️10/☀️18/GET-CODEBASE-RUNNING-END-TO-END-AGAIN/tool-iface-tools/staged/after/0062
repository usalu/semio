import { join } from "node:path";
import { TOOL_REGISTRY_V1, type ToolEntryV1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";

/** 🔌️ What the environment of a tool run depends on besides the registry. */
export type ToolEnvironmentContextV1 = Readonly<{ platform?: NodeJS.Platform; root: string; cacheDirectory?: (...segments: string[]) => string }>;

const flag = (platform: NodeJS.Platform): "iu" | "u" => platform === "win32" ? "iu" : "u";

/** 🧷️ Matches an environment variable name against a pattern whose `*` stands for any characters; Windows names ignore case. */
export function toolEnvironmentNameMatchesV1(pattern: string, name: string, platform: NodeJS.Platform = process.platform): boolean {
  return new RegExp(`^${pattern.replace(/[.+^$()|\\[\]{}?]/gu, "\\$&").replace(/\*/gu, ".*")}$`, flag(platform)).test(name);
}

/** 🌿️ Builds the environment of one tool run: the ambient variables filtered by the registry (IDE and debugger injections are always dropped, `allowlist` tools keep only the base and their own), then the tool's fixed values and cache directories, then what the caller passed explicitly. */
export function toolEnvironmentV1(entry: ToolEntryV1, ambient: Readonly<Record<string, string | undefined>>, explicit: Readonly<Record<string, string | undefined>>, context: ToolEnvironmentContextV1, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): Record<string, string> {
  const platform = context.platform ?? process.platform, matches = (patterns: readonly string[], name: string): boolean => patterns.some(pattern => toolEnvironmentNameMatchesV1(pattern, name, platform));
  const denied = [...registry.environment.deny, ...entry.environment.deny], kept = entry.environment.mode === "allowlist" ? [...registry.environment.base, ...entry.environment.pass] : undefined;
  const result: Record<string, string> = {};
  for (const [name, value] of Object.entries(ambient)) {
    if (value === undefined || matches(denied, name)) continue;
    if (kept !== undefined && !matches(kept, name)) continue;
    result[name] = value;
  }
  for (const [name, value] of Object.entries(entry.environment.set)) result[name] = value;
  const cache = context.cacheDirectory ?? ((...segments: string[]) => join(context.root, ".🧬semio", "🦑️repo", "⚡️cache", ...segments));
  for (const [name, segments] of Object.entries(entry.environment.cacheDirectories ?? {})) result[name] ??= cache(...segments);
  for (const [name, value] of Object.entries(explicit)) if (value === undefined) delete result[name]; else result[name] = value;
  result.SEMIO_TOOL_RUN = entry.id;
  return result;
}
