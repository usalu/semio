#!/usr/bin/env bun
/** 🔎️ R8: launch-registration resolver probe (continues Z1's `🐍️z1-launch-target-audit.ts` + `🐍️z1-launch-preview.ts`).
 * Checks `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` for: duplicate names, every `nx run <project>:<target>`
 * resolving to a declared or plugin-generated target, every compound member resolving to a configuration, and the
 * committed `launch.json` being byte-identical to a fresh render of the seed plus the generated catalog rows. */
import { lstatSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const OUT = join(ROOT, ".tmp-ticket/wp-r8/generated");
const SKIP = new Set(["node_modules", ".git", "dist", "temp", "storybook-static", "🤖️generated", "🎫️tickets", ".🧬semio", "target"]);

const projects = new Map<string, Set<string>>();
function walk(directory: string): void {
  let entries: string[];
  try { entries = readdirSync(directory); } catch { return; }
  if (entries.includes("📋️project.json")) {
    try {
      const parsed = JSON.parse(readFileSync(join(directory, "📋️project.json"), "utf8")) as { name?: string; targets?: Record<string, unknown> };
      if (parsed.name) projects.set(parsed.name, new Set([...(projects.get(parsed.name) ?? []), ...Object.keys(parsed.targets ?? {})]));
    } catch { }
  }
  for (const entry of entries) {
    if (SKIP.has(entry) || entry.startsWith(".")) continue;
    let directoryEntry = false;
    try { directoryEntry = lstatSync(join(directory, entry)).isDirectory(); } catch { continue; }
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

type Row = { name?: string; command?: string; configurations?: readonly string[] };
type LaunchFile = { configurations?: readonly Row[]; compounds?: readonly Row[] };
const report: string[] = [`[walk] project manifests discovered: ${projects.size}`];
const audit = (label: string, file: LaunchFile): { unresolved: number; duplicates: number; brokenCompounds: number } => {
  const placeholders = (file.configurations ?? []).filter((row) => typeof row === "string");
  const rows = (file.configurations ?? []).filter((row): row is Row => typeof row === "object" && row !== null);
  const counts = new Map<string, number>();
  for (const row of rows) counts.set(String(row.name), (counts.get(String(row.name)) ?? 0) + 1);
  const duplicates = [...counts].filter(([, count]) => count > 1);
  const missing: string[] = [];
  const seen = new Set<string>();
  for (const row of rows) {
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
  const names = new Set(rows.map((row) => String(row.name)));
  const broken: string[] = [];
  for (const compound of label === "launch.seed.jsonc" ? [] : file.compounds ?? []) {
    for (const member of compound.configurations ?? []) if (!names.has(member)) broken.push(`${compound.name}  ->  missing member ${member}`);
  }
  report.push(`[${label}] configurations=${rows.length} generatedPlaceholders=${placeholders.length} compounds=${(file.compounds ?? []).length} pairs=${seen.size} unresolved=${missing.length} duplicates=${duplicates.length} brokenCompoundMembers=${broken.length}`);
  for (const line of missing) report.push(`  UNRESOLVED ${line}`);
  for (const [name, count] of duplicates) report.push(`  DUPLICATE x${count}: ${name}`);
  for (const line of broken) report.push(`  COMPOUND ${line}`);
  return { unresolved: missing.length, duplicates: duplicates.length, brokenCompounds: broken.length };
};

const launchText = readFileSync(join(ROOT, ".vscode/launch.json"), "utf8");
const seedText = readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8");
const launch = audit("launch.json", Bun.JSONC.parse(launchText) as LaunchFile);
const seed = audit("launch.seed.jsonc", Bun.JSONC.parse(seedText) as LaunchFile);

const { renderCatalogFiles } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`);
const { generateLaunchJson } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const { playgrounds: rendered, componentLaunchers } = renderCatalogFiles(ROOT);
const next = generateLaunchJson(ROOT, rendered, componentLaunchers);
writeFileSync(join(OUT, "launch-rendered.json"), next);
report.push(`[render] committed ${launchText.split("\n").length} lines, fresh render ${next.split("\n").length} lines, identical: ${launchText === next}`);
const fresh = audit("fresh-render", Bun.JSONC.parse(next) as LaunchFile);
console.log(report.join("\n"));
process.exit(launch.unresolved + launch.duplicates + launch.brokenCompounds + seed.duplicates + seed.brokenCompounds + fresh.unresolved + fresh.duplicates + fresh.brokenCompounds === 0 && launchText === next ? 0 : 1);
