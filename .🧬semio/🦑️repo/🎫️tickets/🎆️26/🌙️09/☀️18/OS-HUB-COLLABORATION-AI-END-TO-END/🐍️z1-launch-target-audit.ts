#!/usr/bin/env bun
/** 🔎️ Z1: resolves every `bun nx run <project>:<target>` in `.vscode/launch.json` against the union of
 * statically declared `📋️project.json` targets, the root script's `register(...)` command targets and
 * the per-playground targets the Nx plugin generates (`📚️library/🟨️.mjs playgroundPreparationTargets`),
 * so launch rows pointing at a missing target surface without running the Nx graph. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", ".git", "dist", "temp", "storybook-static", "🤖️generated", "🎫️tickets", ".🧬semio"]);

const projects = new Map<string, Set<string>>();
function walk(directory: string): void {
  let entries: string[];
  try { entries = readdirSync(directory); } catch { return; }
  if (entries.includes("📋️project.json")) {
    try {
      const parsed = JSON.parse(readFileSync(join(directory, "📋️project.json"), "utf8")) as { name?: string; targets?: Record<string, unknown> };
      if (parsed.name) projects.set(parsed.name, new Set(Object.keys(parsed.targets ?? {})));
    } catch { /* unreadable project manifest */ }
  }
  for (const entry of entries) {
    if (SKIP.has(entry) || entry.startsWith(".")) continue;
    let directoryEntry = false;
    try { directoryEntry = statSync(join(directory, entry)).isDirectory(); } catch { continue; }
    if (directoryEntry) walk(join(directory, entry));
  }
}
walk(ROOT);

const rootTargets = projects.get("workspace") ?? new Set<string>();
for (const match of readFileSync(join(ROOT, "📜️script.ts"), "utf8").matchAll(/\.register\(\s*"([^"]+)"/g)) rootTargets.add(match[1]!);
projects.set("workspace", rootTargets);

const { generatePlaygroundRegistry } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`);
const playgrounds = generatePlaygroundRegistry(ROOT) as readonly { variant: string }[];
const osDev = projects.get("@semio-tech/framework-os-dev") ?? new Set<string>();
for (const { variant } of playgrounds) {
  for (const profile of ["dev", "release"]) {
    for (const renderer of ["react", "wgpu"]) for (const command of ["serve", "dev", "activate", "prepare"]) osDev.add(`${command}-${variant}-${renderer}-${profile}`);
    for (const operation of ["run", "smoke", "prepare"]) osDev.add(`${operation}-${variant}-native-${profile}`);
  }
  osDev.add(`build-${variant}-react-release`);
}
projects.set("@semio-tech/framework-os-dev", osDev);

const launch = Bun.JSONC.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")) as { configurations: readonly { name?: string; command?: string }[] };
const missing: string[] = [];
const seen = new Set<string>();
for (const row of launch.configurations) {
  for (const match of String(row.command ?? "").matchAll(/nx run ([^\s]+):([A-Za-z0-9_.-]+)/g)) {
    const [, project, target] = match as unknown as [string, string, string];
    const key = `${project}:${target}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const targets = projects.get(project);
    if (!targets) missing.push(`${row.name}  ->  UNKNOWN PROJECT ${project}`);
    else if (!targets.has(target)) missing.push(`${row.name}  ->  ${project} has no target ${target}`);
  }
}
console.log(`projects discovered: ${projects.size}, distinct project:target pairs referenced: ${seen.size}, unresolved: ${missing.length}`);
for (const line of missing) console.log(`  ${line}`);
