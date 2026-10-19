/** 📚 Shared model for the cache-s executor: tracked ✏️s manifests and the workspace cache policy replica used by the census. */
import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
export const strip = (s: string) => s.replace(/️/gu, "");
export const POLICY = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const IGNORED = new Set(["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples"].map(strip));
const PROJECT = strip("📋️project.json");

export function manifests(): string[] {
  const git = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--", "✏️s"], { cwd: ROOT });
  return new TextDecoder().decode(git.stdout).split("\0").filter((f) => {
    if (!f || !existsSync(join(ROOT, f))) return false;
    const segs = f.split("/").map(strip);
    if (segs.includes("pkg")) return false;
    if (segs.slice(0, -1).some((s) => IGNORED.has(s))) return false;
    return segs[segs.length - 1] === PROJECT;
  }).sort();
}

const matchesCommand = (name: string, commands: string[]) => commands.some((c) => name === c || name.startsWith(c + "-"));
const matchesUncached = (name: string) => POLICY.uncachedExact.includes(name) || matchesCommand(name, POLICY.uncached);
const mutatingName = (name: string) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/u.test(name);
const liveName = (name: string) => /(?:^|-)(?:e2e|live)(?:-|$)/u.test(name);
export const cacheableFamily = (name: string) =>
  /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/u.test(name) ||
  /(?:^|-)contract(?:-|$)/u.test(name) ||
  (/^generator(?:-|$)/u.test(name) && !name.includes("generator-inputs"));
const verifyCommand = (t: any) => /(?:^|[\s"'])verify(?:[\s"']|$)/u.test(t?.options?.command ?? "");

export function effectiveCache(name: string, target: any): { cache: boolean; continuous: boolean } {
  const defaults = POLICY.targetDefaults?.[name] ?? {};
  const t = { ...defaults, ...target, ...(target.cache === false && target.inputs === undefined ? { inputs: ["default"] } : {}) };
  if (matchesCommand(name, POLICY.continuous) || t.continuous === true) return { cache: false, continuous: true };
  if (t.cache === false) return { cache: false, continuous: false };
  if (matchesUncached(name)) return { cache: false, continuous: false };
  if (POLICY.cachedExact.includes(name)) return { cache: true, continuous: false };
  if (mutatingName(name) || liveName(name)) return { cache: false, continuous: false };
  if (cacheableFamily(name) || verifyCommand(t)) return { cache: true, continuous: false };
  return { cache: t.cache !== false, continuous: false };
}

export const hasCommand = (t: any) => typeof t.command === "string" || typeof t.options?.command === "string" || !!t.options?.commands;
export const isNoop = (t: any) => t.executor === "nx:noop" || t.executor === "repo:noop";

export function classify(name: string, t: any): string {
  const eff = effectiveCache(name, t);
  if (t.continuous === true || eff.continuous) return "continuous";
  if (liveName(name)) return "test-against-live";
  if (matchesUncached(name)) return "side-effecting-or-diagnostic";
  if (mutatingName(name)) return "side-effecting";
  if (eff.cache) return "deterministic-cacheable";
  if (isNoop(t) || !hasCommand(t)) return "aggregate";
  if (cacheableFamily(name)) return "deterministic-but-cache-off";
  return "unclassified-cache-off";
}

export type ProjectFacts = { file: string; root: string; json: any; native: boolean; artifactTypeScript: boolean; rust: boolean };
export function facts(file: string, text: string): ProjectFacts {
  const json = JSON.parse(text);
  const root = dirname(file);
  const tags: string[] = json.tags ?? [];
  const native = existsSync(join(ROOT, root, "Cargo.toml"));
  return { file, root, json, native, artifactTypeScript: tags.includes("role:artifact") && tags.includes("language:typescript"), rust: native || tags.some((t) => /^lang(?:uage)?:rust$/u.test(t)) };
}
export const nativeName = (name: string) => /^(?:build|wasm|native|test|lint|check|verify)(?:-|$)/u.test(name);
